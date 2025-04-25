use serde::{Deserialize, Serialize};

use crate::error::babaerror::BabaError;

use super::{appoptions::AppOptions, appstate::AppState, load_fonts, load_themes};

#[derive(Debug, Default, Serialize, Deserialize)]
pub enum Status {
    #[default]
    /// Initial startup of the program
    Startup,
    /// The settings menu
    Settings,
    /// Information about the application
    About,
    /// An overview of all the baba files and such
    Overview,
}

impl Status {
    pub fn render(
        &mut self,
        ctx: &egui::Context,
        frame: &mut eframe::Frame,
        state: &mut AppState,
        options: &mut AppOptions,
    ) -> Result<(), BabaError> {
        match self {
            Status::Startup => {
                // application setup: load palettes
                let palettes = load_themes()?;
                state.set_themes(palettes);
                // load fonts
                for font in load_fonts()? {
                    if font.name == options.font {
                        ctx.add_font(font);
                    }
                }
            }
            Status::Settings => {}
            Status::About => {}
            Status::Overview => {}
        }
        Ok(())
    }
}
