use serde::{Deserialize, Serialize};

use super::{appoptions::AppOptions, appstate::AppState};

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct App {
    state: AppState,
    options: AppOptions,
}

impl eframe::App for App {
    fn update(&mut self, _ctx: &egui::Context, _frame: &mut eframe::Frame) {
        todo!()
    }
}

impl App {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }
}