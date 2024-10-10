
fn add_chars_n_ref(s: &mut String, c: char, n: i32) -> (){
    let mut i = 0 as i32;
    while i < n{
        s.push(c);
        i += 1;
    }
}

fn add_chars_n(mut s: String, c: char, n: i32) -> String {
    let mut i = 0 as i32;
    while i < n{
        s.push(c);
        i += 1;
    }

    s
}

fn main() {
    let mut s = String::from("");
    let mut another_s: String = String::from("");
    let mut i = 0;
    let alpha_count: i32 = 26;
    while i < alpha_count {
        let c = (i as u8 + 'a' as u8) as char;
        s = add_chars_n(s, c, alpha_count - i);
        add_chars_n_ref(&mut another_s, c, alpha_count - i);

        i += 1;
    }

    print!("{}\n\n{}", s, another_s);
}

