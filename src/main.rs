//! See the readme for more information on this crate.
//! It is not intended to be used as a library.

#![allow(dead_code)]

use application::app::App;
use error::babaerror::BabaError;

pub mod application;
pub mod error;
pub mod files;
pub mod levelpack;
pub mod merge;
pub mod mods;
mod test;

/// The name used by the window.
/// If you've forked this repository, you can change this to indicate so.
const APP_NAME: &str = "Baba Mods Manager";

fn main() -> Result<(), BabaError> {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(APP_NAME, native_options, Box::new(|cc| Ok(Box::new(App::new(cc)))))?;
    Ok(())
}
