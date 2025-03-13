use std::{fs, io, path::PathBuf};

use serde::{Deserialize, Serialize};

use crate::{
    error::BabaError,
    levelpack::{Levelpack, LevelpackError},
    mods::LuaFile,
};

/// A list of "reserved" names that are used by baba.
/// - `baba`, `museum`, and `new_adv` are used to hold data for the game's three campaigns
/// - `debug`, while not explicitly used by the game, is typically not shown to the player without modification
/// - `levels` stores the player's one-off levels
const RESERVED_PACK_NAMES: [&str; 5] = ["baba", "debug", "museum", "new_adv", "levels"];

/// The steam path to Baba is You, if it was installed via steam
const STEAM_PATH: &str = r"C:\Program Files (x86)\Steam\steamapps\common\Baba Is You";

/// The names of all the baba files that contain overridable code.
const BABA_LUA_FILE_NAMES: [&str; 27] = [
    "blocks",
    "changes",
    "clears",
    "colours",
    "conditions",
    "constants",
    "convert",
    "debug",
    "dynamictiling",
    "effects",
    "ending",
    "features",
    "letterunits",
    "load",
    "map",
    "mapcursor",
    "menu",
    "metadata",
    "movement",
    "rules",
    "syntax",
    "tools",
    "undo",
    "update",
    "utf_decoder",
    "values",
    "vision",
];

/// A representation of the Baba is You file structure.
#[derive(Debug, Serialize, Deserialize)]
pub struct BabaFiles {
    // The path to the root folder (that contains the .exe)
    path: PathBuf,
}

impl BabaFiles {
    /// Creates a BabaFiles from a raw path to the root.
    /// This is not usually reccomended, but is
    /// required for e.g. Itch.io installations
    pub fn from_raw(path: PathBuf) -> Self {
        Self { path }
    }
    /// Creates a BabaFiles by looking for the Baba installation from steam.
    ///
    /// # Errors
    /// Returns `Err` under one of two scenarios:
    /// - Steam is not installed (`Err(None)`)
    /// - The file operation returns an error (returns that error in `Err(Some(e))`)
    pub fn from_steam() -> Result<Self, Option<io::Error>> {
        let steam_path = PathBuf::from(STEAM_PATH);
        match fs::exists(steam_path.clone()) {
            Ok(true) => Ok(Self::from_raw(steam_path)),
            Ok(false) => Err(None),
            Err(e) => Err(Some(e)),
        }
    }

    /// Fetches the directory for global mods
    pub fn global_mods_dir(&self) -> PathBuf {
        self.path.join("Lua")
    }
    /// Fetches the directory for levelpacks
    ///
    /// # Errors
    /// Returns [`LevelpackError::LevelpackFolderNotFound`] if the folder for levelpacks doesn't exist
    pub fn levelpacks_dir(&self) -> Result<PathBuf, LevelpackError> {
        let path = self.path.join("Data").join("Worlds");
        if path.exists() {
            Ok(path)
        } else {
            Err(LevelpackError::LevelpackFolderNotFound {
                bad_path: format!("{:?}", path),
            })
        }
    }

    /// Fetches a list of levelpacks from the `Worlds` directory
    ///
    /// # Errors
    /// This function may error if any of the following happen:
    /// - The levelpacks directory does not exist (returns a nested [`LevelpackError::LevelpackFolderNotFound`])
    /// - The directory could not be read (returns a nested [`io::Error`])
    /// - The directory holds zero levelpacks, irrespective of `respect_reserved_names` (returns a nested [`LevelpackError::NoLevelpacksFound`])
    pub fn levelpacks(&self, respect_reserved_names: bool) -> Result<Vec<Levelpack>, BabaError> {
        // get the directory for the levelpacks
        let path = self.levelpacks_dir()?;
        let path_iter = path.read_dir()?;
        // create a list of levelpacks
        let mut result = Vec::new();

        // before we iterate over the entries, check to see if any actually exist
        let iter = path_iter.flatten().collect::<Vec<_>>();
        if iter.len() == 0 {
            return Err(BabaError::LevelpackError(LevelpackError::NoLevelpacksFound));
        }
        // loop over each entry
        'outer: for entry in iter {
            // get the name of the entry
            let path = self.levelpacks_dir()?.join(entry.path());
            // If we're respecting reserved names, we go to the next entry
            // if the file we're on ends in a reserved name
            // see: RESERVED_PACK_NAMES
            if respect_reserved_names {
                for rerserved_name in RESERVED_PACK_NAMES {
                    if path.ends_with(rerserved_name) {
                        continue 'outer;
                    }
                }
            }
            // create a Levelpack from the folder
            let Ok(pack) = Levelpack::new(path) else {
                continue;
            };
            // add it to the list
            result.push(pack);
        }
        Ok(result)
    }

    pub fn native_baba_lua_files(&self) -> Vec<LuaFile> {
        BABA_LUA_FILE_NAMES
            .iter()
            .map(|&name| self.path.join("Data").join(format!("{}.lua", name)))
            .map(|path| fs::read_to_string(path))
            .flatten()
            .map(Into::into)
            .collect()
    }
}
