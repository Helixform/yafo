use std::fs::File;
use std::io::{Read, Seek, SeekFrom, Write};

use super::Driver;
use crate::types::DataChunk;

/// A driver implementation for processing the file-backed data.
pub struct FileDriver {
    file: File,
    buf: Vec<u8>,
    reader_cursor: usize,
    writer_remains: usize,
}

impl FileDriver {
    /// Creates a new `FileDriver` with the specified [`std::fs::File`].
    pub fn new(file: File) -> Self {
        let buf = vec![0; 32 * 1024 * 1024];
        let reader_cursor = buf.len();
        Self {
            file,
            buf,
            reader_cursor,
            writer_remains: 0,
        }
    }

    /// Returns the underlying [`std::fs::File`].
    pub fn file(&self) -> &File {
        &self.file
    }
}

impl Driver for FileDriver {
    type Error = std::io::Error;

    fn pull_chunk(&mut self) -> Result<Option<DataChunk>, Self::Error> {
        let mut reader_cursor = self.reader_cursor;
        if reader_cursor >= self.buf.len() {
            let read = self.file.read(&mut self.buf)?;
            if read == 0 {
                // No more data to process.
                return Ok(None);
            }

            // Seek the file cursor back because we need to write the
            // processed chunks back.
            self.file.seek(SeekFrom::Current(-(read as i64)))?;

            // Reset padding bytes in the buffer.
            self.buf[read..].fill(0);

            reader_cursor = 0;
            self.writer_remains = read;
        }

        let bytes = <[u8; 8]>::try_from(&self.buf[reader_cursor..(reader_cursor + 8)])
            .expect("the slice can form an array");
        self.reader_cursor = reader_cursor + 8;
        return Ok(Some(DataChunk::from(bytes)));
    }

    fn push_chunk(&mut self, chunk: DataChunk) -> Result<(), Self::Error> {
        let writer_remains = self.writer_remains;
        if writer_remains == 0 {
            // Don't write padding bytes, just ignore them.
            return Ok(());
        }

        // Overwrite the last chunk in the buffer.
        let reader_cursor = self.reader_cursor;
        let write_offset = reader_cursor - 8;
        self.buf[write_offset..(write_offset + 8)].copy_from_slice(chunk.as_slice());

        if reader_cursor >= self.buf.len() {
            // All the chunks in the buffer has been processed. Write
            // them back to the file.
            self.file.write(&self.buf[0..writer_remains])?;
            self.writer_remains = 0;
        }

        Ok(())
    }
}
