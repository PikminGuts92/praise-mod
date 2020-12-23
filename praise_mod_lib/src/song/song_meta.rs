use ini::Ini;
use std::error::Error;
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub struct SongMeta {
    pub name: String,
    pub artist: String,
    pub album: String,
    pub year: i32,
}

impl SongMeta {
    pub fn from_path(ini_path: &Path) -> Result<SongMeta, Box<dyn Error>> {
        // TODO: Throw error if ini not found or "song" section not present
        let song_ini = Ini::load_from_file(ini_path)?;
        let song_section = song_ini.section(Some("song")).unwrap();

        Ok(SongMeta {
            name: match song_section.get("name") {
                Some(text) => text.to_owned(),
                None => String::from(""),
            },
            artist: match song_section.get("artist") {
                Some(text) => text.to_owned(),
                None => String::from(""),
            },
            album: match song_section.get("album") {
                Some(text) => text.to_owned(),
                None => String::from(""),
            },
            year: match song_section.get("year") {
                Some(text) => text.parse().unwrap_or(2020),
                None => 2020,
            },
        })
    }
}