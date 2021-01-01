use std::{fs::File, writeln};
use std::io::{Write, BufReader, BufRead, Error};
use crate::midi::*;
use log::{info, warn};
use std::path::{Path, PathBuf};
use crate::xml::*;

impl XmlFile {
    pub fn write_to_file(&self, xml_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
        let mut xml_file = File::create(xml_path)?;

        if let XmlTrack::GuitarBass(beats) = &self.track {
            writeln!(xml_file, "<?xml version='1.1'?>")?;
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
            writeln!(xml_file, "<?xml version='1.1'?>")?;
            writeln!(xml_file, "<lyrics>")?;

            // Iterate over lyrics
            for lyric in lyrics.iter() {
                writeln!(xml_file, "\t<show>{}</show>", lyric.pos)?;
                writeln!(xml_file, "\t<text>{}</text>", lyric.text)?;
                writeln!(xml_file, "\t<remove>{}</remove>", lyric.pos + lyric.length)?;
            }

            writeln!(xml_file, "</lyrics>")?;
        } else if let XmlTrack::Metadata { name, pack_id, albums, songs } = &self.track {
            writeln!(xml_file, "<?xml version=\"1.0\" encoding=\"UTF-8\" standalone=\"no\" ?>")?;
            writeln!(xml_file, "<data>")?;

            writeln!(xml_file, "\t<exp_title>{}</exp_title>", name)?;
            writeln!(xml_file, "\t<difficultyIcon>191042.dpa</difficultyIcon>")?;

            // Write album names
            writeln!(xml_file, "\t<albumNames>")?;
            for (i, album) in albums.iter().enumerate() {
                writeln!(xml_file, "\t\t<Name{0}>{1}</Name{0}>", i, album)?;
            }
            writeln!(xml_file, "\t</albumNames>")?;

            // Write song info
            writeln!(xml_file, "\t<tracks>")?;
            for song in songs.iter() {
                let full_song_id = format!("{:02}{:03}", pack_id, song.song_id);

                writeln!(xml_file, "\t\t<song_{:03}>", song.song_id)?;

                writeln!(xml_file, "\t\t\t<artist>{}</artist>", song.artist)?;
                writeln!(xml_file, "\t\t\t<title>{}</title>", song.title)?;
                writeln!(xml_file, "\t\t\t<short_title></short_title>")?;
                writeln!(xml_file, "\t\t\t<difficulty_easy>1</difficulty_easy>")?;
                writeln!(xml_file, "\t\t\t<difficulty_medium>2</difficulty_medium>")?;
                writeln!(xml_file, "\t\t\t<difficulty_hard>3</difficulty_hard>")?;
                writeln!(xml_file, "\t\t\t<difficulty_expert>4</difficulty_expert>")?;
                writeln!(xml_file, "\t\t\t<bpm_easy>003</bpm_easy>")?;
                writeln!(xml_file, "\t\t\t<bpm_medium>002</bpm_medium>")?;
                writeln!(xml_file, "\t\t\t<bpm_hard>003</bpm_hard>")?;
                writeln!(xml_file, "\t\t\t<albumID>{}</albumID>", song.album_idx)?;
                writeln!(xml_file, "\t\t\t<albumImage>CD_197237.bmp</albumImage>")?;
                writeln!(xml_file, "\t\t\t<cd_website></cd_website>")?;
                writeln!(xml_file, "\t\t\t<artist_website></artist_website>")?;
                writeln!(xml_file, "\t\t\t<wave>GPM{}.dpo</wave>", &full_song_id)?;
                writeln!(xml_file, "\t\t\t<lyrics>GPL{}.xml</lyrics>", &full_song_id)?;
                writeln!(xml_file, "\t\t\t<locked>0</locked>")?;

                writeln!(xml_file, "\t\t</song_{:03}>", song.song_id)?;
            }
            writeln!(xml_file, "\t</tracks>")?;

            writeln!(xml_file, "</data>")?;
        }

        Ok(())
    }
}