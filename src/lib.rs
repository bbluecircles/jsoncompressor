use wasm_bindgen::prelude::*;
use std::io;
use std::io::prelude::*;
use flate2::Compression;
use flate2::bufread::{GzDecoder, GzEncoder};

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

#[wasm_bindgen]
pub fn decompress_json(file_content: &[u8]) -> Result<String, JsValue> {
    let decompress_strategy: JsonDeCompressor = JsonDeCompressor;
    // Map the error into a JsValue type from the stringified error if error is returned.
    decompress_strategy.decompress(file_content)
    .map_err(|e| JsValue::from_str(&e.to_string()))
}
pub fn compress_json(file_content: &[u8]) -> Result<Vec<u8>, JsValue> {
    let compress_strategy: JsonCompressor = JsonCompressor;
    // Map the error into a JsValue type from the stringified error if error is returned.
    compress_strategy.compress(file_content)
    .map_err(|e| JsValue::from_str(&e.to_string()))
}

