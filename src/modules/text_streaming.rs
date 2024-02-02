use std::ffi::CStr;
use std::os::raw::c_char;
use std::sync::Mutex;
use lazy_static::lazy_static;

// Mutex for streaming data into the program.
lazy_static! {
    static ref READ_CURRENT_POS: Mutex<usize> = Mutex::new(0);
}

// Mutex for streaming data out of the program.
lazy_static! {
    static ref DATA_TO_WRITE: Mutex::new(String::new())
}


