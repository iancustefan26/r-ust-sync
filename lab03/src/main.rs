use core::panic;
use std::env;

const MAX_U16: u16 = 65535;
const MAX_U32: u32 = 4_294_967_295;

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

fn main() {
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
            println!("This piece of code should never be executed!");
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
            println!("This piece of code should never be executed!");
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
}
