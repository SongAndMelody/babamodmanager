use serde::{Deserialize, Serialize};

use super::themedata::ThemeData;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct AppState {
    palettes: Vec<ThemeData>
}

impl AppState {
    pub fn set_themes(&mut self, themes: Vec<ThemeData>) {
        self.palettes = themes;
    }
}
