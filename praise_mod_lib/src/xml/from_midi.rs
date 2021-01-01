use crate::midi::*;
use log::{info, warn};
use std::{fs::File, writeln};
use std::io::{Write, BufReader, BufRead, Error};
use std::path::{Path, PathBuf};
use crate::xml::*;

impl XmlFile {
    pub fn from_midi(mid: &MidiFile, track_type: XmlTrackType, track_difficulty: Option<XmlTrackDifficulty>) -> XmlFile {
        XmlFile {
            track: match track_type {
                XmlTrackType::Guitar => XmlFile::parse_guitar_track_from_midi(
                    mid, 
                    false,
                    track_difficulty
                        .unwrap_or(XmlTrackDifficulty::Expert)),
                XmlTrackType::Bass => XmlFile::parse_guitar_track_from_midi(
                    mid, 
                    true, 
                    track_difficulty
                        .unwrap_or(XmlTrackDifficulty::Expert)),
                XmlTrackType::Vocals => {
                    XmlFile::parse_vocal_track_from_midi(mid)
                },
            }
        }
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

        // Iterate over notes
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
                    XmlFile::update_fret_beat_event(beat_event, length, note.pitch - notes_offset, is_sp_note, false);
                } else {
                    // Pop off current note and add to collection
                    let beat_event = current_note.take().unwrap();
                    xml_notes.push(beat_event);
                }
            }

            // Add as new note
            if current_note.is_none() {
                let mut beat_event = BeatEvent::default(pos, length);
                XmlFile::update_fret_beat_event(&mut beat_event, length, note.pitch - notes_offset, is_sp_note, false);

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
                text: XmlFile::concat_text(&split_text)
            };

            lyrics.push(lyric);
        }

        XmlTrack::Vocals(lyrics)
    }
}