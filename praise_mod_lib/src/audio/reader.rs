use super::AudioMeta;
use super::AudioReaderError;
use lewton::inside_ogg::OggStreamReader;
use std::convert::AsRef;
use std::fs::File;
use std::path::{Path};

pub trait AudioReader {
    fn read_to_end(&mut self);
    fn get_samples<'a>(&'a self) -> &'a Vec<Vec<i16>>;
}

pub struct OggReader {
    stream: OggStreamReader<File>,
    eof: bool,
    samples: Vec<Vec<i16>>,
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

        loop {
            // Decode packet
            // If error or no packet data, break out of loop
            let mut packet = match self.stream.read_dec_packet() {
                Ok(packet_res) => match packet_res {
                    Some(pkt) => pkt,
                    None => break,
                },
                Err(_) => break,
            };

            // Iterate over channels and append samples
            for (i, channel_samples) in packet
                .iter_mut()
                .enumerate() {
                self.samples[i].append(channel_samples);
            }
        }

        self.eof = true;
    }

    fn get_samples<'a>(&'a self) -> &'a Vec<Vec<i16>> {
        &self.samples
    }
}