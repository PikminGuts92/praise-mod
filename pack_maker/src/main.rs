// Hide console if release build
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app;
use app::*;
use eframe::{IconData, NativeOptions, run_native};
use image::{load_from_memory, EncodableLayout};

static ICON: &'static [u8] = include_bytes!("../res/icon.png");

fn main() {
    let app = PackApp::default();
    let icon = transform_image_to_icon(ICON);

    let ops = NativeOptions {
        drag_and_drop_support: true,
        icon_data: Some(icon),
        ..NativeOptions::default()
    };

    run_native(
        "Pack Maker for GP",
        ops,
        Box::new(|_cc| Box::new(app))
    );
}

fn transform_image_to_icon(data: &[u8]) -> IconData {
    // Open image
    let img = load_from_memory(data).unwrap();

    // Convert to rgba
    let rgba = img.as_rgba8().unwrap();
    let rgba_data = rgba.as_bytes();

    IconData {
        rgba: rgba_data.into(),
        width: img.width(),
        height: img.height(),
    }
}