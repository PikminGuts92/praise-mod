use crate::apps::{SubApp};
use clap::{Clap};
use log::{info, warn};
use praise_mod_lib::chart::*;
use praise_mod_lib::midi::*;
use praise_mod_lib::pack::*;
use praise_mod_lib::song::*;
use praise_mod_lib::xml::*;
use std::error::Error;
use std::fs::{copy, create_dir_all, read, write};
use std::path::{Path, PathBuf};

#[derive(Clap, Debug)]
pub struct PackCreatorApp {
    #[clap(about = "Path to input CH songs directory", required = true)]
    pub songs_path: String,
    #[clap(about = "Path to output song pack directory", required = true)]
    pub output_path: String,
    #[clap(long, short, about = "Name of song pack")]
    pub name: Option<String>,
    #[clap(long, short, default_value = "4", about = "Numeric id for song pack (must be between 4-98")]
    pub id: u8,
}

impl SubApp for PackCreatorApp {
    fn process(&mut self) -> Result<(), Box<dyn Error>> {
        let song_paths = find_dirs_with_file_name(&self.songs_path, "song.ini")?;

        let pack_name = match &self.name {
            Some(n) => n,
            None => "Custom Song Pack",
        };

        let pack_id = self.id;
        let mut song_id = 0u16;
        let output_dir = Path::new(&self.output_path);

        // Iterate over song directories
        for path in &song_paths {
            let song_meta = convert_song(path, pack_id, song_id, output_dir);

            if song_meta.is_err() {
                warn!("Error parsing song, skipping");
                continue;
            }

            let song_meta = song_meta?;

            song_id += 1; // Increment song id
        }
        Ok(())
    }
}

fn convert_song(path: &Path, pack_id: u8, song_id: u16, output_dir: &Path) -> Result<SongMeta, Box<dyn Error>> {
    info!("Parsing song in \"{}\"", path.to_str().unwrap());

    let song_ini = path.join("song.ini");
    let song_meta = SongMeta::from_path(&song_ini)?;

    info!("Song Information\n\tTitle: {}\n\tArtist: {}\n\tAlbum: {}\n\tYear: {}",
        song_meta.name,
        song_meta.artist,
        song_meta.album,
        song_meta.year,
    );

    let full_song_id = format!("{:02}{:03}", pack_id, song_id);
    let output_dir = output_dir
        .join(&format!("{:02}", pack_id))
        .join(&format!("{:03}", song_id));

    if !output_dir.exists() {
        // Create directory
        create_dir_all(&output_dir)?;
    }

    // Convert chart
    convert_song_chart(path, &output_dir, &full_song_id)?;

    // Copy art
    convert_song_art(path, &output_dir, &full_song_id)?;

    // Convert audio
    convert_song_audio(path, &output_dir, &full_song_id)?;

    Ok(song_meta)
}

fn convert_song_chart(path: &Path, output_dir: &Path, full_song_id: &str) -> Result<(), Box<dyn Error>> {
    let song_chart_path = path.join("notes.chart");
    let song_chart = SongChart::from_path(&song_chart_path)?;

    let instruments = [
        XmlTrackType::Guitar,
        XmlTrackType::Bass,
        XmlTrackType::Vocals,
    ];

    let gtr_difficulties = [
        XmlTrackDifficulty::Easy,
        XmlTrackDifficulty::Medium,
        XmlTrackDifficulty::Hard,
        XmlTrackDifficulty::Expert,
    ];

    for ins_type in &instruments {
        // Parse vocals track
        if *ins_type == XmlTrackType::Vocals {
            let xml_writer = XmlFile::from_chart(&song_chart, *ins_type, None);

            let track_name = format!(
                "GPL{}.xml",
                full_song_id,
            );

            let xml_path = output_dir.join(track_name);
            xml_writer.write_to_file(&xml_path)?;
            continue;
        }

        // Parse guitar/bass tracks
        for (i, diff) in gtr_difficulties.iter().enumerate() {
            let xml_writer = XmlFile::from_chart(&song_chart, *ins_type, Some(*diff));

            let track_name = format!(
                "GP{}{}_{}.xml",
                match &ins_type {
                    XmlTrackType::Guitar => "G",
                    XmlTrackType::Bass => "B",
                    XmlTrackType::Vocals => "L",
                },
                &full_song_id,
                i,
            );

            let xml_path = output_dir.join(track_name);
            xml_writer.write_to_file(&xml_path)?;
        }
    }

    Ok(())
}

fn convert_song_art(path: &Path, output_dir: &Path, full_song_id: &str) -> Result<(), Box<dyn Error>> {
    let album_art_path = path.join("album.png");
    if !album_art_path.exists() {
        info!("No album art found");
        return Ok(());
    }

    // Copy album art to gp song directory
    let gp_art_file_name = format!("GPC{}.png", full_song_id);
    copy(&album_art_path, output_dir.join(&gp_art_file_name))?;

    // TODO: Copy GPK art too
    Ok(())
}

fn convert_song_audio(path: &Path, output_dir: &Path, full_song_id: &str) -> Result<(), Box<dyn Error>> {
    let backing_path = path.join("song.ogg");
    let guitar_path = path.join("guitar.ogg");

    let gp_guitar_file_names = (0..4)
        .map(|i| format!("GPG{}_{}.dpo", full_song_id, i));

    let gp_bass_file_names = (0..2)
        .map(|i| format!("GPB{}_{}.dpo", full_song_id, i));

    // Copy backing track
    if backing_path.exists() {
        let gp_backing_file_name = format!("GPM{}.dpo", full_song_id);
        copy(&backing_path, output_dir.join(gp_backing_file_name))?;

        // TODO: Generate preview audio somehow (for now copy whole song)
        let gp_preview_file_name = format!("GPP{}.dpo", full_song_id);
        copy(&backing_path, output_dir.join(gp_preview_file_name))?;
    }

    // Copy guitar track
    if guitar_path.exists() {
        for gp_guitar_file_name in gp_guitar_file_names {
            let out_guitar_path = output_dir.join(&gp_guitar_file_name);

            // "Encrypt" audio
            let mut data = read(&guitar_path)?;
            for b in data.iter_mut() {
                *b = *b ^ 0x0A;
            }

            // Write to file
            write(&out_guitar_path, data)?;
        }
    }

    Ok(())
}