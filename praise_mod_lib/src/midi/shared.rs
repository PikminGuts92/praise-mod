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
pub enum MidiTextType {
    Event(String),
    Lyric(String)
}

#[derive(Clone, Debug)]
pub struct MidiText {
    pub pos: u64,
    pub text: MidiTextType,
}

impl MidiText {
    pub fn is_lyric(&self) -> bool {
        match self.text {
            MidiTextType::Lyric(_) => true,
            _ => false,
        }
    }

    pub fn get_text<'a>(&'a self) -> &'a String {
        match &self.text {
            MidiTextType::Lyric(text) => text,
            MidiTextType::Event(text) => text,
        }
    }
}

#[derive(Clone, Debug)]
pub struct MidiTrack {
    pub name: Option<String>,
    pub notes: Vec<MidiNote>,
    pub texts: Vec<MidiText>,
}