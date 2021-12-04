use eframe::egui::{CtxRef, FontFamily, TextStyle};
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

impl SimpleQuestion {
    pub fn clone(&self) -> Self {
        let head = String::from(self.question.as_str());
        let mut answers = Vec::<SimpleQuestionAnswer>::new();
        for a in &self.answers {
            match a {
                SimpleQuestionAnswer::NewQuestion(s, q) => {
                    answers.push(SimpleQuestionAnswer::NewQuestion(
                        String::from(s.as_str()),
                        Box::new(SimpleQuestion::clone(q)))
                    );
                }
                SimpleQuestionAnswer::Result(s) => {
                    answers.push(SimpleQuestionAnswer::Result(
                        String::from(s.as_str())
                    ));
                }
            }
        }

        Self { 
            question : head,
            answers
        }
    }
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
            String::from("Я не побоюсь выйти на бой лицом к лицу"),
            Box::new(NoviceGenScreen::hide_question())
        ));

        root.answers.push(SimpleQuestionAnswer::NewQuestion(
            String::from("Буду держаться на расстоянии, атакуя издалека"),
            Box::new(NoviceGenScreen::distan_question())
        ));

        root.answers.push(SimpleQuestionAnswer::NewQuestion(
            String::from("Враг умрет, не заметив моего присутсвия"),
            Box::new(NoviceGenScreen::hide_question())
        ));

        root
    }

    fn magick_question() -> SimpleQuestion {
        let mut root = SimpleQuestion::new();

        root.question = String::from("Ваш персонаж черпает магическую силу из?");

        root.answers.push(SimpleQuestionAnswer::NewQuestion(
            String::from("Глубоких знаний, полученных в академии"),
            Box::new(NoviceGenScreen::hide_question())
        ));

        root.answers.push(SimpleQuestionAnswer::NewQuestion(
            String::from("От темной сущности, что даровала мне силы"),
            Box::new(NoviceGenScreen::hide_question())
        ));

        root.answers.push(SimpleQuestionAnswer::NewQuestion(
            String::from("От природных сил разлитых вокруг"),
            Box::new(NoviceGenScreen::hide_question())
        ));


        root.answers.push(SimpleQuestionAnswer::NewQuestion(
            String::from("Глубоких знаний, полученных в магической консерватории"),
            Box::new(NoviceGenScreen::hide_question())
        ));

        root
    }

    fn distan_question() -> SimpleQuestion {
        let mut root = SimpleQuestion::new();
        root.question = String::from("Мой персонаж больше тягеет к ?");

        root.answers.push(SimpleQuestionAnswer::NewQuestion(
            String::from("Использованию божественных сил"),
            Box::new(NoviceGenScreen::hide_question())
        ));

        root.answers.push(SimpleQuestionAnswer::NewQuestion(
            String::from("Проявлению магического искуства"),
            Box::new(NoviceGenScreen::magick_question())
        ));

        root.answers.push(SimpleQuestionAnswer::NewQuestion(
            String::from("Честной воинской стали"),
            Box::new(NoviceGenScreen::hide_question())
        ));

        root
    }

    fn hide_question() -> SimpleQuestion {
        let mut root = SimpleQuestion::new();
        root.question = String::from("Закон никогда не был препятствием вашему персонажу?");

        root.answers.push(SimpleQuestionAnswer::Result(
            String::from("Несомненно")
        ));
        root.answers.push(SimpleQuestionAnswer::Result(
            String::from("Мой персонаж предпочитает сохранять справедливость в мире")
        ));

        root
    }
}



impl Screen for NoviceGenScreen {
    fn update(&mut self, ctx: &CtxRef, frame: &mut Frame<'_>) -> ScreenResult {

        egui::CentralPanel::default().show(ctx, |ui| {


           ui.add(egui::Label::new(self.question.question.as_str()).text_style(TextStyle::Heading));
            ui.separator();

            let mut new_question = Option::<SimpleQuestion>::None;

            for a in &self.question.answers {
                match a {
                    SimpleQuestionAnswer::NewQuestion(s, q) => {
                        if ui.button(s.as_str()).clicked() {
                            new_question = Option::Some(SimpleQuestion::clone(q));
                        }
                        ui.separator();
                    }
                    SimpleQuestionAnswer::Result(r) => {
                        if ui.button(r.as_str()).clicked() {

                        }
                        ui.separator();
                    }
                }
            }

            match new_question {
                Option::Some(q) => {
                    self.question = q;
                }
                Option::None => {

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