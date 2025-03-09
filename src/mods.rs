use std::{
    collections::HashSet,
    ffi::OsStr,
    fs::{self, read_to_string},
    path::PathBuf, str::FromStr,
};

use serde::{Deserialize, Serialize};

use crate::error::BabaError;

/// The name of the config file.
/// This should be located inside of the mod folder (i.e. `Lua\[mod]\[this value]`)
const CONFIG_FILE_NAME: &str = "Config.js";

/// Represents a Mod in Baba is You
#[derive(Debug)]
pub struct BabaMod {
    /// The path to the mod (should be absolute)
    path: PathBuf,
    /// The config for the mod, if one exists
    config: Option<Config>,
    /// The name of the mod
    name: String,
}

impl BabaMod {
    /// Create a new BabaMod from the path to either the directory, or the file
    pub fn new(path: PathBuf) -> Self {
        let name = path
            .file_name()
            .map(|x| x.to_os_string())
            .unwrap_or("[Invalid Name!]".into())
            .into_string()
            .unwrap_or("[No name Given!]".to_owned());
        let config = Config::new(path.join(CONFIG_FILE_NAME)).ok();
        Self { path, config, name }
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

    /// Returns a set of functions that the mod overrides
    pub fn overriden_functions(&self) -> HashSet<LuaFunctionDefinition> {
        let mut result = HashSet::new();
        let iter = self.all_relevant_files().into_iter().filter(is_lua_file);
        for file in iter {
            let Ok(contents) = fs::read_to_string(file) else {
                continue;
            };
            let set = functions_from_string(&contents);
            result = result.union(&set).map(Clone::clone).collect();
        }
        result
    }

    /// Grabs all the sprites in the sprites folder by name
    ///
    /// # Errors
    /// Will only throw an error if the directory from [`BabaMod::sprites_folder`] is unable to be read
    pub fn sprites_by_name(&self) -> Result<HashSet<String>, BabaError> {
        Ok(self
            .sprites_folder()
            .read_dir()?
            .flatten()
            .map(|x| x.file_name().into_string().unwrap_or_default())
            .collect())
    }

    /// Returns whether this mod is compatible with another mod
    /// via way of function overrides & sprite checks
    pub fn is_compatible_with(&self, other: &Self) -> bool {
        self.overriden_functions()
            .is_disjoint(&other.overriden_functions())
            && self
                .sprites_by_name()
                .unwrap_or_default()
                .is_disjoint(&other.sprites_by_name().unwrap_or_default())
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
    ///
    /// # Errors
    /// Errors if [`serde_json::from_value`] errors.
    pub fn from_json(value: serde_json::Value) -> Result<Self, BabaError> {
        let config: Config = serde_json::from_value(value)?;
        Ok(config)
    }
}

/// An error arised when dealing with mods
#[derive(Debug)]
pub enum ModdingError {
    /// The specified file was not a config file
    NotAConfigFile,
    /// The specified string could not be parsed into a function
    NotALuaFunction
}

// A Lua function used in either a baba mod, or baba is you
#[derive(Debug, Serialize, Deserialize, Hash, PartialEq, Eq, Clone)]
pub struct LuaFunctionDefinition {
    name: String,
    is_baba_native: bool,
}

impl LuaFunctionDefinition {
    pub fn is_baba_native(&self) -> bool {
        self.is_baba_native
    }
    pub fn name(&self) -> String {
        self.name.clone()
    }
}

impl FromStr for LuaFunctionDefinition {
    type Err = ModdingError;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        if !line.starts_with("function") {
            return Err(ModdingError::NotALuaFunction);
        }
        let name = line
            .split(' ')
            .nth(1)
            .ok_or(ModdingError::NotALuaFunction)?
            .to_owned();
        let is_baba_native = baba_function_names().contains(&name);
        let function = LuaFunctionDefinition {
            name,
            is_baba_native,
        };
        Ok(function)
    }
}

pub struct LuaFunction {
    definition: LuaFunctionDefinition,
    code: String
}

impl LuaFunction {
    /// Creates a [`LuaFunction`] from a definition and code.
    /// Note that this can be the whole code file, and it only picks out
    /// the one function.
    /// 
    /// May return [`None`] if the provided code does not have the value.
    pub fn from_definition_and_code(definition: &LuaFunctionDefinition, code: &str) -> Option<Self> {
        let functions = string_to_function_strings(code);
        functions.into_iter().find(|func| func.definition == *definition)
    }
    pub fn code(&self) -> &str {
        &self.code
    }
}

impl FromStr for LuaFunction {
    type Err = ModdingError;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let function = line.parse()?;
        Ok(Self {
            definition: function,
            code: line.to_owned(),
        })
    }
}

/// Returns whether or not the [`PathBuf`] is a 
fn is_lua_file(path: &PathBuf) -> bool {
    path.extension().map(OsStr::to_os_string) == Some("lua".into())
}

pub fn functions_from_string(str: &str) -> HashSet<LuaFunctionDefinition> {
    let mut result = HashSet::new();
    for line in str.lines() {
        let Ok(function) = line.parse() else {
            continue;
        };
        result.insert(function);
    }
    result
}

fn baba_function_names() -> HashSet<String> {
    include_str!("data/babafuncs.txt")
        .split('\n')
        .map(ToOwned::to_owned)
        .collect()
}

/// Splits a string into a set of Lua functions (also as Strings).
///
/// This discards any extraneous data, only containing the functions.
pub fn string_to_function_strings(file: &str) -> Vec<LuaFunction> {
    // Split the string at every use of `function`
    file.split("function")
        // split it again at every `end` without indentation,
        // then grab the first part (so before the end)
        .map(|x| x.split("\nend").next())
        // we should have at least something before the end
        // so this is just type casting from
        // Option<&str> -> &str
        .map(Option::unwrap_or_default)
        // &str -> String
        .map(ToOwned::to_owned)
        // puts the `function` back on the front of the string
        .map(|str| concat_strings("function".to_owned(), str))
        // puts the `end` on the back of the string
        .map(|str| concat_strings(str, "\nend".to_owned()))
        // String -> Result<LuaFunction, Error>
        .map(|arg0| LuaFunction::from_str(&arg0))
        // Result<LuaFunction, Error> -> LuaFunction (discards errors)
        .flatten()
        // collect it into a list
        .collect()
}

/// Concatenates two strings, putting the second at the end of the first.
pub fn concat_strings(mut left: String, right: String) -> String {
    left.push_str(&right);
    left
}