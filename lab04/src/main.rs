use std::{collections::HashMap, fs, io};
const DUMMY_FILE: &str = "assets/dummy.txt";
const ERROR_FILE: &str = "assets/fake.txt";
const SENTENCES_FILE: &str = "assets/sentences.txt";

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

// Problema 2
fn rot13_cipher(input_text: &str) -> Option<String> {
    for byte in input_text.as_bytes() {
        if byte.is_ascii() == false {
            return None;
        } // Am intampinat un caracter care nu este ascii
    }
    let mut cipher: String = String::new();
    for byte in input_text.as_bytes() {
        let ch: char;
        if byte.is_ascii_lowercase() == true {
            // litera mica
            ch = (((byte - 97 + 13) % 26) + 97) as char;
            cipher.push(ch);
        } else if byte.is_ascii_uppercase() == true {
            // litera mare
            ch = (((byte - 65 + 13) % 26) + 65) as char;
            cipher.push(ch);
        } else {
            // orice alt caracter
            ch = *byte as char;
            cipher.push(ch);
        }
    }
    Some(cipher)
}

// Problema 3

fn modify_sentence_abb(s: &str, abb: &HashMap<&str, &str>) -> Option<String> {
    let mut mod_s = String::new();
    let mut key_found = false;
    for word in s.split_whitespace() {
        if abb.contains_key(word) {
            mod_s.push_str(abb.get(word).unwrap());
            key_found = true;
        } else {
            mod_s.push_str(word);
        }
        mod_s.push(' ');
    }
    if key_found == false {
        return None;
    } // nu contine abrevieri (pt cazul de eroare)

    Some(mod_s)
}

fn main() {
    // Problema 1
    // Succes
    let dummy_text: String =
        fs::read_to_string(DUMMY_FILE).expect("Text file path should be valid!");
    let biggest_lines = get_biggest_lines(&dummy_text).expect("io Error");
    println!(
        "File : {}\nBiggest chars line: {}\nBiggest bytes line: {}\n",
        DUMMY_FILE, biggest_lines.0, biggest_lines.1
    );
    // Eroare
    let error_text = fs::read_to_string(&ERROR_FILE);
    match error_text {
        Ok(text) => {
            let error_case_lines =
                get_biggest_lines(&text).expect("Error calculating lines length");
            println!(
                "File : {}\nBiggest chars line: {}\nBiggest bytes line: {}\n",
                ERROR_FILE, error_case_lines.0, error_case_lines.1
            );
        }
        Err(e) => println!("File : {ERROR_FILE}\nError : {e}\n"),
    }
    // Problema 2
    // Succes
    let text = "strings are FUN ?";
    let cipher;
    if let Some(cipher_option) = rot13_cipher(&text) {
        cipher = cipher_option;
        println!("Text: {}\nROT13 cipher: {}\n", text, cipher);
    } else {
        println!("Cannot encode ROT13, the string contains non-ASCII characters.");
    }

    // Eroare
    let text = "dsadas 🎁🎶🎉👀🎈🎃🍕☕🍉";
    let cipher;
    if let Some(cipher_option) = rot13_cipher(&text) {
        cipher = cipher_option;
        println!("Text: {}\nROT13 cipher: {}\n", text, cipher);
    } else {
        println!(
            "Text : {}\nCannot encode ROT13, the string contains non-ASCII characters.\n",
            text
        );
    }
    // Problema 3
    let abbreviations = HashMap::from([
        ("dl", "domnul"),
        ("pt", "pentru"),
        ("dna", "doamna"),
        ("dvs", "dumneavoastra"),
    ]);
    let mut sentences = fs::read_to_string(SENTENCES_FILE).expect("No such file for sentences");
    println!("Initial sentences:\n{}\n", sentences);
    let mut new_sentences = String::new();
    for s in sentences.lines() {
        if let Some(mod_s) = modify_sentence_abb(s, &abbreviations) {
            new_sentences.push_str(&mod_s);
        } else {
            new_sentences.push_str("No abreviations found for this sentence!\n");
        }
        new_sentences.push('\n');
    }
    sentences = new_sentences;
    println!("Modified sentences:\n{}", sentences);
}
