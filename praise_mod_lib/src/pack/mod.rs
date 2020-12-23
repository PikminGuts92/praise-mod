use std::error::Error;
use std::path::{Path, PathBuf};
use walkdir::{DirEntry, WalkDir};

pub fn find_dirs_with_file_name(dir_path: &str, file_name: &str) -> Result<Vec<PathBuf>, Box<dyn Error>> {
    Ok(WalkDir::new(dir_path)
        .into_iter()
        .filter(|e| match e {
            Ok(e) => does_entry_match_file_name(e, file_name),
            Err(_) => false,
        })
        .map(|d| d
            .unwrap()
            .path()
            .parent()
            .unwrap()
            .to_owned())
        .collect())
}

fn does_entry_match_file_name(entry: &DirEntry, file_name: &str) -> bool {
    entry.file_name()
         .to_str()
         .map(|s| s.eq_ignore_ascii_case(file_name))
         .unwrap_or(false)
}