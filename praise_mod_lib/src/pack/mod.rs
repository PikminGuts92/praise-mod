use std::error::Error;
use std::path::{Path, PathBuf};
use walkdir::{DirEntry, WalkDir};

pub fn find_dirs_with_file_name(dir_path: &str, file_name: &str) {
    let song_paths: Vec<PathBuf> = WalkDir::new(dir_path)
        .into_iter()
        .filter(|e| match e {
            Ok(e) => does_entry_match_file_name(e, file_name),
            Err(_) => false,
        })
        .map(|d| d.unwrap().path().to_owned())
        .collect();
    
    for ini_path in &song_paths {
        let p = ini_path.display().to_string();
        println!("{}", &p);
    }
}

fn does_entry_match_file_name(entry: &DirEntry, file_name: &str) -> bool {
    entry.file_name()
         .to_str()
         .map(|s| s.eq_ignore_ascii_case(file_name))
         .unwrap_or(false)
}