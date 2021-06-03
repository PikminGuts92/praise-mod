use crate::audio::*;
use crate::chart::*;
use crate::image::*;
use crate::midi::*;
use crate::pack::*;
use crate::shared::*;
use crate::song::*;
use crate::xml::*;
use log::{debug, error, info, warn};
use rayon::prelude::*;
use std::collections::HashMap;
use std::error::Error;
use std::fs::{copy, create_dir_all, read, write};
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};
use std::sync::{Arc, Mutex};

pub fn create_pack(ops: &PackOptions) -> Result<(), Box<dyn Error>> {
    // Start timer
    let overall_start_time = Instant::now();

    let song_paths = find_dirs_with_file_name(&ops.songs_path, "song.ini")?;
    let song_count = song_paths.len();
    let digit_count: usize;

    match song_count {
        0 => {
            error!("No songs found in \"{}\"", &ops.songs_path);
            return Ok(())
        },
        1 => {
            digit_count = 0;
            info!("Found 1 song");
        },
        1000..=usize::MAX => {
            error!("Found {} songs which is over 1000 song limit", song_count);
            return Ok(())
        },
        _ => {
            // Update digit count
            digit_count = match song_count {
                0..=9 => 1,
                10..=99 => 2,
                _ => 3
            };

            info!("Found {} songs", song_count);
        }
    }

    let pack_name = match &ops.name {
        Some(n) => n,
        None => "Custom Song Pack",
    };
    let pack_id = ops.id;

    if pack_id < 4 || pack_id > 98 {
        error!("Pack id value of {} is not valid (must be between 4-98)", pack_id);
        return Ok(())
    }

    info!(
        "Creating song pack with name \"{}\" and id {:03}",
        pack_name,
        pack_id
    );

    let output_dir = Path::new(&ops.output_path)
        .join(format!("ep{:02}", pack_id));

    let global_song_index = Arc::new(Mutex::new(0));

    // Iterate over song directories
    let mut song_results: Vec<(SongMeta, u16)> = song_paths
        .par_iter()
        .enumerate()
        .map(|(id, path)| {
            let song_id = id as u16; // Use index as id

            // Attempt to convert song
            let song_meta = convert_song(path, pack_id, song_id, &output_dir);

            // Update index
            let i: i32;
            {
                // Increment song index
                let mut song_index = global_song_index.lock().unwrap();
                *song_index += 1;

                // Assign value from global index to local
                i = *song_index;
            }

            if song_meta.is_err() {
                warn!(
                    "({:0width$}/{}) Error parsing song in \"{}\", skipping",
                    i,
                    song_count,
                    path.to_str().unwrap(),
                    width = digit_count
                );
                return None
            }

            let song_meta = song_meta.unwrap();

            info!(
                "({:0width$}/{}) Successfully converted \"{} - {}\"",
                i,
                song_count,
                &song_meta.name,
                &song_meta.artist,
                width = digit_count
            );

            return Some((song_meta, song_id))
        })
        .filter(|res| res.is_some())
        .map(|res| res.unwrap())
        .collect();

    if song_results.len() == 0 {
        error!("No songs found could be converted");
        return Ok(());
    }

    // Sort songs by id
    song_results.sort_by(|a, b| a.1.cmp(&b.1));

    // Add songs to builder
    let mut song_builder = XmlSongMetaBuilder::new(pack_name, pack_id);
    for (meta, id) in song_results.iter() {
        song_builder.add_song(meta, *id);
    }

    // Write song pack xml
    let xml_meta = song_builder.to_xml_meta();
    xml_meta.write_to_file(&output_dir.join("master.xml"))?;

    // End timer
    let total_time = format_duration(&overall_start_time.elapsed());
    info!("Complete in {}", &total_time);

    Ok(())
}

fn format_duration(duration: &Duration) -> String {
    let total = duration.as_millis();

    let milli =  total %  1000;
    let secs  = (total /  1000           ) % 60;
    let mins  = (total / (1000 * 60)     ) % 60;
    let hours = (total / (1000 * 60 * 60)) % 60;

    format!(
        "{:02}h:{:02}m:{:02}s:{:03}ms",
        hours,
        mins,
        secs,
        milli
    )
}

