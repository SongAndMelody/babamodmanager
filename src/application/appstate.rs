use serde::{Deserialize, Serialize};

use crate::files::babafiles::BabaFiles;

use super::themedata::ThemeData;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct AppState {
    pub palettes: Vec<ThemeData>,
    files: Option<BabaFiles>
}

impl AppState {
    pub fn files(&mut self) -> Option<&mut BabaFiles> {
        self.files.as_mut()
    }
    pub fn set_files(&mut self, files: BabaFiles) {
        self.files = Some(files);
    }
}
