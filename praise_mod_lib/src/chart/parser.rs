use character::complete::{alphanumeric1, char};
use error::context;
use nom::*;
use nom::bytes::complete::{is_not, tag};
use nom::sequence::{delimited};

fn get_section_name(text: &str) -> IResult<&str, &str> {
    delimited(
        char('['),
        is_not("]"),
        char(']'))
        (text)
}

pub fn parse_chart(text: &str) {
    //let text = &text[..6];

    let res = get_section_name(text);

    if let Ok((next, matched)) = res {
        println!("Matched: {}", matched);
        //println!("Next: {}", next);
    } else {
        let err = res.unwrap_err();
    }
}