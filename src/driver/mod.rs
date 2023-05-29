mod file;
mod in_mem;

pub use file::FileDriver;
pub use in_mem::InMemoryDriver;

use crate::cipher::Cipher;
use crate::types::DataChunk;

/// Trait implemented by types that drive the encryption or decryption process.
///
/// ## Notes on timing
///
/// During the process, [`Driver::push_chunk`] method is guaranteed to be called
/// after a chunk is processed. And [`Driver::pull_chunk`] method will not be
/// called again before the returned chunk is processed and pushed.
pub trait Driver {
    type Error;

    /// Gets a chunk to process and advances the driver state.
    ///
    /// Returns [`None`] when there are no more chunks, indicating the whole
    /// process is finished.
    fn pull_chunk(&mut self) -> Result<Option<DataChunk>, Self::Error>;

    /// Consumes a processes chunk.
    fn push_chunk(&mut self, chunk: DataChunk) -> Result<(), Self::Error>;

    /// Process the data with the specified cipher.
    fn process(&mut self, cipher: &mut dyn Cipher) -> Result<(), Self::Error> {
        while let Some(mut chunk) = self.pull_chunk()? {
            cipher.process_chunk(&mut chunk);
            self.push_chunk(chunk)?;
        }
        Ok(())
    }
}
