extern crate uuid;
use std::process::id;
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
    fn compress(&self, input: &str) -> &str;
}
struct JsonCompressor;
impl CompressStrategy for JsonCompressor {
    fn compress(&self, input: &str) -> &str {
        let mut e = zlibEncoder::new(Vec::new(), Compression::default());
        e.write_all(input);
        let compressed_bytes = e.finish();
        let str = "hello testing..";
        return input;
    }
}
fn main() {
    println!("Hello, world!");
}
