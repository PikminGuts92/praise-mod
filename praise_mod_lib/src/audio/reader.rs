use super::AudioMeta;
use super::AudioReaderError;
use lewton::inside_ogg::OggStreamReader;
use std::convert::AsRef;
use std::fs::File;
use std::path::{Path};

pub trait AudioReader {
    fn read_to_end(&mut self) -> Vec<Vec<i16>>;
}

pub struct OggReader {
    stream: OggStreamReader<File>,
    pos: u64,
    eof: bool,
    total_samples: usize,
}

impl OggReader {
    pub fn from_path<T: AsRef<Path>>(ogg_path: T) -> Result<OggReader, AudioReaderError> {
        let ogg_path = ogg_path.as_ref();

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
            pos: 0,
            eof: false,
            total_samples: 0,
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
    fn read_to_end(&mut self) -> Vec<Vec<i16>> {
        let mut samples = Vec::new();

        if self.eof {
            return samples
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

            samples.append(&mut packet);
        }

        self.pos = samples.len() as u64;
        self.eof = true;
        self.total_samples = samples.len();

        samples
    }
}