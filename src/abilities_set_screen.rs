use crate::screen::*;
use crate::character::*;
use eframe::{egui, epi};

enum AbState {
    Init,
    SetDrops,
    Fill
}

pub struct AutoSetAbilitiesScreen {
    charadcter : Character,
    drops : Vec<i32>,
    cubes : Vec<i32>,
    ability_place : Vec<AbilityType>,
    state : AbState,
    rng : oorandom::Rand32
}

impl AutoSetAbilitiesScreen {
    pub fn new(character : &Character) -> Self {
        unsafe {
            Self {
                charadcter: character.clone(),
                drops: vec![],
                cubes: vec![],
                ability_place: vec![],
                state: AbState::Init,
                rng: oorandom::Rand32::new(crate::start_window::frames_count.clone())
            }
        }
    }

    pub fn rand6(&mut self) -> i32 {
        self.rng.rand_range(1..7) as i32
    }
}

impl Screen for AutoSetAbilitiesScreen {
    fn update(&mut self, ctx: &egui::CtxRef, frame: &mut epi::Frame<'_>) -> ScreenResult {
        let mut res = ScreenResult::Ok;
        let anim_step = 5000_f32;

        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.vertical_centered(|ui| {
                    ui.heading("Определение характеристик персонажа");
                    ui.separator();
                    ui.separator();

                    match self.state {
                        AbState::Init => {
                            ui.label("Автоматические броски кубиков для выбора значений характеристик");
                            ui.label("Значение характеристик персонажа определяется как сумма трех наибольших значений от бросков четырех шестигранных кубиков.");

                            if ui.button("Начать броски кубиков").clicked() {
                                self.state = AbState::SetDrops;

                                while self.drops.len() < 6 {
                                    let mut cubes = vec![self.rand6(), self.rand6(), self.rand6(), self.rand6()];
                                    cubes.sort_unstable();
                                    let value = cubes[1] + cubes[2] + cubes[3];
                                    self.drops.push(value);
                                    self.cubes.extend(cubes);
                                }

                                let mut ability_order = crate::race_select::SelectRaceDesc::get_ability_order(&self.charadcter.class);
                                let mut ability_idx = vec![AbilityType::Wisdom, AbilityType::Wisdom, AbilityType::Wisdom, AbilityType::Wisdom, AbilityType::Wisdom, AbilityType::Wisdom];
                                let mut drops = self.drops.clone();
                                for a_idx in 0..ability_order.len() {
                                    let mut mav = drops[0];
                                    let mut mav_idx = 0_usize;
                                    for i in 0..drops.len() {
                                        if mav < drops[i] {
                                            mav = drops[i];
                                            mav_idx = i;
                                        }
                                    }

                                    ability_idx[mav_idx] = ability_order[a_idx].clone();
                                    drops[mav_idx] = 0;
                                }

                                self.ability_place = ability_idx;

                            }

                        }
                        AbState::SetDrops => {
                            ui.label("Автоматические броски кубиков для выбора значений характеристик");
                            ui.label("Значение характеристик персонажа определяется как сумма трех наибольших значений от бросков четырех шестигранных кубиков.");

                            egui::ScrollArea::horizontal().show(ui, |ui| {
                                egui::Grid::new("attributes_grid_set").striped(true).show(ui, |ui| {
                                    for i in 0..4 {
                                        ui.label(format!("Куб {}", i + 1));
    
                                        ui.label(format!("{}", self.cubes[i]));
                                        ui.label(format!("{}", self.cubes[4 + i]));
                                        ui.label(format!("{}", self.cubes[2 * 4 + i]));
                                        ui.label(format!("{}", self.cubes[3 * 4 + i]));
                                        ui.label(format!("{}", self.cubes[4 * 4 + i]));
                                        ui.label(format!("{}", self.cubes[5 * 4 + i]));
                                        ui.end_row();
                                    }
    
                                    ui.label("Результат");
    
                                    for i in 0..self.drops.len() {
                                        ui.label(format!("{}", self.drops[i]));
                                    }
                                    ui.end_row();

                                    ui.label("Выбранная \nхарактеристика");
                                    for i in 0..self.ability_place.len() {
                                        ui.label(format!("{:?}", self.ability_place[i]));
                                    }
                                    ui.end_row();

                                    let race_base = Character::get_ability_buf(&self.charadcter.exact_race);
                                    ui.label("Бонус \nрасы");
                                    for i in 0..self.ability_place.len() {
                                        ui.label(format!("+{}", race_base.get_by_type(&self.ability_place[i]).value));
                                    }
                                    ui.end_row();

                                    ui.label("Значения \nхарактеристик \nперсонажа");
                                    for i in 0..self.ability_place.len() {
                                        ui.label(format!("{}", race_base.get_by_type(&self.ability_place[i]).value + self.drops[i]));
                                    }
                                    ui.end_row();
    
                                });
                            });
                            
                            
                        }
                        AbState::Fill => {

                        }
                    }
                });
            });
        });

        res
    }

}