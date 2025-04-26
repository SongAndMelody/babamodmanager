use thiserror::Error;

use crate::error::moddingerror::ModdingError;
use std::{fmt::Display, io};

use super::{applicationerror::ApplicationError, levelpackerror::LevelpackError};

/// A generic error that holds any given error that the program may arise
#[derive(Debug, Error)]
pub enum BabaError {
    /// There was an issue with fetching levelpacks
    Levelpack(#[from] LevelpackError),
    /// There was an error when using [`io`] or working with files
    IO(io::Error),
    /// There was an error when fetching or working with mods
    Modding(#[from] ModdingError),
    /// There was an error when using [`serde_json`]
    SerdeJson(serde_json::Error),
    /// There was an error when using [`diff_match_patch_rs`]
    Dmp(diff_match_patch_rs::Error),
    /// An error arose from the application itself (usually the UI side of things)
    Application(#[from] ApplicationError),
    /// An error came from eframe
    EFrame(#[from] eframe::Error)
}

impl From<diff_match_patch_rs::Error> for BabaError {
    fn from(v: diff_match_patch_rs::Error) -> Self {
        Self::Dmp(v)
    }
}

impl From<io::Error> for BabaError {
    fn from(value: io::Error) -> Self {
        Self::IO(value)
    }
}

impl From<serde_json::Error> for BabaError {
    fn from(value: serde_json::Error) -> Self {
        Self::SerdeJson(value)
    }
}

impl Display for BabaError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let message = match self {
            BabaError::Levelpack(levelpack_error) => format!("{}", levelpack_error),
            BabaError::IO(error) => format!("Error when working with io:\n{}", error),
            BabaError::Modding(modding_error) => format!("{}", modding_error),
            BabaError::SerdeJson(error) => format!("Error when parsing json:\n{}", error),
            BabaError::Dmp(error) => format!("Error when merging files:\n{:#?}", error),
            BabaError::Application(application_error) => format!("Application error:\n{}", application_error),
            BabaError::EFrame(error) => format!("Eframe error:\n{}", error),
        };
        write!(f, "{}", message)
    }
}
