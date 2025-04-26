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
}

impl<'a> Debug for ActiveApp<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ActiveApp")
            .field("ctx", &self.ctx)
            .field("state", &self.state)
            .field("status", &self.status)
            .field("options", &self.options)
            .finish()
    }
}
