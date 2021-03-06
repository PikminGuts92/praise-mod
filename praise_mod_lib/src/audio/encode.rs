use lewton::VorbisError;
use lewton::inside_ogg::OggStreamReader;
use log::{error, info, warn};
use std::convert::AsRef;
use std::error::Error;
use std::fs::File;
use std::io::{self, Cursor};
use std::mem;
use std::path::{Path, PathBuf};
use vorbis_encoder;

pub fn read_ogg_from_file<T: AsRef<Path>>(ogg_path: T) -> Result<(), Box<dyn Error>> {
    let ogg_path = ogg_path.as_ref();
    let ogg_file = File::open(ogg_path)?;

    // Use extension as hint if found
    if let Some(ext) = ogg_path.extension() {
        if let Some(ext) = ext.to_str() {
            //hint.with_extension(ext);
        }
    }

    let mut stream = OggStreamReader::new(ogg_file)?;

    // Read packets
    let mut n = 0;
    let mut len_play = 0.0;
    let mut samples_count = 0;
    while let Some(packet) = stream.read_dec_packet()? {
        n += 1;
        // This is guaranteed by the docs
        assert_eq!(packet.len(), stream.ident_hdr.audio_channels as usize);
        assert_eq!(packet[0].len(), packet[1].len());

        len_play += packet[0].len() as f32 / stream.ident_hdr.audio_sample_rate as f32;
        samples_count += packet[0].len();
    }

    Ok(())
}
