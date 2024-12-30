use anyhow::Result;
use sync::ReadOnly;

mod cli_parsing;
mod errors;
mod sync;
pub mod utils;

fn main() -> Result<()> {
    cli_parsing::parse_args()?;
    let locations = cli_parsing::retrieve_locations()?;
    for loc in locations {
        println!("{}", loc);
        loc.list_files();
    }
    Ok(())
}
