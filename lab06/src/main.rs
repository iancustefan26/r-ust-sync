use anyhow::Result;
use errors::CommandErrors::{self, *};
use std::{fs, time};
mod errors;
use rusqlite::Connection;

const INPUT1: &str = "assets/input1.txt";
const DATABASE: &str = "assets/database/bookmarks.db";

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

struct BookmarkCommand {
    connection: Connection,
}

struct Bookmark {
    name: String,
    link: String,
}

impl BookmarkCommand {
    fn new() -> Result<Self> {
        let instance = BookmarkCommand {
            connection: Connection::open(DATABASE)?,
        };
        let create = r"
        CREATE TABLE IF NOT EXISTS bookmarks (
            name text    NOT NULL,
            link text    NOT NULL
        )
        ";
        instance.connection.execute(create, ())?;

        Ok(instance)
    }
    fn add(&mut self, name: &str, link: &str) -> Result<()> {
        let added = self.connection.execute(
            "INSERT INTO bookmarks (name, link) VALUES
            (?1, ?2)",
            (name, link),
        )?;
        println!("Added rows: {added}");
        Ok(())
    }
    fn search(&self, name: &str) -> Result<()> {
        let like_pattern = format!("%{}%", name);
        let mut stmt = self
            .connection
            .prepare("SELECT DISTINCT * FROM bookmarks WHERE name LIKE ?1")?;
        let bookmark_iter = stmt.query_map([&like_pattern], |row| {
            Ok(Bookmark {
                name: row.get("name")?,
                link: row.get("link")?,
            })
        })?;
        for i in bookmark_iter {
            let i = i?;
            println!("name={}, link={}", i.name, i.link);
        }
        Ok(())
    }
}

impl Emulate for BookmarkCommand {
    fn get_name(&self) -> &str {
        "bk"
    }
    fn exec(&mut self, args: &str) -> Result<(), CommandErrors> {
        let args: Vec<&str> = args.split_ascii_whitespace().collect();
        let n_of_args = args.len();
        // Aici un improvement ar fi sa folosesc un regex dar o voi face ulterior
        if n_of_args == 2 && args[0] == "search" {
            // search
            println!("Args search: {} {}", args[0], args[1]);
            match self.search(args[1]) {
                Err(e) => {
                    println!("Error (db): {}", e);
                    return Err(Unexpected);
                }
                Ok(_) => return Ok(()),
            }
        } else if n_of_args == 3 && args[0] == "add" {
            // add
            println!("Args add: {} {} {}", args[0], args[1], args[2]);
            match self.add(args[1], args[2]) {
                Err(e) => {
                    println!("Error (db): {}", e);
                    return Err(Unexpected);
                }
                Ok(_) => return Ok(()),
            }
        } else {
            return Err(BadArgs);
        }
    }
}

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
    terminal.register(Box::new(BookmarkCommand::new()?));
    terminal.run()?;

    Ok(())
}
