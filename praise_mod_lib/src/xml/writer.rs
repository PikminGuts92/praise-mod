use std::{fs::File, writeln};
use std::io::{Write, BufReader, BufRead, Error};
use crate::midi::*;
use log::{info, warn};
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub enum XmlTrackType {
    Guitar,
    Bass,
    Vocals,
}

#[derive(Debug)]
pub struct BeatEvent {
    pub pos: u64,
    pub length: u64,
    pub green: bool,
    pub red: bool,
    pub yellow: bool,
    pub blue: bool,
    pub orange: bool,
    pub star_power: bool,
}

impl BeatEvent {
    fn default(pos: u64, length: u64) -> BeatEvent {
        BeatEvent {
            pos,
            length,
            green: false,
            red: false,
            yellow: false,
            blue: false,
            orange: false,
            star_power: false,
        }
    }

    pub fn is_sustain(&self) -> bool {
        self.length > 0
    }

    fn get_note_name(&self) -> String {
        format!(
            "{}{}{}{}{}{}{}",
            match self.green {
                true => "left_",
                _ => ""
            },
            match self.red {
                true => "down_",
                _ => ""
            },
            match self.yellow {
                true => "up_",
                _ => ""
            },
            match self.blue {
                true => "right_",
                _ => ""
            },
            match self.orange {
                true => "five_",
                _ => ""
            },
            match self.is_sustain() {
                true => "float_",
                _ => ""
            },
            match self.star_power {
                true => "spinner_",
                _ => ""
            }
        )
    }
}

#[derive(Debug)]
pub enum XmlTrack {
    GuitarBass(Vec<BeatEvent>),
    Vocals()
}

#[derive(Debug)]
pub struct XmlWriter {
    track: XmlTrack,
}

impl XmlWriter {
    pub fn from_midi(mid: &MidiFile, track_type: XmlTrackType) -> XmlWriter {
        XmlWriter {
            track: match track_type {
                XmlTrackType::Guitar => XmlWriter::parse_guitar_track_from_midi(mid, false),
                XmlTrackType::Bass => XmlWriter::parse_guitar_track_from_midi(mid, true),
                XmlTrackType::Vocals => {
                    info!("Parsing of vocals from midi is not supported yet");
                    XmlTrack::Vocals()
                },
            }
        }
    }

    pub fn write_to_file(&self, xml_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
        let mut xml_file = File::create(xml_path)?;

        writeln!(xml_file, "<?xml version='1.1'?>")?;
        writeln!(xml_file, "<beats>")?;

        if let XmlTrack::GuitarBass(beats) = &self.track {
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
        }

        writeln!(xml_file, "</beats>")?;
        Ok(())
    }

    fn parse_guitar_track_from_midi(mid: &MidiFile, is_bass: bool) -> XmlTrack {
        let track_name = match is_bass {
            true => "PART BASS",
            _ => "PART GUITAR",
        };

        let sustain_length = (mid.ticks_per_quarter / 4) as u64; // 16th note

        // Find guitar/bass track
        let guitar_track = mid.tracks
            .iter()
            .find(|&track|
                match &track.name {
                    Some(name) => name.eq(&track_name),
                    None => false,
                });

        if guitar_track.is_none() {
            return XmlTrack::GuitarBass(Vec::new());
        }

        let midi_notes = &guitar_track.unwrap().notes;
        let mut xml_notes: Vec<BeatEvent> = Vec::new();

        let expert_pos = 96u8;

        let mut current_note: Option<BeatEvent> = None;

        // Iterate over expert notes
        for note in midi_notes
            .into_iter()
            .filter(|note|
                note.pitch >= expert_pos &&
                note.pitch <= expert_pos + 5) {
            let pos = note.pos_realtime as u64;
            let length = match note.length {
                l if l > sustain_length  => note.length_realtime as u64,
                _ => 0,
            };

            if let Some(beat_event) = &mut current_note {
                if beat_event.pos == pos {
                    // Is part of chord, update current note
                    XmlWriter::update_fret_beat_event(beat_event, length, note.pitch - expert_pos);
                } else {
                    // Pop off current note and add to collection
                    let beat_event = current_note.take().unwrap();
                    xml_notes.push(beat_event);
                }
            }

            // Add as new note
            if current_note.is_none() {
                let mut beat_event = BeatEvent::default(pos, length);
                XmlWriter::update_fret_beat_event(&mut beat_event, length, note.pitch - expert_pos);

                current_note = Some(beat_event);
            }
        }

        if current_note.is_some() {
            // Pop off current note and add to collection
            let beat_event = current_note.take().unwrap();
            xml_notes.push(beat_event);
        }

        XmlTrack::GuitarBass(xml_notes)
    }

    fn update_fret_beat_event(note: &mut BeatEvent, length: u64, index: u8) {
        // Update length if small
        if note.length < length {
            note.length = length;
        }

        // Update fret (0 = green, 1 = red, etc.)
        match index {
            0 => note.green = true,
            1 => note.red = true,
            2 => note.yellow = true,
            3 => note.blue = true,
            4 => note.orange = true,
            _ => return,
        }
    }
}