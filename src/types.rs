use std::iter::Cloned;
use std::ops::{Index, IndexMut};
use std::slice::Iter;

/// Struct representing a fixed-length data chunk.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct DataChunk(pub [u8; 8]);

impl DataChunk {
    /// Extracts a slice containing the entire array.
    pub fn as_slice(&self) -> &[u8; 8] {
        &self.0
    }

    /// Extracts a mutable slice containing the entire array.
    pub fn as_mut_slice(&mut self) -> &mut [u8; 8] {
        &mut self.0
    }
}

impl From<[u8; 8]> for DataChunk {
    fn from(value: [u8; 8]) -> Self {
        Self(value)
    }
}

impl<'a> IntoIterator for &'a DataChunk {
    type Item = u8;
    type IntoIter = Cloned<Iter<'a, u8>>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter().cloned()
    }
}

impl<'a> Index<usize> for DataChunk {
    type Output = u8;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0.index(index)
    }
}

impl<'a> IndexMut<usize> for DataChunk {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.0.index_mut(index)
    }
}
