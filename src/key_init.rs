use sha1::{Digest, Sha1};

use crate::types::DataChunk;

const MASK_BITS: [u8; 8] = [1, 2, 4, 8, 16, 32, 64, 128];

/// Types which can be initialized from key.
pub trait KeyInit: Sized {
    /// Creates new value with the given data chunk as key.
    fn with_key(key: DataChunk) -> Self;

    fn with_key_from<K>(key: K) -> Self
    where
        K: Into<DataChunk>,
    {
        Self::with_key(key.into())
    }

    /// Creates new value with the key derived from the given seed phrase.
    fn with_seed_phrase(phrase: &str) -> Self {
        let mut hasher = Sha1::new();

        hasher.update(phrase.as_bytes());
        let digest = hasher.finalize();

        let mut seed_chunk =
            <[u8; 8]>::try_from(&digest.as_slice()[0..8]).expect("the slice can form an array");
        for (byte, mask) in seed_chunk.iter_mut().zip(MASK_BITS.iter().cloned()) {
            *byte = *byte ^ mask;
        }

        Self::with_key(DataChunk::from(seed_chunk))
    }
}
