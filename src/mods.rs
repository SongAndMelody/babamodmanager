use std::{collections::HashSet, ffi::OsStr, fmt::Display, fs, path::PathBuf, str::FromStr};

use serde::{Deserialize, Serialize};

use crate::{error::BabaError, files::CONFIG_FILE_NAME};

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
            return Err(BabaError::ModdingError(ModdingError::NotAConfigFile(path)));
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
                format!("There was an error when attempting to preform a rename while merging")
            }
            ModdingError::NotABabaFunction => {
                format!("The given function was not a baba function, despite being declared one.")
            }
            ModdingError::CodeRemoval => {
                format!("Mods cannot be valid candidates for merging if they remove code from the original.")
            }
            ModdingError::IncompletePatching => {
                format!("The two mods could not be properly merged, as at least one patch could not be applied correctly.")
            }
        };
        write!(f, "{}", message)
    }
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
            return Err(ModdingError::NotALuaFunction(line.to_owned()));
        }
        let name = line
            .split(' ')
            .nth(1)
            .ok_or(ModdingError::NotALuaFunction(line.to_owned()))?
            .split('(')
            .next()
            .ok_or(ModdingError::NotALuaFunction(line.to_owned()))?
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
    pub fn from_definition_and_code(definition: &LuaFuncDef, code: &str) -> Option<Self> {
        let functions = code_to_funcs(code);
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

    fn from_str(code: &str) -> Result<Self, Self::Err> {
        let function = code.parse()?;
        let mut new_code = String::new();
        // CHECK:
        // we don't want any functions that use this form:
        // x = function(args...)
        // replace with the following
        // function x(args...)
        for line in code.lines() {
            if line.contains('=') && line.contains("function") {
                let mut iter = line.split(' ');
                let Some(mut name) = iter.next() else {
                    continue;
                };
                // removing the local
                name = if name == "local" {
                    let Some(name) = iter.next() else {
                        continue;
                    };
                    name
                } else {
                    name
                };
                // intentionally discard the '='
                iter.next();
                // grab the rest
                let rest = iter.fold("".to_owned(), |mut init, next| {
                    init.push_str(next);
                    init
                });
                // split at the function seperator
                let Some((_, mut args)) = rest
                    .split_once('(')
                    .map(|(x, y)| (x.to_owned(), y.to_owned()))
                else {
                    continue;
                };
                // add back on the delimiter
                args.insert(0, '(');
                // format it
                let result = format!("function {name}{args}");
                new_code.push_str(&result);
            } else {
                new_code.push_str(line);
            }
        }
        Ok(Self {
            definition: function,
            code: new_code,
        })
    }
}

/// Returns whether or not the [`PathBuf`] is a lua file
pub fn is_lua_file(path: &PathBuf) -> bool {
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

pub fn baba_function_names() -> HashSet<String> {
    include_str!("data/babafuncs.txt")
        .split('\n')
        .map(ToOwned::to_owned)
        .collect()
}

/// Splits a string into a set of Lua functions (also as Strings).
///
/// This discards any extraneous data, only containing the functions.
pub fn code_to_funcs(file: &str) -> Vec<LuaFunction> {
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
