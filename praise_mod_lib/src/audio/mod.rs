mod encode;

pub use self::encode::*;
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