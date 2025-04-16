use std::{fmt::Display, path::PathBuf};

/// Error that might arise when trying to do stuff with levelpacks
#[derive(Debug)]
pub enum LevelpackError {
    /// The requested levelpack does not exist
    LevelpackDoesNotExist(PathBuf),
    /// When parsing a field in `world_data.txt`,
    /// Either the field was not properly formatted (true)
    /// or the field was not what we were looking for (false)
    FieldParsingError(bool),
    /// While parsing a string, something went awry
    StringParsingError(String),
    /// The levelpack does not have an Icon
    IconNotFound(PathBuf),
    /// The levelpack folder does not exist
    LevelpackFolderNotFound {
        /// The path that was attempted to be browsed for the levelpack
        bad_path: String,
    },
    /// The levelpack folder exists, but no pack folders or files were found
    NoLevelpacksFound,
}

impl Display for LevelpackError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let message = match self {
            LevelpackError::LevelpackDoesNotExist(path_buf) => {
                format!("The levelpack at {:?} does not exist", path_buf)
            }
            LevelpackError::FieldParsingError(flag) => match flag {
                true => "Malformed world_data.txt".to_string(),
                false => "Incorrect field".to_string(),
            },
            LevelpackError::StringParsingError(str) => {
                format!("Could not parse the following as data: {}", str)
            }
            LevelpackError::IconNotFound(path_buf) => {
                format!("{:?} Is not a valid path for an icon", path_buf)
            }
            LevelpackError::LevelpackFolderNotFound { bad_path } => {
                format!(
                    "The path {:?} should have a levelpack folder, but it does not.",
                    bad_path
                )
            }
            LevelpackError::NoLevelpacksFound => {
                String::new()
            }
        };
        write!(f, "{}", message)
    }
}
