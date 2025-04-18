use egui::ecolor::ParseHexColorError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ApplicationError {
    #[error("Attempt to parse a hex code failed")]
    ColorParsing(ParseHexColorError),
    #[error("The given image was either too large or too small (most likely the latter)")]
    ImageSize,
    #[error("Error when working with images")]
    ImageError(#[from] image::ImageError),
}

impl From<ParseHexColorError> for ApplicationError {
    fn from(v: ParseHexColorError) -> Self {
        Self::ColorParsing(v)
    }
}