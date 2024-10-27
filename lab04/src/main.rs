use rayon::prelude::*;
use std::fs::File;
use std::io::Write;
use std::time::Instant;
use std::{
    collections::HashMap,
    fs::{self},
    io,
};

const DUMMY_FILE: &str = "assets/dummy.txt";
const ERROR_FILE: &str = "assets/fake.txt";
const SENTENCES_FILE: &str = "assets/sentences.txt";
const HOSTS_FILE: &str = "/etc/hosts";
const SAMPLE_FILE: &str = "assets/sample_text.txt";
const HUGE_FILE: &str = "assets/huge.txt";

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

// Problema 4
fn print_hosts(file_path: &str) -> Result<(), io::Error> {
    let text = fs::read_to_string(file_path)?;
    for line in text.lines() {
        if line.starts_with("#") {
            continue;
        } else {
            if let Some((ip_addr, host_name)) = line.split_once(char::is_whitespace) {
                println!("{} => {}", ip_addr, host_name.trim_ascii_start());
            } else {
                println!("Could not split spaces between host name and IP");
            }
        }
    }

    Ok(())
}

// Bonus

fn generate_huge_file(path: &str, gbs: u8) -> Result<(), io::Error> {
    let check = fs::exists(path)?;
    if check == true {
        println!("Generated file already exists!");
        return Ok(());
    }
    let mut file = File::create(path)?;

    let size_in_bytes: u64 = gbs as u64 * 1024 * 1024 * 1024;
    let sample_text = fs::read_to_string(SAMPLE_FILE)?;
    let mut final_sample = String::new();
    final_sample.push_str(&sample_text);
    let n_of_paste: u64 = size_in_bytes / final_sample.len() as u64;
    for _ in 1..n_of_paste {
        file.write_all(final_sample.as_bytes())?;
    }

    Ok(())
}

fn rot13_cipher_fast(input_text: &str) -> Option<String> {
    // Construita dupa research pe internet
    let cipher: String = input_text
        .par_bytes() // Iterare paralela
        .map(|byte| match byte {
            b'a'..=b'z' => ((byte - b'a' + 13) % 26 + b'a') as char,
            b'A'..=b'Z' => ((byte - b'A' + 13) % 26 + b'A') as char,
            _ => byte as char,
        })
        .collect();

    Some(cipher)
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
        ("ptr", "pentru"),
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

    // Problema 4
    println!("Hosts:");
    match print_hosts(HOSTS_FILE) {
        Ok(_) => {}
        Err(e) => {
            println!("Could not print hosts: {e}")
        }
    }

    // Bonus
    println!("\nBonus :");
    match generate_huge_file(HUGE_FILE, 4) {
        Ok(_) => {
            println!("Generated huge file!")
        }
        Err(e) => {
            println!("Error creating huge file : {e}");
        }
    }
    let text: String = fs::read_to_string(HUGE_FILE).expect("Could not read from file");
    println!(
        "Size of file: {:.4} GB",
        text.len() as f64 / (1024 * 1024 * 1024) as f64
    );
    let start = Instant::now();
    println!("Encrypting...");
    if let Some(cipher) = rot13_cipher_fast(&text) {
        println!("Time elapsed : {:?}", start.elapsed());
        println!("Len of cipher: {} bytes", cipher.len());
        // Prelucrare cipher... eventual suprascrierea lui in fisier
    } else {
        println!("Error but time is : {:?}", start.elapsed());
    }
}
