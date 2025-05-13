use serde::{Deserialize, Serialize};

use crate::error::babaerror::BabaError;

use super::{activeapp::ActiveApp, appoptions::AppOptions, appstate::AppState};

#[derive(Debug, Default, Serialize, Deserialize, Clone, Copy, Hash)]
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
        let mut active = ActiveApp::new(ctx, frame, state, self, options);
        active.install_image_loaders();
        active.render()
    }
}
