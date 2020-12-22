use crate::apps::{SubApp};
use clap::{Clap};
use praise_mod_lib::midi::*;
use praise_mod_lib::pack::*;
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
        let res = find_dirs_with_file_name(&self.songs_path, "song.ini");
        Ok(())
    }
}