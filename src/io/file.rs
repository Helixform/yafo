use std::cell::Cell;
use std::fs::File;
use std::io::{Error as IoError, ErrorKind as IoErrorKind, Read, Result, Write};
use std::path::Path;
use std::rc::Rc;
use std::result::Result as StdResult;

/// Creates a duplex file that can be read and written simultaneously.
///
/// The bytes to be written should not be more than the bytes that
/// are read (i.e. no overlapping is allowed).
pub(crate) fn duplex_file<P>(path: P) -> StdResult<(DuplexFileReader, DuplexFileWriter), IoError>
where
    P: AsRef<Path>,
{
    let path = path.as_ref();
    let rd = File::options().read(true).open(path)?;
    let wr = File::options().write(true).open(path)?;

    let state = Rc::new(DuplexFileState {
        rd_cnt: Cell::new(0),
        wr_cnt: Cell::new(0),
    });

    let reader = DuplexFileReader {
        rd,
        state: Rc::clone(&state),
    };
    let writer = DuplexFileWriter { wr, state };
    Ok((reader, writer))
}

#[derive(Debug)]
pub(crate) struct DuplexFileReader {
    rd: File,
    state: Rc<DuplexFileState>,
}

#[derive(Debug)]
pub(crate) struct DuplexFileWriter {
    wr: File,
    state: Rc<DuplexFileState>,
}

#[derive(Debug)]
struct DuplexFileState {
    rd_cnt: Cell<usize>,
    wr_cnt: Cell<usize>,
}

impl DuplexFileReader {
    pub fn file_len(&self) -> Result<u64> {
        let metadata = self.rd.metadata()?;
        Ok(metadata.len())
    }
}

impl Read for DuplexFileReader {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        let rd_len = self.rd.read(buf)?;
        self.state.rd_cnt.set(self.state.rd_cnt.get() + rd_len);
        Ok(rd_len)
    }
}

impl Write for DuplexFileWriter {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        let wr_cnt = self.state.wr_cnt.get();
        let rd_cnt = self.state.rd_cnt.get();

        let total_wr = wr_cnt + buf.len();
        if total_wr > rd_cnt {
            return Err(IoError::from(IoErrorKind::Other));
        }

        let wr_len = self.wr.write(buf)?;
        self.state.wr_cnt.set(wr_cnt + wr_len);
        Ok(wr_len)
    }

    fn flush(&mut self) -> Result<()> {
        self.wr.flush()
    }
}
