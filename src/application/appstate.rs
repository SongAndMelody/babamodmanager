use serde::{Deserialize, Serialize};

use super::themedata::ThemeData;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct AppState {
    pub palettes: Vec<ThemeData>
}

impl AppState {
    
}
