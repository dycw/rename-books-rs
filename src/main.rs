pub mod error;

use dirs::home_dir;
use error::RenameBooksError;
use std::{fs::read_dir, path::PathBuf};

fn get_dropbox_temporary() -> Result<PathBuf, RenameBooksError> {
    let home = home_dir().ok_or(RenameBooksError::HomeError);
    let path = home?.join("dropbox").join("Temporary");
    if !path.exists() {
        return Err(RenameBooksError::TemporaryError { path });
    }
    if !path.is_dir() {
        return Err(RenameBooksError::TemporaryNotADirectoryError { path });
    }
    Ok(path)
}

fn get_temporary_files() -> Result<Vec<PathBuf>, RenameBooksError> {
    let temp = get_dropbox_temporary()?;
    let files: Vec<PathBuf> = read_dir(&temp)?
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.path().is_file())
        .filter(|entry| {
            entry
                .path()
                .extension()
                .map_or_else(|| false, |ext| ext == "epub" || ext == "pdf")
        })
        .map(|entry| entry.path())
        .collect();
    Ok(files)
}

fn main() {
    match get_temporary_files() {
        Ok(files) => {
            let file_count = files.len();
            let file_text = if file_count == 1 { "file" } else { "files" };
            println!("Found {} {}: ", file_count, file_text);
            for file in files {
                println!("{}", file.display())
            }
        }
        Err(e) => {
            eprintln!("Error: {}", e)
        }
    }
}
