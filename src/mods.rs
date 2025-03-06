use std::{
    collections::HashSet,
    ffi::OsStr,
    fs::{self, read_to_string},
    path::PathBuf,
};

use serde::{Deserialize, Serialize};

use crate::error::BabaError;

/// The name of the config file.
/// This should be located inside of the mod folder (i.e. `Lua\[mod]\[this value]`)
const CONFIG_FILE_NAME: &str = "Config.js";
/// The set of characters to search for when ignoring files
/// This is used sometimes for init files and the like
/// This pattern needs to appear as the first line of the file
const IGNORE_FILE_HEADER: &str = "-- BABAMODMANAGER: IGNORE";

/// Represents a Mod in Baba is You
#[derive(Debug)]
pub struct BabaMod {
    path: PathBuf,
    config: Option<Config>,
    name: String,
}

impl BabaMod {
    /// Create a new BabaMod from the path to either the directory, or the file
    pub fn new(path: PathBuf) -> Result<Self, BabaError> {
        let name = path
            .file_name()
            .map(|x| x.to_os_string())
            .unwrap_or("[Invalid Name!]".into())
            .into_string()
            .unwrap_or("[No name Given!]".to_owned());
        let config = Config::new(path.join(CONFIG_FILE_NAME)).ok();
        Ok(Self { path, config, name })
    }

    /// Reports whether the mod is a singleton (i.e. a standalone lua file)
    pub fn is_singleton(&self) -> bool {
        self.path.extension().is_some()
    }

    /// Returns whether this BabaMod has a config file associated with it
    pub fn has_config(&self) -> bool {
        self.config.is_some()
    }

    /// Gets the path for the sprites folder
    pub fn sprites_folder(&self) -> PathBuf {
        self.path.join(r"..\..\Sprites")
    }

    /// Returns a vector of any relevant files to the mod.
    pub fn all_relevant_files(&self) -> Vec<PathBuf> {
        let mut result = Vec::new();
        result.push(self.path.clone());
        // If there's no config, we only worry about ourselves
        let Some(config) = self.config.clone() else {
            return result;
        };
        // first, we push on every file as called for in the config's files set
        config
            .files
            .iter()
            .for_each(|file| result.push(PathBuf::from(file)));
        // TODO: add sprites, etc. to this list
        result
    }

    pub fn overriden_functions(&self) -> HashSet<LuaFunction> {
        let mut result = HashSet::new();
        let iter = self.all_relevant_files().into_iter().filter(is_lua_file);
        for file in iter {
            let Ok(contents) = fs::read_to_string(file) else {
                continue;
            };
            let set = functions_from_string(contents);
            result = result.union(&set).map(Clone::clone).collect();
        }
        result
    }

    pub fn is_compatible_with(&self, other: Self) -> bool {
        self.overriden_functions().is_disjoint(&other.overriden_functions())
    }
}

/// Represents a configuration file for a mod, unique to the manager.
/// this also represents a mod that could be fetched from elsewhere
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct Config {
    /// The mod ID, used for compatibilities
    modid: String,
    /// The authors of the mod
    authors: Vec<String>,
    /// The description of the mod
    description: String,
    /// A url for an icon (Optional)
    icon_url: String,
    /// A url for a banner (Optional)
    banner_url: String,
    /// whether or not the mod is global
    global: bool,
    /// the set of associated tags (max of four)
    tags: Vec<String>,
    /// Any relevant links to the mod
    links: Vec<String>,
    /// A list of all files that belong to the mod
    files: Vec<String>,
    /// A list of sprites that belong to the mod
    sprites: Vec<String>,
}

impl Config {
    /// tries to find a config file from the given path
    /// (should end in [CONFIG_FILE_NAME])
    pub fn new(path: PathBuf) -> Result<Self, BabaError> {
        if !path.ends_with(CONFIG_FILE_NAME) {
            return Err(BabaError::ModdingError(ModdingError::NotAConfigFile));
        }
        // read out the file as a string
        let file = fs::read_to_string(path)?;
        // parse it as a Config
        let config: Config = serde_json::from_str(&file)?;
        Ok(config)
    }

    /// creates a config directly from json data
    pub fn from_json(value: serde_json::Value) -> Result<Self, BabaError> {
        let config: Config = serde_json::from_value(value)?;
        Ok(config)
    }
}

#[derive(Debug)]
pub enum ModdingError {
    NotAConfigFile,
}

// A Lua function used in either a baba mod, or baba is you
#[derive(Debug, Serialize, Deserialize, Hash, PartialEq, Eq, Clone)]
pub struct LuaFunction {
    name: String,
    is_baba_native: bool,
}

fn is_lua_file(path: &PathBuf) -> bool {
    path.extension().map(OsStr::to_os_string) == Some("lua".into())
}

fn functions_from_string(str: String) -> HashSet<LuaFunction> {
    let mut result = HashSet::new();
    for line in str.lines() {
        if line.starts_with("function") {
            let name = line
                .split(' ')
                .nth(1)
                .unwrap_or("FUNCTION_NOT_FOUND")
                .to_owned();
            let is_baba_native = baba_function_names().contains(&name);
            let function = LuaFunction {
                name,
                is_baba_native,
            };
            result.insert(function);
        }
    }
    result
}

fn baba_function_names() -> HashSet<String> {
    include_str!("data/babafuncs.txt")
        .split('\n')
        .map(ToOwned::to_owned)
        .collect()
}
