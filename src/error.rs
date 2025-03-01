use std::io;

use crate::{levelpack::LevelpackError, mods::ModdingError};

#[derive(Debug)]
pub enum BabaError {
    LevelpackError(LevelpackError),
    IOError(io::Error),
    ModdingError(ModdingError),
    SerdeJsonError(serde_json::Error)
}

impl From<LevelpackError> for BabaError {
    fn from(value: LevelpackError) -> Self {
        Self::LevelpackError(value)
    }
}

impl From<io::Error> for BabaError {
    fn from(value: io::Error) -> Self {
        Self::IOError(value)
    }
}

impl From<ModdingError> for BabaError {
    fn from(value: ModdingError) -> Self {
        Self::ModdingError(value)
    }
}

impl From<serde_json::Error> for BabaError {
    fn from(value: serde_json::Error) -> Self {
        Self::SerdeJsonError(value)
    }
}