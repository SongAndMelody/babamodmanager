use std::str::FromStr;

use crate::error::levelpackerror::LevelpackError;

/// Represents a file inside the levelpack folder
#[derive(Debug)]
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
