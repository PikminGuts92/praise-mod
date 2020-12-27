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

pub fn parse_chart(text: &str) -> Result<(), ChartParseError> {
    let (_, mapped_sections) = get_sections_mapped(text)
        .map_err(|_| ChartParseError::InitialParseFail)?;

    for sec_name in mapped_sections.keys() {
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
    }

    Ok(())
}