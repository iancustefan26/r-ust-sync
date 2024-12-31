use anyhow::Result;
use sync::ReadOnly;

pub mod cli_parsing;
pub mod errors;
pub mod sync;
pub mod utils;

fn main() -> Result<()> {
    cli_parsing::parse_args()?;
    let locations = cli_parsing::retrieve_locations()?;
    for loc in locations {
        println!("{}", loc);
        let files = loc.list_files();
        match files {
            Ok(files) => {
                for file in files {
                    let file_bytes = file.0.read_file();
                    match file_bytes {
                        Some(bytes) => {
                            println!("Content of file {} : {:?}", file.0, bytes);
                        }
                        None => {
                            print!("")
                        }
                    }
                }
            }
            Err(e) => {
                println!(
                    "An error occured while listing files inside : {} - {}",
                    loc, e
                )
            }
        }
    }
    Ok(())
}
