use std::io::{BufReader, BufWriter, Read, Result, Write};
use std::path::Path;

use crate::io::file::duplex_file;
use crate::types::{DataChunk, CHUNK_SIZE};
use crate::Cipher;

/// Trait implemented by progress reporters for receiving
/// the statistics while processing the data.
pub trait ProgressReporter {
    /// Called when bytes of the given length are processed.
    ///
    /// Argument `n` is the length of bytes processed in this round.
    /// Note that it's not the length of total processed bytes.
    ///
    /// An optional total size is presented when available.
    fn bytes_processed(&mut self, n: usize, total: Option<usize>);
}

/// A no-op [`ProgressReporter`] as a placeholder type.
#[derive(Clone, Copy)]
pub struct NopReporter;

impl ProgressReporter for NopReporter {
    fn bytes_processed(&mut self, _n: usize, _total: Option<usize>) {}
}

/// A configurable pipeline to process data with some specified cipher.
pub struct Pipeline<R> {
    buffered: bool,
    progress_reporter: R,
}

impl Pipeline<NopReporter> {
    /// Creates a new pipeline with default configurations.
    pub fn new() -> Pipeline<NopReporter> {
        Self {
            buffered: false,
            progress_reporter: NopReporter,
        }
    }
}

impl<R> Pipeline<R> {
    /// Returns a buffered version of the current pipeline.
    pub fn with_buffer(mut self) -> Pipeline<R> {
        self.buffered = true;
        self
    }

    /// Replaces the progress reporter for the current pipeline.
    pub fn with_progress_reporter<NR>(self, reporter: NR) -> Pipeline<NR>
    where
        NR: ProgressReporter,
    {
        Pipeline {
            buffered: self.buffered,
            progress_reporter: reporter,
        }
    }
}

impl<R> Pipeline<R>
where
    R: ProgressReporter,
{
    /// Consumes the pipeline and processes the data by given
    /// input & output stream and cipher.
    pub fn process<I: Read, O: Write, C: Cipher>(
        self,
        input: I,
        output: O,
        cipher: C,
        total_len: Option<usize>,
    ) -> Result<()> {
        struct WithProgress<W: Write, R: ProgressReporter> {
            inner: W,
            reporter: R,
            total_len: Option<usize>,
        }

        impl<W: Write, R: ProgressReporter> Write for WithProgress<W, R> {
            fn write(&mut self, buf: &[u8]) -> Result<usize> {
                let wr_len = self.inner.write(buf)?;
                self.reporter.bytes_processed(wr_len, self.total_len);
                Ok(wr_len)
            }

            fn flush(&mut self) -> Result<()> {
                self.inner.flush()
            }
        }

        let output_with_progress = WithProgress {
            inner: output,
            reporter: self.progress_reporter,
            total_len,
        };

        if self.buffered {
            // TODO: make this tweakable for library users.
            const BUF_SIZE: usize = 1024 * 1024 * 4;
            let buf_input = BufReader::with_capacity(BUF_SIZE, input);
            let buf_output = BufWriter::with_capacity(BUF_SIZE, output_with_progress);
            Self::process_inner(buf_input, buf_output, cipher)
        } else {
            Self::process_inner(input, output_with_progress, cipher)
        }
    }

    pub fn process_file<P, C>(self, path: P, cipher: C) -> Result<()>
    where
        P: AsRef<Path>,
        C: Cipher,
    {
        let (rd, wr) = duplex_file(path)?;
        let file_len = rd.file_len()?;
        self.process(rd, wr, cipher, Some(file_len as usize))
    }

    fn process_inner<I: Read, O: Write, C: Cipher>(
        mut input: I,
        mut output: O,
        mut cipher: C,
    ) -> Result<()> {
        let mut chunk = DataChunk::default();
        loop {
            // FIXME: handle the case where the read chunk is less than 8
            // bytes before reaching EOF.
            let rd_len = input.read(chunk.as_mut_slice())?;
            if rd_len == 0 {
                return Ok(());
            } else if rd_len != CHUNK_SIZE {
                chunk.as_mut_slice()[rd_len..CHUNK_SIZE].fill(0);
            }

            cipher.process_chunk(&mut chunk);

            output.write_all(&chunk.as_ref()[0..rd_len])?
        }
    }
}

impl<R> Clone for Pipeline<R>
where
    R: Clone,
{
    fn clone(&self) -> Self {
        Self {
            buffered: self.buffered,
            progress_reporter: self.progress_reporter.clone(),
        }
    }
}
