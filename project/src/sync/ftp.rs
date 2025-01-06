use crate::sync::LocTypes;
use anyhow::Result;
use chrono::{Datelike, NaiveDateTime};
use ftp::FtpStream;
use std::collections::HashMap;

use std::io::Cursor;
use std::path::Path;

use std::time::SystemTime;

use super::CreateType;

pub fn connect_to_ftp(
    user: &str,
    password: &str,
    url: &str,
    path: &str,
) -> Result<HashMap<String, (LocTypes, SystemTime, String)>> {
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

pub fn put_file(
    file_bytes: &[u8],
    user: &str,
    pass: &str,
    url: &str,
    ftp_path: &str,
) -> Result<()> {
    let mut ftp_stream = FtpStream::connect(format!("{}:21", url))?;

    // Log in with the provided username and password
    ftp_stream.login(user, pass)?;
    let wdir = ftp_path.rsplit_once("/");
    if let Some(wdir) = wdir {
        ftp_stream.cwd(wdir.0)?;
        let mut reader = Cursor::new(file_bytes);
        ftp_stream.put(wdir.1, &mut reader)?;
    } else {
        let mut reader = Cursor::new(file_bytes);
        ftp_stream.put(ftp_path, &mut reader)?;
    }

    // Logout and close the connection
    ftp_stream.quit()?;

    Ok(())
}

pub fn create_ftp_file(
    user: &str,
    pass: &str,
    url: &str,
    path: &str,
    create_type: CreateType,
) -> Result<()> {
    let mut ftp_stream = FtpStream::connect(format!("{}:21", url))?;
    ftp_stream.login(user, pass)?;

    let path = Path::new(path);
    if let Some(parent_dirs) = path.parent() {
        for dir in parent_dirs.iter() {
            let dir_str = dir.to_string_lossy();
            if ftp_stream.cwd(&dir_str).is_err() {
                // Dir does not exist, create it
                ftp_stream.mkdir(&dir_str)?;
                ftp_stream.cwd(&dir_str)?;
            }
        }
    }
    match create_type {
        CreateType::File => {
            let file_name = path
                .file_name()
                .unwrap_or_default()
                .to_str()
                .unwrap_or_default();
            let empty_bytes: Vec<u8> = Vec::new();
            let mut file_contents = Cursor::new(empty_bytes);
            ftp_stream.put(file_name, &mut file_contents)?;
        }
        CreateType::Folder => {
            // if the path itself a directory
            let dir_str = path.to_string_lossy();
            ftp_stream.mkdir(&dir_str)?;
        }
    }
    ftp_stream.quit()?;
    Ok(())
}

pub fn delete_ftp_file(user: &str, pass: &str, url: &str, path: &str) -> Result<()> {
    // Connect to the FTP server
    let mut ftp_stream = FtpStream::connect(format!("{}:21", url))?;
    ftp_stream.login(user, pass)?;

    recursive_delete(&mut ftp_stream, path)?;

    ftp_stream.quit()?;
    Ok(())
}

fn recursive_delete(ftp_stream: &mut FtpStream, path: &str) -> Result<()> {
    // Check if the path is a directory
    if ftp_stream.cwd(path).is_ok() {
        // If it's a directory, list its contents
        let items = ftp_stream.nlst(None)?;
        for item in items {
            let item_path = format!("{}/{}", path, item);
            // Recursively delete the item
            recursive_delete(ftp_stream, &item_path)?;
        }
        // Change back to parent directory before deleting
        ftp_stream.cdup()?;
        // Delete the directory itself
        let dir = path.rsplit_once("/").unwrap_or(("", path));
        ftp_stream.rmdir(dir.1)?;
    } else {
        // If it's a file, delete it
        ftp_stream.rm(path)?;
    }
    Ok(())
}
