use eframe::egui::CtxRef;
use eframe::epi::Frame;
use crate::screen::*;
use eframe::{egui, epi};

pub struct MainScreen {

}

impl MainScreen {
    pub fn new() -> Self {
        Self {

        }
    }
}

impl Screen for MainScreen {
    fn update(&mut self, ctx: &CtxRef, frame: &mut Frame<'_>) -> ScreenResult {

        let mut res = ScreenResult::Ok;

        egui::TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
           if ui.button("Новый герой").clicked() {
               res = ScreenResult::NextScreen(
                   Box::new(crate::character_gen::GenerationTypeScreen::new())
               )
           }
        });

        res
    }
}