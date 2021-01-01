use crate::song::SongMeta;
use std::collections::{HashMap, HashSet};
use crate::xml::*;

pub struct XmlSongMetaBuilder {
    pub pack_name: String,
    pub pack_id: u8,
    song_metas: Vec<(SongMeta, u16)>,
}

impl XmlSongMetaBuilder {
    pub fn new(name: &str, pack_id: u8) -> XmlSongMetaBuilder {
        XmlSongMetaBuilder {
            pack_name: name.to_string(),
            pack_id,
            song_metas: Vec::new(),
        }
    }

    pub fn add_song(&mut self, song_meta: &SongMeta, id: u16) {
        self.song_metas.push((song_meta.clone(), id)); // Assume ids won't be duplicated
    }

    pub fn to_xml_meta(&self) -> XmlFile {
        // Get unique album names
        let mut albums = self.song_metas
            .iter()
            .map(|(meta, _)| meta.album.as_str())
            .collect::<HashSet<&str>>()
            .into_iter()
            .collect::<Vec<&str>>();

        albums.sort();

        let albums_by_id = albums
            .iter()
            .enumerate()
            .map(|(i, album)| (*album, i as u16))
            .collect::<HashMap<&str, u16>>();

        let song_data = self.song_metas
            .iter()
            .map(|(meta, id)| XmlSong {
                song_id: *id,
                artist: meta.artist.to_owned(),
                title: meta.name.to_owned(),
                album_idx: match albums_by_id.get(meta.artist.as_str()) {
                    Some(id) => *id,
                    None => 0,
                },
            })
            .collect();

        XmlFile {
            track: XmlTrack::Metadata {
                name: self.pack_name.to_owned(),
                pack_id: self.pack_id,
                albums: albums
                    .into_iter()
                    .map(|album| album.to_string())
                    .collect(),
                songs: song_data,
            }
        }
    }
}