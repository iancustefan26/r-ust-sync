use anyhow::Result;
use clap::{Arg, Command};
use regex::Regex;

use crate::errors::*;

pub fn parse_args() -> Result<Option<Vec<String>>> {
    let matches = Command::new("advanced_rsync")
        .version("1.0")
        .author("Iancu Stefan <iancustefanteodor@gmail.com>")
        .about("An advanced version of rsync that automates synchronization using Rust")
        .arg(
            Arg::new("locations")
                .short('s')
                .long("set")
                .value_name("LOCATIONS")
                .help("Source and destination locations in the format <LOCATION_TYPE>:<Path_in_location>")
                .required(false)
                .num_args(1..=100),
        )
        .get_matches();

    let locations: Option<Vec<String>> = matches
        .get_many::<String>("locations")
        .map(|vals| vals.cloned().collect());
    let location_regex = Regex::new(r"^(ftp|zip|folder):.+$")?;

    match locations {
        Some(locations) => {
            for loc in &locations {
                if !location_regex.is_match(loc) {
                    return Err(ArgErrors::InvalidLocation(loc.clone()).into());
                }
            }
            Ok(Some(locations))
        }
        None => {
            println!("Running...");
            Ok(None)
        }
    }
}
