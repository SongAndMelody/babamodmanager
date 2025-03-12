use std::{
    collections::{HashMap, HashSet},
    convert::Infallible,
    ffi::OsStr,
    fs::{self, read_to_string},
    path::PathBuf,
    str::FromStr,
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
    /// Create a new BabaMod from the path to either the directory, or the file that exists at the location.
    /// 
    /// # Errors
    /// Errors are tossed out - if the name is invalid, the name of the mod is set to `"[Invalid Name!]"`,
    /// and if no name is given, the name of the mod is set to `"[No name Given!]"`. 
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

    /// Returns whether this mod has a config file associated with it.
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

    /// Returns a set of functions that the mod defines.
    /// This is a [`HashSet`] of [`LuaFuncDef`]s, best for comparing
    /// this mod against another.
    pub fn defined_functions(&self) -> HashSet<LuaFuncDef> {
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
    /// via way of function overrides & sprite checks.
    pub fn is_compatible_with(&self, other: &Self) -> bool {
        self.defined_functions()
            .is_disjoint(&other.defined_functions())
            && self
                .sprites_by_name()
                .unwrap_or_default()
                .is_disjoint(&other.sprites_by_name().unwrap_or_default())
    }
}

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
    NotALuaFunction,
    /// While merging functions, the rename could not properly be specified
    RenameError,
    /// While merging functions, the given function was not a baba function,
    /// despite having been declared one
    NotABabaFunction,
    /// While merging functions, code was removed
    CodeRemoval
}

// A Lua function used in either a baba mod, or baba is you
#[derive(Debug, Serialize, Deserialize, Hash, PartialEq, Eq, Clone)]
pub struct LuaFuncDef {
    name: String,
    is_baba_native: bool,
}

impl LuaFuncDef {
    pub fn is_baba_native(&self) -> bool {
        self.is_baba_native
    }
    pub fn name(&self) -> String {
        self.name.clone()
    }
}

impl FromStr for LuaFuncDef {
    type Err = ModdingError;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        if !line.starts_with("function") {
            return Err(ModdingError::NotALuaFunction);
        }
        let name = line
            .split(' ')
            .nth(1)
            .ok_or(ModdingError::NotALuaFunction)?
            .split('(')
            .next()
            .ok_or(ModdingError::NotALuaFunction)?
            .to_owned();
        let is_baba_native = baba_function_names().contains(&name);
        let function = LuaFuncDef {
            name,
            is_baba_native,
        };
        Ok(function)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct LuaFunction {
    definition: LuaFuncDef,
    code: String,
}

impl LuaFunction {
    /// Creates a [`LuaFunction`] from a definition and code.
    /// Note that this can be the whole code file, and it only picks out
    /// the one function.
    ///
    /// May return [`None`] if the provided code does not have the value.
    pub fn from_definition_and_code(
        definition: &LuaFuncDef,
        code: &str,
    ) -> Option<Self> {
        let functions = string_to_function_strings(code);
        functions
            .into_iter()
            .find(|func| func.definition == *definition)
    }
    pub fn code(&self) -> &str {
        &self.code
    }
    pub fn definition(&self) -> LuaFuncDef {
        self.definition.clone()
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

/// A representation of an entire lua file.
///
/// Best to create this via parsing a [`String`],
/// or using a [`From`] implementation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LuaFile {
    /// The set of functions (code included)
    functions: Vec<LuaFunction>,
    /// A dictionary of renamed, baba native functions
    /// - Key: the original baba function
    /// - Value: the renamed function
    renamed_functions: HashMap<String, String>,
    /// The entire code (unaltered)
    code: String,
}

impl LuaFile {
    /// Returns the whole code of the file.
    ///
    /// This is from tip to tail, including things not relevant
    /// to the program
    pub fn code(&self) -> String {
        self.code.clone()
    }
    /// Returns the set of function definitions.
    ///
    /// This is merely for quicker use in deciding function
    /// collissions, for better working with functions, use
    /// [`LuaFile::functions`].
    pub fn definitions(&self) -> HashSet<LuaFuncDef> {
        self.functions()
            .into_iter()
            .map(|func| func.definition)
            .collect()
    }
    /// Returns a list of functions in the file.
    ///
    /// These functions are in a more workable format, they can be edited,
    /// altered, etc. You can merge two functions with [`crate::merge::merge_functions`].
    pub fn functions(&self) -> Vec<LuaFunction> {
        self.functions.clone()
    }
    /// Returns a dictionary of renamed functions (for the purposes of the injection method).
    ///
    /// The keys are the old names (see [`baba_function_names`]), and the
    /// values are the new names.
    ///
    /// Supports these kinds of syntax (on structure creation):
    /// - `local new_name = old_name`
    /// - `new_name = old_name`
    pub fn renamed_functions(&self) -> HashMap<String, String> {
        self.renamed_functions.clone()
    }
    /// Returns whether a specified function uses injection.
    ///
    /// This checks whether the name of the definition is found in either
    /// the keys or the values (so it can check either the old or new name).
    /// 
    /// This takes a reference to a [`LuaFuncDef`] - for more
    /// generalized use see [`LuaFile::function_uses_injection_str`].
    pub fn function_uses_injection(&self, func: &LuaFuncDef) -> bool {
        self.function_uses_injection_str(&func.name)
    }

    /// Returns whether a specified function uses injection.
    /// 
    /// This takes a `&str` for generalized use.
    pub fn function_uses_injection_str(&self, func_name: &str) -> bool {
        self.renamed_functions.contains_key(func_name)
            || self
                .renamed_functions
                .values()
                .fold(false, |prev, y| prev || *y == func_name)
    }

    /// Grabs the renamed function for a given definition, if it exists.
    /// 
    /// Returns [`None`] if the rename doesn't exist.
    pub fn injection_data(&self, func: &LuaFuncDef) -> Option<String> {
        self.renamed_functions.get(&func.name()).map(Clone::clone)
    }
}

impl FromStr for LuaFile {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let functions = string_to_function_strings(s);
        // for the renamed functions, they look like this:
        // local new = old
        // new = old
        let mut renamed_functions = HashMap::new();
        'outer: for line in s.lines() {
            for name in baba_function_names() {
                if line.contains(&name) && !line.contains("function") {
                    // removing the `local`
                    let line = line.replace("local", "");
                    let rename = line.split('=').next().unwrap_or("RENAME_NOT_FOUND");
                    // the replace removes spaces so it's just the name
                    renamed_functions.insert(name, rename.to_owned().replace(' ', ""));
                }
            }
        }
        Ok(Self {
            functions,
            renamed_functions,
            code: s.to_owned(),
        })
    }
}

impl From<String> for LuaFile {
    fn from(value: String) -> Self {
        value.parse().unwrap()
    }
}

impl From<&str> for LuaFile {
    fn from(value: &str) -> Self {
        value.parse().unwrap()
    }
}

/// Returns whether or not the [`PathBuf`] is a lua file
fn is_lua_file(path: &PathBuf) -> bool {
    path.extension().map(OsStr::to_os_string) == Some("lua".into())
}

/// Procures a set of [`LuaFuncDef`]s from a string.
/// 
/// This is only the definitions and related data, everything else in the
/// string is ignored
pub fn functions_from_string(str: &str) -> HashSet<LuaFuncDef> {
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
/// A line break is added before the new string.
pub fn concat_strings(mut left: String, right: String) -> String {
    left.push('\n');
    left.push_str(&right);
    left
}
