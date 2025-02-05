use anyhow::Result;
use clap::{Arg, Command};
use regex::Regex;
use std::collections::HashSet;
use std::fs::OpenOptions;
use std::fs::{self, File};
use std::io::{BufRead, BufReader, Write};
use std::path::Path;
use std::path::PathBuf;

use crate::{errors::*, sync::*};

// Function that retrieves the config file (and creates it if it does not exist)
fn config_file() -> Result<String> {
    let home_dir = dirs_next::home_dir().expect("Failed to find home directory; could not retrieve locations; Try creating /home/user/.adv_rsync/cfg/locations.cfg");

    let cfg_path: PathBuf = home_dir.join(".adv_rsync/cfg/locations.cfg");
    if cfg_path.exists() {
        return Ok(cfg_path.to_str().unwrap_or("").to_string());
    }
    if let Some(parent) = cfg_path.parent() {
        fs::create_dir_all(parent)?;
    }

    fs::File::create(&cfg_path)?;

    Ok(cfg_path.to_str().unwrap_or("").to_string())
}

// Appending the given arguments to the config file
fn append_to_cfg(locations: &Vec<String>) -> Result<()> {
    let cfg_file = config_file()?;
    let mut existing_locations = HashSet::new();
    let file = fs::File::open(&cfg_file)?;
    let reader = BufReader::new(file);
    for line in reader.lines() {
        existing_locations.insert(line?);
    }

    let mut cfg_file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(cfg_file)?;

    for location in locations {
        if !existing_locations.contains(location) {
            writeln!(cfg_file, "\n{}", location)?;
        }
    }
    Ok(())
}

// Parsing the given arguments
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
                .help("Source and destination locations that will be added to the cfg file in the format <LOCATION_TYPE>:<Path_in_location>")
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
            append_to_cfg(&locations)?;
            Ok(Some(locations))
        }
        None => {
            println!("Running...");
            Ok(None)
        }
    }
}

// Reading from the CFG file for running
pub fn retrieve_locations() -> Result<Vec<LocTypes>> {
    let cfg_file = config_file()?;
    if !Path::new(&cfg_file).exists() {
        File::create(&cfg_file)?;
    }
    let content = fs::read_to_string(cfg_file)?;
    if content.is_empty() {
        return Err(ArgErrors::EmptyCfg.into());
    }
    let mut locations = Vec::new();
    let location_regex = Regex::new(r"^(ftp|zip|folder):.+$")?;

    for (index, line) in content.lines().enumerate() {
        if !location_regex.is_match(line) {
            if line != "\n" {
                println!(
                "Line {}: Could not parse location and will not be taken into consideration : {}",
                index, line
            );
            }
        } else {
            let type_path = line.split_once(":").unwrap();
            match type_path.0 {
                "ftp" => {
                    let (user_pass, url_path) = type_path.1.split_once("@").unwrap_or_default();
                    let user_pass = user_pass.split_once(":").unwrap_or_default();
                    let url_path = url_path.split_once("/").unwrap_or_default();
                    locations.push(LocTypes::Ftp(
                        user_pass.0.to_string(),
                        user_pass.1.to_string(),
                        url_path.0.to_string(),
                        url_path.1.to_string(),
                    ));
                }
                "folder" => {
                    locations.push(LocTypes::Folder(type_path.1.to_string()));
                }
                "zip" => {
                    locations.push(LocTypes::Zip(type_path.1.to_string()));
                }
                _ => {
                    println!("Line {}: file type unrecognized: {}", index, line);
                }
            }
        }
    }

    Ok(locations)
}
