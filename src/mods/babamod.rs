use std::{collections::HashSet, fs, path::PathBuf, str::FromStr};

use crate::{
    error::babaerror::BabaError,
    files::{babafiles::BabaFiles, luafile::LuaFile, writeinto::WriteInto, CONFIG_FILE_NAME},
    merge::{merge_mods, mergeoptions::MergeOptions},
};

use super::{
    config::Config, functions_from_string, is_lua_file, luafuncdef::LuaFuncDef,
    luafunction::LuaFunction,
};

/// Represents a Mod in Baba is You
#[derive(Debug)]
pub struct BabaMod {
    /// The path to the mod (should be absolute)
    path: PathBuf,
    /// The config for the mod, if one exists
    config: Option<Config>,
    /// The name of the mod
    name: String,
}

impl BabaMod {
    /// Create a new BabaMod from the path to either the directory, or the file that exists at the location.
    ///
    /// # Errors
    /// Errors are tossed out - if the name is invalid, the name of the mod is set to `"[Invalid Name!]"`,
    /// and if no name is given, the name of the mod is set to `"[No name Given!]"`.
    pub fn new(path: PathBuf) -> Self {
        let name = path
            .file_name()
            .map(|x| x.to_os_string())
            .unwrap_or("[Invalid Name!]".into())
            .into_string()
            .unwrap_or("[No name Given!]".to_owned());
        let config = Config::new(path.join(CONFIG_FILE_NAME)).ok();
        Self { path, config, name }
    }

    /// Partially initializes a mod.
    /// Do not use this; use [BabaMod::init] or [BabaMod::init_with_options]
    fn partial_init(path: &mut PathBuf, config: &Config) -> Result<String, BabaError> {
        let mod_id = config.modid();
        let init_file_path = config.suitable_init();
        let init_file = include_str!("../../src/data/init.lua").replace("__name__", &mod_id);
        init_file.write_into_using(&*path, &init_file_path)?;
        path.push(&mod_id);
        fs::create_dir(&*path)?;
        config.write_into(&*path)?;
        Ok(mod_id)
    }

    /// Initializes a new mod, given a single lua file, a path to put it in, and a config
    pub fn init(file: LuaFile, mut path: PathBuf, config: Config) -> Result<Self, BabaError> {
        let mod_id = Self::partial_init(&mut path, &config)?;
        // lua file
        file.write_into(&path)?;
        Ok(Self {
            path,
            config: Some(config),
            name: mod_id,
        })
    }

    /// Initializes a new mod, given a single lua file, a path to put it in, a config, and [MergeOptions].
    pub fn init_with_options(
        file: LuaFile,
        mut path: PathBuf,
        config: Config,
        options: MergeOptions,
    ) -> Result<Self, BabaError> {
        let mod_id = Self::partial_init(&mut path, &config)?;
        file.write_into_using(&path, &options.file_name)?;
        Ok(Self {
            path,
            config: Some(config),
            name: mod_id,
        })
    }

    /// Reports whether the mod is a singleton (i.e. a standalone lua file)
    pub fn is_singleton(&self) -> bool {
        self.path.extension().is_some()
    }

    /// Returns whether this mod has a config file associated with it.
    pub fn has_config(&self) -> bool {
        self.config.is_some()
    }

    /// Gets the path for the sprites folder
    pub fn sprites_folder(&self) -> PathBuf {
        self.path.join(r"..\..\Sprites")
    }

