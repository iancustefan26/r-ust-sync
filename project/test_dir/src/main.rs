use anyhow::Result;
use sync::Synchronizer;

pub mod cli_parsing;
pub mod errors;
pub mod sync;
pub mod utils;

fn main() -> Result<()> {
    cli_parsing::parse_args()?;
    let locations = cli_parsing::retrieve_locations()?;
    let adv_rsync = Synchronizer::new(locations);
    adv_rsync.sync()?;
    // Fara comentariu
    Ok(())
}
