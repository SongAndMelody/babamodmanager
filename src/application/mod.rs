use std::fs;

use egui::{epaint::text::FontInsert, FontData};
use themedata::ThemeData;

use crate::error::babaerror::BabaError;

pub mod app;
pub mod appoptions;
pub mod appstate;
pub mod status;
pub mod themedata;
pub mod activeapp;

/// Taken from the documentation for [`egui::ColorImage::from_rgba_unmultiplied`]
pub fn load_image_from_path(path: &std::path::Path) -> Result<egui::ColorImage, image::ImageError> {
    let image = image::ImageReader::open(path)?.decode()?;
    let size = [image.width() as _, image.height() as _];
    let image_buffer = image.to_rgba8();
    let pixels = image_buffer.as_flat_samples();
    Ok(egui::ColorImage::from_rgba_unmultiplied(
        size,
        pixels.as_slice(),
    ))
}

pub fn load_fonts() -> Result<Vec<FontInsert>, BabaError> {
    let mut result = Vec::new();
    for file in fs::read_dir("src\\data\\fonts")? {
        let file = file?;
        let data = fs::read(file.path())?;
        let name = file.file_name().into_string().unwrap_or("".to_owned());
        result.push(FontInsert {
            name,
            data: FontData::from_owned(data),
            families: Vec::new(),
        })
    }
    Ok(result)
}

pub fn load_themes() -> Result<Vec<ThemeData>, BabaError> {
    let mut result = Vec::new();
    for file in fs::read_dir("src\\data\\palettes")? {
        let file = file?;
        let theme = ThemeData::from_image_file(&file.path())?;
        result.push(theme);
    }
    Ok(result)
}

pub const fn pixel_index(x: usize, y: usize) -> usize {
    (y * 7) + x
}