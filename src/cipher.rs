use crate::types::DataChunk;

pub trait Cipher {
    fn process_chunk(&mut self, chunk: &mut DataChunk);
}

macro_rules! impl_state {
    ($($name:ident),*) => {
        $(impl_state!(@ $name);)*
    };

    (@ $name:ident) => {
        pub struct $name {
            current_key: DataChunk,
        }

        impl $crate::key_init::KeyInit for $name {
            fn with_key(key: DataChunk) -> Self {
                Self { current_key: key }
            }
        }
    };
}

impl_state!(EncryptState, DecryptState);

impl Cipher for EncryptState {
    fn process_chunk(&mut self, chunk: &mut DataChunk) {
        let c_sum = calculate_sum(chunk);
        let k_sum = calculate_sum(&self.current_key);

        for (chunk_byte, key_byte) in chunk.0.iter_mut().zip(&self.current_key) {
            let factor_a = (k_sum ^ key_byte) as u32;
            *chunk_byte = chunk_byte.rotate_left(factor_a) ^ key_byte;
        }

        rotate_key(&mut self.current_key, c_sum);
    }
}

impl Cipher for DecryptState {
    fn process_chunk(&mut self, chunk: &mut DataChunk) {
        let k_sum = calculate_sum(&self.current_key);

        for (chunk_byte, key_byte) in chunk.0.iter_mut().zip(&self.current_key) {
            let factor_a = (k_sum ^ key_byte) as u32;
            *chunk_byte = (*chunk_byte ^ key_byte).rotate_right(factor_a);
        }

        let c_sum = calculate_sum(chunk);
        rotate_key(&mut self.current_key, c_sum);
    }
}

#[inline(always)]
fn calculate_sum(chunk: &DataChunk) -> u8 {
    let mut sum = 0;
    for byte in chunk {
        sum ^= byte;
    }
    sum
}

#[inline(always)]
fn rotate_key(key: &mut DataChunk, mut sum: u8) {
    for byte in key.as_mut_slice() {
        *byte ^= sum;
        sum = sum.rotate_left(1);
    }

    // Rotate the chunk left by byte.
    key.0.rotate_left(1);
}

#[cfg(test)]
mod tests {
    extern crate test;
    use test::Bencher;

    use super::{Cipher, EncryptState};
    use crate::key_init::KeyInit;
    use crate::types::DataChunk;

    #[test]
    fn test_encrypt() {
        let mut state = EncryptState::with_key_from([1, 2, 4, 8, 16, 32, 64, 128]);
        let mut plain_data_chunk_1 = DataChunk::from([1, 1, 1, 1, 2, 2, 2, 2]);
        let mut plain_data_chunk_2 = DataChunk::from([3, 3, 3, 3, 4, 4, 4, 4]);
        state.process_chunk(&mut plain_data_chunk_1);
        state.process_chunk(&mut plain_data_chunk_2);

        assert_eq!(plain_data_chunk_1.0, [65, 34, 12, 136, 17, 33, 65, 129]);
        assert_eq!(plain_data_chunk_2.0, [98, 28, 137, 145, 34, 66, 130, 0]);
    }

    #[bench]
    fn bench_our_algorithm(b: &mut Bencher) {
        let mut state = EncryptState::with_key_from(test::black_box([1, 2, 4, 8, 16, 32, 64, 128]));
        let mut plain_data_chunk_1 = DataChunk::from(test::black_box([1, 1, 1, 1, 2, 2, 2, 2]));
        let mut plain_data_chunk_2 = DataChunk::from(test::black_box([3, 3, 3, 3, 4, 4, 4, 4]));

        b.iter(|| {
            state.process_chunk(&mut plain_data_chunk_1);
            state.process_chunk(&mut plain_data_chunk_2);
        });
    }

    #[bench]
    fn bench_aes(b: &mut Bencher) {
        use aes::cipher::{generic_array::GenericArray, BlockEncrypt, KeyInit};
        use aes::Aes128;

        let key = GenericArray::from(test::black_box([1u8; 16]));
        let mut block = GenericArray::from(test::black_box([42u8; 16]));
        let cipher = Aes128::new(&key);

        b.iter(|| {
            cipher.encrypt_block(&mut block);
        });
    }

    #[bench]
    fn bench_tea(b: &mut Bencher) {
        use tea_soft::block_cipher::{generic_array::GenericArray, BlockCipher, NewBlockCipher};
        use tea_soft::Tea16;

        let key = GenericArray::from(test::black_box([1u8; 16]));
        let mut block_1 = GenericArray::from(test::black_box([42u8; 8]));
        let mut block_2 = GenericArray::from(test::black_box([42u8; 8]));
        let cipher = Tea16::new(&key);

        b.iter(|| {
            cipher.encrypt_block(&mut block_1);
            cipher.encrypt_block(&mut block_2);
        });
    }
}
