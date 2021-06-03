mod encode;
mod errors;
mod meta;
mod reader;
mod writer;

pub use self::encode::*;
pub use self::errors::*;
pub use self::meta::*;
pub use self::reader::*;
pub use self::writer::*;
use std::error::Error;
use std::fs::{copy, create_dir_all, read, write};
use std::path::{Path, PathBuf};

const SILENT_DPO: &'static [u8] = include_bytes!("silence.dpo");

pub fn ogg_to_dpo(in_path: &Path, out_path: &Path) -> Result<(), Box<dyn Error>> {
    // Read in bytes
    let mut data = read(&in_path)?;

    // "Encrypt" audio
    for b in data.iter_mut() {
        *b = *b ^ 0x0A;
    }

    // Write to file
    write(&out_path, data)?;
    Ok(())
}

pub fn save_dpo_slience(out_path: &Path) -> Result<(), Box<dyn Error>> {
    // Write to file (already "encrypted")
    write(&out_path, &SILENT_DPO)?;
    Ok(())
}

pub fn copy_ogg_file(in_path: &Path, out_path: &Path) -> Result<(), Box<dyn Error>> {
    // Copy file
    copy(&in_path, &out_path)?;
    Ok(())
}