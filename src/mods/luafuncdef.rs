use std::str::FromStr;

use serde::{Deserialize, Serialize};

use crate::error::moddingerror::ModdingError;

use super::baba_function_names;

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
