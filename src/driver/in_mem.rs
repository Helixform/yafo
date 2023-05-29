use std::convert::Infallible;

use super::Driver;
use crate::types::DataChunk;

/// A driver implementation for processing the in-memory data.
pub struct InMemoryDriver {
    buf: Vec<u8>,
    reader_cursor: usize,
    writer_cursor: usize,
}

impl InMemoryDriver {
    /// Creates a new `InMemoryDriver` with the specified buffer.
    pub fn new(buf: Vec<u8>) -> Self {
        Self {
            buf,
            reader_cursor: 0,
            writer_cursor: 0,
        }
    }

    /// Extracts a slice containing the entire contents of the buffer.
    pub fn as_slice(&self) -> &[u8] {
        &self.buf
    }

    /// Consumes the driver and returns inner buffer.
    pub fn into_vec(self) -> Vec<u8> {
        self.buf
    }
}

impl Driver for InMemoryDriver {
    type Error = Infallible;

    fn pull_chunk(&mut self) -> Result<Option<DataChunk>, Self::Error> {
        let cursor = self.reader_cursor;
        let remain_len = self.buf.len() - cursor;
        if remain_len == 0 {
            return Ok(None);
        }

        let mut bytes = [0u8; 8];

        let read_len = remain_len.min(8);
        bytes[0..read_len].copy_from_slice(&self.buf[cursor..(cursor + read_len)]);

        self.reader_cursor = cursor + read_len;

        Ok(Some(DataChunk::from(bytes)))
    }

    fn push_chunk(&mut self, chunk: DataChunk) -> Result<(), Self::Error> {
        let cursor = self.writer_cursor;
        let remain_len = self.buf.len() - cursor;

        let write_len = remain_len.min(8);
        self.buf[cursor..(cursor + write_len)].copy_from_slice(&chunk.as_slice()[0..write_len]);

        self.writer_cursor = cursor + write_len;

        Ok(())
    }
}
