use crate::chart::SongChart;
use crate::midi::MidiFile;

#[derive(Debug)]
pub enum ChartFile {
    Chart(SongChart),
    Midi(MidiFile),
}

impl ChartFile {
    fn is_midi(&self) -> bool {
        match &self {
            ChartFile::Midi(_) => true,
            _ => false,
        }
    }
}