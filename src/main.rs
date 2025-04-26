//! See the readme for more information on this crate.
//! It is not intended to be used as a library.

#![allow(dead_code)]

use application::{app::App, icon};
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
const APP_NAME: &str = "Baba Mod Manager";

fn main() -> Result<(), BabaError> {
    let mut native_options = eframe::NativeOptions::default();
    native_options.viewport = native_options.viewport.with_icon(icon()?);
    eframe::run_native(APP_NAME, native_options, Box::new(|cc| Ok(Box::new(App::new(cc)))))?;
    Ok(())
}
