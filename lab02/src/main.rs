

fn add_chars_n(mut s: String, c: char, n: i32) -> String {  //first problem
    let mut i = 0 as i32;
    while i < n{
        s.push(c);
        i += 1;
    }

    s
}

fn add_chars_n_ref(s: &mut String, c: char, n: i32) -> (){  //second problem
    let mut i = 0 as i32;
    while i < n{
        s.push(c);
        i += 1;
    }
}

fn add_space(s: &mut String) -> (){     //third problem
    s.push(' ');
}

fn add_str(s: &mut String, to_add_s: &str) -> (){
    s.push_str(to_add_s);
}

fn add_integer(s: &mut String, to_add_integer: i32) -> (){
    let new_integer_s = to_add_integer.to_string() as String;
    s.push_str(&new_integer_s);
}


fn add_float(s: &mut String, to_add_float: f64) -> (){
    let float_as_s: String = to_add_float.to_string();
    s.push_str(&float_as_s);
}

fn main() {
    let mut s = String::from("");
    let mut another_s: String = String::from("");
    let mut i = 0;
    let alpha_count: i32 = 26;
    while i < alpha_count {
        let c = (i as u8 + 'a' as u8) as char;
        s = add_chars_n(s, c, alpha_count - i);  //first problem
        add_chars_n_ref(&mut another_s, c, alpha_count - i); //second problem

        i += 1;
    }
    add_space(&mut s);
    print!("{}\n\n{}", s, another_s);   //third problem
    let example: String = String::from("                                        I 💚
                                        RUST.

    Most            crate      306_437_968           and     lastest         is
         downloaded        has             downloads     the         version    2.038.
                    
");
    
}

