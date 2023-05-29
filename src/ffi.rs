use std::ffi::CStr;
use std::fs::File;
use std::os::raw::c_char;
use std::sync::Mutex;

use crate::cipher::{Cipher, DecryptState, EncryptState};
use crate::driver::FileDriver;
use crate::{Driver, KeyInit};

pub const ERR_OK: i32 = 0;
pub const ERR_IO_ERROR: i32 = 1;
pub const ERR_INVALID_PATH: i32 = 2;

pub struct Handle {
    cipher: Mutex<Box<dyn Cipher>>,
}

#[no_mangle]
pub extern "C" fn yafo_create_handle(seed_phrase: *const c_char, decrypt: bool) -> *mut Handle {
    let seed_phrase_str = unsafe { CStr::from_ptr(seed_phrase) }.to_string_lossy();
    let cipher: Box<dyn Cipher> = {
        if decrypt {
            Box::new(DecryptState::with_seed_phrase(seed_phrase_str.as_ref()))
        } else {
            Box::new(EncryptState::with_seed_phrase(seed_phrase_str.as_ref()))
        }
    };

    let handle = Handle {
        cipher: Mutex::new(cipher),
    };
    Box::into_raw(Box::new(handle))
}

#[no_mangle]
pub extern "C" fn yafo_process_file(handle: *mut Handle, path: *const c_char) -> i32 {
    let path_str = match unsafe { CStr::from_ptr(path) }.to_str() {
        Ok(str) => str,
        _ => return ERR_INVALID_PATH,
    };
    let file = match File::options().read(true).write(true).open(path_str) {
        Ok(file) => file,
        _ => return ERR_IO_ERROR,
    };
    let mut file_driver = FileDriver::new(file);

    let handle_mut = unsafe { &mut *handle };
    let mut locked_cipher = handle_mut.cipher.lock().expect("lock the mutex");

    match file_driver.process(locked_cipher.as_mut()) {
        Ok(_) => ERR_OK,
        Err(_) => ERR_IO_ERROR,
    }
}

#[no_mangle]
pub extern "C" fn yafo_destroy_handle(handle: *mut Handle) {
    let handle = unsafe { Box::from_raw(handle) };

    // Just to emphasize this operation.
    drop(handle);
}
