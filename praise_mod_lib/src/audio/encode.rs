use ogg::*;
use std::convert::AsRef;
use std::error::Error;
use std::fs::File;
use std::io::{self, Cursor};
use std::path::{Path, PathBuf};
use vorbis_sys as vorbis;

pub fn read_ogg_from_file<T: AsRef<Path>>(ogg_path: T) -> Result<(), Box<dyn Error>> {
    let ogg_path = ogg_path.as_ref();
    let mut ogg_file = File::open(ogg_path)?;

    let mut reader = PacketReader::new(ogg_file);

    //reader.
    Ok(())
}

pub fn read_ogg_from_bytes(data: &[u8]) -> Result<(), Box<dyn Error>> {
    let mut data_reader = Cursor::new(data);

    let mut reader = PacketReader::new(data_reader);

    let packetResult = reader.read_packet()?;

    /*let mut vorbis_info = vorbis::vorbis_info {

    };*/
    //vorbis::vorbis_info_init(&mut vorbis_info);

    if let Some(packet) = &packetResult {
        //vorbis::vorbis_sy
    }


    Ok(())
}