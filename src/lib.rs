use std::io;
use std::io::prelude::*;
use std::ffi::{CString};
use std::os::raw::c_char;
use std::sync::Mutex;
use flate2::Compression;
use flate2::bufread::{GzDecoder, GzEncoder};
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json, map::Map};

trait CompressStrategy {
    fn compress(&self, input: &[u8]) -> Result<Vec<u8>, std::io::Error>;
}
struct JsonCompressor;
impl CompressStrategy for JsonCompressor {
    fn compress(&self, input: &[u8]) -> Result<Vec<u8>, std::io::Error> {
        let mut gz = GzEncoder::new(input, Compression::best());
        let mut buffer = Vec::new();
        gz.read_to_end(&mut buffer)?;
        Ok(buffer)
    }
}

trait DeCompressStrategy {
    fn decompress(&self, input: &[u8]) -> io::Result<String>;
}

struct JsonDeCompressor;
impl DeCompressStrategy for JsonDeCompressor {
    fn decompress(&self, input: &[u8]) -> Result<String, std::io::Error>{
        let mut gz: GzDecoder<&[u8]> = GzDecoder::new(input);
        let mut s = String::new();
        gz.read_to_string(&mut s)?;
        Ok(s)
    }
}

// Sorting
fn sort_items(items: &mut Vec<Value>, property_key: &str, ascending: bool) {
    items.sort_by(|a: &Value, b: &Value| {
        let a_val: &str = a.get(property_key).and_then(Value::as_str).unwrap_or_default();
        let b_val: &str = b.get(property_key).and_then(Value::as_str).unwrap_or_default();
        let ord = a_val.cmp(&b_val);
        if ascending { ord } else { ord.reverse() }
    });
}

// Filtering
fn filter_items(items: &mut Vec<Value>, property_key: &str, filter_value: &str) -> Vec<Value> {
    items.iter()
        .filter(|item: &&Value|
            item.get(property_key)
                .and_then(Value::as_str)
                .map_or(false, |val| val == filter_value)
        )
        .cloned()
        .collect()
}

// Error handling 
lazy_static! {
    static ref LAST_ERROR: Mutex<String> = Mutex::new(String::new());
}
fn set_last_error(err: String) {
    let mut last_error = LAST_ERROR.lock().unwrap();
    *last_error = err;
}

#[no_mangle]
pub extern "C" fn decompress_json(file_content: *const u8, len: usize, out_len: *mut usize) -> *mut c_char {
    let bytes = unsafe { std::slice::from_raw_parts(file_content, len) }; 
    let decompress_strategy: JsonDeCompressor = JsonDeCompressor;
    // Map the error into a JsValue type from the stringified error if error is returned.
    match decompress_strategy.decompress(bytes) {
        Ok(decompressed_data) => {
            let data_length = decompressed_data.len();

            unsafe {
                *out_len = data_length;
                CString::new(decompressed_data).unwrap().into_raw()
            }
        },
        Err(e) => {
            set_last_error(e.to_string());
            let error = LAST_ERROR.lock().unwrap();
            CString::new(error.clone()).unwrap().into_raw()
        }
    }
}
// Return a pointer to convert into an array of bytes.
#[no_mangle]
pub extern "C" fn compress_json(file_content: *const u8, len: usize, out_len: *mut usize) -> *mut u8 {
    let bytes = unsafe { std::slice::from_raw_parts(file_content, len) }; 
    let compress_strategy: JsonCompressor = JsonCompressor;

    match compress_strategy.compress(bytes) {
        Ok(compressed_data) => {
            let data_length = compressed_data.len();

            unsafe { 
                *out_len = data_length; 
                let to_boxed = compressed_data.into_boxed_slice();
                let ptr = Box::into_raw(to_boxed) as *mut u8;

                ptr
            }
        },
        Err(e) => {
            set_last_error(e.to_string());
            std::ptr::null_mut()
        }
    }
}

#[no_mangle]
pub extern "C" fn get_last_error() -> *mut c_char {
    let error = LAST_ERROR.lock().unwrap();
    CString::new(error.clone()).unwrap().into_raw()
}

#[no_mangle]
pub extern "C" fn free_memory(ptr: *mut u8, len: usize, cap: usize) {
    unsafe {
        let _ = Vec::from_raw_parts(ptr, len, cap);
    }
}

#[no_mangle]
pub extern "C" fn free_string(ptr: *mut c_char) {
    unsafe {
        if !ptr.is_null() {
            let _ = CString::from_raw(ptr);
        }
    }
}

