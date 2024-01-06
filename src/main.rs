extern crate uuid;
use std::io;
use std::io::prelude::*;
use std::fs;
use flate2::Compression;
use flate2::Decompress;
use flate2::write::ZlibEncoder;

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
    println!("Please enter a JSON file you'd like to compress.");
    let mut file_path = String::new();
    // Read input from terminal.
    io::stdin().read_line(&mut file_path)
    .expect("Failed to read the file.");
    // Trim the newline character at the end.
    file_path = file_path.trim_end().to_string();
    // Ensure content has been read.
    let content: Vec<u8> = fs::read(&file_path)
        .expect("Filed to read the file");
    // Print the number of bytes read before the compression.
    println!("Number of bytes read before parse: {}", content.len());
    // Try to compress file.
    let result: Result<Vec<u8>, io::Error> = compress_json(&content);
    let vec = result.expect("Failed to compress JSON file.");
    // Print the number of bytes after compressing.
    println!("Number of bytes read after parse: {}", vec.len());
    let mut compressed_file = fs::File::create("compressed.json")
    .expect("Failed to create file.");
    compressed_file.write_all(&vec).expect("Failed to write compressed data to file.");
    println!("Done!");
}