    /// Returns a vector of any relevant files to the mod.
    pub fn all_relevant_files(&self) -> Result<Vec<PathBuf>, BabaError> {
        let mut result = Vec::new();
        result.push(self.path.clone());
        // If there's no config, we only worry about ourselves
        let Some(config) = self.config.clone() else {
            return Ok(result);
        };
        // first, we push on every file as called for in the config's files set
        config
            .files()
            .iter()
            .for_each(|file| result.push(PathBuf::from(file)));
        // add sprites
        let sprites = self.sprites_folder();
        'outer: for sprite in sprites.read_dir()?.flatten().map(|entry| entry.path()) {
            'inner: for held_name in self.defined_sprites() {
                let Some(inspected_name) = sprite.file_name().and_then(|name| name.to_str()) else {
                    continue 'inner;
                };
                if inspected_name.contains(&held_name) {
                    result.push(sprite.clone());
                    continue 'outer;
                }
            }
        }
        Ok(result)
    }

    /// Returns a vector of all lua file paths that the mod uses.
    pub fn lua_file_paths(&self, include_init: bool) -> Vec<PathBuf> {
        let mut result: Vec<PathBuf> = self
            .all_relevant_files()
            .unwrap_or_default()
            .into_iter()
            .filter(|path: &PathBuf| is_lua_file(path))
            .collect();
        if include_init {
            if let Some(config) = &self.config {
                if let Some(init) = &config.init() {
                    result.push(PathBuf::from(&init));
                }
            }
        }
        result
    }

    // Returns a vector of all lua files that the mod uses.
    pub fn lua_files(&self, include_init: bool) -> Vec<LuaFile> {
        self.lua_file_paths(include_init)
            .into_iter()
            .flat_map(LuaFile::try_from)
            .collect()
    }

    /// Returns a set of functions that the mod defines.
    /// This is a [`HashSet`] of [`LuaFuncDef`]s, best for comparing
    /// this mod against another.
    pub fn defined_function_definitions(&self) -> HashSet<LuaFuncDef> {
        let mut result = HashSet::new();
        let iter = self
            .all_relevant_files()
            .unwrap_or_default()
            .into_iter()
            .filter(|path: &PathBuf| is_lua_file(path));
        for file in iter {
            let Ok(contents) = fs::read_to_string(file) else {
                continue;
            };
            let set = functions_from_string(&contents);
            result = result.union(&set).map(Clone::clone).collect();
        }
        result
    }

    pub fn defined_functions(&self) -> HashSet<LuaFunction> {
        let mut result = HashSet::new();
        let iter = self
            .all_relevant_files()
            .unwrap_or_default()
            .into_iter()
            .filter(|path: &PathBuf| is_lua_file(path));
        for file in iter {
            let Ok(contents) = fs::read_to_string(file) else {
                continue;
            };
            let Ok(file) = LuaFile::from_str(&contents);
            for func in file.functions() {
                result.insert(func);
            }
        }
        result
    }

    pub fn defined_sprites(&self) -> HashSet<String> {
        if let Some(config) = &self.config {
            config.sprites().into_iter().collect()
        } else {
            HashSet::new()
        }
    }

    /// Grabs all the sprites in the sprites folder by name
    ///
    /// # Errors
    /// Will only throw an error if the directory from [`BabaMod::sprites_folder`] is unable to be read
    pub fn sprites_by_name(&self) -> Result<HashSet<String>, BabaError> {
        Ok(self
            .sprites_folder()
            .read_dir()?
            .flatten()
            .map(|x| x.file_name().into_string().unwrap_or_default())
            .collect())
    }

    /// Returns whether this mod is compatible with another mod
    /// via way of function overrides & sprite checks.
    pub fn is_compatible_with(&self, other: &Self) -> bool {
        self.defined_function_definitions()
            .is_disjoint(&other.defined_function_definitions())
            && self
                .sprites_by_name()
                .unwrap_or_default()
                .is_disjoint(&other.sprites_by_name().unwrap_or_default())
    }

    /// Gets the mod id, or if the config doesn't exist, gets the name instead
    pub fn mod_id(&self) -> String {
        match &self.config {
            Some(config) => config.modid().clone(),
            None => self.name.clone(),
        }
    }

    /// Gets the list of authors, or if the config doesn't exist, returns an empty vector
    pub fn authors(&self) -> Vec<String> {
        match &self.config {
            Some(config) => config.authors().clone(),
            None => vec![],
        }
    }

    /// Gets the name of the mod
    pub fn name(&self) -> String {
        self.name.clone()
    }

    /// Gets the description of the mod, or if the config doesn't exist, returns `"No description given..."`
    pub fn description(&self) -> String {
        match &self.config {
            Some(config) => config.description(),
            None => "No description given...".to_owned(),
        }
    }

    pub fn merge_with(&self, other: &BabaMod, files: &BabaFiles, options: MergeOptions) -> Result<BabaMod, BabaError> {
        merge_mods(self, other, files.native_baba_lua_functions(), options)
    }
}
