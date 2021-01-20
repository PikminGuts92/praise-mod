use crate::apps::{SubApp};
use clap::{Clap};
use log::{info, warn};
use praise_mod_lib::audio::*;
use praise_mod_lib::chart::*;
use praise_mod_lib::image::*;
use praise_mod_lib::midi::*;
use praise_mod_lib::pack::*;
use praise_mod_lib::song::*;
use praise_mod_lib::xml::*;
use std::collections::HashMap;
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

#[derive(Debug)]
enum ChartFile {
    Chart(SongChart),
    Midi(MidiFile),
}

impl ChartFile {
    fn is_midi(&self) -> bool {
        match &self {
            ChartFile::Midi(_) => true,
            _ => false,
        }
    }
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
        let output_dir = Path::new(&self.output_path)
            .join(format!("ep{:02}", pack_id));

        let mut song_builder = XmlSongMetaBuilder::new(pack_name, pack_id);

        // Iterate over song directories
        for path in &song_paths {
            let song_meta = convert_song(path, pack_id, song_id, &output_dir);

            if song_meta.is_err() {
                warn!("Error parsing song, skipping");
                continue;
            }

            let song_meta = song_meta?;
            song_builder.add_song(&song_meta, song_id);

            song_id += 1; // Increment song id
        }

        if song_id == 0 {
            warn!("No songs found in input directory");
            return Ok(());
        }

        // Write song pack xml
        let xml_meta = song_builder.to_xml_meta();
        xml_meta.write_to_file(&output_dir.join("master.xml"))?;

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
    let mut song_chart_path = path.join("notes.chart");
    let chart_file;

    if song_chart_path.exists() {
        let song_chart = SongChart::from_path(&song_chart_path)?;
        chart_file = ChartFile::Chart(song_chart);
    } else {
        // Chart not found, try mid
        song_chart_path = path.join("notes.mid");

        if !song_chart_path.exists() {
            warn!("No chart in either .chart or .mid format found");
        }

        // TODO: Throw custom error instead
        let mid = MidiFile::from_path(&song_chart_path)?;
        chart_file = ChartFile::Midi(mid);
    }

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
            let xml_writer = match &chart_file {
                ChartFile::Chart(chart) => XmlFile::from_chart(chart, *ins_type, None),
                ChartFile::Midi(mid) => XmlFile::from_midi(mid, *ins_type, None)
            };

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
            let xml_writer = match &chart_file {
                ChartFile::Chart(chart) => XmlFile::from_chart(chart, *ins_type, Some(*diff)),
                ChartFile::Midi(mid) => XmlFile::from_midi(mid, *ins_type, Some(*diff))
            };

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
    let gp_art_file_path = output_dir.join(format!("GPC{}.png", full_song_id));

    // Resize image
    let resize_res = resize_and_save_image(&album_art_path, &gp_art_file_path, 256, 256);

    if resize_res.is_err() {
        let error = resize_res.unwrap_err();
        warn!("{}", error);
        return Err(Box::new(error));
    }

    // TODO: Copy GPK art too
    Ok(())
}

fn is_file_preview<T: AsRef<Path>>(file_path: T, ext: &str) -> bool {
    match file_path.as_ref().file_name() {
        Some(f_name) => match f_name.to_str() {
            Some(f_name_str) => f_name_str.eq_ignore_ascii_case(&format!("preview.{}", ext)),
            None => false,
        },
        None => false,
    }
}

fn convert_song_audio(path: &Path, output_dir: &Path, full_song_id: &str) -> Result<(), Box<dyn Error>> {
    let ogg_paths = get_files_in_dir(path, Some(&"ogg"))?;

    let mut ogg_preview_path = None;
    let ogg_stem_paths = ogg_paths
        .iter()
        .filter(|p| {
            if is_file_preview(p, "ogg") {
                ogg_preview_path = Some(*p);
                false
            } else {
                true
            }
        })
        .collect::<Vec<&PathBuf>>();
    
    let gp_backing_file_path = output_dir.join(format!("GPM{}.dpo", full_song_id));
    let gp_preview_file_path = output_dir.join(format!("GPP{}.dpo", full_song_id));

    let gp_guitar_file_paths = (0..4)
        .map(|i| output_dir.join(format!("GPG{}_{}.dpo", full_song_id, i)));

    let gp_bass_file_paths = (0..2)
        .map(|i| output_dir.join(format!("GPB{}_{}.dpo", full_song_id, i)));

    match ogg_stem_paths.len() {
        0 => return Ok(()), // TODO: Return error (no audio found)
        1 => {
            // Only single stem found, no need for re-encoding
            let backing_path = ogg_stem_paths[0];
            ogg_to_dpo(backing_path, &gp_backing_file_path)?;
        },
        _ => {
            // Read each ogg stem (initial metadata)
            let mut ogg_stems: Vec<OggReader> = ogg_stem_paths
                .iter()
                .map(|p| OggReader::from_path(p))
                .filter_map(Result::ok) // Only map ok results TODO: Maybe log warnings for skipped stems?
                .collect();

            // Gets the most common sample rate
            let common_sample_rate = *ogg_stems
                .iter()
                .map(|o| o.get_sample_rate())
                .fold(HashMap::<u32, usize>::new(), |mut m, sr| {
                    *m.entry(sr).or_default() += 1;
                    m
                })
                .iter()
                .max_by_key(|(_, f)| *f)
                .map(|(sr, _)| sr)
                .unwrap();

            // Multiple stems found, mix and re-encode
            let mut ogg_writer = None;
            for ogg_file in ogg_stems.iter_mut() {
                if ogg_writer.is_none() {
                    ogg_writer = Some(AudioWriter::new(ogg_file.get_sample_rate()));
                }

                if let Some(writer) = &mut ogg_writer {
                    // Decode audio
                    ogg_file.read_to_end();

                    let mut data = ogg_file.get_samples();

                    if ogg_file.get_sample_rate() != common_sample_rate {
                        // Resample audio to properly merge
                    }

                    // Merge with existing track
                    writer.merge_from(data);
                }
            }

            if ogg_writer.is_none() {
                // TODO: Log error and skip song
                return Ok(())
            }

            // Encode mixed audio and write to file
            let ogg_writer = ogg_writer.unwrap();
            ogg_writer.save_as_ogg(&gp_backing_file_path, None);

            // "Encrypt"
            ogg_to_dpo(&gp_backing_file_path, &gp_backing_file_path)?;

            // Generate preview audio
            let preview_writer = ogg_writer.create_sub_writer(20_000.0, 30_000.0);
            preview_writer.save_as_ogg(&gp_preview_file_path, None);
            ogg_preview_path = Some(&gp_preview_file_path);
        }
    }

    // Write silence for guitar tracks
    for out_guitar_path in gp_guitar_file_paths {
        save_dpo_slience(&out_guitar_path)?;
    }

    // Write silence for bass tracks
    for out_bass_path in gp_bass_file_paths {
        save_dpo_slience(&out_bass_path)?;
    }

    if let Some(prevew_path) = ogg_preview_path {
        // Preview exists, just copy and "encrypt"
        ogg_to_dpo(prevew_path, &gp_preview_file_path)?;
    } else {
        // Generate preview from mixed audio
    }

    Ok(())
}