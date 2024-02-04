use std::{io, u8};
use std::io::prelude::*;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::sync::Mutex;
use flate2::Compression;
use flate2::bufread::{GzDecoder, GzEncoder};
use lazy_static::lazy_static;
use log::{info, warn};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json, map::Map, from_str, to_string};

mod modules;
use modules::text_streaming::{self, extract_written_data, initialize_json_streaming, process_text_chunk, update_written_data};

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
#[derive(Serialize, Deserialize, Debug)]
struct Filter {
    operator: String,
    value: serde_json::Value,
    field: String
}
#[derive(Serialize, Deserialize, Debug)]
struct ComplexFilter {
    logic: String,
    filters: Vec<Filter>
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
fn filter_items(items: &[Value], filter_value: &str) -> Vec<Value> {
    let complex_filter: ComplexFilter = match from_str(filter_value) {
        Ok(f) => f,
        Err(_) => return Vec::new()
    };

    items.iter().filter(|item| {
        apply_complex_filter(item, &complex_filter)
    }).cloned().collect()
}
fn apply_complex_filter(item: &Value, complex_filter: &ComplexFilter) -> bool {
    match complex_filter.logic.as_str() {
        "AND" => complex_filter.filters.iter().all(|filter| {
            apply_filter(item, filter)
        }),
        "OR" => complex_filter.filters.iter().any(|filter| {
            apply_filter(item, filter)
        }),
        _ => false
    }
}

fn apply_filter(item: &Value, filter: &Filter) -> bool {
    match item.get(&filter.field) {
        Some(item_value) => {
            match filter.operator.as_str() {
                "contains" => {
                    item_value.to_string().contains(&filter.value.to_string())
                },
                _ => false
            }
        },
        None => false
    }
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
pub extern "C" fn process_streamed_json(chunk_content: *const u8, chunk_len: usize) {
    let bytes = unsafe { std::slice::from_raw_parts(chunk_content, chunk_len) }; 
    process_text_chunk(bytes, decompress_json_locally);
}

#[no_mangle]
pub extern "C" fn run_action_on_processed_json(action: *const c_char, action_value: *const c_char) {
    info!("Starting action.");
    let mut data: Vec<Value> = extract_written_data();
    let action_type_to_str: &CStr = unsafe {
        assert!(!action.is_null());
        CStr::from_ptr(action)
    };
    let action_value_to_str: &CStr = unsafe {
        assert!(!action_value.is_null());
        CStr::from_ptr(action_value)
    };
    match action_type_to_str.to_str() {
        Ok(str) => {
            // Extract action config.
            let parsed_action_value: Value = match action_value_to_str.to_str() {
                Ok(val) => {
                    serde_json::from_str(val).expect("JSON was not valid.")
                },
                Err(e) => {
                    let msg: String = "Failed to parse JSON".to_string();
                    warn!("Something went wrong parsing action value: {}", msg);
                    serde_json::Value::String(msg)
                }
            };
            // SORT
            if str == "sort" {
                let field_to_sort = match parsed_action_value.get("field") {
                    Some(value) => value.as_str().unwrap_or("No Value"),
                    None => "No Value"
                };
                let dir = match parsed_action_value.get("dir") {
                    Some(value) => value.as_str().unwrap_or("desc"),
                    None => "desc"
                };
                let ascending: bool = dir == "asc";
                info!("About to attempt a sort.");
                sort_items(&mut data, field_to_sort, ascending);
            } 
            info!("About to update written data");
            update_written_data(data);
            // Begin preparing the JSON for streaming.
            initialize_json_streaming();
        },
        Err(e) => {
            set_last_error(e.to_string());
            warn!("Something went wrong parsing action type: {}", e.to_string())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::CString;

    #[test]
    fn test_run_action_on_processed_json() {
        let input = CSString::new("sort").expect("CString::new failed")
        let ptr = input.as_ptr();
    }
}


pub fn decompress_json_locally(bytes: &[u8]) -> Value {
    let decompress_strategy: JsonDeCompressor = JsonDeCompressor;

    match decompress_strategy.decompress(bytes) {
        Ok(decompressed_data) => {
            let parsed: Value = serde_json::from_str(&decompressed_data).expect("Parsing decompressed failed.");
            parsed
        },
        Err(e) => {
            let err_json: Value = serde_json::from_str(r#"{ "Error": "Something went wrong." }"#).expect("Failed.");
            err_json
        }
    }
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

