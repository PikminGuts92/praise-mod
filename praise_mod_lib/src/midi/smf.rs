use crate::midi::*;
use std::path::Path;

#[derive(Debug)]
pub struct MidiFile {
    pub format: u16,
    pub ticks_per_quarter: u16,
    pub tracks: Vec<MidiTrack>,
    pub tempo: Vec<MidiTempo>,
}

impl MidiFile {
    pub fn from_path(midi_path: &Path) -> Result<MidiFile, Box<dyn std::error::Error>> {
        let reader = MidiReader::from_path(midi_path)?;
        let midi = reader.get_midi();

        Ok(midi)
    }
}