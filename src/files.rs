use std::{
    collections::{HashMap, HashSet},
    convert::Infallible,
    fs,
    str::FromStr,
};

use serde::{Deserialize, Serialize};

use crate::{
    error::BabaError,
    mods::{baba_function_names, string_to_function_strings, LuaFuncDef, LuaFunction},
};

#[derive(Debug, Serialize, Deserialize)]
struct EditorFuncs {
    menufuncs: Vec<String>,
    menudata_customscript: Vec<String>,
}

pub fn editor_functions() -> Result<Vec<String>, BabaError> {
    let file = fs::read_to_string(r"data\editorfuncs.json")?;
    let funcs: EditorFuncs = serde_json::from_str(&file)?;
    let menudata = funcs
        .menudata_customscript
        .into_iter()
        .map(|name| format!("menudata_customscript.[{name}]"));
    let menufuncs = funcs
        .menufuncs
        .into_iter()
        .map(|name| format!("menufuncs.[{name}].enter"));
    let result = menudata.chain(menufuncs).collect();
    Ok(result)
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
            .map(|func| func.definition())
            .collect()
    }
    /// Returns a list of functions in the file.
    ///
    /// These functions are in a more workable format, they can be edited,
    /// altered, etc. You can merge two functions with [`crate::merge::merge_files`].
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
        self.function_uses_injection_str(&func.name())
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
        for line in s.lines() {
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
