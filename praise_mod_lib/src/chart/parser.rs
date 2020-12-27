use character::complete::{alphanumeric1, char};
use nom::*;
use nom::bytes::complete::{is_not, tag, take_till, take_while};
use nom::combinator::*;
use nom::error::Error;
use nom::sequence::{delimited, pair, preceded, separated_pair, terminated};
use nom::multi::{many0};
use std::collections::HashMap;

static WS_CHARACTERS: &str = " \t\r\n";

fn take_ws(text: &str) -> IResult<&str, &str> {
    take_while(move |c| WS_CHARACTERS.contains(c))(text)
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

pub fn parse_chart(text: &str) {
    let mapped_sections = get_sections_mapped(text);

    if let Ok((next, mapped_sections)) = mapped_sections {
        for sec_name in mapped_sections.keys() {
            println!("{}", *sec_name);
        }

        //let bodyRes = get_section_body(newNext);

        //println!("Name: {}", name);
        //println!("Body: {}", body);
        //println!("Next: {}", next);
    } else {
        //let err = res.unwrap_err();
    }
}