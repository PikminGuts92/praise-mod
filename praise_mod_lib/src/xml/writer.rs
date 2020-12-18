use std::{fs::File, writeln};
use std::io::{Write, BufReader, BufRead, Error};
use crate::midi::*;
use log::{info, warn};
use std::path::{Path, PathBuf};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum XmlTrackType {
    Guitar,
    Bass,
    Vocals,
}

#[derive(Clone, Copy, Debug)]
pub enum XmlTrackDifficulty {
    Easy,
    Medium,
    Hard,
    Expert,
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
pub struct LyricEvent {
    pub pos: u64,
    pub length: u64,
    pub text: String,
}

#[derive(Debug)]
pub enum XmlTrack {
    GuitarBass(Vec<BeatEvent>),
    Vocals(Vec<LyricEvent>)
}

#[derive(Debug)]
pub struct XmlWriter {
    track: XmlTrack,
}

impl XmlWriter {
    pub fn from_midi(mid: &MidiFile, track_type: XmlTrackType, track_difficulty: Option<XmlTrackDifficulty>) -> XmlWriter {
        XmlWriter {
            track: match track_type {
                XmlTrackType::Guitar => XmlWriter::parse_guitar_track_from_midi(
                    mid, 
                    false,
                    track_difficulty
                        .unwrap_or(XmlTrackDifficulty::Expert)),
                XmlTrackType::Bass => XmlWriter::parse_guitar_track_from_midi(
                    mid, 
                    true, 
                    track_difficulty
                        .unwrap_or(XmlTrackDifficulty::Expert)),
                XmlTrackType::Vocals => {
                    XmlWriter::parse_vocal_track_from_midi(mid)
                },
            }
        }
    }

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

    fn parse_guitar_track_from_midi(mid: &MidiFile, is_bass: bool, track_difficulty: XmlTrackDifficulty) -> XmlTrack {
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

        let notes_offset = match track_difficulty {
            XmlTrackDifficulty::Easy => 60u8,
            XmlTrackDifficulty::Medium => 72u8,
            XmlTrackDifficulty::Hard => 84u8,
            XmlTrackDifficulty::Expert => 96u8,
        };
        let sp_offset = 116u8;

        // Star power notes
        let mut star_power = midi_notes
            .into_iter()
            .filter(|note| note.pitch == sp_offset);
        let mut current_sp_note: Option<&MidiNote> = star_power.next();

        let mut current_note: Option<BeatEvent> = None;

        // Iterate over expert notes
        for note in midi_notes
            .into_iter()
            .filter(|note|
                note.pitch >= notes_offset &&
                note.pitch <= notes_offset + 5) {
            let pos = note.pos_realtime as u64;
            let length = match note.length {
                l if l > sustain_length  => note.length_realtime as u64,
                _ => 0,
            };

            let mut is_sp_note = false;
            while current_sp_note.is_some() {
                let sp_note = current_sp_note.unwrap();
                let sp_note_end = sp_note.pos + sp_note.length;

                if note.pos >= sp_note.pos && note.pos < sp_note_end {
                    // Note is in range of sp
                    is_sp_note = true;
                    break;
                } else if note.pos >= sp_note_end {
                    // Update current star power
                    current_sp_note = star_power.next();
                } else { // note.pos < sp_note.pos
                    break;
                }
            }

            if let Some(beat_event) = &mut current_note {
                if beat_event.pos == pos {
                    // Is part of chord, update current note
                    XmlWriter::update_fret_beat_event(beat_event, length, note.pitch - notes_offset, is_sp_note);
                } else {
                    // Pop off current note and add to collection
                    let beat_event = current_note.take().unwrap();
                    xml_notes.push(beat_event);
                }
            }

            // Add as new note
            if current_note.is_none() {
                let mut beat_event = BeatEvent::default(pos, length);
                XmlWriter::update_fret_beat_event(&mut beat_event, length, note.pitch - notes_offset, is_sp_note);

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

    fn update_fret_beat_event(note: &mut BeatEvent, length: u64, index: u8, sp: bool) {
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

        // Update star power
        if sp {
            note.star_power = true
        }
    }

    fn parse_vocal_track_from_midi(mid: &MidiFile) -> XmlTrack {
        // Get vocal track
        let vocal_track = mid.tracks
            .iter()
            .find(|&track|
                match &track.name {
                    Some(name) => name.eq("PART VOCALS"),
                    None => false,
                });

        if vocal_track.is_none() {
            return XmlTrack::Vocals(Vec::new());
        }

        let midi_notes = &vocal_track.unwrap().notes;
        let text_events = &vocal_track.unwrap().texts;
        let mut lyrics = Vec::new();

        let phrase_offset_high = 106u8;
        let phrase_offset_low = 105u8;

        // Lyrics
        let mut lyric_events = text_events
            .into_iter()
            .filter(|event|
                event.is_lyric());
        let mut current_lyric: Option<&MidiText> = lyric_events.next();

        // Phrase sections
        let phrases = midi_notes
            .into_iter()
            .filter(|note|
                note.pitch <= phrase_offset_high &&
                note.pitch >= phrase_offset_low);

        // Iterate over phrases
        for phrase in phrases {
            let phrase_end_pos = phrase.pos + phrase.length;
            let mut split_text: Vec<String> = Vec::new();

            // Iterate over lyrics
            while current_lyric.is_some() {
                let lyric_event = current_lyric.unwrap();
                if lyric_event.pos >= phrase_end_pos {
                    break;
                }

                let text = lyric_event
                    .get_text()
                    .to_owned();

                split_text.push(text);
                current_lyric = lyric_events.next();
            }

            if split_text.is_empty() {
                continue;
            }

            let lyric = LyricEvent {
                pos: phrase.pos_realtime as u64,
                length: phrase.length_realtime as u64,
                text: XmlWriter::concat_text(&split_text)
            };

            lyrics.push(lyric);
        }

        XmlTrack::Vocals(lyrics)
    }

    fn concat_text(text: &Vec<String>) -> String {
        let mut new_text = String::new();
        let mut prev_concat = false;

        for (i, t) in text.iter().enumerate() {
            let mut has_dash = false;
            let mut is_unpitched = false;

            if t.eq("+") {
                continue;
            } else if t.ends_with("-") || t.ends_with("=") {
                has_dash = true;
            } else if t.ends_with("#") {
                is_unpitched = true;
            }

            if !prev_concat && i > 0 {
                new_text += " ";
            }

            let t_max_idx = match has_dash || is_unpitched {
                true => t.len() - 1,
                _ => t.len(),
            };

            new_text += &t[..t_max_idx];
            prev_concat = has_dash;
        }

        new_text
    }
}