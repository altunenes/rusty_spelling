use std::fs;

fn main() {
    fs::copy("word_list.txt", "target/release/word_list.txt").unwrap();
}