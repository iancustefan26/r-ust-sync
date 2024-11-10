use anyhow::Result;
use errors::CommandErrors::{self, *};
use std::{fs, time};
mod errors;

const INPUT1: &str = "assets/input1.txt";

fn split_by_newline_or_whitespace(string: &str) -> Option<(&str, &str)> {
    if let Some(sth) = string.trim_ascii().split_once(" ") {
        return Some(sth);
    }
    Some((string, ""))
}

trait Emulate {
    fn get_name(&self) -> &str;
    fn exec(&mut self, args: &str) -> Result<(), CommandErrors>;
}

struct PingCommand {}
struct CountCommand {}
struct TimesCommand {
    count: u32,
}
struct ElapsedCommand {
    elapsed: time::Instant,
}
struct StopCommand {}

impl Emulate for PingCommand {
    fn get_name(&self) -> &str {
        "ping"
    }
    fn exec(&mut self, args: &str) -> Result<(), CommandErrors> {
        if args.is_empty() {
            println!("pong!");
            return Ok(());
        }
        Err(BadArgs)
    }
}

impl Emulate for CountCommand {
    fn get_name(&self) -> &str {
        "count"
    }
    fn exec(&mut self, args: &str) -> Result<(), CommandErrors> {
        if args.is_empty() {
            return Err(BadArgs);
        }
        println!("counted {} args", args.split_whitespace().count());
        Ok(())
    }
}

impl Emulate for TimesCommand {
    fn get_name(&self) -> &str {
        "times"
    }
    fn exec(&mut self, args: &str) -> Result<(), CommandErrors> {
        if !args.is_empty() {
            return Err(BadArgs);
        }
        println!("times : {}", self.count);
        self.count += 1;
        Ok(())
    }
}

impl Emulate for StopCommand {
    fn get_name(&self) -> &str {
        "stop"
    }
    fn exec(&mut self, args: &str) -> Result<(), CommandErrors> {
        if !args.is_empty() {
            return Err(CommandErrors::BadArgs);
        }
        println!("{}", CommandErrors::Stop);
        Ok(())
    }
}

impl Emulate for ElapsedCommand {
    fn get_name(&self) -> &str {
        "elapsed"
    }
    fn exec(&mut self, args: &str) -> Result<(), CommandErrors> {
        if !args.is_empty() {
            return Err(CommandErrors::BadArgs);
        }
        println!("Elapsed: {:?}", self.elapsed);
        Ok(())
    }
}

#[derive(Default)]
struct Terminal {
    commands: Vec<Box<dyn Emulate>>,
}

impl Terminal {
    fn new() -> Self {
        Terminal::default()
    }
    fn register(&mut self, to_add: Box<dyn Emulate>) {
        self.commands.push(to_add);
    }
    fn run(&mut self) -> Result<()> {
        let text_input = fs::read_to_string(INPUT1)?;
        'outer_line: for line in text_input.lines() {
            let command = split_by_newline_or_whitespace(line);
            match command {
                Some(command) => {
                    let args = command.1;
                    let command = command.0.to_string();
                    for comm in self.commands.iter_mut() {
                        if command == comm.get_name() {
                            match comm.exec(args) {
                                Err(e) => match e {
                                    Stop => return Ok(()),
                                    _ => println!("Error: {}", e),
                                },
                                Ok(_) => continue 'outer_line,
                            }
                            continue 'outer_line;
                        } else if command.to_lowercase() == comm.get_name() {
                            println!("Error: {}, try lowercase", CommandErrors::IncorrectSpell);
                            continue 'outer_line;
                        }
                    }
                    println!("Error: {}", CommandErrors::NotFound);
                }
                None => println!("Error: {}", CommandErrors::NotFound),
            }
        }
        Ok(())
    }
}

fn main() -> Result<()> {
    let mut terminal = Terminal::new();
    terminal.register(Box::new(PingCommand {}));
    terminal.register(Box::new(CountCommand {}));
    terminal.register(Box::new(TimesCommand { count: 0 }));
    terminal.register(Box::new(ElapsedCommand {
        elapsed: time::Instant::now(),
    }));
    terminal.register(Box::new(StopCommand {}));
    terminal.run()?;

    Ok(())
}
