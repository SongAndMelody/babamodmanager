use egui::{
    text::LayoutJob, CentralPanel, FontId, Rect, SidePanel, TextFormat, TopBottomPanel, Ui,
};

use crate::error::{applicationerror::ApplicationError, babaerror::BabaError};
use std::fmt::Debug;

use super::{
    appoptions::AppOptions, appstate::AppState, load_fonts, load_themes, status::Status,
    themedata::ThemeData,
};

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
                central_panel().show(self.ctx, |_ui| {
                    let mut job = LayoutJob::default();
                    let font_id = self.currently_selected_font().unwrap_or_default();
                    job.append(
                        "baba",
                        0.0,
                        TextFormat::simple(font_id, self.theme_data().bonus.color()),
                    );
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

    pub fn currently_selected_font(&mut self) -> Result<FontId, BabaError> {
        for font in load_fonts()? {
            if font.name == self.options.font {
                return Ok(FontId::new(1.0, egui::FontFamily::Name(font.name.into())));
            }
        }
        Err(BabaError::Application(ApplicationError::FontUnavailible))
    }

    pub fn theme_data(&self) -> ThemeData {
        self.options.theme
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

/// Explitily do nothing with a [Ui] object.
fn do_nothing_with_ui(_ui: &mut Ui) {}
