use crate::character::*;
use crate::screen::*;
use eframe::egui;
use eframe::egui::{CtxRef};
use eframe::epi::Frame;


pub struct SelectRaceDesc {}

struct RaceCost {
    pub race : ExactRace,
    pub cost : i32
}

pub struct RaceSelectScreen {
    character : Character,
    costs : Vec<RaceCost>
}

impl RaceCost {
    pub fn new() -> Self {
        Self {
            race : ExactRace::Undefined,
            cost : 0_i32
        }
    }

    pub fn make_cost(
        race : &ExactRace,
        cost : &i32
    ) -> Self {
        Self {
            race : race.clone(),
            cost : cost.clone()
        }
    }
}

impl RaceSelectScreen {
    pub fn new(character : &Character) -> Self {

        let races = SelectRaceDesc::get_all_race();
        let important = SelectRaceDesc::get_important_abilities(&character.class);

        let mut costs = Vec::<RaceCost>::new();
        for race in races {
            let mut cost = 0_i32;
            let race_base = Character::get_ability_buf(&race);
            for imp in &important {
                match imp {
                    AbilityType::Charisma => {
                        cost += race_base.charisma.value;
                    }
                    AbilityType::Wisdom => {
                        cost += race_base.wisdom.value;
                    }
                    AbilityType::Intelligence => {
                        cost += race_base.intelligence.value;
                    }
                    AbilityType::Strength => {
                        cost += race_base.strength.value;
                    }
                    AbilityType::Dexterity => {
                        cost += race_base.dexterity.value;
                    }
                    AbilityType::Constitution => {
                        cost += race_base.constitution.value;
                    }
                }
            }

            costs.push(RaceCost::make_cost(&race, &cost));
        }

        Self {
            character : character.clone(),
            costs
        }
    }
}

impl Screen for RaceSelectScreen {
    
    fn update(&mut self, ctx: &CtxRef, frame: &mut Frame<'_>) -> ScreenResult {

        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.vertical_centered(|ui| {
                    ui.label("Пришло время решить судьбу твоего появления в мире. Выбери частью какого народа ты будешь являться:");
                    ui.separator();
                    ui.separator();

                    for race_cost in &self.costs {
                        ui.button(format!("{:?}", race_cost.race));
                    }
                });
            });
        });

        ScreenResult::Ok
    }
}

impl SelectRaceDesc {

    pub fn get_all_race() -> Vec<ExactRace> {
        vec![
            ExactRace::DarkElf,
            ExactRace::Dragonborn,
            ExactRace::ForestElf,
            ExactRace::ForestGnome,
            ExactRace::HalfElf,
            ExactRace::HalfOrc,
            ExactRace::HighElf,
            ExactRace::HillDwarf,
            ExactRace::Human,
            ExactRace::LightfootHalfling,
            ExactRace::MountainDwarf,
            ExactRace::RockGnome,
            ExactRace::StoutHalfling,
            ExactRace::Tiefling,
        ]
    }

    pub fn get_important_abilities(class : &CharacterClass) -> Vec<AbilityType> {
        let mut res = Vec::<AbilityType>::new();

        match class {
            CharacterClass::Barbarian => {
                res.push(AbilityType::Strength);
                res.push(AbilityType::Constitution);
            }
            CharacterClass::Bard => {
                res.push(AbilityType::Charisma);
            }
            CharacterClass::Cleric => {
                res.push(AbilityType::Wisdom);
            }
            CharacterClass::Druid => {
                res.push(AbilityType::Wisdom);
            }
            CharacterClass::Fighter => {
                res.push(AbilityType::Strength);
                res.push(AbilityType::Dexterity);
                res.push(AbilityType::Constitution);
            }
            CharacterClass::Monk => {
                res.push(AbilityType::Dexterity);
                res.push(AbilityType::Wisdom);
            }
            CharacterClass::Paladin => {
                res.push(AbilityType::Strength);
                res.push(AbilityType::Charisma);
                res.push(AbilityType::Constitution);
            }
            CharacterClass::Ranger => {
                res.push(AbilityType::Dexterity);
                res.push(AbilityType::Wisdom);
            }
            CharacterClass::Rogue => {
                res.push(AbilityType::Dexterity);
            }
            CharacterClass::Sorcerer => {
                res.push(AbilityType::Charisma);
            }
            CharacterClass::Warlok => {
                res.push(AbilityType::Charisma);
            }
            CharacterClass::Wizard => {
                res.push(AbilityType::Intelligence);
            }
            CharacterClass::Undefined => {
            }
        }

        res
    }
}