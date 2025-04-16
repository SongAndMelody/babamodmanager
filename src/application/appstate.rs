use serde::{Deserialize, Serialize};

use crate::files::babafiles::BabaFiles;

/// The current state of the application
#[derive(Default, Debug, Serialize, Deserialize)]
pub enum AppState {
    /// Currently setting up everything for the user
    #[default]
    Setup,
    Built(BabaFiles),
}