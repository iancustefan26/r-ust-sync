use crate::sync::LocTypes;
use anyhow::Result;
use chrono::{Datelike, NaiveDateTime};
use ftp::FtpStream;
use std::collections::HashMap;

use std::time::SystemTime;

pub fn connect_to_ftp(
    user: &str,
    password: &str,
    url: &str,
    path: &str,
) -> Result<HashMap<String, (LocTypes, SystemTime, String)>> {
    println!("{} - {} - {} - {}", user, password, url, path);
    let mut ftp_stream = FtpStream::connect(format!("{}:21", url))?;

    ftp_stream.login(user, password)?;
    ftp_stream.cwd(path)?;

    let mut files: HashMap<String, (LocTypes, SystemTime, String)> = HashMap::new();
    recursive_list(&mut ftp_stream, "".to_string(), &mut files)?;

    ftp_stream.quit()?;

    Ok(files)
}

fn recursive_list(
    ftp_stream: &mut FtpStream,
    current_path: String,
    hash_map: &mut HashMap<String, (LocTypes, SystemTime, String)>,
) -> Result<()> {
    let entries = ftp_stream.list(None)?;

    for entry in entries {
        let (entry, system_time, human_read_systime) = extract_ftp_file_data(entry);
        let path = format!("{}{}", current_path, entry);
        // Try changing into the entry to check if it’s a directory
        if ftp_stream.cwd(&entry).is_ok() {
            // Insert folder info before recursion
            hash_map.insert(
                path.clone(),
                (
                    LocTypes::Folder(path.clone()),
                    system_time,
                    human_read_systime,
                ),
            );

            // Recursive call to process subdirectory
            {
                // Create a shorter borrow scope for the recursive call
                let new_path = format!("{}/", entry);
                recursive_list(ftp_stream, new_path, hash_map)?;
            }

            // Navigate back to the parent directory
            ftp_stream.cdup()?; // cd ..
        } else {
            // Insert file info
            hash_map.insert(
                path.clone(),
                (
                    LocTypes::SimpleFile(path.clone()),
                    system_time,
                    human_read_systime,
                ),
            );
        }
    }
    Ok(())
}

fn extract_ftp_file_data(entry: String) -> (String, SystemTime, String) {
    let columns: Vec<&str> = entry.split_whitespace().collect();

    let file_name = columns[8..].join(" ");

    let month = columns[5];
    let day = columns[6];
    let time_or_year = columns[7];

    let current_year = chrono::Local::now().year(); // Current year
    let year: i32;
    let time: &str;

    if time_or_year.contains(':') {
        year = current_year;
        time = time_or_year;
    } else {
        year = time_or_year.parse().unwrap();
        time = "00:00"; // Default
    }

    let datetime_str = format!("{} {} {} {}", year, month, day, time);
    let naive_datetime = NaiveDateTime::parse_from_str(&datetime_str, "%Y %b %d %H:%M")
        .expect("Failed to parse datetime");

    let system_time = SystemTime::UNIX_EPOCH
        + std::time::Duration::from_secs(naive_datetime.and_utc().timestamp() as u64);

    let human_readable = naive_datetime.format("%Y-%m-%d %H:%M:%S").to_string();

    (file_name, system_time, human_readable)
}
