use log::{info, warn};
use std::{fs::File, writeln};
use std::io::{Write, BufReader, BufRead, Error};
use std::path::{Path, PathBuf};
use crate::xml::*;

#[derive(Debug)]
pub struct XmlFile {
    pub track: XmlTrack,
}

impl XmlFile {
    pub(crate) fn update_fret_beat_event(note: &mut BeatEvent, length: u64, index: u8, sp: bool) {
        // Update length if small
        if note.length < length {
            note.length = length;
        }

        // Update fret (0 = green, 1 = red, etc.)
        match index {
            0 => note.green = true,
            1 => note.red = true,
            2 => note.yellow = true,
            3 => note.blue = true,
            4 => note.orange = true,
            _ => return,
        }

        // Update star power
        if sp {
            note.star_power = true
        }
    }

    pub(crate) fn concat_text(text: &Vec<String>) -> String {
        let mut new_text = String::new();
        let mut prev_concat = false;

        for (i, t) in text.iter().enumerate() {
            let mut has_dash = false;
            let mut is_unpitched = false;

            if t.eq("+") {
                continue;
            } else if t.ends_with("-") || t.ends_with("=") {
                has_dash = true;
            } else if t.ends_with("#") {
                is_unpitched = true;
            }

            if !prev_concat && i > 0 {
                new_text += " ";
            }

            let t_max_idx = match has_dash || is_unpitched {
                true => t.len() - 1,
                _ => t.len(),
            };

            new_text += &t[..t_max_idx];
            prev_concat = has_dash;
        }

        new_text
    }
}