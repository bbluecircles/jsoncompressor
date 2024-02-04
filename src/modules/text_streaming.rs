use std::ffi::CStr;
use std::os::raw::c_char;
use std::sync::Mutex;
use lazy_static::lazy_static;
use log::info;
use serde_json::Value;

// Mutex for streaming data into the program.
lazy_static! {
    static ref SERIALIZED_JSON_DATA: Mutex<Option<String>> = Mutex::new(None);
    static ref READ_POSITION: Mutex<usize> = Mutex::new(0);
}

// Mutex for streaming data out of the program.
lazy_static! {
    static ref WRITTEN_DATA: Mutex<Vec<Value>> = Mutex::new(Vec::new());
}
// Expose the following functions.
pub fn process_text_chunk<F>(bytes: &[u8], map_func: F) where F: Fn(&[u8]) -> Value {
    let bytes_to_json: Value = map_func(bytes);
    let mut data = WRITTEN_DATA.lock().unwrap();
    data.push(bytes_to_json);
}

// Extract written data (should only be called by `run_action_on_processed_json`)
pub fn extract_written_data() -> Vec<Value> {
    return WRITTEN_DATA.lock().unwrap().to_vec();
}

// Update WRITTEN_DATA value.
pub fn update_written_data(replaced_data: Vec<Value>) {
    let mut data = WRITTEN_DATA.lock().unwrap();
    *data = replaced_data;
}

pub fn initialize_json_streaming() {
    let data = WRITTEN_DATA.lock().unwrap().to_vec();
    let to_serialized = serde_json::to_string(&data).expect("Failed to serialize the JSON.");
    info!("To serialized after action: {}", to_serialized);
    let mut serialized_data = SERIALIZED_JSON_DATA.lock().unwrap();
    *serialized_data = Some(to_serialized);
    let mut read_position = READ_POSITION.lock().unwrap();
    *read_position = 0;
}

#[no_mangle]
pub extern "C" fn get_json_chunk(buffer: *mut c_char, buffer_len: usize) -> bool {
    let json_data = SERIALIZED_JSON_DATA.lock().unwrap();
    let mut position = READ_POSITION.lock().unwrap();

    if let Some(ref data) = *json_data {
        if *position >= data.len() {
            return false; 
        }

        let chunk = &data[*position..std::cmp::min(*position + buffer_len, data.len())];
        unsafe {
            std::ptr::copy_nonoverlapping(chunk.as_ptr(), buffer as *mut u8, chunk.len());
        }
        *position += buffer_len;
        true
    } else {
        false 
    }
}

#[no_mangle]
pub extern "C" fn finalize_json_streaming() {
    let mut json_data = SERIALIZED_JSON_DATA.lock().unwrap();
    *json_data = None;
    let mut position = READ_POSITION.lock().unwrap();
    *position = 0;
}