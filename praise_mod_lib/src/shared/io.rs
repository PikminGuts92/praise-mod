use std::error::Error;
use std::fs::{DirEntry, read_dir};
use std::path::{Path, PathBuf};
use walkdir::{DirEntry as WalkDirEntry, WalkDir};

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

fn does_entry_match_file_name(entry: &WalkDirEntry, file_name: &str) -> bool {
    entry.file_name()
         .to_str()
         .map(|s| s.eq_ignore_ascii_case(file_name))
         .unwrap_or(false)
}

pub fn get_files_in_dir<T: AsRef<Path>>(dir_path: T, ext: Option<&str>) -> Result<Vec<PathBuf>, Box<dyn Error>> {
    // Get file paths in given directory
    Ok(read_dir(dir_path)?
        .filter(|d| d.is_ok())
        .map(|d| d.unwrap())
        .filter(|d| d.path().is_file())
        .map(|d| d.path())
        .filter(|p| match ext {
            // Ext ext filter given, filter out file paths without given extension
            Some(ex) => match p.extension() {
                Some(path_ext) => match path_ext.to_str() {
                    Some(path_ext_str) => path_ext_str.eq_ignore_ascii_case(ex),
                    None => false,
                },
                None => false,
            },
            // No ext filter giver, return all
            None => true,
        })
        .collect())
}