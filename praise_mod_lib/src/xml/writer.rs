use std::{fs::File, writeln};
use std::io::{Write, BufReader, BufRead, Error};
use crate::midi::*;
use log::{info, warn};
use std::path::{Path, PathBuf};
use crate::xml::*;

impl XmlFile {
    pub fn write_to_file(&self, xml_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
        let mut xml_file = File::create(xml_path)?;

        writeln!(xml_file, "<?xml version='1.1'?>")?;

        if let XmlTrack::GuitarBass(beats) = &self.track {
            writeln!(xml_file, "<beats>")?;

            // Iterate over notes
            for beat in beats.iter() {
                // Set show position to be 2500ms before hit or 0
                let show_pos = match beat.pos {
                    p if p >= 2500 => beat.pos - 2500,
                    _ => 0,
                };

                let note_name = beat.get_note_name();
                writeln!(xml_file, "\t<{}>", note_name)?;

                writeln!(xml_file, "\t\t<show>{}</show>", show_pos)?;
                writeln!(xml_file, "\t\t<target>{}</target>", beat.pos)?;

                // Write end pos if sustain
                if beat.is_sustain() {
                    writeln!(xml_file, "\t\t<end>{}</end>", beat.pos + beat.length)?;
                }

                writeln!(xml_file, "\t</{}>", note_name)?;
            }

            writeln!(xml_file, "</beats>")?;
        } else if let XmlTrack::Vocals(lyrics) = &self.track {
            writeln!(xml_file, "<lyrics>")?;

            // Iterate over lyrics
            for lyric in lyrics.iter() {
                writeln!(xml_file, "\t<show>{}</show>", lyric.pos)?;
                writeln!(xml_file, "\t<text>{}</text>", lyric.text)?;
                writeln!(xml_file, "\t<remove>{}</remove>", lyric.pos + lyric.length)?;
            }

            writeln!(xml_file, "</lyrics>")?;
        }

        Ok(())
    }
}