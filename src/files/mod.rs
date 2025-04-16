pub mod babafiles;
pub mod editorfuncs;
pub mod luafile;

/// The name of the config file.
/// This should be located inside of the mod folder (i.e. `Lua\[mod]\[this value]`)
pub const CONFIG_FILE_NAME: &str = "Config.json";

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
