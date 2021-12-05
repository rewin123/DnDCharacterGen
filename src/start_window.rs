use eframe::{egui, epi};
use crate::screen::*;
use crate::main_screen::*;


pub struct TemplateApp {
    screen : Box<dyn Screen>
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            screen : Box::new(ScreenStack::new(Box::new(MainScreen::new())))
        }
    }
}

impl epi::App for TemplateApp {
    fn name(&self) -> &str {
        "eframe template"
    }

    fn setup(
        &mut self,
        _ctx: &egui::CtxRef,
        _frame: &mut epi::Frame<'_>,
        _storage: Option<&dyn epi::Storage>,
    ) {}

    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::CtxRef, frame: &mut epi::Frame<'_>) {

        self.screen.update(ctx, frame);
    }
}
