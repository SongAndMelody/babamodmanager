use egui::{CentralPanel, Context, FullOutput, RawInput, Rect, SidePanel, TopBottomPanel};

use crate::error::babaerror::BabaError;
use std::fmt::Debug;

use super::{appoptions::AppOptions, appstate::AppState, load_fonts, load_themes, status::Status};

/// Represents the currently running application.
/// Has access to all the data involved by mutable reference.
pub struct ActiveApp<'a> {
    /// The currently running context.
    /// Used for hooking into [`egui`] and general GUI-related functions.
    ctx: &'a egui::Context,
    /// The surroundings of the application.
    /// Contains metadata and so forth.
    frame: &'a mut eframe::Frame,
    state: &'a mut AppState,
    status: &'a mut Status,
    options: &'a mut AppOptions,
}

impl<'a> ActiveApp<'a> {
    /// Creates a new "active" application from its baseline data.
    pub fn new(
        ctx: &'a egui::Context,
        frame: &'a mut eframe::Frame,
        state: &'a mut AppState,
        status: &'a mut Status,
        options: &'a mut AppOptions,
    ) -> Self {
        Self {
            ctx,
            frame,
            state,
            status,
            options,
        }
    }

    /// This is the main function called whenever updates need to be run.
    pub fn render(&mut self) -> Result<(), BabaError> {
        match self.status {
            Status::Startup => {
                // application setup: load palettes
                let palettes = load_themes()?;
                self.state.palettes = palettes;
                // load font
                self.load_currently_selected_font()?;
                // startup options
                self.run(|ctx| {
                    central_panel().show(ctx, |ui| {
                    });
                });
            }
            Status::Settings => {}
            Status::About => {}
            Status::Overview => {}
        }
        Ok(())
    }

    pub fn load_currently_selected_font(&mut self) -> Result<(), BabaError> {
        for font in load_fonts()? {
            if font.name == self.options.font {
                self.ctx.add_font(font);
            }
        }
        Ok(())
    }

    pub fn run(&mut self, run_ui: impl FnMut(&Context)) -> FullOutput {
        let input = RawInput::default();
        self.ctx.run(input, run_ui)
    }

    pub fn rendering_area(&self) -> Rect {
        self.ctx.screen_rect()
    }

    pub fn relative_coordinates_to_floaty(&self, x: f32, y: f32) -> (f32, f32) {
        let area = self.rendering_area();
        (area.width() * x, area.height() * y)
    }

    pub fn relative_coordinates_to_absolute(&self, x: f32, y: f32) -> (u64, u64) {
        let area = self.rendering_area();
        ((area.width() * x) as u64, (area.height() * y) as u64)
    }
}

impl<'a> Debug for ActiveApp<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ActiveApp")
            .field("ctx", &self.ctx)
            .field("state", &self.state)
            .field("status", &self.status)
            .field("options", &self.options)
            .finish_non_exhaustive()
    }
}

fn side_panel_left(id: &'static str) -> SidePanel {
    SidePanel::left(id)
}

fn side_panel_right(id: &'static str) -> SidePanel {
    SidePanel::right(id)
}

fn top_panel(id: &'static str) -> TopBottomPanel {
    TopBottomPanel::top(id)
}

fn bottom_panel(id: &'static str) -> TopBottomPanel {
    TopBottomPanel::bottom(id)
}

fn central_panel() -> CentralPanel {
    CentralPanel::default()
}
