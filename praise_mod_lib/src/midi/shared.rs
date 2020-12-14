#[derive(Clone, Copy, Debug)]
pub struct MidiInfo {
    pub format: u16,
    pub ticks_per_quarter: u16, // Usually 480
}

#[derive(Clone, Copy, Debug)]
pub struct MidiNote {
    pub pos: u64,
    pub pos_realtime: f64, // Milliseconds
    pub length: u64,
    pub length_realtime: f64, // Milliseconds
    pub pitch: u8,
    pub channel: u8,
    pub velocity: u8
}

#[derive(Clone, Copy, Debug)]
pub struct MidiTempo {
    pub pos: u64,
    pub pos_realtime: f64, // Milliseconds
    pub mpq: u32,
    pub bpm: f64,
}

#[derive(Clone, Debug)]
pub struct MidiTrack {
    pub name: Option<String>,
    pub notes: Vec<MidiNote>,
}