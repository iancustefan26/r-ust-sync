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
    recursive_list(
        user,
        password,
        url,
        path.to_string(),
        "".to_string(),
        &mut ftp_stream,
        &mut files,
    )?;

    ftp_stream.quit()?;

    Ok(files)
}

fn recursive_list(
    user: &str,
    pass: &str,
    url: &str,
    root_path: String,
    rel_path: String,
    ftp_stream: &mut FtpStream,
    hash_map: &mut HashMap<String, (LocTypes, SystemTime, String)>,
) -> Result<()> {
    let entries = ftp_stream.list(None)?;

    for entry in entries {
        let (entry, system_time, human_read_systime) = extract_ftp_file_data(entry);
        let abs_path = format!("{}/{}", root_path.clone(), entry);
        let rel_path = format!("{}/{}", rel_path.clone(), entry)
            .trim_start_matches("/")
            .to_string();
        if ftp_stream.cwd(&entry).is_ok() {
            // Insert folder info before recursion
            hash_map.insert(
                rel_path.clone(),
                (
                    LocTypes::Ftp(
                        user.to_string(),
                        pass.to_string(),
                        url.to_string(),
                        abs_path.clone(),
                    ),
                    system_time,
                    human_read_systime,
                ),
            );
            // Recursive call to process subdirectory
            {
                // Create a shorter borrow scope for the recursive call
                let new_root_path = format!("{}/{}", root_path, entry);
                recursive_list(
                    user,
                    pass,
                    url,
                    new_root_path,
                    rel_path,
                    ftp_stream,
                    hash_map,
                )?;
            }
            ftp_stream.cdup()?; // cd ..
        } else {
            hash_map.insert(
                rel_path.clone(),
                (
                    LocTypes::Ftp(
                        user.to_string(),
                        pass.to_string(),
                        url.to_string(),
                        abs_path.clone(),
                    ),
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

pub fn read_ftp_file(user: &str, pass: &str, url: &str, path: &str) -> Option<Vec<u8>> {
    let mut ftp_stream = FtpStream::connect(format!("{}:21", url)).ok()?;

    ftp_stream.login(user, pass).ok()?;

    if ftp_stream.cwd(path).is_ok() {
        ftp_stream.cdup().ok()?; // cd ..
        return Some(Vec::new());
    }

    if let Some((dir, file_name)) = path.rsplit_once('/') {
        ftp_stream.cwd(dir).ok()?;
        let reader = ftp_stream.simple_retr(file_name).ok()?;
        Some(reader.into_inner())
    } else {
        let reader = ftp_stream.simple_retr(path).ok()?;
        Some(reader.into_inner())
    }
}
