use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::baba::BabaFiles;

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct App {
    state: AppState,
    options: AppOptions
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        todo!()
    }
}

impl App {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }
}

/// The current state of the application
#[derive(Default, Debug, Serialize, Deserialize)]
pub enum AppState {
    /// Currently setting up everything for the user
    #[default]
    Setup,
    Built(BabaFiles),
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct AppOptions {
    
}