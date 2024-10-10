fn is_prime(n: i32) -> bool {
    if n < 2 {
        return false;
    }
    if n == 2 {
        return true;
    }
    if n % 2 == 0 {
        return false;
    }
    let mut d: i32 = 3;
    while d * d <= n {
        if n % d == 0 {
            return false;
        }
        d += 2;
    }

    true
}

fn coprimes(mut x: i32, mut y: i32) -> bool {
    let cmmdc: i32 = loop {
        if x > y {
            x -= y;
        } else if x < y {
            y -= x;
        } else {
            break x;
        }
    };
    if cmmdc == 1 {
        return true;
    }

    false
}

fn main() {
    println!("Hello, world!\n");
    let mut i = 0i32; //first exercise
    while i <= 100 {
        println!("is_prime({}) = {}", i, is_prime(i));
        i += 1;
    }
    let mut number: i32 = 1; //second
    while number <= 100 {
        let mut another_number: i32 = 1;
        println!("\n----------------\nCoprimes for {number}: \n");
        while another_number <= 100 {
            if coprimes(number, another_number) {
                print!("({}, {}) ", number, another_number);
            }
            another_number += 1;
        }
        number += 1;
    }
    let mut beers: i32 = 99;
    while beers >= 1 {
        println!("{} bottles of beer on the wall\n{} bottles of beer\nTake one down, pass it around\n{} bottles of beer on the wall\n", beers, beers, beers - 1);

        beers -= 1;
    }
}
