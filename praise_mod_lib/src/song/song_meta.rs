use ini::Ini;
use log::{info};
use std::io::Read;
use std::error::Error;
use std::fs::{File};
use std::path::{Path, PathBuf};

#[derive(Clone, Debug)]
pub struct SongMeta {
    pub name: String,
    pub artist: String,
    pub album: String,
    pub year: i32,
    pub preview_start: Option<u32>, // ms
}

impl SongMeta {
    pub fn from_path(ini_path: &Path) -> Result<SongMeta, Box<dyn Error>> {
        // TODO: Throw error if ini not found or "song" section not present
        let song_ini = Ini::load_from_file(ini_path)?;

        // Can be either "song" or "Song"
        let song_section_name = song_ini
            .sections()
            .find(|s| match s {
                Some(s) => s.eq_ignore_ascii_case("song"),
                None => false,
            });

        let song_section_name = song_section_name
            .unwrap()
            .unwrap();

        let song_section = song_ini
            .section(Some(song_section_name))
            .unwrap();

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
            preview_start: match song_section.get("preview_start_time") {
                Some(text) => match text.parse() {
                    Ok(time) => Some(time),
                    Err(_) => None
                },
                None => None,
            }
        })
    }
}
