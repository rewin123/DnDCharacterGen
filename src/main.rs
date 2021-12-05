#![forbid(unsafe_code)]
#![cfg_attr(not(debug_assertions), deny(warnings))] // Forbid warnings in release builds
#![warn(clippy::all, rust_2018_idioms)]

use DnDCharacterGen::*;

// When compiling natively:
#[cfg(not(target_arch = "wasm32"))]
fn main() {
    let app = start_window::TemplateApp::default();
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(Box::new(app), native_options);
}
