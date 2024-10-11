fn add_chars_n(mut s: String, c: char, n: i32) -> String {
    //first problem
    let mut i = 0 as i32;
    while i < n {
        s.push(c);
        i += 1;
    }

    s
}

fn add_chars_n_ref(s: &mut String, c: char, n: i32) -> () {
    //second problem
    let mut i = 0 as i32;
    while i < n {
        s.push(c);
        i += 1;
    }
}

fn add_space(s: &mut String, n_spaces: i32) -> () {
    //third problem
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
    let mut test_int: i32 = to_add_integer;
    let mut n_digits: i32 = 0;
    while test_int > 0 {
        n_digits += 1;
        test_int /= 10;
    }
    let mut i: i32 = 1;
    let mut every_3_space: i32 = 0;
    while i <= n_digits {
        if every_3_space == 3 {
            every_3_space = 0;
            s.push('_');
        }
        let digit: u8;
        let base: i32 = 10;
        let d: i32 = base.pow((n_digits - i) as u32);
        digit = ((to_add_integer / d) % 10) as u8;
        s.push((digit + 48) as char);
        every_3_space += 1;
        i += 1;
    }
}

fn add_float(s: &mut String, to_add_float: f64) -> () {}

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
    println!("{}\n\n{}", s, another_s); //third problem

    let mut test: String = String::from("");
    add_integer(&mut test, 306437968);
    print!("{}", test);
}
