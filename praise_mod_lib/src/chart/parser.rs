use character::complete::{alphanumeric1, char};
use crate::chart::ChartParseError;
use nom::*;
use nom::branch::{alt};
use nom::bytes::complete::{is_not, tag, take_till, take_while};
use nom::combinator::{map};
use nom::error::Error;
use nom::sequence::{delimited, pair, preceded, separated_pair, terminated};
use nom::multi::{many0};
use std::collections::HashMap;

static WS_CHARACTERS: &str = " \t\r\n";
static SPACE_CHARACTERS: &str = " \t";
static NEWLINE_CHARACTERS: &str = "\r\n";

fn take_ws(text: &str) -> IResult<&str, &str> {
    take_while(move |c| WS_CHARACTERS.contains(c))(text)
}

fn take_ws_no_newline(text: &str) -> IResult<&str, &str> {
    take_while(move |c| SPACE_CHARACTERS.contains(c))(text)
}

fn take_newline(text: &str) -> IResult<&str, &str> {
    take_while(move |c| NEWLINE_CHARACTERS.contains(c))(text)
}

fn take_until_newline(text: &str) -> IResult<&str, &str> {
    take_till(move |c| NEWLINE_CHARACTERS.contains(c))(text)
}

fn get_section_name(text: &str) -> IResult<&str, &str> {
    delimited(
        char('['),
        is_not("]"),
        char(']'))
        (text)
}

fn get_section_body(text: &str) -> IResult<&str, &str> {
    delimited(
        char('{'),
        is_not("}"),
        char('}'))
        (text)
}

fn get_section(text: &str) -> IResult<&str, (&str, &str)> {
    pair(
        preceded(
            take_ws, 
            get_section_name,
        ),
        preceded(
            take_ws, 
            get_section_body,
        )
    )(text)
}

fn get_sections(text: &str) -> IResult<&str, Vec<(&str, &str)>> {
    many0(get_section)(text)
}

fn get_sections_mapped(text: &str) -> IResult<&str, HashMap<&str, &str>> {
    let mut mapper = map(
        get_sections,
        |sections| sections
            .into_iter()
            .collect::<HashMap<&str, &str>>(),
    );

    mapper(text)
}

fn get_key_value_pair(text: &str) -> IResult<&str, (&str, &str)> {
    separated_pair(
        preceded(
            take_ws,
            alphanumeric1,
        ),
        preceded(
            take_ws_no_newline,
            char('=')
        ),
        map(
            preceded(
                take_ws_no_newline, 
                take_until_newline,
            ),
            |s| s.trim(),
        ),
    )(text)
}

fn get_key_value_pairs(text: &str) -> IResult<&str, Vec<(&str, &str)>> {
    many0(get_key_value_pair)(text)
}

fn get_key_value_pairs_mapped(text: &str) -> IResult<&str, HashMap<&str, &str>> {
    let mut mapper = map(
        get_key_value_pairs,
        |value_pair| value_pair
            .into_iter()
            .collect::<HashMap<&str, &str>>(),
    );

    mapper(text)
}

fn get_sync_track_parsed(text: &str) -> Result<Vec<(u64, &str, u32)>, ChartParseError> {
    let (_, events) = get_key_value_pairs(text)
        .map_err(|_| ChartParseError::CantParseSyncTrackSection)?;
    
    let res: Vec<(u64, &str, u32)> = events
        .into_iter()
        .map(|(pos, raw_text)| {
            let split_text: Vec<&str> = raw_text.split_whitespace().collect();

            // (pos, ev_type, value)
            (pos.parse::<u64>().unwrap(),
                match split_text.get(0) {
                    Some(v) => *v,
                    None => &"",
                },
                match split_text.get(1) {
                    Some(v) => v.parse().unwrap_or_default(),
                    None => 0,
                })
        })
        .collect();
    
    Ok(res)
}

fn get_guitar_track_parsed<'a>(text: &'a str, track_name: &'a str) -> Result<Vec<(u64, &'a str, u32, u32)>, ChartParseError> {
    let (_, events) = get_key_value_pairs(text)
        .map_err(|_| ChartParseError::CantParseGuitarBassTrackSection{
            track_name: track_name.to_string(),
        })?;

    let res: Vec<(u64, &str, u32, u32)> = events
        .into_iter()
        .map(|(pos, raw_text)| {
            let split_text: Vec<&str> = raw_text.split_whitespace().collect();

            // (pos, ev_type, value_1, value_2)
            (pos.parse::<u64>().unwrap(),
                match split_text.get(0) {
                    Some(v) => *v,
                    None => &"",
                },
                match split_text.get(1) {
                    Some(v) => v.parse().unwrap_or_default(),
                    None => 0,
                },
                match split_text.get(2) {
                    Some(v) => v.parse().unwrap_or_default(),
                    None => 0,
                })
        })
        .collect();

    Ok(res)
}

pub fn parse_chart(text: &str) -> Result<(), ChartParseError> {
    let (_, mapped_sections) = get_sections_mapped(text)
        .map_err(|_| ChartParseError::InitialParseFail)?;

    let mut resolution = 480u16;

    // Parse song/chart metadata
    if let Some(song_section) = mapped_sections.get("Song") {
        let (_, song_meta)= get_key_value_pairs_mapped(song_section)
            .map_err(|_| ChartParseError::CantParseSongSection)?;

        // For now only care about resolution
        if let Some(res_text) = song_meta.get("Resolution") {
            // Update tpq if found in song meta
            if let Ok(res) = res_text.parse::<u16>() {
                resolution = res;
            }
        }
    }

    // Parse tempo track
    if let Some(song_section) = mapped_sections.get("SyncTrack") {
        let sync_track = get_sync_track_parsed(song_section)?;

        for (pos, typ, val) in &sync_track {
            println!("Pos: {}, Type: {}, Value: {}", pos, typ, val);
        }
    }

    let track_difficulties = [
        "Easy",
        "Medium",
        "Hard",
        "Expert",
    ];

    let track_names = [
        "Single",
        "DoubleBass"
    ];

    // Parse guitar tracks
    for diff_name in &track_difficulties {
        let track_name = diff_name.to_string() + "Single";

        if let Some(song_section) = mapped_sections.get(&track_name[..]) {
            let guitar_track = get_guitar_track_parsed(song_section, &track_name)?;
    
            for (pos, typ, val1, val2) in &guitar_track {
                println!("Pos: {}, Type: {}, Value 1: {}, Value 2: {}", pos, typ, val1, val2);
            }
        }
    }

    /*for sec_name in mapped_sections.keys() {
        println!("{}", *sec_name);

        if !sec_name.eq(&"Song") {
            continue;
        }

        let section = *mapped_sections.get(sec_name).unwrap();
        let (next, song_meta)= get_key_value_pairs_mapped(section)
            .map_err(|_| ChartParseError::CantParseSongSection)?;

        
        for meta_key in song_meta.keys() {
            let meta_value = *song_meta.get(meta_key).unwrap();

            println!("\t{}, {}", meta_key, meta_value);
        }
        println!("Next: {}", next);
    }*/

    Ok(())
}