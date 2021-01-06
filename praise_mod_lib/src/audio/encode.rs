use default::get_probe;
use ogg::*;
use std::convert::AsRef;
use std::error::Error;
use std::fs::File;
use std::io::{self, Cursor};
use std::mem;
use std::path::{Path, PathBuf};
use symphonia;
//use symphonia::core::errors::{Result, Error};
use symphonia::core::codecs::DecoderOptions;
use symphonia::core::formats::{Cue, FormatReader, FormatOptions, SeekTo, Stream};
use symphonia::core::meta::{ColorMode, MetadataOptions, Tag, Visual};
use symphonia::core::io::{MediaSourceStream, MediaSource, ReadOnlySource};
use symphonia::core::probe::{Hint, ProbeResult};
use symphonia::core::units::{Duration, Time};
use symphonia::default;
use vorbis_encoder;

pub fn read_ogg_from_file<T: AsRef<Path>>(ogg_path: T) -> Result<(), Box<dyn Error>> {
    let ogg_path = ogg_path.as_ref();
    let mut ogg_file = File::open(ogg_path)?;

    let mut hint = Hint::new();

    // Use extension as hint if found
    if let Some(ext) = ogg_path.extension() {
        if let Some(ext) = ext.to_str() {
            hint.with_extension(ext);
        }
    }

    let mut media = MediaSourceStream::new(Box::new(ogg_file));
    let format_opts: FormatOptions = Default::default();
    let metadata_opts: MetadataOptions = Default::default();

    let decoder_options = DecoderOptions {
        verify: false,
    };

    // TODO: Handle errors
    let probe = symphonia::default::get_probe().format(&hint, media, &format_opts, &metadata_opts)?;

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