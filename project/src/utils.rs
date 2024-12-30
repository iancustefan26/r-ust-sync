use anyhow::Result;
use chrono::{DateTime, Local};
use std::collections::HashMap;
use std::fs;
use std::time::SystemTime;
use walkdir::WalkDir;

pub fn list_files_recursive(dir: &str) -> Result<HashMap<String, (SystemTime, String)>> {
    let mut files = HashMap::new();
    for entry in WalkDir::new(dir) {
        let entry = entry?;
        let entry_path = entry.path().to_str().unwrap().to_string();
        if !entry.file_type().is_dir() {
            let last_modified_tuple = get_last_modified_time(&entry_path)?;
            println!(
                "Path : {}\nLast modified time : {:?} -- {}",
                entry.path().display(),
                last_modified_tuple.0,
                last_modified_tuple.1
            );
            files.insert(entry_path, last_modified_tuple);
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
