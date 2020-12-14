//use ghakuf::messages::MidiEvent;
use ghakuf::messages::*;
use ghakuf::reader::*;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

#[derive(Debug)]
struct MidiInfo {
    format: u16,
    ticks_per_quarter: u16, // Usually 480
}

#[derive(Clone, Copy, Debug)]
struct PendingMidiNote {
    pos: u64,
    channel: u8,
    velocity: u8
}

#[derive(Clone, Copy, Debug)]
struct MidiNote {
    pos: u64,
    length: u64,
    pitch: u8,
    channel: u8,
    velocity: u8
}

#[derive(Debug)]
struct MidiTrack {
    name: Option<String>,
    notes: Vec<MidiNote>,
}

pub struct MidiReader {
    info: Option<MidiInfo>,
    current_track_index: i32,
    current_pos: u64,
    pending_notes: [Option<PendingMidiNote>; 0x80],
    current_track: Option<MidiTrack>,
    tracks: Vec<MidiTrack>
}

impl MidiReader {
    pub fn from_path(path: &Path) -> Result<MidiReader, Box<dyn std::error::Error>> {
        let mut midi_reader = MidiReader {
            info: None,
            current_track_index: -1,
            current_pos: 0,
            pending_notes: [None; 0x80],
            current_track: None,
            tracks: Vec::new()
        };

        let mut reader = Reader::new(
            &mut midi_reader,
            &path)
            .unwrap();
        
        reader.read().unwrap();

        midi_reader.finalize_track(); // Finalize last track
        Ok(midi_reader)
    }
}

impl Handler for MidiReader {
    fn header(&mut self, format: u16, track: u16, time_base: u16) {
        self.info = Some(MidiInfo {
            format,
            ticks_per_quarter: time_base,
        });
    }

    fn meta_event(&mut self, delta_time: u32, event: &MetaEvent, data: &Vec<u8>) {
        self.update_pos(delta_time);

        match event {
            MetaEvent::SequenceOrTrackName => {
                // Set track name
                if let Some(track) = &mut self.current_track {
                    if track.name.is_none() {
                        track.name = String::from_utf8(data.to_owned()).ok();
                    }
                }
            },
            _ => {
                // Skip text event
                return;
            },
        }
    }

    fn midi_event(&mut self, delta_time: u32, event: &MidiEvent) {
        self.update_pos(delta_time);

        let (note_on, channel, note, velocity) = match event {
            MidiEvent::NoteOn { ch, note, velocity} => {
                (*velocity > 0, *ch, *note, *velocity)
            },
            MidiEvent::NoteOff { ch, note, velocity} => {
                (false, *ch, *note, *velocity)
            },
            _ => {
                return;
            }
        };

        if note_on && self.pending_notes[note as usize].is_some() {
            // Double on note, ignore
        } else if note_on {
            // Set note
            self.pending_notes[note as usize] = Some(PendingMidiNote {
                pos: self.current_pos,
                channel,
                velocity,
            })
        } else if !note_on && self.pending_notes[note as usize].is_none() {
            // Double off note, ignore
        } else {
            // Finalize pending note
            let pending_note = self.pending_notes[note as usize].unwrap();
            self.pending_notes[note as usize] = None;

            let final_note = self.finalize_note(&pending_note, note, self.current_pos);

            if let Some(track) = &mut self.current_track {
                track.notes.push(final_note);
            }
        }
    }

    fn sys_ex_event(&mut self, delta_time: u32, event: &SysExEvent, data: &Vec<u8>) {
        self.update_pos(delta_time);

    }

    fn track_change(&mut self) {
        self.finalize_track();

        self.current_track_index += 1;
        self.current_pos = 0;
        self.current_track = Some(MidiTrack {
            name: None,
            notes: Vec::new()
        });
    }
}

impl MidiReader {
    fn update_pos(&mut self, delta_time: u32) {
        self.current_pos += delta_time as u64;
    }

    fn finalize_note(&self, note: &PendingMidiNote, pitch: u8, end_pos: u64) -> MidiNote {
        MidiNote {
            pos: note.pos,
            length: end_pos - note.pos,
            pitch,
            channel: note.channel,
            velocity: note.velocity,
        }
    }

    fn finalize_track(&mut self) {
        if self.current_track.is_none() {
            return;
        }

        let track = self.current_track.take().unwrap();
        self.current_track = None;

        // TODO: Iterate over pending notes and finalize

        // Add to tracks
        self.tracks.push(track);
    }
}