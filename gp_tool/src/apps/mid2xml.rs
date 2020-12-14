use crate::apps::{SubApp};
use clap::{Clap};
use praise_mod_lib::midi::*;
use std::error::Error;
use std::path::{Path, PathBuf};

#[derive(Clap, Debug)]
pub struct Mid2XmlApp {
    #[clap(about = "Path to input mid", required = true)]
    pub mid_path: String,
    #[clap(about = "Path to output xml", required = true)]
    pub xml_path: String,
}

impl SubApp for Mid2XmlApp {
    fn process(&mut self) -> Result<(), Box<dyn Error>> {
        let midi_path = Path::new(&self.mid_path);
        let mut midi_reader = MidiReader::from_path(midi_path)?;

        println!("This is mid2xml app!");

        Ok(())
    }
}