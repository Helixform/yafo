#![feature(test)]
#![cfg_attr(test, feature(assert_matches))]

pub mod cipher;
pub mod driver;
pub mod ffi;
pub mod key_init;
pub mod types;

pub use cipher::{DecryptState, EncryptState};
pub use driver::Driver;
pub use key_init::KeyInit;

#[cfg(test)]
mod tests {
    use std::assert_matches::assert_matches;

    use crate::driver::{statistics, InMemoryDriver};
    use crate::{DecryptState, Driver, EncryptState, KeyInit};

    #[test]
    fn encrypt_decrypt_text() {
        let plain_text = "hello";
        let seed_phrase = "you can not see me";

        let cipher_text = {
            let mut driver = InMemoryDriver::new(plain_text.bytes().collect());
            let mut encrypt = EncryptState::with_seed_phrase(seed_phrase);

            assert_matches!(driver.process(&mut encrypt), Ok(()));

            driver.into_vec()
        };
        assert_eq!(cipher_text, vec![183, 72, 155, 142, 186]);

        let decrypted_text = {
            let mut driver = InMemoryDriver::new(cipher_text);
            let mut decrypt = DecryptState::with_seed_phrase(seed_phrase);

            assert_matches!(driver.process(&mut decrypt), Ok(()));
            driver.into_vec()
        };
        assert_eq!(
            String::from_utf8(decrypted_text).expect("failed to decode"),
            plain_text
        );
    }

    #[test]
    fn encrypt_with_stat_reports() {
        let plain_text = vec![1u8; 1024 * 1024];
        let seed_phrase = "whatever";

        struct MockReporter(usize);

        impl statistics::Reporter for MockReporter {
            fn bytes_processed(&mut self, n: usize) {
                self.0 += n;
            }
        }

        let mut driver =
            statistics::Statistics::new(InMemoryDriver::new(plain_text), MockReporter(0));
        let mut encrypt = EncryptState::with_seed_phrase(seed_phrase);

        assert_matches!(driver.process(&mut encrypt), Ok(()));
        assert_eq!(driver.reporter().0, 1024 * 1024);
    }
}
