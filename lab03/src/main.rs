use core::panic;
use std::env;
use thiserror::Error;

const MAX_U16: u16 = 65535;
const MAX_U32: u32 = 4_294_967_295;

#[derive(Error, Debug)]
enum Errors {
    #[error("Overtaking u32 value")]
    OvertakingU32,
}
#[derive(Debug)]
enum CharErrors {
    NotASCII,
    NotDigit,
    NotBase16,
    NotLetter,
    NotPrintable,
}

//For problem 1

fn is_prime(x: u16) -> bool {
    if x < 2 {
        return false;
    }
    if x == 2 {
        return true;
    }
    if x % 2 == 0 {
        return false;
    }
    let mut d: u16 = 3;
    while d <= 231 && d * d <= x {
        if x % d == 0 {
            return false;
        }
        d += 2
    }

    true
}

fn next_prime(x: u16) -> Option<u16> {
    if x >= 65534 {
        return None;
    }
    let mut next: u16 = x + 1;
    if next % 2 == 0 {
        next += 1;
    }
    while next < MAX_U16 && is_prime(next) == false {
        next += 2;
    } //max_u16
    if next == MAX_U16 {
        return None;
    }

    return Some(next);
}

// For problem 2

fn check_dif(x: u32, y: u32) -> Option<u32> {
    let dif = MAX_U32 - x;
    if dif < y {
        None
    } else {
        Some(x + y)
    }
}

fn check_multiply(x: u32, y: u32) -> Option<u32> {
    let div: u32 = MAX_U32 / x;
    if div < y {
        None
    } else {
        Some(x * y)
    }
}

fn checked_addition_u32(x: u32, y: u32) -> Option<u32> {
    if let Some(s) = check_dif(x, y) {
        return Some(s);
    } else {
        panic!(
            "Checked addition failed with values {} and {} overtaking MAX_U32 ({})",
            x, y, MAX_U32
        );
    }
}

fn checked_multiply_u32(x: u32, y: u32) -> Option<u32> {
    if let Some(m) = check_multiply(x, y) {
        Some(m)
    } else {
        panic!(
            "Check multiply failed with values {} and {} overtaking MAX_U32 ({})",
            x, y, MAX_U32
        );
    }
}

// For problem 3

fn checked_addition_u32_result(x: u32, y: u32) -> Result<u32, Errors> {
    if let Some(result) = check_dif(x, y) {
        Ok(result)
    } else {
        Err(Errors::OvertakingU32)
    }
}

fn checked_multiply_u32_result(x: u32, y: u32) -> Result<u32, Errors> {
    if let Some(result) = check_multiply(x, y) {
        Ok(result)
    } else {
        Err(Errors::OvertakingU32)
    }
}

// For porblem 4

use CharErrors::*;

fn is_ascii(c: char) -> Result<bool, CharErrors> {
    if c as u32 <= 0x7F {
        Ok(true)
    } else {
        Err(NotASCII)
    }
}

fn is_letter(c: char) -> Result<char, CharErrors> {
    let casted_char: u32 = c as u32;
    if (casted_char >= 0x41 && casted_char <= 0x5A) || (casted_char >= 0x61 && casted_char <= 0x7A)
    {
        Ok(c)
    } else {
        Err(NotLetter)
    }
}

fn is_digit(c: char) -> Result<bool, CharErrors> {
    if c as u32 >= 0x30 && c as u32 <= 0x39 {
        Ok(true)
    } else {
        Err(NotDigit)
    }
}

fn is_base16(c: char) -> Result<bool, CharErrors> {
    if is_digit(c).is_ok() == true {
        Ok(true)
    } else if c as u32 >= 0x41 && c as u32 <= 0x46 {
        Ok(true)
    } else {
        Err(NotBase16)
    }
}

fn is_printable(c: char) -> Result<bool, CharErrors> {
    let char_u32 = c as u32;
    if char_u32 > 32 && char_u32 < 127 {
        Ok(true)
    } else {
        Err(NotPrintable)
    }
}

fn to_uppercase(c: char) -> Result<char, CharErrors> {
    let upper_letter = is_letter(c)?.to_ascii_uppercase();

    Ok(upper_letter)
}

fn to_lowercase(c: char) -> Result<char, CharErrors> {
    let lower_letter = is_letter(c)?.to_ascii_lowercase();

    Ok(lower_letter)
}

fn print_char(c: char) -> Result<(), CharErrors> {
    if is_printable(c).is_ok() {
        println!("char : {}", c);
        Ok(())
    } else {
        Err(NotPrintable)
    }
}

fn char_to_number(c: char) -> Result<u32, CharErrors> {
    if is_ascii(c).is_ok() || is_digit(c).is_ok() {
        Ok(c as u32)
    } else {
        Err(NotASCII)
    }
}

fn char_to_number_hex(c: char) -> Result<u32, CharErrors> {
    if is_ascii(c).is_ok() || is_base16(c).is_ok() {
        Ok(c as u32)
    } else {
        Err(NotBase16)
    }
}

fn print_error(error: CharErrors) -> () {
    match error {
        NotASCII => println!("Error : Char not a ASCII character"),
        NotBase16 => println!("Error : Char not a Base16 digit"),
        NotDigit => println!("Error : Char not a digit character"),
        NotLetter => println!("Error : Char not a letter character"),
        NotPrintable => println!("Error : Char not a printable character"),
    }
}

