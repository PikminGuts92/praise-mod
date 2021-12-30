use crate::apps::{SubApp};
use clap::Parser;
use log::{info, warn};
use praise_mod_lib::pack::*;
use std::error::Error;

#[derive(Parser, Debug)]
pub struct PackCreatorApp {
    #[clap(help = "Path to input CH songs directory", required = true)]
    pub songs_path: String,
    #[clap(help = "Path to output song pack directory", required = true)]
    pub output_path: String,
    #[clap(long, short, help = "Name of song pack")]
    pub name: Option<String>,
    #[clap(long, short, default_value = "4", help = "Numeric id for song pack (must be between 4-98)")]
    pub id: u8,
}

impl PackCreatorApp {
    fn to_pack_ops(&self) -> PackOptions {
        PackOptions {
            songs_path: self.songs_path
                .to_owned(),
            output_path: self.output_path
                .to_owned(),
            name: self.name
                .to_owned(),
            id: self.id
        }
    }
}

impl SubApp for PackCreatorApp {
    fn process(&mut self) -> Result<(), Box<dyn Error>> {
        let ops = self.to_pack_ops();
        create_pack(&ops)
    }
}
