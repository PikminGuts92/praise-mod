use default::get_probe;
use log::{error, info, warn};
use ogg::*;
use std::convert::AsRef;
use std::error::Error;
use std::fs::File;
use std::io::{self, Cursor};
use std::mem;
use std::path::{Path, PathBuf};
use symphonia;
//use symphonia::core::errors::{Result, Error as SymError};
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
    let ogg_file = File::open(ogg_path)?;

    let mut hint = Hint::new();

    // Use extension as hint if found
    if let Some(ext) = ogg_path.extension() {
        if let Some(ext) = ext.to_str() {
            hint.with_extension(ext);
        }
    }

    let media = MediaSourceStream::new(Box::new(ogg_file));
    let format_opts: FormatOptions = Default::default();
    let metadata_opts: MetadataOptions = Default::default();

    let decoder_options = DecoderOptions {
        verify: false,
    };

    // TODO: Handle errors
    let probeRes = symphonia::default::get_probe().format(&hint, media, &format_opts, &metadata_opts);
    
    let probe = match probeRes {
        Ok(prob) => prob,
        Err(err) => {
            error!("file not supported. reason? {}", err);
            panic!();
        }
    };
    
    let mut reader = probe.format;

    let stream = reader.default_stream().unwrap();

    let mut decoder = symphonia::default::get_codecs().make(&stream.codec_params, &decoder_options)?;

    // TODO: Handle errors
    let result = loop {
        // Read the next packet.
        match reader.next_packet() {
            Ok(packet) => {
                // Decode the packet.
                match decoder.decode(&packet) {
                    Err(symphonia::core::errors::Error::DecodeError(err)) => {
                        // warn!("decode error: {}", err);
                        continue;
                    },
                    Err(err) => break,
                    buffResult        => {
                        if let Ok(buff) = buffResult {
                            
                        }
                    },
                }
            }
            Err(err) => break
        }
    };

    // Close the decoder.
    decoder.close();

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