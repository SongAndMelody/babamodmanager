use serde::{Deserialize, Serialize};

use super::themedata::ThemeData;

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct AppOptions {
    theme: ThemeData,
    light_mode: bool,
}
