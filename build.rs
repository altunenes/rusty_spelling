use std::fs;

fn main() {
    let src_path = "word_list.txt";
    let dst_path = concat!(env!("CARGO_MANIFEST_DIR"), "/target/release/word_list.txt");
    fs::copy(src_path, dst_path).expect("Failed to copy file");
}