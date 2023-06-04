#![feature(test)]
#![cfg_attr(test, feature(assert_matches))]

pub mod cipher;
#[cfg(feature = "ffi")]
pub mod ffi;
pub mod key_init;
pub mod pipeline;
pub mod types;

mod io;

pub use cipher::{Cipher, DecryptState, EncryptState};
pub use key_init::KeyInit;
pub use pipeline::Pipeline;

#[cfg(test)]
mod tests {
    use std::assert_matches::assert_matches;
    use std::io::Cursor;

    use crate::{DecryptState, EncryptState, KeyInit, Pipeline};

    #[test]
    fn encrypt_decrypt_text() {
        let plain_text = "hello";
        let seed_phrase = "you can not see me";

        let cipher_text = {
            let input = Cursor::new(plain_text.as_bytes());
            let encrypt = EncryptState::with_seed_phrase(seed_phrase);

            let mut cipher_text: Vec<u8> = vec![];
            let pipeline = Pipeline::new();
            assert_matches!(
                pipeline.process(input, &mut cipher_text, encrypt, None),
                Ok(())
            );

            cipher_text
        };
        assert_eq!(cipher_text, vec![183, 72, 155, 142, 186]);

        let decrypted_text = {
            let input = Cursor::new(&cipher_text);
            let decrypt = DecryptState::with_seed_phrase(seed_phrase);

            let mut decrypted_text: Vec<u8> = vec![];
            let pipeline = Pipeline::new();
            assert_matches!(
                pipeline.process(input, &mut decrypted_text, decrypt, None),
                Ok(())
            );

            decrypted_text
        };
        assert_eq!(
            String::from_utf8(decrypted_text).expect("failed to decode"),
            plain_text
        );
    }
}
