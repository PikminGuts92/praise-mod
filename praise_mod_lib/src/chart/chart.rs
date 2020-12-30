use crate::chart::parser::*;
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
                    self.sync_track.events.push(SyncEvent {
                    pos: 0,
                    pos_realtime: 0.0,
                    value: SyncEventType::Beat(120_000),
                });
        }
    }
}