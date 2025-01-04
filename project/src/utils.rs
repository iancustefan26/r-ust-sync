use anyhow::Result;
use chrono::{DateTime, Local};
use fs::{create_dir_all, metadata, File};
use std::collections::HashMap;
use std::io::Read;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};
use std::{fs, time};
use walkdir::WalkDir;
use zip::ZipArchive;

pub use crate::sync::*;

pub fn list_files_in_zip(
    zip_path: &str,
) -> Result<HashMap<String, (LocTypes, SystemTime, String)>> {
    let mut files = HashMap::new();
    let file = File::open(zip_path)?;
    let mut archive = ZipArchive::new(file)?;

    for i in 0..archive.len() {
        let file = archive.by_index(i)?;
        if file.is_dir() {
            continue;
        }
        let file_path = format!("{}/{}", zip_path, file.name());

        let (system_time, human_readable_time) = match file.last_modified() {
            Some(time) => {
                let naive_date = chrono::NaiveDate::from_ymd_opt(
                    time.year().into(),
                    time.month().into(),
                    time.day().into(),
                )
                .and_then(|date| {
                    date.and_hms_opt(
                        time.hour().into(),
                        time.minute().into(),
                        time.second().into(),
                    )
                });
                match naive_date {
                    Some(naive_date) => {
                        let unix_timestamp = naive_date.and_utc().timestamp() as u64;
                        let system_time = UNIX_EPOCH + time::Duration::from_secs(unix_timestamp);
                        (
                            system_time,
                            format!(
                                "{:04}-{:02}-{:02} {:02}:{:02}:{:02}",
                                time.year(),
                                time.month(),
                                time.day(),
                                time.hour(),
                                time.minute(),
                                time.second()
                            ),
                        )
                    }
                    None => (UNIX_EPOCH, "Unknown".to_string()),
                }
            }
            None => (UNIX_EPOCH, "Unknown".to_string()),
        };
        let rel_path = relative_path(zip_path, &file_path).unwrap();
        /*
        println!(
            "Absolute Path : {}\nRelative Path : {}\nLast modified time : {:?} -- {}",
            file_path, rel_path, system_time, human_readable_time
        );
        */
        files.insert(
            rel_path,
            (LocTypes::Zip(file_path), system_time, human_readable_time),
        );
    }
    Ok(files)
}

// Result <Hashmap<relative_path, (absolute_path, unix_epoch modif time, human read. modif time)
pub fn list_files_recursive(dir: &str) -> Result<HashMap<String, (LocTypes, SystemTime, String)>> {
    let mut files = HashMap::new();
    for entry in WalkDir::new(dir) {
        let entry = entry?;
        let entry_path = entry.path().to_str().unwrap().to_string();
        let last_modified_tuple = get_last_modified_time(&entry_path)?;
        let rel_path = relative_path(dir, &entry_path).unwrap();
        /*
        println!(
            "Absoulte Path : {}\nRelative Path: {}\nLast modified time : {:?} -- {}",
            entry.path().display(),
            rel_path,
            last_modified_tuple.0,
            last_modified_tuple.1
        );
        */
        if entry.path().is_dir() {
            files.insert(
                rel_path,
                (
                    LocTypes::Folder(entry_path),
                    last_modified_tuple.0,
                    last_modified_tuple.1,
                ),
            );
        } else {
            files.insert(
                rel_path,
                (
                    LocTypes::SimpleFile(entry_path),
                    last_modified_tuple.0,
                    last_modified_tuple.1,
                ),
            );
        }
    }
    Ok(files)
}

// (unix epoch - human readable time)
fn get_last_modified_time(file_path: &str) -> Result<(SystemTime, String)> {
    let metadata = fs::metadata(file_path)?;
    let modified_time = metadata.modified()?;
    let datetime: DateTime<Local> = modified_time.into();
    Ok((
        modified_time,
        datetime.format("%Y-%m-%d %H:%M:%S").to_string(),
    ))
}

pub fn file_as_bytes(file_path: &str) -> Option<Vec<u8>> {
    if let Some(zip_pos) = file_path.find(".zip") {
        let (zip_path, inner_path) = file_path.split_at(zip_pos + 4); // +4 for ".zip"
        let inner_path = inner_path.trim_start_matches('/');
        let zip_file = File::open(zip_path).ok()?;
        let mut zip_archive = ZipArchive::new(zip_file).ok()?;
        let mut zip_file = zip_archive.by_name(inner_path).ok()?;
        let mut buffer = Vec::new();
        zip_file.read_to_end(&mut buffer).ok()?;

        return Some(buffer);
    }

    let mut file = File::open(file_path).ok()?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).ok()?;
    Some(buffer)
}

pub fn paste_to_file(path: &str, content: &[u8]) -> Result<()> {
    Ok(fs::write(path, content)?)
}

pub fn delete(path: &str) -> Result<()> {
    let path = Path::new(path);

    if path.is_dir() {
        fs::remove_dir_all(path)?;
    } else if path.is_file() {
        fs::remove_file(path)?;
    } else {
        return Err(anyhow::anyhow!("No such file or directory"));
    }

    Ok(())
}

pub fn create(path: &str) -> Result<()> {
    let path = Path::new(path);

    if let Ok(meta) = metadata(path) {
        if meta.is_file() {
            // If the path already exists and is a file
            println!("File exists: {:?}", path);
            return Ok(());
        } else if meta.is_dir() {
            // If the path already exists and is a directory
            println!("Path exists as a directory: {:?}", path);
            return Ok(());
        }
    }

    // If the path doesn't exist or isn't a file, create it
    if let Some(parent) = path.parent() {
        create_dir_all(parent)?;
    }

    File::create(path)?;
    println!("Created: {:?}", path);
    Ok(())
}

fn relative_path(base: &str, target: &str) -> Option<String> {
    let base_path = Path::new(base);
    let target_path = Path::new(target);

    let rel = target_path
        .strip_prefix(base_path)
        .map(|rel_path| rel_path.to_string_lossy().to_string())
        .ok();

    match rel {
        Some(rel) => {
            if rel.is_empty() {
                Some(".".to_string())
            } else {
                Some(rel)
            }
        }
        None => None,
    }
}
