use std::{fs, path::PathBuf};

use crate::{
    error::{babaerror::BabaError, moddingerror::ModdingError},
    files::CONFIG_FILE_NAME,
};

/// Represents a configuration file for a mod, unique to the manager.
/// This also represents a mod that could be fetched from elsewhere.
///
/// # Notes
///
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct Config {
    /// The mod ID, used for compatibilities
    modid: String,
    /// The authors of the mod
    authors: Vec<String>,
    /// The description of the mod
    description: String,
    /// A url for an icon (Optional)
    icon_url: Option<String>,
    /// A url for a banner (Optional)
    banner_url: Option<String>,
    /// whether or not the mod is global
    global: bool,
    /// the set of associated tags (max of four)
    tags: Vec<String>,
    /// Any relevant links to the mod
    links: Vec<String>,
    /// A list of all files that belong to the mod
    files: Vec<String>,
    /// The init file (outside the folder)
    init: Option<String>,
    /// A list of sprites that belong to the mod
    sprites: Vec<String>,
}

impl Config {
    /// Tries to find a config file, given a path to it.
    pub fn new(path: PathBuf) -> Result<Self, BabaError> {
        if !path.ends_with(CONFIG_FILE_NAME) {
            return Err(BabaError::Modding(ModdingError::NotAConfigFile(path)));
        }
        // read out the file as a string
        let file = fs::read_to_string(path)?;
        // parse it as a Config
        let config: Config = serde_json::from_str(&file)?;
        Ok(config)
    }

    pub fn files(&self) -> Vec<String> {
        self.files.clone()
    }

    pub fn init(&self) -> Option<String> {
        self.init.clone()
    }

    pub fn modid(&self) -> String {
        self.modid.clone()
    }

    pub fn authors(&self) -> Vec<String> {
        self.authors.clone()
    }

    pub fn description(&self) -> String {
        self.description.clone()
    }

    pub fn sprites(&self) -> Vec<String> {
        self.sprites.clone()
    }

    /// creates a config directly from json data
    ///
    /// # Errors
    /// Errors if [`serde_json::from_value`] errors.
    pub fn from_json(value: serde_json::Value) -> Result<Self, BabaError> {
        let config: Config = serde_json::from_value(value)?;
        Ok(config)
    }
}
