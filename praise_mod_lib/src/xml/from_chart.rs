use crate::chart::*;
use crate::xml::*;

#[derive(Clone, Copy, Debug)]
enum GuitarInstrument {
    Guitar,
    Bass,
    Rhythm,
}

impl XmlFile {
    pub fn from_chart(chart: &SongChart, track_type: XmlTrackType, track_difficulty: Option<XmlTrackDifficulty>) -> XmlFile {
        XmlFile {
            track: match track_type {
                XmlTrackType::Guitar => parse_guitar_track_from_chart(
                    chart, 
                    false,
                    track_difficulty
                        .unwrap_or(XmlTrackDifficulty::Expert)),
                XmlTrackType::Bass => parse_guitar_track_from_chart(
                    chart, 
                    true, 
                    track_difficulty
                        .unwrap_or(XmlTrackDifficulty::Expert)),
                XmlTrackType::Vocals => {
                    parse_vocal_track_from_chart(chart)
                },
            }
        }
    }
}

fn parse_guitar_track_from_chart(chart: &SongChart, is_bass: bool, track_difficulty: XmlTrackDifficulty) -> XmlTrack {
    let mut track_name = match is_bass {
        true => get_track_name(GuitarInstrument::Bass, track_difficulty),
        _ => get_track_name(GuitarInstrument::Guitar, track_difficulty),
    };

    let mut guitar_track = get_track_by_name(chart, &track_name);
    
    if guitar_track.is_none() && !is_bass {
        return XmlTrack::GuitarBass(Vec::new());
    } else if guitar_track.is_none() {
        // Try getting rhythm track
        track_name = get_track_name(GuitarInstrument::Rhythm, track_difficulty);
        guitar_track = get_track_by_name(chart, &track_name);

        if guitar_track.is_none() {
            return XmlTrack::GuitarBass(Vec::new())
        }
    }

    let sustain_length = (chart.resolution / 4) as u64; // 16th note
    let guitar_notes = &guitar_track
        .unwrap() // Would've exited function if None by now
        .events;

    let fret_notes = guitar_notes
        .iter()
        .filter(|&e| match e.value {
            GuitarEventType::Note(n) => n <= 4,
            _ => false,
        });

    let mut star_power = guitar_notes
        .iter()
        .filter(|e| match e.value {
            GuitarEventType::Starpower => true,
            _ => false,
        });
    
    let mut current_sp_note: Option<&GuitarEvent> = star_power.next();
    let mut current_note: Option<BeatEvent> = None;

    
    
    XmlTrack::GuitarBass(Vec::new())
}

fn get_track_by_name<'a>(chart: &'a SongChart, track_name: &str) -> Option<&'a GuitarTrack> {
    chart
        .guitar_tracks
        .iter()
        .find(|t| t.name.eq(track_name))
}

fn parse_vocal_track_from_chart(chart: &SongChart) -> XmlTrack {
    XmlTrack::Vocals(Vec::new())
}

fn get_track_name(ins: GuitarInstrument, diff: XmlTrackDifficulty) -> String {
    let instrument_name = match ins {
        GuitarInstrument::Guitar => "Single",
        GuitarInstrument::Bass => "DoubleBass",
        GuitarInstrument::Rhythm => "DoubleRhythm",
    };

    let diff_text = match diff {
        XmlTrackDifficulty::Easy => "Easy",
        XmlTrackDifficulty::Medium => "Medium",
        XmlTrackDifficulty::Hard => "Hard",
        XmlTrackDifficulty::Expert => "Expert",
    };

    diff_text.to_string() + instrument_name
}