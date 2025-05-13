use egui::{CentralPanel, Color32, FontId, Rect, SidePanel, TopBottomPanel, Ui};

use crate::error::{applicationerror::ApplicationError, babaerror::BabaError};
use std::fmt::Debug;

use super::{
    appoptions::AppOptions, appstate::AppState, load_fonts, load_themes, status::Status,
    themedata::ThemeData,
};

/// A quick way to create an [egui::Image] via an invocation of [egui::include_image].
macro_rules! image {
    () => {
        compile_error!("This macro requires at least one string literal to grab the image from")
    };
    ($path:expr $(,)?) => {
        egui::Image::new(egui::include_image!($path))
    };
}

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
        let mut this = Self {
            ctx,
            frame,
            state,
            status,
            options,
        };
        let _ = this.setup();
        this
    }

    /// This is the main function called whenever updates need to be run.
    pub fn render(&mut self) -> Result<(), BabaError> {
        match self.status {
            Status::Startup => self.startup(),
            Status::Settings => self.settings(),
            Status::About => self.about(),
            Status::Overview => self.overview(),
        }
    }

    fn setup(&mut self) -> Result<(), BabaError> {
        // application setup: load palettes
        let palettes = load_themes()?;
        self.state.palettes = palettes;
        // load font
        self.load_currently_selected_font()?;
        Ok(())
    }

    pub fn startup(&mut self) -> Result<(), BabaError> {
        Ok(())
    }

    pub fn settings(&mut self) -> Result<(), BabaError> {
        Ok(())
    }

    pub fn about(&mut self) -> Result<(), BabaError> {
        Ok(())
    }

    pub fn overview(&mut self) -> Result<(), BabaError> {
        Ok(())
    }

    pub fn load_currently_selected_font(&self) -> Result<(), BabaError> {
        for font in load_fonts()? {
            if font.name == self.options.font {
                self.ctx.add_font(font);
            }
        }
        Ok(())
    }

    pub fn currently_selected_font_with_size(&self, size: f32) -> Result<FontId, BabaError> {
        for font in load_fonts()? {
            if font.name == self.options.font {
                return Ok(FontId::new(size, egui::FontFamily::Name(font.name.into())));
            }
        }
        Err(BabaError::Application(ApplicationError::FontUnavailible))
    }

    pub fn install_image_loaders(&self) {
        egui_extras::install_image_loaders(self.ctx);
    }

    pub fn currently_selected_font(&self) -> Result<FontId, BabaError> {
        self.currently_selected_font_with_size(1.0)
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

    pub fn dark_color(&self) -> Color32 {
        self.options.theme.dark.color()
    }
    pub fn dark_accent_color(&self) -> Color32 {
        self.options.theme.dark_accent.color()
    }
    pub fn light_color(&self) -> Color32 {
        self.options.theme.light.color()
    }
    pub fn light_accent_color(&self) -> Color32 {
        self.options.theme.light_accent.color()
    }
    pub fn grey_color(&self) -> Color32 {
        self.options.theme.grey.color()
    }
    pub fn error_color(&self) -> Color32 {
        self.options.theme.error.color()
    }
    pub fn warning_color(&self) -> Color32 {
        self.options.theme.warning.color()
    }
    pub fn accept_color(&self) -> Color32 {
        self.options.theme.accept.color()
    }
    pub fn link_color(&self) -> Color32 {
        self.options.theme.link.color()
    }
    pub fn link_visited_color(&self) -> Color32 {
        self.options.theme.link_visited.color()
    }
    pub fn spore_color(&self) -> Color32 {
        self.options.theme.spore.color()
    }
    pub fn blossom_color(&self) -> Color32 {
        self.options.theme.blossom.color()
    }
    pub fn bonus_color(&self) -> Color32 {
        self.options.theme.bonus.color()
    }

    pub fn text_color(&self) -> Color32 {
        match self.options.light_mode {
            true => self.dark_color(),
            false => self.light_color(),
        }
    }
}

impl Debug for ActiveApp<'_> {
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
fn do_nothing(_ui: &mut Ui) {}
