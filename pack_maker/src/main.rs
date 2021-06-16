// Hide console if release build
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app;
use app::*;

fn main() {
    let app = PackApp::default();
    let ops = eframe::NativeOptions::default();

    eframe::run_native(Box::new(app), ops);
}