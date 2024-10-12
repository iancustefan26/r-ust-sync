// lab02 Rust homework

fn add_chars_n(mut s: String, c: char, n: i32) -> String {
    let mut i = 0 as i32;
    while i < n {
        s.push(c);
        i += 1;
    }

    s
}

fn add_chars_n_ref(s: &mut String, c: char, n: i32) -> () {
    let mut i = 0 as i32;
    while i < n {
        s.push(c);
        i += 1;
    }
}

fn add_space(s: &mut String, n_spaces: i32) -> () {
    let mut i: i32 = 0;
    while i < n_spaces {
        s.push(' ');
        i += 1;
    }
}

fn add_str(s: &mut String, to_add_s: &str) -> () {
    s.push_str(to_add_s);
}

fn add_integer(s: &mut String, to_add_integer: i32) -> () {
    if to_add_integer == 0 {
        s.push((to_add_integer as u8 + 48) as char);
    }
    //number of figures is need for pushing the integer from the front to the back
    let mut test_int: i32 = to_add_integer;
    let mut n_digits: u32 = 0;
    while test_int > 0 {
        n_digits += 1;
        test_int /= 10;
    }
    let mut i: u32 = 1;
    let mut every_3_space: i32 = 0;
    while i <= n_digits {
        if every_3_space == 3 {
            //every 3 spaces a '_' will be pushed to the string
            every_3_space = 0;
            s.push('_');
        }
        let digit: u8;
        let base: i32 = 10;
        let d: i32 = base.pow(n_digits - i);
        digit = ((to_add_integer / d) % 10) as u8;
        s.push((digit + 48) as char);
        every_3_space += 1;
        i += 1;
    }
}

fn add_float(s: &mut String, to_add_float: f32, mut n_of_decimals: i32) -> () {
    //I put n_of_decimals as an arg because when I pass 2.038 as f32 as an arg
    //It converts it to 2.03800011 because single-precision floating point can't hold this float number
    //Moreover, some numbers like 2.10038 will not be converted the right way (2.100379..) (as I have tested)

    let integer_part: i32 = to_add_float as i32; // 2
    let mut float_part: f32 = to_add_float - integer_part as f32; // 0.038

    //push the integer part to the string
    add_integer(s, integer_part);
    s.push('.');
    //the decimal part
    while n_of_decimals > 0 {
        float_part = 10 as f32 * float_part;
        add_integer(s, (float_part as i32) % 10);
        n_of_decimals -= 1;
    }
}

fn main() {
    let mut s = String::from("");
    let mut another_s: String = String::from("");
    let mut i = 0;
    let alpha_count: i32 = 26;
    while i < alpha_count {
        let c = (i as u8 + 'a' as u8) as char;
        s = add_chars_n(s, c, alpha_count - i); //first problem
        add_chars_n_ref(&mut another_s, c, alpha_count - i); //second problem
        i += 1;
    }
    println!("First problem:\n{}\n\nSecond Problem:\n{}\n", s, another_s);

    let mut i_love_rust_string: String = String::from(""); //third problem
    add_space(&mut i_love_rust_string, 40);
    add_str(&mut i_love_rust_string, "I");
    add_space(&mut i_love_rust_string, 1);
    add_str(&mut i_love_rust_string, "💚\n");
    add_space(&mut i_love_rust_string, 40);
    add_str(&mut i_love_rust_string, "RUST\n\n");
    add_space(&mut i_love_rust_string, 4);
    add_str(&mut i_love_rust_string, "Most");
    add_space(&mut i_love_rust_string, 12);
    add_str(&mut i_love_rust_string, "crate");
    add_space(&mut i_love_rust_string, 6);
    add_integer(&mut i_love_rust_string, 306437968);
    add_space(&mut i_love_rust_string, 11);
    add_str(&mut i_love_rust_string, "and");
    add_space(&mut i_love_rust_string, 5);
    add_str(&mut i_love_rust_string, "latest");
    add_space(&mut i_love_rust_string, 9);
    add_str(&mut i_love_rust_string, "is\n");
    add_space(&mut i_love_rust_string, 9);
    add_str(&mut i_love_rust_string, "downloaded");
    add_space(&mut i_love_rust_string, 8);
    add_str(&mut i_love_rust_string, "has");
    add_space(&mut i_love_rust_string, 13);
    add_str(&mut i_love_rust_string, "downloads");
    add_space(&mut i_love_rust_string, 5);
    add_str(&mut i_love_rust_string, "the");
    add_space(&mut i_love_rust_string, 9);
    add_str(&mut i_love_rust_string, "version");
    add_space(&mut i_love_rust_string, 4);
    add_float(&mut i_love_rust_string, 2.038 as f32, 3);
    add_str(&mut i_love_rust_string, ".");

    print!("Third problem:\n{}", i_love_rust_string);
}
