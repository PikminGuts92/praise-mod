use super::AudioMeta;
use super::AudioReaderError;
use fon::chan::Ch16;
use fon::{Audio, Sink, Stream};
use lewton::VorbisError;
use lewton::audio::AudioReadError;
use lewton::inside_ogg::OggStreamReader;
use log::{debug, warn};
use std::{convert::AsRef, iter::Zip};
use std::fs::File;
use std::path::{Path};

pub trait AudioReader {
    fn read_to_end(&mut self);
    fn get_samples<'a>(&'a self) -> &'a Vec<Vec<i16>>;
    fn resample(&self, sample_rate: u32) -> Option<ResampledReader>;
}

pub struct OggReader {
    stream: OggStreamReader<File>,
    eof: bool,
    samples: Vec<Vec<i16>>,
}

pub struct ResampledReader {
    sample_rate: u32,
    samples: Vec<Vec<i16>>,
}

impl AudioMeta for ResampledReader {
    fn get_channel_count(&self) -> u8 {
        self.samples.len() as u8
    }

    fn get_sample_rate(&self) -> u32 {
        self.sample_rate
    }
}

impl AudioReader for ResampledReader {
    fn read_to_end(&mut self) {
        // Do nothing
    }

    fn get_samples<'a>(&'a self) -> &'a Vec<Vec<i16>> {
        &self.samples
    }

    fn resample(&self, sample_rate: u32) -> Option<ResampledReader> {
        resample_audio(self, sample_rate)
    }
}

impl OggReader {
    pub fn from_path<T: AsRef<Path>>(ogg_path: T) -> Result<OggReader, AudioReaderError> {
        // To try open file
        let ogg_file = match File::open(ogg_path) {
            Ok(file) => file,
            Err(err) => return Err(AudioReaderError::CantOpenAudioFile {
                text: err.to_string(),
            })
        };

        // Try to initially parse stream
        let stream = match OggStreamReader::new(ogg_file) {
            Ok(reader) => reader,
            Err(err) => return Err(AudioReaderError::CantDecodeAudioFile {
                text: err.to_string(),
            })
        };

        return Ok(OggReader {
            stream,
            eof: false,
            samples: Vec::new(),
        })
    }
}

impl AudioMeta for OggReader {
    fn get_channel_count(&self) -> u8 {
        self.stream.ident_hdr.audio_channels
    }

    fn get_sample_rate(&self) -> u32 {
        self.stream.ident_hdr.audio_sample_rate
    }
}

impl AudioReader for OggReader {
    fn read_to_end(&mut self) {
        if self.eof {
            return
        }

        // Add vector for each channel
        for _ in 0..self.get_channel_count() {
            self.samples.push(Vec::new());
        }

        let mut packet_index = 0;

        loop {
            // Decode packet
            // If error or no packet data, break out of loop
            let mut packet = match self.stream.read_dec_packet() {
                Ok(packet_res) => match packet_res {
                    Some(pkt) => pkt,
                    None => break,
                },
                Err(err) => {
                    warn!("Error parsing ogg packet {}: {}", packet_index, &err);

                    // Skip packet if bad audio or break out of loop otherwise
                    match &err {
                        VorbisError::BadAudio(audio_err) => {
                            match audio_err {
                                AudioReadError::AudioBadFormat => continue,
                                _ => break,
                            }
                        },
                        _ => break,
                    }
                },
            };

            // Iterate over channels and append samples
            for (i, channel_samples) in packet
                .iter_mut()
                .enumerate() {
                self.samples[i].append(channel_samples);
            }

            packet_index += 1;
        }

        self.eof = true;
    }

    fn get_samples<'a>(&'a self) -> &'a Vec<Vec<i16>> {
        &self.samples
    }

    fn resample(&self, sample_rate: u32) -> Option<ResampledReader> {
        resample_audio(self, sample_rate)
    }
}

fn resample_audio<T: AudioReader + AudioMeta>(audio: &T, sample_rate: u32) -> Option<ResampledReader> {
    let in_sample_rate = audio.get_sample_rate();

    let samples = match audio.get_channel_count() {
        1 => {
            vec![
                resample_mono(&audio.get_samples()[0], in_sample_rate, sample_rate)
            ]
        }
        2 => {
            /*
            // Interleave audio
            let l_audio = &audio.get_samples()[0];
            let r_audio = &audio.get_samples()[1];

            let in_samples: Vec<i16> = l_audio
                .iter()
                .zip(r_audio.iter())
                .flat_map(|(l, r)| vec![*l, *r])
                .collect();

            let in_audio = Audio::<Stereo16>::with_i16_buffer(in_sample_rate, in_samples);
            let mut out_audio = Audio::<Stereo16>::with_stream(sample_rate, &in_audio);
            */

            vec![
                // For now it's just way easier to resample each channel instead of interleave -> resample -> deinterleave
                resample_mono(&audio.get_samples()[0], in_sample_rate, sample_rate),
                resample_mono(&audio.get_samples()[1], in_sample_rate, sample_rate)
            ]
        },
        _ => return None
    };

    Some(ResampledReader {
        sample_rate,
        samples
    })
}

fn resample_mono(samples: &Vec<i16>, in_sample_rate: u32, out_sample_rate: u32) -> Vec<i16> {
    // Need to copy data :/
    let in_samples = samples.to_owned();

    let in_audio = Audio::<Ch16, 1>::with_i16_buffer(in_sample_rate, in_samples);
    let mut out_audio = Audio::<Ch16, 1>::with_audio(out_sample_rate, &in_audio);

    out_audio
        .as_i16_slice()
        .iter()
        .map(|v| *v)
        .collect::<Vec<i16>>()
}