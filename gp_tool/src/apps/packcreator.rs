use crate::apps::{SubApp};
use clap::{Clap};
use log::{info, warn};
use praise_mod_lib::chart::*;
use praise_mod_lib::midi::*;
use praise_mod_lib::pack::*;
use praise_mod_lib::song::*;
use praise_mod_lib::xml::*;
use std::error::Error;
use std::path::{Path, PathBuf};

#[derive(Clap, Debug)]
pub struct PackCreatorApp {
    #[clap(about = "Path to input CH songs directory", required = true)]
    pub songs_path: String,
    #[clap(about = "Path to output pack directory", required = true)]
    pub output_path: String,
}

impl SubApp for PackCreatorApp {
    fn process(&mut self) -> Result<(), Box<dyn Error>> {
        let song_paths = find_dirs_with_file_name(&self.songs_path, "song.ini")?;

        // Iterate over song directories
        for path in &song_paths {
            info!("Parsing song in \"{}\"", path.to_str().unwrap());

            let song_ini = path.join("song.ini");
            let song_meta = SongMeta::from_path(&song_ini)?;

            info!("Song Information\n\tTitle: {}\n\tArtist: {}\n\tAlbum: {}\n\tYear: {}",
                song_meta.name,
                song_meta.artist,
                song_meta.album,
                song_meta.year,
            );

            let song_chart_path = path.join("notes.chart");
            let song_chart = SongChart::from_path(&song_chart_path)?;
        }
        Ok(())
    }
}