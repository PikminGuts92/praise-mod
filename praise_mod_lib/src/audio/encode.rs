use lewton::VorbisError;
use lewton::inside_ogg::OggStreamReader;
use log::{error, info, warn};
use ogg::*;
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
    while let Some(packet) = stream.read_dec_packet()? {
        n += 1;
        // This is guaranteed by the docs
        assert_eq!(packet.len(), stream.ident_hdr.audio_channels as usize);
        len_play += packet[0].len() as f32 / stream.ident_hdr.audio_sample_rate as f32;
    }

    Ok(())
}

pub fn read_ogg_from_bytes(data: &[u8]) -> Result<(), Box<dyn Error>> {
    let mut data_reader = Cursor::new(data);


    let mut reader = PacketReader::new(data_reader);
    let packetResult = reader.read_packet()?;

    if let Some(packet) = &packetResult {

    } 

    Ok(())
}