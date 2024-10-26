use std::{fs, io};
const DUMMY_FILE: &str = "assets/dummy.txt";

// Problema 1
fn get_biggest_lines(text: &String) -> Result<(&str, &str), io::Error> {
    let mut longest_chars: &str = "";
    let mut longest_bytes: &str = "";
    let mut max_chars = 0usize;
    let mut max_bytes: usize = 0;
    for line in text.lines() {
        let length: usize = line.chars().count();
        let bytes_length: usize = line.len();
        if length > max_chars {
            longest_chars = line;
            max_chars = length;
        }
        if line.bytes().len() > max_bytes {
            longest_bytes = line;
            max_bytes = bytes_length;
        }
    }

    Ok((longest_chars, longest_bytes))
}

fn main() {
    // Problema 1
    let dummy_text: String =
        fs::read_to_string(DUMMY_FILE).expect("Text file path should be valid!");
    let biggest_lines = get_biggest_lines(&dummy_text).expect("io Error");
    println!(
        "File : {}\nBiggest chars line: {}\nBiggest bytes line: {}",
        DUMMY_FILE, biggest_lines.0, biggest_lines.1
    );
}
