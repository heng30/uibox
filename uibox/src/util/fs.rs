use std::fs;
use std::io;
use std::path::Path;

pub fn dir_size(path: &str) -> Result<String, Box<dyn std::error::Error>> {
    let mut total_size: u64 = 0;
    let metadata = fs::metadata(path)?;

    if metadata.is_dir() {
        total_size += get_dir_size(Path::new(path))?;
    } else {
        total_size += metadata.len();
    }

    Ok(format!("{:.2}M", total_size as f64 / 1024.0 / 1024.0))
}

fn get_dir_size(path: &Path) -> Result<u64, Box<dyn std::error::Error>> {
    let mut total_size: u64 = 0;

    for entry in fs::read_dir(path)? {
        let path = entry?.path();
        let metadata = fs::metadata(&path)?;

        if metadata.is_dir() {
            total_size += get_dir_size(&path)?;
        } else {
            total_size += metadata.len();
        }
    }

    Ok(total_size)
}

pub fn remove_dir_files(path: &str) -> io::Result<()> {
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        if entry.file_type()?.is_file() {
            fs::remove_file(entry.path())?;
        } else if entry.file_type()?.is_dir() {
            match entry.path().to_str() {
                Some(path) => remove_dir_files(path)?,
                _ => (),
            }
            fs::remove_dir(entry.path())?;
        }
    }
    Ok(())
}

#[allow(dead_code)]
pub fn file_exist(path: &str) -> bool {
    match fs::metadata(path) {
        Ok(md) => md.is_file(),
        _ => false,
    }
}