fn convert_song(path: &Path, pack_id: u8, song_id: u16, output_dir: &Path) -> Result<SongMeta, Box<dyn Error>> {
    debug!("Parsing song in \"{}\"", path.to_str().unwrap());

    let song_ini = path.join("song.ini");
    let song_meta = SongMeta::from_path(&song_ini)?;

    debug!("Song Information\n\tTitle: {}\n\tArtist: {}\n\tAlbum: {}\n\tYear: {}\n\tPreview: {}",
        song_meta.name,
        song_meta.artist,
        song_meta.album,
        song_meta.year,
        song_meta.preview_start.unwrap_or(0),
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
    convert_song_audio(path, &output_dir, &full_song_id, &song_meta)?;

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
            error!("No chart in either .chart or .mid format found");
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
        error!("{}", error);
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

fn convert_song_audio(path: &Path, output_dir: &Path, full_song_id: &str, song_meta: &SongMeta) -> Result<(), Box<dyn Error>> {
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

    match ogg_stem_paths.len() {
        0 => return Ok(()), // TODO: Return error (no audio found)
        /* 1 => {
            // Only single stem found, no need for re-encoding
            let backing_path = ogg_stem_paths[0];
            ogg_to_dpo(backing_path, &gp_backing_file_path)?;
        }, */
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

            // Decode audio in parallel
            ogg_stems
                .par_iter_mut()
                .for_each(|reader| {
                    reader.read_to_end();
                });

            for ogg_file in ogg_stems.iter_mut() {
                if ogg_writer.is_none() {
                    ogg_writer = Some(AudioWriter::new(common_sample_rate));
                }

                if let Some(writer) = &mut ogg_writer {
                    // Decode audio
                    // ogg_file.read_to_end();

                    if ogg_file.get_sample_rate() != common_sample_rate {
                        // Resample audio to properly merge
                        let resampled = ogg_file.resample(common_sample_rate).unwrap();

                        // Merge with existing track
                        let data = resampled.get_samples();
                        writer.merge_from(data);
                    } else {
                        // Merge with existing track
                        let data = ogg_file.get_samples();
                        writer.merge_from(data);
                    }
                }
            }

            if ogg_writer.is_none() {
                // TODO: Log error and skip song
                return Ok(())
            }

            // Encode mixed audio and write to file
            let mut ogg_writer = ogg_writer.unwrap();
            ogg_writer.fix_clipping();
            ogg_writer.save_as_ogg(&gp_backing_file_path, None);

            // "Encrypt"
            ogg_to_dpo(&gp_backing_file_path, &gp_backing_file_path)?;

            // Generate preview audio
            let preview_writer = create_preview_audio(&ogg_writer, song_meta.preview_start.unwrap_or(20_000));
            preview_writer.save_as_ogg(&gp_preview_file_path, None);
            ogg_preview_path = Some(&gp_preview_file_path);

            // Write silence for instrument "stems"
            ogg_writer.make_silent();
            save_instrument_stems(&ogg_writer, output_dir, full_song_id)?;
        }
    }

    if let Some(prevew_path) = ogg_preview_path {
        // Preview exists, just copy and "encrypt"
        ogg_to_dpo(prevew_path, &gp_preview_file_path)?;
    } else {
        // Generate preview from mixed audio
    }

    Ok(())
}

fn create_preview_audio(mixed_audio: &AudioWriter, preview_start: u32) -> AudioWriter {
    let preview_start = preview_start as f64;
    let preview_time = 30_000.0;

    let (start_pos, preview_len) = match mixed_audio.get_length_in_ms() {
        l if l >= (preview_start + preview_time) => (preview_start, preview_time),
        l if l >= preview_time => ((l - preview_time), preview_time),
        l => (0.0, l),
    };

    mixed_audio.create_sub_writer(start_pos, preview_len)
}

fn save_instrument_stems(silent_audio: &AudioWriter, output_dir: &Path, full_song_id: &str) -> Result<(), Box<dyn Error>> {
    // Create paths for guitar/bass stems
    let audio_paths: Vec<PathBuf> = (0..4)
        .map(|i| output_dir.join(format!("GPG{}_{}.dpo", full_song_id, i)))
        .chain((0..2)
            .map(|i| output_dir.join(format!("GPB{}_{}.dpo", full_song_id, i))))
        .collect();

    // Save and "encrypt"
    let audio_path = &audio_paths[0];
    silent_audio.save_as_ogg(&audio_path, None);
    ogg_to_dpo(&audio_path, &audio_path)?;

    // Copy audio for other paths
    for out_audio_path in audio_paths.iter().skip(1) {
        copy_ogg_file(&audio_path, &out_audio_path)?;
    }

    Ok(())
}