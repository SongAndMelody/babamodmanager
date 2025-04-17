use std::str::FromStr;

use serde::{Deserialize, Serialize};

use crate::error::moddingerror::ModdingError;

use super::{code_to_funcs, luafuncdef::LuaFuncDef};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
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
