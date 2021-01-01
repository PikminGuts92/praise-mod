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
    pub tap: bool,
    pub star_power: bool,
}

impl BeatEvent {
    pub fn default(pos: u64, length: u64) -> BeatEvent {
        BeatEvent {
            pos,
            length,
            green: false,
            red: false,
            yellow: false,
            blue: false,
            orange: false,
            tap: false,
            star_power: false,
        }
    }

    pub fn is_sustain(&self) -> bool {
        self.length > 0
    }

    pub fn get_note_name(&self) -> String {
        format!(
            "{}{}{}{}{}{}{}{}",
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
            match self.tap {
                true => "quick_",
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
pub struct XmlSong {
    pub song_id: u16,
    pub artist: String,
    pub title: String,
    pub album_idx: u16,
}

#[derive(Debug)]
pub enum XmlTrack {
    GuitarBass(Vec<BeatEvent>),
    Vocals(Vec<LyricEvent>),
    Metadata {
        name: String,
        pack_id: u8,
        albums: Vec<String>,
        songs: Vec<XmlSong>,
    }
}