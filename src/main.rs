extern crate uuid;
use std::process::id;
use uuid::Uuid;

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
fn main() {
    println!("Hello, world!");
}
