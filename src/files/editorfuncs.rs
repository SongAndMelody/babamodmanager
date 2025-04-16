use std::fs;

use serde::{Deserialize, Serialize};

use crate::error::babaerror::BabaError;

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
