use std::{fmt::Display, path::PathBuf};

/// An error arised when dealing with mods
#[derive(Debug)]
pub enum ModdingError {
    /// The specified file was not a config file
    NotAConfigFile(PathBuf),
    /// The specified string could not be parsed into a function
    NotALuaFunction(String),
    /// While merging functions, the rename could not properly be specified
    RenameError,
    /// While merging functions, the given function was not a baba function,
    /// despite having been declared one
    NotABabaFunction,
    /// While merging functions, code was removed
    CodeRemoval,
    /// While patching together functions, at least one patch didn't work correctly
    IncompletePatching,
}

impl Display for ModdingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let message = match self {
            ModdingError::NotAConfigFile(path_buf) => {
                format!(
                    "The file at {:?} is not a valid configuration file.",
                    path_buf
                )
            }
            ModdingError::NotALuaFunction(str) => {
                format!(
                    "The following was expected to be a lua function, but it wasn't:\n{}",
                    str
                )
            }
            ModdingError::RenameError => {
                "There was an error when attempting to preform a rename while merging".to_string()
            }
            ModdingError::NotABabaFunction => {
                "The given function was not a baba function, despite being declared one.".to_string()
            }
            ModdingError::CodeRemoval => {
                "Mods cannot be valid candidates for merging if they remove code from the original.".to_string()
            }
            ModdingError::IncompletePatching => {
                "The two mods could not be properly merged, as at least one patch could not be applied correctly.".to_string()
            }
        };
        write!(f, "{}", message)
    }
}
