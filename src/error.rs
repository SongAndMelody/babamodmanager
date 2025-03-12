use std::io;

use crate::{levelpack::LevelpackError, mods::ModdingError};

/// A generic error that holds any given error that the program may arise
#[derive(Debug)]
pub enum BabaError {
    /// There was an issue with fetching levelpacks
    LevelpackError(LevelpackError),
    /// There was an error when using [`io`] or working with files
    IOError(io::Error),
    /// There was an error when fetching or working with mods
    ModdingError(ModdingError),
    /// There was an error when using [`serde_json`]
    SerdeJsonError(serde_json::Error),
    /// There was an error when using [`diff_match_patch_rs`]
    DmpError(diff_match_patch_rs::Error)
}

impl From<diff_match_patch_rs::Error> for BabaError {
    fn from(v: diff_match_patch_rs::Error) -> Self {
        Self::DmpError(v)
    }
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