use anyhow::Result;
use sync::{ReadOnly, ReadWrite};

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
                    let bytes: Vec<u8> = vec![10, 20, 30, 32, 31, 3, 3, 123, 21, 32, 3, 12];
                    if file.0.to_string().contains("sync/") {
                        println!("File path to delete: {}", file.0);
                        //file.0.delete_file()?;
                        file.0.write_file(&bytes)?;
                    }
                    if file.0.to_string().contains("utils.rs") {
                        println!("File path to copy: {}", file.0);
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
