use image::{ImageBuffer, ImageFormat, load_from_memory, open, RgbaImage};
use image::imageops::FilterType;
use std::fs::read;
use std::path::Path;

mod errors;
pub use errors::*;

pub fn resize_and_save_image(in_path: &Path, out_path: &Path, width: u32, height: u32) -> Result<(), ResizeImageError> {
    // Read file to bytes
    let data = read(in_path)
        .map_err(|e| ResizeImageError::CantLoadImageFromFile { text: e.to_string() })?;
    
    // Open image
    let png = load_from_memory(&data)
        .map_err(|e| ResizeImageError::CantLoadImageFromMemory { text: e.to_string() })?;

    // Resize image
    let resized_png = png.resize_exact(width, height, FilterType::Gaussian);

    // Save resized image to file as .png
    resized_png.save_with_format(out_path, ImageFormat::Png)
        .map_err(|e| ResizeImageError::CantSaveImageToFile { text: e.to_string() })?;
    Ok(())
}