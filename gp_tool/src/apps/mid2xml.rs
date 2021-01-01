use crate::apps::{SubApp};
use clap::{Clap};
use praise_mod_lib::midi::*;
use praise_mod_lib::xml::*;
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
        let song_id = "80001"; // TODO: Get from args

        let midi_path = Path::new(&self.mid_path);
        let output_dir = Path::new(&self.xml_path);

        let instruments = [
            XmlTrackType::Guitar,
            XmlTrackType::Bass,
            XmlTrackType::Vocals,
        ];

        let gtr_difficulties = [
            XmlTrackDifficulty::Easy,
            XmlTrackDifficulty::Medium,
            XmlTrackDifficulty::Hard,
            XmlTrackDifficulty::Expert,
        ];

        let midi_file = MidiFile::from_path(midi_path)?;

        for ins_type in &instruments {
            // Parse vocals track
            if *ins_type == XmlTrackType::Vocals {
                let xml_writer = XmlFile::from_midi(&midi_file, *ins_type, None);

                let track_name = format!(
                    "GPL{}.xml",
                    song_id,
                );

                let xml_path = output_dir.join(track_name);
                xml_writer.write_to_file(&xml_path)?;
                continue;
            }

            // Parse guitar/bass tracks
            for (i, diff) in gtr_difficulties.iter().enumerate() {
                let xml_writer = XmlFile::from_midi(&midi_file, *ins_type, Some(*diff));

                let track_name = format!(
                    "GP{}{}_{}.xml",
                    match &ins_type {
                        XmlTrackType::Guitar => "G",
                        XmlTrackType::Bass => "B",
                        XmlTrackType::Vocals => "L",
                    },
                    song_id,
                    i,
                );

                let xml_path = output_dir.join(track_name);
                xml_writer.write_to_file(&xml_path)?;
            }
        }

        println!("This is mid2xml app!");
        Ok(())
    }
}