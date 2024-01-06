extern crate uuid;
use std::io;
use std::io::prelude::*;
use std::fs;
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
pub fn decompress_json(file_content: &[u8]) -> Result<String, std::io::Error> {
    let decompress_strategy: JsonDeCompressor = JsonDeCompressor;
    return decompress_strategy.decompress(file_content);
}
pub fn compress_json(file_content: &[u8]) -> Result<Vec<u8>, std::io::Error> {
    let compress_strategy: JsonCompressor = JsonCompressor;
    return compress_strategy.compress(file_content);
}
fn main() {
    // let should_decompress: bool = true;
    // let mut file_path = String::new();
    
    // if !should_decompress {
    //     println!("Please enter a JSON file you'd like to compress.");
    //     // Read input from terminal.
    //     io::stdin().read_line(&mut file_path)
    //     .expect("Failed to read the file.");
    //     // Trim the newline character at the end.
    //     file_path = file_path.trim_end().to_string();
    //     // Ensure content has been read.
    //     let content: Vec<u8> = fs::read(&file_path)
    //         .expect("Failed to read the file");
    //     // Print the number of bytes read before the compression.
    //     println!("Number of bytes read before parse: {}", content.len());
    //     // Try to compress file.
    //     let result: Result<Vec<u8>, io::Error> = compress_json(&content);
    //     let vec = result.expect("Failed to compress JSON file.");
    //     // Print the number of bytes after compressing.
    //     println!("Number of bytes read after parse: {}", vec.len());
    //     let mut compressed_file = fs::File::create("compressed.json")
    //     .expect("Failed to create file.");
    //     compressed_file.write_all(&vec).expect("Failed to write compressed data to file.");
    // } else {
    //     println!("Please enter a JSON file you'd like to decompress");
    //     // Read input from terminal.
    //     io::stdin().read_line(&mut file_path)
    //     .expect("Failed to read the file.");
    //     // Trim the newline character at the end.
    //     file_path = file_path.trim_end().to_string();
    //     // Ensure content has been read.
    //     let content: Vec<u8> = fs::read(&file_path)
    //         .expect("Failed to read the file");
    //     // Print the number of bytes read before the compression.
    //     println!("Number of bytes read before parse: {}", content.len());
    //     // Decompress file.
    //     let result: Result<String, io::Error> = decompress_json(&content);
    //     let string_result = result.expect("Failed to decompress JSON file.");
    //     let string_result_as_bytes = string_result.as_bytes();
    //     // Read number of bytes after decompression
    //     println!("Number of bytes read after parse: {}", string_result_as_bytes.len());
    //     // Create a new JSON that will contain decompressed JSON.
    //     let mut compressed_file = fs::File::create("decompressed.json")
    //     .expect("Failed to create file.");
    //     compressed_file.write_all(string_result_as_bytes).expect("Failed to write decompressed data to file.");
    // }
    // println!("Done!");
}
