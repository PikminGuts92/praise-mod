use crate::chart::*;
use crate::xml::*;
use std::collections::HashSet;

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
        .filter(|&e| match &e.value {
            GuitarEventType::Note(0..=4) => true,
            _ => false,
        });

    let mut star_power = guitar_notes
        .iter()
        .filter(|e| match &e.value {
            GuitarEventType::Starpower => true,
            _ => false,
        });

    let tap_notes = guitar_notes
        .iter()
        .filter(|&e| match &e.value {
            GuitarEventType::Tap => true,
            _ => false,
        })
        .map(|e| e.pos.to_owned())
        .collect::<HashSet<u64>>();
    
    let mut current_sp_note: Option<&GuitarEvent> = star_power.next();
    let mut current_note: Option<BeatEvent> = None;
    let mut xml_notes: Vec<BeatEvent> = Vec::new();

    // Iterate over notes
    for note in fret_notes {
        let pos = note.pos_realtime as u64;
        let length = match note.length {
            l if l > sustain_length  => note.length_realtime as u64,
            _ => 0,
        };

        let fret_number = match note.value {
            GuitarEventType::Note(n) => n as u8,
            _ => 0,
        };

        let mut is_sp_note = false;
        while current_sp_note.is_some() {
            let sp_note = current_sp_note.unwrap();
            let sp_note_end = sp_note.pos + sp_note.length;

            if note.pos >= sp_note.pos && note.pos < sp_note_end {
                // Note is in range of sp
                is_sp_note = true;
                break;
            } else if note.pos >= sp_note_end {
                // Update current star power
                current_sp_note = star_power.next();
            } else { // note.pos < sp_note.pos
                break;
            }
        }

        if let Some(beat_event) = &mut current_note {
            if beat_event.pos == pos {
                // Is part of chord, update current note
                XmlFile::update_fret_beat_event(beat_event, length, fret_number, is_sp_note, tap_notes.contains(&note.pos));
            } else {
                // Pop off current note and add to collection
                let beat_event = current_note.take().unwrap();
                xml_notes.push(beat_event);
            }
        }

        // Add as new note
        if current_note.is_none() {
            let mut beat_event = BeatEvent::default(pos, length);
            XmlFile::update_fret_beat_event(&mut beat_event, length, fret_number, is_sp_note, tap_notes.contains(&note.pos));

            current_note = Some(beat_event);
        }
    }

    if current_note.is_some() {
        // Pop off current note and add to collection
        let beat_event = current_note.take().unwrap();
        xml_notes.push(beat_event);
    }

    XmlTrack::GuitarBass(xml_notes)
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