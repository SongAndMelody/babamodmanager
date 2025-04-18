use egui::ecolor::ParseHexColorError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ApplicationError {
    #[error("Attempt to parse a hex code failed")]
    ColorParsing(ParseHexColorError)
}

impl From<ParseHexColorError> for ApplicationError {
    fn from(v: ParseHexColorError) -> Self {
        Self::ColorParsing(v)
    }
}