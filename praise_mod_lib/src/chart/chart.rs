use std::error::Error;
use std::fs::read_to_string;
use std::path::{Path, PathBuf};

pub struct SongChart {
    pub name: String,
}

impl SongChart {
    pub fn from_path(path: &Path) -> Result<SongChart, Box<dyn Error>> {
        let text = read_to_string(path)?;

        Ok(SongChart {
            name: String::from("")
        })
    }
}