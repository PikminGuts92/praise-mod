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
}

impl SongMeta {
    pub fn from_path(ini_path: &Path) -> Result<SongMeta, Box<dyn Error>> {
        // Read file to string
        // Note: This is a workaround until rust-ini is updated to support UTF-8 BOM #79
        let mut file_str = String::new();
        File::open(ini_path)?.read_to_string(&mut file_str)?;
        let ini_str = skip_bom(&file_str);

        // TODO: Throw error if ini not found or "song" section not present
        //let song_ini = Ini::load_from_file(ini_path)?;
        let song_ini = Ini::load_from_str(&ini_str)?;

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
        })
    }
}

fn skip_bom(s: &str) -> &str {
    // Skip BOM if present
    if s.starts_with("\u{feff}") {
        &s[3..]
    } else {
        s
    }
}