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
    samples: Vec<i32>, // Interleaved samples
}

impl AudioWriter {
    pub fn new(sample_rate: u32) -> AudioWriter {
        AudioWriter {
            channels: 2,
            sample_rate,
            samples: Vec::new(),
        }
    }

    pub fn save_as_ogg<T: AsRef<Path>>(&self, ogg_path: T, quality: Option<f32>) {
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
        let data = encoder.encode(&self.samples_as_i16()).unwrap();
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
                let to_mix_sample = match i % 2 {
                    0 => left_src[sample_idx],
                    _ => right_src[sample_idx],
                };

                // *sample = mix_samples(*sample, to_mix_sample);
                *sample = mix_samples_i32(*sample, to_mix_sample as i32);
        }

        // Append audio
        for i in sample_length..source_sample_length {
            self.samples.push(left_src[i] as i32);
            self.samples.push(right_src[i] as i32);
        }
    }

    fn get_samples_from<'a>(&'a self, start_ms: f64, length_ms: f64) -> &'a [i32] {
        let start_hz = ((start_ms / 1000.0) * self.sample_rate as f64) as usize * (self.channels as usize);
        let end_hz = start_hz + ((length_ms / 1000.0) * self.sample_rate as f64) as usize * (self.channels as usize);

        &self.samples[start_hz..end_hz]
    }

    pub fn create_sub_writer(&self, start_ms: f64, length_ms: f64) -> AudioWriter {
        let sub_samples = self.get_samples_from(start_ms, length_ms);

        AudioWriter {
            channels: self.channels,
            sample_rate: self.sample_rate,
            // Copy samples
            samples: sub_samples
                .iter()
                .map(|v| *v)
                .collect()
        }
    }

    pub fn get_length_in_ms(&self) -> f64 {
        if self.samples.is_empty() {
            return 0.0
        }

        ((self.samples.len() / (self.channels as usize)) as f64 / self.sample_rate as f64) * 1000.0
    }

    fn samples_as_i16(&self) -> Vec<i16> {
        self.samples
            .iter()
            .map(|s| *s as i16)
            .collect()
    }

    pub fn fix_clipping(&mut self) {
        if self.samples.is_empty() {
            return
        }

        // Find max value
        let abs_max = self.samples
            .iter()
            .map(|s| s.abs())
            .max()
            .unwrap();

        if abs_max < MAX_SAMPLE_VALUE {
            return
        }

        let scale = (abs_max as f64) / (MAX_SAMPLE_VALUE as f64);

        for s in self.samples.iter_mut() {
            let new_value = ((*s as f64) / scale) as i32;

            if new_value < MIN_SAMPLE_VALUE {
                *s = MIN_SAMPLE_VALUE;
            } else if new_value < MAX_SAMPLE_VALUE {
                *s = new_value;
            } else {
                *s = MAX_SAMPLE_VALUE
            }
        }
    }

    pub fn make_silent(&mut self) {
        for s in self.samples.iter_mut() {
            *s = 0;
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

fn mix_samples_i32(a: i32, b: i32) -> i32 {
    let mixed = (a as i64) + (b as i64);

    // Merge samples and cut off any clipping
    if mixed <= (i32::MIN as i64) {
        i32::MIN
    } else if mixed < (i32::MAX as i64) {
        mixed as i32
    } else {
        i32::MAX
    }
}