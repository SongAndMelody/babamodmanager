use std::{fmt::Display, fs, path::PathBuf, str::FromStr};

use crate::{error::BabaError, files::BabaMod};

/// The name of the file that holds the world data
const WORLD_DATA_FILE_NAME: &str = "world_data.txt";

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

/// Error that might arise when trying to do stuff with levelpacks
#[derive(Debug)]
pub enum LevelpackError {
    /// The requested levelpack does not exist
    LevelpackDoesNotExist(PathBuf),
    /// When parsing a field in `world_data.txt`,
    /// Either the field was not properly formatted (true)
    /// or the field was not what we were looking for (false)
    FieldParsingError(bool),
    /// While parsing a string, something went awry
    StringParsingError(String),
    /// The levelpack does not have an Icon
    IconNotFound(PathBuf),
    /// The levelpack folder does not exist
    LevelpackFolderNotFound {
        /// The path that was attempted to be browsed for the levelpack
        bad_path: String,
    },
    /// The levelpack folder exists, but no pack folders or files were found
    NoLevelpacksFound,
}

impl Display for LevelpackError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let message = match self {
            LevelpackError::LevelpackDoesNotExist(path_buf) => {
                format!("The levelpack at {:?} does not exist", path_buf)
            }
            LevelpackError::FieldParsingError(flag) => match flag {
                true => format!("Malformed world_data.txt"),
                false => format!("Incorrect field"),
            },
            LevelpackError::StringParsingError(str) => {
                format!("Could not parse the following as data: {}", str)
            }
            LevelpackError::IconNotFound(path_buf) => {
                format!("{:?} Is not a valid path for an icon", path_buf)
            }
            LevelpackError::LevelpackFolderNotFound { bad_path } => {
                format!(
                    "The path {:?} should have a levelpack folder, but it does not.",
                    bad_path
                )
            }
            LevelpackError::NoLevelpacksFound => {
                format!("")
            }
        };
        write!(f, "{}", message)
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

/// Represents a file inside the levelpack folder
pub enum LevelpackFile {
    /// The world data file (`world_data.txt`)
    WorldDataTxt,
    /// The levelpack icon (`icon.png`)
    IconPng,
    /// The folder used to hold large images (such as world maps)
    Images,
    /// The folder used to hold mods
    Lua,
    /// The folder used to hold music data (in `.ogg` format)
    Music,
    /// The folder used to hold palettes (5px tall by 7px wide sprites)
    Palettes,
    /// The folder used to hold sprites (24px squared)
    Sprites,
    /// The folder used to hold theme data
    Themes,
}

impl FromStr for LevelpackFile {
    type Err = LevelpackError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s.to_lowercase().as_str() {
            "worlddata" | "world_data.txt" => Self::WorldDataTxt,
            "images" => Self::Images,
            "lua" | "mods" => Self::Lua,
            "palettes" => Self::Palettes,
            "sprites" => Self::Sprites,
            "themes" => Self::Themes,
            "icon" | "icon.png" => Self::IconPng,
            _ => return Err(LevelpackError::StringParsingError(s.to_owned())),
        })
    }
}

impl From<LevelpackFile> for String {
    fn from(value: LevelpackFile) -> Self {
        match value {
            LevelpackFile::WorldDataTxt => "world_data.txt",
            LevelpackFile::Images => "Images",
            LevelpackFile::Lua => "Lua",
            LevelpackFile::Music => "Music",
            LevelpackFile::Palettes => "Palettes",
            LevelpackFile::Sprites => "Sprites",
            LevelpackFile::Themes => "Themes",
            LevelpackFile::IconPng => "icon.png",
        }
        .to_owned()
    }
}

/// Attempts to get and parse a field from a line of text.
///
/// For example: giving `fetch_field<usize>("name", "name=abc")`
/// should return `Ok("abc")`
///
/// # Errors
/// This function may error if:
/// - When parsing the field, it did not match the `"lhs=rhs"` format
/// - The left hand side did not match the `field` parameter
/// - The right hand side could not be properly parsed into the desired type
pub fn fetch_field<T>(field: &str, data: &str) -> Result<T, LevelpackError>
where
    T: FromStr,
{
    let split = data
        .split_once('=')
        .ok_or(LevelpackError::FieldParsingError(true))?;
    if split.0 != field {
        Err(LevelpackError::FieldParsingError(false))
    } else {
        split
            .1
            .parse()
            .map_err(|_| LevelpackError::StringParsingError("Malformed world_data.txt".to_owned()))
    }
}
