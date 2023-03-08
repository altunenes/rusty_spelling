use std::fs;
use std::path::Path;

fn main() {
    let target_dir = Path::new("target/release");
    if !target_dir.exists() {
        fs::create_dir_all(target_dir).unwrap();
    }
    fs::copy("word_list.txt", target_dir.join("word_list.txt")).unwrap();
}