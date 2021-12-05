use eframe::egui::{CtxRef, FontFamily, TextStyle};
use eframe::epi::Frame;
use crate::screen::*;
use crate::character::*;
use eframe::egui;
use crate::class_overwiew::*;

enum SimpleQuestionAnswer {
    NewQuestion(String, Box<SimpleQuestion>),
    Result(String, Character)
}

pub struct SimpleQuestion {
    question : String,
    answers : Vec<SimpleQuestionAnswer>
}

pub struct GenerationTypeScreen {

}

pub struct NoviceGenScreen {
    question : SimpleQuestion
}

pub struct ClassShowScreen {
    character : Character
}

impl ClassShowScreen {
    pub fn new(character : Character) -> Self {
        Self {
            character
        }
    }
}

impl Screen for ClassShowScreen {
    
    fn update(&mut self, ctx: &CtxRef, frame: &mut Frame<'_>) -> ScreenResult {

        let mut res = ScreenResult::Ok;

        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.vertical_centered(|ui| {
                    ui.separator();
                    ui.label(format!("Вы {:?}", Character::get_class_name(&self.character)));
    
                    ui.separator();
                    ui.label(ClassOverview::get(&self.character.class));
    
                    ui.separator();
                    if ui.button("Далее").clicked() {
                        res = ScreenResult::NextScreen(
                            Box::new(crate::race_select::RaceSelectScreen::new(&self.character))
                        );
                    }
                });
            });
            
        });

        res
    }
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
                SimpleQuestionAnswer::Result(s, ch) => {
                    answers.push(SimpleQuestionAnswer::Result(
                        String::from(s.as_str()),
                        ch.clone()
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

impl NoviceGenScreen {
    pub fn new() -> Self {
        Self {
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
            Box::new(NoviceGenScreen::warrior_question())
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

    fn warrior_question() -> SimpleQuestion {
        let mut root = SimpleQuestion::new();
        root.question = String::from("Мой персонаж тяготеет к?");

        root.answers.push(SimpleQuestionAnswer::Result(
            String::from("Вкладыванию в битве всего себя, как эмоционально, так и физически"),
            Character::new_from_class(CharacterClass::Barbarian)
        ));

        root.answers.push(SimpleQuestionAnswer::Result(
            String::from("Выверенному стилю боя, построенному на стиле боя"),
            Character::new_from_class(CharacterClass::Fighter)
        ));

        root.answers.push(SimpleQuestionAnswer::Result(
            String::from("Глубокому пониманию своей внутренней энергии и духовному развитию"),
            Character::new_from_class(CharacterClass::Monk)
        ));
        
        root.answers.push(SimpleQuestionAnswer::Result(
            String::from("Пониманию и единению с природой и природной энергией"),
            Character::new_from_class(CharacterClass::Druid)
        ));

        root
    }

    fn magick_question() -> SimpleQuestion {
        let mut root = SimpleQuestion::new();

        root.question = String::from("Ваш персонаж черпает магическую силу из?");

        root.answers.push(SimpleQuestionAnswer::Result(
            String::from("Глубоких знаний, полученных в академии"),
            Character::new_from_class(CharacterClass::Wizard)
        ));

        root.answers.push(SimpleQuestionAnswer::Result(
            String::from("От темной сущности, что даровала мне силы"),
            Character::new_from_class(CharacterClass::Warlok)
        ));

        root.answers.push(SimpleQuestionAnswer::Result(
            String::from("От природных сил разлитых вокруг"),
            Character::new_from_class(CharacterClass::Sorcerer)
        ));


        root.answers.push(SimpleQuestionAnswer::Result(
            String::from("Глубоких знаний, полученных в магической консерватории"),
            Character::new_from_class(CharacterClass::Bard)
        ));

        root
    }

    fn distan_question() -> SimpleQuestion {
        let mut root = SimpleQuestion::new();
        root.question = String::from("Мой персонаж больше тягеет к ?");

        root.answers.push(SimpleQuestionAnswer::Result(
            String::from("Использованию божественных сил"),
            Character::new_from_class(CharacterClass::Cleric)
        ));

        root.answers.push(SimpleQuestionAnswer::NewQuestion(
            String::from("Проявлению магического искуства"),
            Box::new(NoviceGenScreen::magick_question())
        ));

        root.answers.push(SimpleQuestionAnswer::Result(
            String::from("Честной воинской стали"),
            Character::new_from_class(CharacterClass::Ranger)
        ));

        root
    }

    fn hide_question() -> SimpleQuestion {
        let mut root = SimpleQuestion::new();
        root.question = String::from("Закон никогда не был препятствием вашему персонажу?");

        

        root.answers.push(SimpleQuestionAnswer::Result(
            String::from("Несомненно"), 
            {
                let mut ch = Character::new();
                ch.class = CharacterClass::Rogue;
                ch
            }
        ));
        root.answers.push(SimpleQuestionAnswer::Result(
            String::from("Мой персонаж предпочитает сохранять справедливость в мире"),
            Character::new_from_class(CharacterClass::Ranger)
        ));

        root
    }
}



impl Screen for NoviceGenScreen {
    fn update(&mut self, ctx: &CtxRef, frame: &mut Frame<'_>) -> ScreenResult {

        let mut res = ScreenResult::Ok;

        egui::CentralPanel::default().show(ctx, |ui| {

            let mut new_question = Option::<SimpleQuestion>::None;
            ui.vertical_centered(|ui| {
                ui.add(egui::Label::new(self.question.question.as_str()).text_style(TextStyle::Heading));
                ui.separator();


                for a in &self.question.answers {
                    match a {
                        SimpleQuestionAnswer::NewQuestion(s, q) => {
                            if ui.button(s.as_str()).clicked() {
                                new_question = Option::Some(SimpleQuestion::clone(q));
                            }
                            ui.separator();
                        }
                        SimpleQuestionAnswer::Result(r, ch) => {
                            if ui.button(r.as_str()).clicked() {
                                res = ScreenResult::NextScreen(Box::new(
                                    ClassShowScreen::new(ch.clone())
                                ));
                            }
                            ui.separator();
                        }
                    }
                }
            });


            

            match new_question {
                Option::Some(q) => {
                    self.question = q;
                }
                Option::None => {

                }
            }
        });

        res
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
            ui.vertical_centered(|ui| {
                ui.separator();
                if ui.button("Генерация для новичка").clicked() {
                    res = ScreenResult::NextScreen(Box::new(NoviceGenScreen::default()));
                }
                ui.separator();
            });
            
        });

        res
    }
}