use std::ffi::CStr;
use std::os::raw::c_char;
use std::sync::Mutex;
use lazy_static::lazy_static;
use serde_json::Value;

// Mutex for streaming data into the program.
lazy_static! {
    static ref READ_CURRENT_POS: Mutex<usize> = Mutex::new(0);
}

// Mutex for streaming data out of the program.
lazy_static! {
    static ref WRITTEN_DATA: Mutex<Vec<Value>> = Mutex::new(Vec::new());
}
// For running filters on the written data.
lazy_static! {
    static ref WRITTEN_DATA_FILTERED: Mutex<Vec<Value>> = Mutex::new(Vec::new());
}
// Expose the following functions.
pub fn process_text_chunk<F>(bytes: &[u8], mapFunc: F) where F: Fn(&[u8]) -> Value {
    let bytes_to_json: Value = mapFunc(bytes);
    let mut data = WRITTEN_DATA.lock().unwrap();
    data.push(bytes_to_json);
}

// Extract written data (should only be called by `run_action_on_processed_json`)
pub fn extract_written_data() -> Vec<Value> {
    return WRITTEN_DATA.lock().unwrap().to_vec();
}

// Update WRITTEN_DATA value.
pub fn update_written_data(action_type: String, replaced_data: Vec<Value>) {
    if action_type == "sort" {
        let mut data = WRITTEN_DATA.lock().unwrap();
        *data = replaced_data;
    }
}
