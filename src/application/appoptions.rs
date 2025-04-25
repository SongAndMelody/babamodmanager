use serde::{Deserialize, Serialize};

use super::themedata::ThemeData;

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct AppOptions {
    pub theme: ThemeData,
    pub light_mode: bool,
    pub font: String,
}
