use anyhow::Result;
use serde_derive::Deserialize;
use std::env;
use std::{fmt, fs};
use std::{thread, time};

const STUDENTS_FILE_CSV: &str = "assets/students.csv";
const STUDENTS_FILE_JSON: &str = "assets/students.jsonl";
const BLINKER_FILE: &str = "assets/life.game";
const PULSAR_FILE: &str = "assets/life2.game";

// Problema 1

#[derive(Debug, Clone, Deserialize)]
struct Student {
    name: String,
    phone: String,
    age: Option<u8>,
}

impl Student {
    fn new() -> Self {
        Student {
            name: "".to_string(),
            phone: "".to_string(),
            age: None,
        }
    }
    fn is_older_than(&self, other: &Student) -> bool {
        match (self.age, other.age) {
            (Some(self_age), Some(other_age)) => self_age > other_age,
            (Some(_), None) => true,
            _ => false,
        }
    }
}

impl fmt::Display for Student {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "\nName: {}\nPhone: {}\nAge: {}\n",
            self.name,
            self.phone,
            self.age.unwrap_or(0)
        )
    }
}

fn main() -> Result<(), anyhow::Error> {
    env::set_var("RUST_BACKTRACE", "1");
    // Problema 1
    let students_text = fs::read_to_string(STUDENTS_FILE_CSV)?;
    let mut oldest_student = Student::new();
    let mut youngest_student = Student::new();
    for student in students_text.lines() {
        let fields_vec: Vec<&str> = student.split(",").collect();
        let fields: [&str; 3] = fields_vec.as_slice().try_into()?;
        let temp_student = Student {
            name: fields[0].trim().to_string(),
            phone: fields[1].trim().to_string(),
            age: Some(
                fields[2]
                    .trim()
                    .parse::<u8>()
                    .expect("Couldn't fit age field into u8 (Invalid age)"),
            ),
        };
        match oldest_student.age {
            None => {
                oldest_student = temp_student.clone();
                youngest_student = temp_student;
                continue;
            }
            Some(_) => (),
        }
        if temp_student.is_older_than(&oldest_student) {
            oldest_student = temp_student;
        } else {
            youngest_student = temp_student;
        }
    }
    println!(
        "Youngest student: {}\nOldest student: {}",
        youngest_student, oldest_student
    );

    // Problema 2 (p2_🎃.rs, pumpkin.png)

    // Problema 3
    let students_content = fs::read_to_string(STUDENTS_FILE_JSON)?;
    let mut oldest_student = Student::new();
    let mut youngest_student = Student::new();
    for line in students_content.lines() {
        if line.is_empty() {
            break; // Potential empty whitespace lines
        }
        let student: Student = serde_json::from_str(line)?;
        match oldest_student.age {
            None => {
                oldest_student = student.clone();
                youngest_student = student;
            }
            Some(_) => {
                if student.is_older_than(&oldest_student) {
                    oldest_student = student;
                } else {
                    youngest_student = student;
                }
            }
        }
    }
    println!(
        "Youngest student (using jsons): {youngest_student}\nOldest student (using jsons): {}",
        oldest_student
    );

    // Bonus

    let matrix_content = fs::read_to_string(BLINKER_FILE)?;
    let mut matrix: Matrix = Vec::new();
    for line in matrix_content.lines() {
        matrix.push(line.bytes().collect());
    }
    simulate_game(&mut matrix, 5)?;
    let matrix_content = fs::read_to_string(PULSAR_FILE)?;
    let mut matrix: Matrix = Vec::new();
    for line in matrix_content.lines() {
        matrix.push(line.bytes().collect());
    }
    simulate_game(&mut matrix, 5)?;
    Ok(())
}

// Bonus

type Matrix = Vec<Vec<u8>>;
const NI: [i8; 8] = [-1, -1, -1, 0, 0, 1, 1, 1];
const NJ: [i8; 8] = [-1, 0, 1, -1, 1, -1, 0, 1];

fn get_n_of_neighbours_alive(
    matrix: &Matrix,
    x: usize,
    y: usize,
    n: usize,
    m: usize,
) -> Result<u8> {
    let mut alives: u8 = 0;
    for dirs in NI.iter().zip(NJ.iter()) {
        if (dirs.0 + x as i8) as usize == n
            || (dirs.1 + y as i8) as usize == m
            || dirs.0 + x as i8 == -1
            || dirs.1 + y as i8 == -1
            || matrix[(dirs.0 + x as i8) as usize][(dirs.1 + y as i8) as usize] == 32
        {
            alives += 0;
        } else {
            alives += 1;
        }
    }

    Ok(alives)
}

fn simulate_game(matrix: &mut Matrix, n_of_sims: i32) -> Result<()> {
    let n_lines = matrix.len();
    let mut copy_matrix: Matrix = matrix.clone();
    print(matrix);
    for _ in 0..n_of_sims {
        thread::sleep(time::Duration::from_millis(200));
        print!("\x1B[2J\x1B[H");
        for i in 0..n_lines {
            let m_columns = matrix[i].len();
            for j in 0..m_columns {
                let neighbours = get_n_of_neighbours_alive(matrix, i, j, n_lines, m_columns)?;
                copy_matrix[i][j] = match matrix[i][j] {
                    32 => match neighbours {
                        3 => 120,
                        _ => 32,
                    },
                    _ => match neighbours {
                        2 | 3 => 120,
                        _ => 32,
                    },
                };
            }
        }
        *matrix = copy_matrix.clone();
        print(matrix);
    }
    Ok(())
}

fn print(matrix: &Matrix) {
    for line in matrix {
        for c in line {
            print!("{} ", *c as char);
        }
        println!();
    }
    println!();
}
