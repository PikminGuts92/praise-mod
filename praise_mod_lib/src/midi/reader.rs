//use ghakuf::messages::MidiEvent;
use ghakuf::messages::*;
use ghakuf::reader::*;
use crate::midi::shared::*;
use crate::midi::smf::*;
use std::collections::HashMap;
use std::path::Path;

#[derive(Clone, Copy, Debug)]
struct PendingMidiNote {
    pos: u64,
    channel: u8,
    velocity: u8
}

pub(crate) struct MidiReader {
    info: Option<MidiInfo>,
    current_track_index: i32,
    current_pos: u64,
    pending_notes: [Option<PendingMidiNote>; 0x80],
    current_track: Option<MidiTrack>,
    tracks: Vec<MidiTrack>,
    tempo_track: Vec<MidiTempo>,
}

impl MidiReader {
    pub fn from_path(path: &Path) -> Result<MidiReader, Box<dyn std::error::Error>> {
        let mut midi_reader = MidiReader {
            info: None,
            current_track_index: -1,
            current_pos: 0,
            pending_notes: [None; 0x80],
            current_track: None,
            tracks: Vec::new(),
            tempo_track: Vec::new(),
        };

        let mut reader = Reader::new(
            &mut midi_reader,
            &path)
            .unwrap();
        
        reader.read().unwrap();

        midi_reader.finalize_track(); // Finalize last track
        Ok(midi_reader)
    }

    /*pub fn parse(&mut self) {

    }*/

    pub fn get_midi(&self) -> MidiFile {
        let info = self.info.unwrap();

        MidiFile {
            format: info.format,
            ticks_per_quarter: info.ticks_per_quarter,
            tracks: self.tracks.to_vec(),
            tempo: self.tempo_track.to_vec(),
        }
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
            MetaEvent::TextEvent => {                
                if let Some(track) = &mut self.current_track {
                    let text = String::from_utf8(data.to_owned())
                        .unwrap_or(String::from(""));

                    track.texts.push(MidiText {
                        pos: self.current_pos,
                        text: MidiTextType::Event(text),
                    });
                }
            },
            MetaEvent::Lyric => {                
                if let Some(track) = &mut self.current_track {
                    let text = String::from_utf8(data.to_owned())
                        .unwrap_or(String::from(""));

                    track.texts.push(MidiText {
                        pos: self.current_pos,
                        text: MidiTextType::Lyric(text),
                    });
                }
            },
            MetaEvent::SetTempo => {
                if self.current_track_index > 0 {
                    // Only parse tempo events from first track
                    return;
                }

                let mpq = self.mpq_from_raw_tempo(data);

                // Calculate bpm
                let bpm = 60_000_000.0 / mpq as f64;
                let mut pos_realtime = 0.0;

                // Calculate realtime position
                if let Some(last_tempo) = self.tempo_track.last() {
                    let tpq = match &self.info {
                        Some(info) => info.ticks_per_quarter,
                        None => 480,
                    };

                    let delta_ticks = self.current_pos - last_tempo.pos;
                    
                    let delta_ms = (last_tempo.mpq as u64 * delta_ticks) as f64 / (1_000 * tpq as u32) as f64;
                    pos_realtime = last_tempo.pos_realtime + delta_ms;
                }

                self.tempo_track.push(MidiTempo {
                    pos: self.current_pos,
                    pos_realtime,
                    mpq,
                    bpm,
                });
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

            let final_note = MidiReader::finalize_note(&pending_note, note, self.current_pos);

            if let Some(track) = &mut self.current_track {
                track.notes.push(final_note);
            }
        }
    }

    fn sys_ex_event(&mut self, delta_time: u32, event: &SysExEvent, data: &Vec<u8>) {
        self.update_pos(delta_time);

    }

    fn track_change(&mut self) {
        if self.current_track_index == 0
            && self.tempo_track.len() == 0 {
            // No tempo changes found, default to 120bpm
            self.tempo_track.push(MidiTempo {
                pos: 0,
                pos_realtime: 0.0,
                mpq: 60_000_000 / 120,
                bpm: 120.0,
            })
        }
        else if self.current_track_index > 0 {
            // Skip adding tempo track (has dedicated track instead)
            self.finalize_track();
        }

        self.current_track_index += 1;
        self.current_pos = 0;
        self.current_track = Some(MidiTrack {
            name: None,
            notes: Vec::new(),
            texts: Vec::new(),
        });
    }
}

impl MidiReader {
    fn update_pos(&mut self, delta_time: u32) {
        self.current_pos += delta_time as u64;
    }

    fn finalize_note(note: &PendingMidiNote, pitch: u8, end_pos: u64) -> MidiNote {
        MidiNote {
            pos: note.pos,
            pos_realtime: 0.0,
            length: end_pos - note.pos,
            length_realtime: 0.0,
            pitch,
            channel: note.channel,
            velocity: note.velocity,
        }
    }

    fn finalize_track(&mut self) {
        if self.current_track.is_none() {
            return;
        }

        let mut track = self.current_track.take().unwrap();
        self.current_track = None;

        // Iterate over pending notes and finalize
        // Note: This would only be needed if the input midi was missing off notes
        for (i, note) in self.pending_notes.iter_mut().enumerate() {
            if let Some(pending_note) = note {
                // Finalize note
                let final_note = MidiReader::finalize_note(pending_note, i as u8, self.current_pos);
                track.notes.push(final_note);

                // Clear out old value
                *note = None;
            }
        }

        // Sort notes in track
        track.notes.sort_by(|a, b| {
            // Sort by position then pitch
            if a.pos == b.pos {
                a.pitch.partial_cmp(&b.pitch).unwrap()
            } else {
                a.pos.partial_cmp(&b.pos).unwrap()
            }
        });

        // Update realtime positions
        let mut tempo_itr = self.tempo_track.iter().rev();
        let mut current_tempo = tempo_itr.next().unwrap();

        for note in track.notes.iter_mut().rev() {
            let start_pos = note.pos;
            let end_pos = start_pos + note.length;

            // Calculate realtime end position
            while current_tempo.pos > end_pos {
                current_tempo = tempo_itr.next().unwrap();
            }
            let end_pos_realtime = self.calculate_realtime_ms(current_tempo, end_pos);

            // Calculate realtime start position
            while current_tempo.pos > start_pos {
                current_tempo = tempo_itr.next().unwrap();
            }
            let start_pos_realtime = self.calculate_realtime_ms(current_tempo, start_pos);

            note.pos_realtime = start_pos_realtime;
            note.length_realtime = end_pos_realtime - start_pos_realtime;
        }

        // Add to tracks
        self.tracks.push(track);
    }

    fn calculate_realtime_ms(&self, tempo: &MidiTempo, pos_ticks: u64) -> f64 {
        let tpq = match &self.info {
            Some(info) => info.ticks_per_quarter,
            None => 480,
        };

        let delta_ticks = pos_ticks - tempo.pos;

        let delta_ms = (tempo.mpq as u64 * delta_ticks) as f64 / (1_000 * tpq as u32) as f64;
        tempo.pos_realtime + delta_ms
    }

    fn mpq_from_raw_tempo(&self, data: &Vec<u8>) -> u32 {
        (data[0] as u32) << 16 | (data[1] as u32) << 8 as u32 | data[2] as u32
    }
}