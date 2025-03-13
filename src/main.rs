#![allow(dead_code)]
#![warn(clippy::todo)]

mod app;
mod baba;
mod levelpack;
mod test;
mod mods;
mod error;
mod merge;

use eframe::{self, NativeOptions};

/// The name used by the window.
/// If you've forked this repository, you can change this to indicate so.
const APP_NAME: &str = "Baba Mods Manager";

fn main() -> eframe::Result {
    let native_options = NativeOptions {
        ..Default::default()
    };
    eframe::run_native(
        APP_NAME,
        native_options,
        Box::new(|cc| Ok(Box::new(app::App::new(cc)))),
    )
}
