use std::convert::AsRef;
use std::io::Write;
use std::fs::File;
use std::path::{Path};
use vorbis_encoder::Encoder;

const MIN_SAMPLE_VALUE: i32 = i16::MIN as i32;
const MAX_SAMPLE_VALUE: i32 = i16::MAX as i32;

pub struct AudioWriter {
    channels: u32,
    sample_rate: u32,
    samples: Vec<i16>, // Interleaved samples
}

impl AudioWriter {
    pub fn new(sample_rate: u32) -> AudioWriter {
        AudioWriter {
            channels: 2,
            sample_rate,
            samples: Vec::new(),
        }
    }

    pub fn save_as_ogg<T: AsRef<Path>>(&mut self, ogg_path: T, quality: Option<f32>) {
        // TODO: Handle exceptions

        // Create encoder
        let mut encoder = Encoder::new(
            self.channels,
            self.sample_rate as u64,
            match quality {
                Some(q) => q,
                None => 0.5,
        }).unwrap();

        // Create file
        let mut ogg_file = File::create(ogg_path).unwrap();

        // Encode data + write to file
        let data = encoder.encode(&self.samples).unwrap();
        ogg_file.write(&data).unwrap();

        // Finalize file
        let data = encoder.flush().unwrap();
        ogg_file.write(&data).unwrap();
    }

    pub fn merge_from(&mut self, source: &Vec<Vec<i16>>) {
        let source_channels = source.len() as u32;

        let is_mono = match source_channels {
            n if n == 0 || n > 2 => return,
            1 => true,
            _ => false,
        };

        let left_src = &source[0];
        let right_src = match is_mono {
            true => &source[0],
            _ => &source[1],
        };

        let sample_length = self.samples.len() / self.channels as usize;
        let source_sample_length = source[0].len();

        // Merge audio
        let source_sample_length_itr = source_sample_length * source_channels as usize;
        for (i, sample) in self.samples
            .iter_mut()
            .enumerate() {
                if i == source_sample_length_itr {
                    break
                }

                let sample_idx = i >> 1;
                let to_mix_sample = match i % 1 {
                    0 => left_src[sample_idx],
                    _ => right_src[sample_idx],
                };

                *sample = mix_samples(*sample, to_mix_sample);
        }

        // Append audio
        for i in sample_length..source_sample_length {
            self.samples.push(left_src[i]);
            self.samples.push(right_src[i]);
        }
    }
}

fn mix_samples(a: i16, b: i16) -> i16 {
    let mixed = (a as i32) + (b as i32);

    // Merge samples and cut off any clipping
    if mixed <= MIN_SAMPLE_VALUE {
        i16::MIN
    } else if mixed < MAX_SAMPLE_VALUE {
        mixed as i16
    } else {
        i16::MAX
    }
}