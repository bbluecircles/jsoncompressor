use std::io;
use std::io::prelude::*;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::sync::Mutex;
use flate2::Compression;
use flate2::bufread::{GzDecoder, GzEncoder};
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json, map::Map, from_str, to_string};

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
pub extern "C" fn decompress_json_and_run_action(file_content: *const u8, len: usize, out_len: *mut usize, action: *const c_char, action_value: *const c_char) -> *mut c_char {
    let bytes = unsafe { std::slice::from_raw_parts(file_content, len) }; 
    let decompress_strategy: JsonDeCompressor = JsonDeCompressor;
    // Map the error into a JsValue type from the stringified error if error is returned.
    match decompress_strategy.decompress(bytes) {
        Ok(decompressed_data) => {
            let data_length = decompressed_data.len();
            match from_str::<Vec<Value>>(&decompressed_data) {
                Ok(mut data) => {
                    let action_type_to_str: &CStr = unsafe {
                        assert!(!action.is_null());
                        CStr::from_ptr(action)
                    };
                    // For this example we'll just sort desc.
                    let action_value_to_str: &CStr = unsafe {
                        assert!(!action_value.is_null());
                        CStr::from_ptr(action_value)
                    };
                    match action_type_to_str.to_str() {
                        Ok(str) => {
                            let parsedActionValue: Value = match action_value_to_str.to_str() {
                                Ok(val) => {
                                    serde_json::from_str(val).expect("JSON was not valid.")
                                },
                                Err(e) => {
                                    let msg: String = "Failed to parse JSON".to_string();
                                    serde_json::Value::String(msg)
                                }
                            };
                            if str == "sort" {
                                let Some(fieldToSort) = parsedActionValue.get("field").and_then(Value::as_str);
                                let Some(dir) = parsedActionValue.get("dir").and_then(Value::as_str);
                                let ascending: bool = dir == "asc";
                                let to_sorted: () = sort_items(&mut data, fieldToSort, ascending);
                                match to_string(&to_sorted) {
                                    Ok(return_string) => {
                                        unsafe {
                                            *out_len = data_length;
                                            CString::new(return_string).unwrap().into_raw()
                                        }
                                    },
                                    Err(e) => {
                                        set_last_error(e.to_string());
                                        let error = LAST_ERROR.lock().unwrap();
                                        CString::new(error.clone()).unwrap().into_raw()
                                    }
                                }
                            } else {
                                // Then should be filter.

                            }
                        },
                        Err(e) => {
                            set_last_error(e.to_string());
                            let error = LAST_ERROR.lock().unwrap();
                            CString::new(error.clone()).unwrap().into_raw()
                        }
                    }
                },
                Err(e) => {
                    set_last_error(e.to_string());
                    let error = LAST_ERROR.lock().unwrap();
                    CString::new(error.clone()).unwrap().into_raw()
                }
            }
        },
        Err(e) => {
            set_last_error(e.to_string());
            let error = LAST_ERROR.lock().unwrap();
            CString::new(error.clone()).unwrap().into_raw()
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

