use std::{fmt::Display, fs, path::PathBuf};

use crate::{
    error::{babaerror::BabaError, levelpackerror::LevelpackError},
    mods::babamod::BabaMod,
};

use super::{fetch_field, levelpackfile::LevelpackFile, WORLD_DATA_FILE_NAME};

/// Represents a single levelpack in Baba is you.
#[derive(Default, Debug)]
pub struct Levelpack {
    /// The path to the levelpack (absolute)
    path: PathBuf,
    /// The name of the pack
    name: String,
    /// The author of the pack
    author: String,
    /// The required amount of Spores for 100%
    prize_max: usize,
    /// The required amount of World Map Clears for 100%
    clear_max: usize,
    /// The required amount of Bonuses for 100%
    bonus_max: usize,
    /// Whether or not mods are enabled
    mods_enabled: bool,
}

impl Levelpack {
    /// Create a new Levelpack from a path.
    ///
    /// # Errors
    /// This function may error if:
    /// - The given path does not exist ([`LevelpackError::LevelpackDoesNotExist`])
    /// - There was an issue opening the `world_data.txt`
    pub fn new(path: PathBuf) -> Result<Self, BabaError> {
        // if the levelpack doesn't exist, return early
        if !fs::exists(path.clone())? {
            return Err(BabaError::LevelpackError(
                LevelpackError::LevelpackDoesNotExist(path),
            ));
        }

        // load the world_data.txt into a String
        let world_data = fs::read_to_string(path.join(WORLD_DATA_FILE_NAME))?;

        // Initialize the Levelpack with dummy data
        let mut this = Self {
            path,
            ..Default::default()
        };

        // set data based on each line in the file
        for line in world_data.lines() {
            // this should be read as:
            // Fetch the line in world_data.txt with the leading part "name",
            // if it exists, parse it and return it as `name`
            // else, disregard and continue
            if let Ok(name) = fetch_field("name", line) {
                this.name = name;
            }
            if let Ok(author) = fetch_field("author", line) {
                this.author = author;
            }
            if let Ok(prize_max) = fetch_field("prize_max", line) {
                this.prize_max = prize_max;
            }
            if let Ok(clear_max) = fetch_field("clear_max", line) {
                this.clear_max = clear_max;
            }
            if let Ok(bonus_max) = fetch_field("bonus_max", line) {
                this.bonus_max = bonus_max;
            }

            // special case: the mods toggle is based on whether mods=1, but it's a boolean
            // that is moreso easily found based on whether the line exists, but we
            // check anyways just in case mods=0
            if let Ok(mods) = fetch_field("mods", line) {
                let mods: usize = mods;
                this.mods_enabled = mods != 0;
            }
        }

        Ok(this)
    }

    /// Attempts to find the set of mods in the levelpack.
    /// This may be zero.
    ///
    /// # Errors
    /// This function may error if there was an error reading the mods directory ([`std::io::Error`])
    pub fn mods(&self) -> Result<Vec<BabaMod>, BabaError> {
        // if no mods are meant to be loaded, return an empty set of mods
        if !self.mods_enabled {
            return Ok(vec![]);
        }
        let lua_path = self.pack_file(LevelpackFile::Lua);
        let path_iter = lua_path.read_dir()?;
        // create a list of levelpacks
        let mut result = Vec::new();

        // before we iterate over the entries, check to see if any actually exist
        let iter = path_iter.flatten().collect::<Vec<_>>();
        // if not, just return an empty vector
        if iter.len() == 0 {
            return Ok(vec![]);
        }
        // iterate over each entry
        for entry in iter {
            // create a BabaMod from the entry
            let baba_mod = BabaMod::new(entry.path());
            // push it onto the list
            result.push(baba_mod);
        }
        Ok(result)
    }

    /// Gets the path of a [`LevelpackFile`].
    /// This is generaly an absolute path rather than a relative one.
    pub fn pack_file(&self, file: LevelpackFile) -> PathBuf {
        let joiner: String = file.into();
        self.path.join(joiner)
    }
}

impl Display for Levelpack {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} by {}\nCounts: {}/{}/{}\nMods enabled: {}\nFound at: {:?}",
            self.name,
            self.author,
            self.prize_max,
            self.clear_max,
            self.bonus_max,
            self.mods_enabled,
            self.path
        )
    }
}
