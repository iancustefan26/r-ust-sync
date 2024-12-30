use anyhow::Result;

mod cli_parsing;
mod errors;
mod sync;

fn main() -> Result<()> {
    cli_parsing::parse_args()?;
    let locations = cli_parsing::retrieve_locations()?;
    for loc in locations {
        println!("{}", loc);
    }
    Ok(())
}
