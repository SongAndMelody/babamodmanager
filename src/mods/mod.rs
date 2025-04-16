use std::{collections::HashSet, ffi::OsStr, path::Path, str::FromStr};

use luafuncdef::LuaFuncDef;
use luafunction::LuaFunction;

pub mod babamod;
pub mod config;
pub mod luafuncdef;
pub mod luafunction;

/// Returns whether or not the [`PathBuf`] is a lua file
pub fn is_lua_file(path: &Path) -> bool {
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
    include_str!("../data/babafuncs.txt")
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
        .flat_map(|arg0| LuaFunction::from_str(&arg0))
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