fn main() {
    // Main function with examples

    env::set_var("RUST_BACKTRACE", "full");
    //Problem 1
    let mut x = 6000;
    loop {
        let next: Option<u16> = next_prime(x);
        match next {
            Some(value) => println!("Next prime for {} is {}", x, value),
            _ => {
                println!("The next prime can't be represented on a u16, returned None");
                break;
            }
        }
        x += 1;
    }
    //Problem 2
    //first
    let x: u32 = 1000;
    let y: u32 = 2000;
    let result = checked_addition_u32(x, y);
    match result {
        Some(value) => {
            println!(
                "Checked addition for u32 type done for values {} and {} : result is {}",
                x, y, value
            );
        }
        _ => {
            panic!("This piece of code should never be executed!");
        }
    }
    /* This code panics if executed

    let x: u32 = MAX_U32 - 2;
    let y: u32 = 3;

    let result = checked_addition_u32(x, y);
    match result{
        Some(value) => {println!("Checked addition for u32 type done for values {} and {} : result is {}", x, y, value);}
        _ => {println!("This piece of code should never be executed!");}
    }
    */

    //second
    let x: u32 = 1000;
    let y: u32 = 2000;
    let result = checked_multiply_u32(x, y);
    match result {
        Some(value) => {
            println!(
                "Checked multiply for u32 type done for values {} and {} : result is {}",
                x, y, value
            );
        }
        _ => {
            panic!("This piece of code should never be executed!");
        }
    }
    /*  This code panics if executed

    let x: u32 = MAX_U32 / 2;
    let y: u32 = 3;
    let result = checked_multiply_u32(x, y);
    match result{
        Some(value) => {println!("Checked multiply for u32 type done for values {} and {} : result is {}", x, y, value);}
        _ => {println!("This piece of code should never be executed!");}
    }
    */

    //Problem 3
    //first

    let x: u32 = 1000;
    let y: u32 = 2000;
    let result: Result<u32, Errors> = checked_addition_u32_result(x, y);
    match result{
        Ok(value) => println!(
            "Checked addition for u32 type using Result method done for {} and {} : Result is {}",
            x,
            y,
            value
        ),
        Err(e) => println!("Error : {}", e)
    }

    let x: u32 = MAX_U32 - 2;
    let y: u32 = 3;
    let result: Result<u32, Errors> = checked_addition_u32_result(x, y);
    match result{
        Ok(value) => println!(
            "Checked addition for u32 type using Result method done for {} and {} : Result is {}",
            x,
            y,
            value
        ),
        Err(e) => println!("Error : {}", e)
    }

    //second

    let x: u32 = 1000;
    let y: u32 = 2000;
    let result: Result<u32, Errors> = checked_multiply_u32_result(x, y);
    if result.is_ok() {
        println!(
            "Checked addition for u32 type using Result method done for {} and {} : Result is {}",
            x,
            y,
            result.unwrap()
        );
    } else {
        //print!(("Checked addition for u32 type using Result method propagated an error for values {} and {} : {}", x, y, result.))
        let error = result
            .err()
            .expect("Error when trying to unwrap error type (Errors enum)");
        match error {
            Errors::OvertakingU32 => {
                println!("Checked addition for u32 type using Result method propagated an error for values {} and {} : Overtaking MAX_U32", x, y)
            } //_ => {panic!("This piece of code should never be executed, undefined behavior")}
        }
    }

    let x: u32 = MAX_U32 / 2;
    let y: u32 = 3;
    let result: Result<u32, Errors> = checked_multiply_u32_result(x, y);
    if result.is_ok() {
        println!(
            "Checked multiply for u32 type using Result method done for {} and {} : Result is {}",
            x,
            y,
            result.unwrap()
        );
    } else {
        //print!(("Checked addition for u32 type using Result method propagated an error for values {} and {} : {}", x, y, result.))
        let error = result
            .err()
            .expect("Error when trying to unwrap error type (Errors enum)");
        match error {
            Errors::OvertakingU32 => {
                println!("Checked multiply for u32 type using Result method propagated an error for values {} and {} : Overtaking MAX_U32", x, y)
            } //_ => {panic!("This piece of code should never be executed, undefined behavior")}
        }
    }

    // Problem 4 and 5

    app(); //implements all the functions
}

// Problem 5
// Simple app that iterates through all possible values of u8 and prints info about each one
// (next prime number available on u16, and all the possible states from CharErrors)

fn app() -> () {
    let mut c: u8 = 255;
    print!("\n\nSimple app:\n\n");
    loop {
        println!("Information about value(u32) : {} ", c);
        println!(
            "Next prime number for value {} is {}",
            c,
            next_prime(c as u16).expect("Failed to unwrap next prime number on u16")
        );
        match to_uppercase(c as char) {
            Ok(value) => println!("Uppercase: {}", value),
            Err(e) => print_error(e),
        }
        match to_lowercase(c as char) {
            Ok(value) => println!("Lowercase: {}", value),
            Err(e) => print_error(e),
        }
        match print_char(c as char) {
            Ok(()) => print!(""),
            Err(e) => print_error(e),
        }
        match char_to_number(c as char) {
            Ok(v) => println!("Char to number: {}", v),
            Err(e) => print_error(e),
        }
        match char_to_number_hex(c as char) {
            Ok(v) => {
                let hex_string = format!("{:X}", v);
                println!("Char to number hex: 0x{}", hex_string);
            }
            Err(e) => print_error(e),
        }
        print!("\n");
        if c == 0 {
            return;
        } else {
            c -= 1;
        }
    }
}
