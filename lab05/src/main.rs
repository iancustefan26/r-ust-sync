use anyhow::Result;
use serde_derive::Deserialize;
use std::{fmt, fs};

const STUDENTS_FILE_CSV: &str = "assets/students.csv";
const STUDENTS_FILE_JSON: &str = "assets/students.jsonl";

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
    // Problema 1
    let students_text = fs::read_to_string(STUDENTS_FILE_CSV)?;
    let mut oldest_student = Student::new();
    let mut youngest_student = Student::new();
    for student in students_text.lines() {
        let fields: Vec<&str> = student.split(",").collect();
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
        "Youngest student (using jsons): {}\nOldest student (using jsons): {}",
        youngest_student, oldest_student
    );
    Ok(())
}
