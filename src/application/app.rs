use serde::{Deserialize, Serialize};

use super::{appoptions::AppOptions, appstate::AppState, status::Status};

#[derive(Default, Debug, Serialize, Deserialize)]
/// A general template to hold Application-related data.
/// All working logic is stored within the implementation details for
/// ActiveApp, however.
pub struct App {
    state: AppState,
    status: Status,
    options: AppOptions,
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        match self.status
            .render(ctx, frame, &mut self.state, &mut self.options) {
                Ok(_) => {}, // explicitly do nothing if everything went alright when updating
                Err(e) => todo!("Should log errors somewhere instead of crashing. Error: {}", e),
            }
    }
}

impl App {
    /// Creates a new Application with a [eframe::CreationContext].
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }
}
