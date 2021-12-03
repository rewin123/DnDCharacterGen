use eframe::egui::{CtxRef, TextStyle};
use eframe::epi::Frame;
use crate::screen::*;
use crate::character::*;
use eframe::egui;

enum SimpleQuestionAnswer {
    NewQuestion(String, Box<SimpleQuestion>),
    Result(String)
}

pub struct SimpleQuestion {
    question : String,
    answers : Vec<SimpleQuestionAnswer>
}

pub struct GenerationTypeScreen {

}

pub struct NoviceGenScreen {
    character : Character,
    question : SimpleQuestion
}

impl NoviceGenScreen {
    pub fn new() -> Self {
        Self {
            character : Character::new(),
            question : SimpleQuestion::new()
        }
    }

    pub fn default() -> Self {
        let mut screen = NoviceGenScreen::new();
        screen.question = NoviceGenScreen::root_question();

        screen
    }

    fn root_question() -> SimpleQuestion {
        let mut root = SimpleQuestion::new();
        root.question = String::from("Каков стиль боя вашего персонажа?");

        root.answers.push(SimpleQuestionAnswer::NewQuestion(
            String::from("Буду сражаться с врагом лицом к лицу"),
            Box::new(NoviceGenScreen::hide_question())
        ));

        root.answers.push(SimpleQuestionAnswer::NewQuestion(
            String::from("Буду держаться на расстоянии, атакуя издалека"),
            Box::new(NoviceGenScreen::hide_question())
        ));

        root.answers.push(SimpleQuestionAnswer::NewQuestion(
            String::from("Враг умрет, не заметив моего присутсвия"),
            Box::new(NoviceGenScreen::hide_question())
        ));

        root
    }

    fn hide_question() -> SimpleQuestion {
        let mut root = SimpleQuestion::new();
        root.question = String::from("Закон никогда не был препятствием вашему персонажу?");

        root
    }
}

impl Screen for NoviceGenScreen {
    fn update(&mut self, ctx: &CtxRef, frame: &mut Frame<'_>) -> ScreenResult {

        egui::CentralPanel::default().show(ctx, |ui| {


           ui.add(egui::Label::new(self.question.question.as_str()).text_style(TextStyle::Heading));

            for a in &self.question.answers {
                match a {
                    SimpleQuestionAnswer::NewQuestion(s, q) => {
                        if ui.button(s.as_str()).clicked() {

                        }
                    }
                    SimpleQuestionAnswer::Result(r) => {

                    }
                }
            }
        });

        ScreenResult::Ok
    }
}

impl SimpleQuestion {
    pub fn new() -> Self {
        Self {
            question : String::from(""),
            answers : vec![]
        }
    }
}

impl GenerationTypeScreen {
    pub fn new() -> Self {
        Self {

        }
    }
}

impl Screen for GenerationTypeScreen {
    fn update(&mut self, ctx: &CtxRef, frame: &mut Frame<'_>) -> ScreenResult {

        let mut res = ScreenResult::Ok;

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:
            if ui.button("Назад").clicked() {
                res = ScreenResult::Back;
            }
        });


        egui::CentralPanel::default().show(ctx, |ui| {
            ui.button("Ручное создание (not worker)");
            if ui.button("Генерация для новичка").clicked() {
                res = ScreenResult::NextScreen(Box::new(NoviceGenScreen::default()));
            }
            ui.button("Рандомное безумие! (not worked)");
        });

        res
    }
}