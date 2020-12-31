use crate::chart::parser::*;
use crate::shared::*;
use std::error::Error;
use std::fs::read_to_string;
use std::path::{Path, PathBuf};

pub enum SyncEventType {
    Beat(u32), // bpm * 1000
    TimeSignature(u32, Option<u32>), // 2nd number is encoded as base 2 power (3/8 = 3,3)
}

pub struct SyncEvent {
    pub pos: u64,
    pub pos_realtime: f64, // Milliseconds
    pub value: SyncEventType,
}

pub struct SyncTrack {
    pub events: Vec<SyncEvent>,
}

pub enum GuitarEventType {
    Note(u32),
    Starpower,
    Forced,
    Tap,
    Open,
}

pub struct GuitarEvent {
    pub pos: u64,
    pub pos_realtime: f64, // Milliseconds
    pub length: u64,
    pub length_realtime: f64, // Milliseconds
    pub value: GuitarEventType,
}

pub struct GuitarTrack {
    pub name: String,
    pub events: Vec<GuitarEvent>,
}

pub struct SongChart {
    pub resolution: u16,
    pub sync_track: SyncTrack,
    pub guitar_tracks: Vec<GuitarTrack>,
}

impl SongChart {
    pub fn from_path(path: &Path) -> Result<SongChart, Box<dyn Error>> {
        let text = read_to_string(path)?;
        let mut chart = parse_chart(&text)?;
        chart.update_realtime_positions();

        Ok(chart)
    }

    fn update_realtime_positions(&mut self) {
        // Add default tempo event if not found
        if self.sync_track
            .events
            .iter()
            .any(
                |e| match e.value {
                    SyncEventType::Beat(_) => true,
                    _ => false
                }) {
                    self.sync_track.events.insert(0, SyncEvent {
                        pos: 0,
                        pos_realtime: 0.0,
                        value: SyncEventType::Beat(120_000),
                    });
        }

        // Remove ts change events
        // TODO: Refactor for other music games
        self.sync_track
            .events
            .retain(|e| match e.value {
                SyncEventType::Beat(_) => true,
                _ => false
            });

        // Update tempo map positions
        update_realtime_positions_tempo(&mut self.sync_track.events, self.resolution);

        // Update positions in each guitar/bass track
        for guitar_track in self.guitar_tracks.iter_mut() {
            update_realtime_positions(&mut guitar_track.events, &self.sync_track.events, self.resolution);
        }
    }
}

impl RealtimeNote for GuitarEvent {
    fn get_pos(&self) -> u64 {
        self.pos
    }

    fn get_pos_realtime(&self) -> f64 {
        self.pos_realtime
    }

    fn get_length(&self) -> u64 {
        self.length
    }

    fn get_length_realtime(&self) -> f64 {
        self.length_realtime
    }

    fn set_pos_realtime(&mut self, pos: f64) {
        self.pos_realtime = pos;
    }

    fn set_length_realtime(&mut self, length: f64) {
        self.length_realtime = length;
    }
}

impl RealtimeNote for SyncEvent {
    fn get_pos(&self) -> u64 {
        self.pos
    }

    fn get_pos_realtime(&self) -> f64 {
        self.pos_realtime
    }

    fn get_length(&self) -> u64 {
        0
    }

    fn get_length_realtime(&self) -> f64 {
        0.0
    }

    fn set_pos_realtime(&mut self, pos: f64) {
        self.pos_realtime = pos;
    }

    fn set_length_realtime(&mut self, _length: f64) {
        // Do nothing
    }
}

impl RealtimeTempoNote for SyncEvent {
    fn get_mpq(&self) -> u32 {
        match self.value {
            SyncEventType::Beat(bpm) => 60_000_000 / (bpm / 1000),
            _ => 0,
        }
    }

    fn get_bpm(&self) -> f64 {
        match self.value {
            SyncEventType::Beat(bpm) => bpm as f64 / 1000.0,
            _ => 0.0,
        }
    }
}