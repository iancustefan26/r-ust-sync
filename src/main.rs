use anyhow::Result;
use sync::Synchronizer;

pub mod cli_parsing;
pub mod errors;
pub mod sync;
pub mod utils;

fn main() -> Result<()> {
    cli_parsing::parse_args()?;
    let locations = cli_parsing::retrieve_locations()?;
    let mut adv_rsync = Synchronizer::new(locations, None);
    adv_rsync.sync()?;
    Ok(())
}
