use wasm_bindgen::prelude::*;
use std::io;
use std::slice;
use std::io::prelude::*;
use std::ffi::CString;
use std::os::raw::c_char;
use std::ptr;
use flate2::Compression;
use flate2::bufread::{GzDecoder, GzEncoder};
use serde::{Serialize, Deserialize};

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

#[no_mangle]
pub extern "C" fn decompress_json(file_content: *const u8, len: usize) -> *mut c_char {
    let bytes = unsafe { std::slice::from_raw_parts(file_content, len) }; 
    let decompress_strategy: JsonDeCompressor = JsonDeCompressor;
    // Map the error into a JsValue type from the stringified error if error is returned.
    let result = decompress_strategy.decompress(bytes);
    let json_result = match result {
        Ok(data) => serde_json::to_string(&data).unwrap_or_else(|_| "{\"error\": \"Failed to serialize data\"}".to_string()),
        Err(e) => serde_json::to_string(&format!("{{\"error\": \"{}\"}}", e)).unwrap_or_else(|_| "{\"error\": \"Unknown error\"}".to_string()),
    };
    CString::new(json_result).unwrap().into_raw()
}
#[no_mangle]
pub extern "C" fn compress_json(file_content: &[u8]) -> Result<Vec<u8>, std::io::Error> {
    let compress_strategy: JsonCompressor = JsonCompressor;
    // Map the error into a JsValue type from the stringified error if error is returned.
    compress_strategy.compress(file_content)
}

