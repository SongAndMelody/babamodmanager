use std::path::PathBuf;

use serde::{Deserialize, Serialize};

/// A set of options to be configured when merging two mods.
#[derive(Debug, Serialize, Deserialize)]
pub struct MergeOptions {
    /// Whether or not to include init files, if nessecary
    pub include_init: bool,
    /// Where to drop off the merged code
    pub location: PathBuf,
    /// the name of the lua file to be deposited
    pub file_name: String
}