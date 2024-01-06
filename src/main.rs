extern crate uuid;
use std::fmt::Error;
use uuid::Uuid;
use std::io::prelude::*;
use flate2::Compression;
use flate2::write::ZlibEncoder;

struct File {
    file_id: Uuid,
    file_data: String
}
impl File {
    fn new(file_data: String) -> File {
        let file_id: Uuid = Uuid::new_v4();
        File { file_id, file_data }
    }
}
trait CompressStrategy {
    fn compress(&self, input: &[u8]) -> Result<Vec<u8>, std::io::Error>;
}
struct JsonCompressor;
impl CompressStrategy for JsonCompressor {
    fn compress(&self, input: &[u8]) -> Result<Vec<u8>, std::io::Error> {
        let mut e = ZlibEncoder::new(Vec::new(), Compression::default());
        let _ = e.write_all(input);
        return e.finish();
    }
}

pub fn compress_json(file_content: &[u8]) -> Result<Vec<u8>, std::io::Error> {
    let compress_strategy: JsonCompressor = JsonCompressor;
    return compress_strategy.compress(file_content);
}
fn main() {
    println!("Hello, world!");
}
