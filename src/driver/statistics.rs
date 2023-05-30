use super::Driver;
use crate::types::DataChunk;

/// Trait implemented by reporters for [`Statistics`] to
/// receive the statistics data.
pub trait Reporter {
    /// Called when bytes of the given length are processed.
    fn bytes_processed(&mut self, n: usize);
}

/// A driver wrapper that reports the data statistics for
/// its underlying driver.
pub struct Statistics<D, R> {
    driver: D,
    reporter: R,
}

impl<D, R> Statistics<D, R> {
    /// Creates a new `Statistics` with the specified driver and reporter.
    pub fn new(driver: D, reporter: R) -> Self {
        Self { driver, reporter }
    }

    /// Returns the reference of the underlying driver.
    pub fn driver(&self) -> &D {
        &self.driver
    }

    /// Consumes the driver and returns the inner driver.
    pub fn into_driver(self) -> D {
        self.driver
    }

    /// Returns the reference of the reporter.
    pub fn reporter(&self) -> &R {
        &self.reporter
    }
}

impl<D, R> Driver for Statistics<D, R>
where
    D: Driver,
    R: Reporter,
{
    type Error = <D as Driver>::Error;

    fn pull_chunk(&mut self) -> Result<Option<DataChunk>, Self::Error> {
        self.driver.pull_chunk()
    }

    fn push_chunk(&mut self, chunk: DataChunk) -> Result<(), Self::Error> {
        let len = chunk.as_slice().len();

        let res = self.driver.push_chunk(chunk);
        if res.is_ok() {
            self.reporter.bytes_processed(len);
        }
        res
    }
}
