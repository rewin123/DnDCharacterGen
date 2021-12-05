
pub enum AbilityType {
    Strength,
    Dexterity,
    Constitution,
    Wisdom,
    Intelligence,
    Charisma
}

#[derive(Clone, Debug)]
pub struct Ability {
    pub value : i32
}

#[derive(Clone, Debug)]
pub struct AbilityBase {
    pub strength : Ability,
    pub dexterity : Ability,
    pub constitution : Ability,
    pub wisdom : Ability,
    pub intelligence: Ability,
    pub charisma : Ability
}

#[derive(Clone, Debug)]
pub enum CharacterClass {
    Barbarian,
    Bard,
    Cleric,
    Druid,
    Fighter,
    Monk,
    Paladin,
    Ranger,
    Rogue,
    Sorcerer,
    Warlok,
    Wizard,
    Undefined
}

#[derive(Clone, Debug)]
pub enum Race {
    Dwarf,
    Elf,
    Halfling,
    Human,
    Dragonborn,
    Gnome,
    HalfElf,
    HalfOrc,
    Tiefling,
    Undefined
}

#[derive(Clone, Debug)]
pub enum ExactRace {
    MountainDwarf,
    HillDwarf,
    HighElf,
    ForestElf,
    DarkElf,
    LightfootHalfling,
    StoutHalfling,
    Human,
    Dragonborn,
    ForestGnome,
    RockGnome,
    HalfElf,
    HalfOrc,
    Tiefling,
    Undefined,
}

#[derive(Clone, Debug)]
pub struct Character {
    pub abilities :  AbilityBase,
    pub class : CharacterClass,
    pub race : Race,
    pub exact_race : ExactRace
}

impl Ability {
    pub fn new(value : i32) -> Self {
        Self {
            value
        }
    }
}

impl AbilityBase {
    pub fn new() -> Self {
        Self {
            strength : Ability::new(0),
            dexterity : Ability::new(0),
            intelligence : Ability::new(0),
            constitution : Ability::new(0),
            charisma : Ability::new(0),
            wisdom : Ability::new(0)
        }
    }
}

impl Character {
    pub fn new() -> Self {
        Self {
            abilities : AbilityBase::new(), 
            class : CharacterClass::Undefined,
            race : Race::Undefined,
            exact_race : ExactRace::Undefined
        }
    }

    pub fn new_from_class(class : CharacterClass) -> Self {
        Self {
            abilities : AbilityBase::new(),
            class,
            race : Race::Undefined,
            exact_race : ExactRace::Undefined
        }
    }

    pub fn get_ability_buf(race : &ExactRace) -> AbilityBase {
        let mut base = AbilityBase::new();

        match race {
            ExactRace::MountainDwarf => {
                base.constitution.value = 2;
                base.strength.value = 2;
            }
            ExactRace::HillDwarf => {
                base.constitution.value = 2;
                base.wisdom.value = 1;
            }
            ExactRace::HighElf => {
                base.dexterity.value = 2;
                base.intelligence.value = 1;
            }
            ExactRace::ForestElf => {
                base.dexterity.value = 2;
                base.wisdom.value = 1;
            }
            ExactRace::DarkElf => {
                base.dexterity.value = 2;
                base.charisma.value = 1;
            }
            ExactRace::LightfootHalfling => {
                base.dexterity.value = 2;
                base.charisma.value = 1;
            }
            ExactRace::StoutHalfling => {
                base.dexterity.value = 2;
                base.constitution.value = 1;
            }
            ExactRace::Human => {
                base.charisma.value = 1;
                base.constitution.value = 1;
                base.dexterity.value = 1;
                base.intelligence.value = 1;
                base.strength.value = 1;
                base.wisdom.value = 1;
            }
            ExactRace::Dragonborn => {
                base.strength.value = 2;
                base.charisma.value = 1;
            }
            ExactRace::ForestGnome => {
                base.intelligence.value = 2;
                base.dexterity.value = 1;
            }
            ExactRace::RockGnome => {
                base.intelligence.value = 2;
                base.constitution.value = 1;
            }
            ExactRace::HalfElf => {
                base.charisma.value = 2;
            }
            ExactRace::HalfOrc => {
                base.strength.value = 2;
                base.constitution.value = 1;
            }
            ExactRace::Tiefling => {
                base.intelligence.value = 1;
                base.charisma.value = 2;
            }
            ExactRace::Undefined => {}
        }

        base
    }

    pub fn get_race_name(race : &ExactRace) -> String {

        match race {
            ExactRace::MountainDwarf => {
                String::from("Горный дварф")
            }
            ExactRace::HillDwarf => {
                String::from("Холмовой дварф")
            }
            ExactRace::HighElf => {
                String::from("Высокий эльф")
            }
            ExactRace::ForestElf => {
                String::from("Лесной эльф")
            }
            ExactRace::DarkElf => {
                String::from("Темный эльф (Дроу)")
            }
            ExactRace::LightfootHalfling => {
                String::from("Легконогий полурослик")
            }
            ExactRace::StoutHalfling => {
                String::from("Коренастый полурослик")
            }
            ExactRace::Human => {
                String::from("Человек")
            }
            ExactRace::Dragonborn => {
                String::from("Драконорожденный")
            }
            ExactRace::ForestGnome => {
                String::from("Лесной гном")
            }
            ExactRace::RockGnome => {
                String::from("Скальный гном")
            }
            ExactRace::HalfElf => {
                String::from("Полуэльф")
            }
            ExactRace::HalfOrc => {
                String::from("Полуорк")
            }
            ExactRace::Tiefling => {
                String::from("Тифлинг")
            }
            ExactRace::Undefined => {
                String::from("Неизвестная раса")
            }
        }
    }

    pub fn get_class_name(&self) -> String {
        let class = &self.class;
        let mut res = String::from("");
        match class {
            CharacterClass::Barbarian => {
                res = String::from("Варвар");
            }
            CharacterClass::Bard => {
                res = String::from("Бард");
            }
            CharacterClass::Cleric => {
                res = String::from("Жрец");
            }
            CharacterClass::Druid => {
                res = String::from("Друид");
            }
            CharacterClass::Fighter => {
                res = String::from("Воин");
            }
            CharacterClass::Monk => {
                res = String::from("Монах");
            }
            CharacterClass::Paladin => {
                res = String::from("Паладин");
            }
            CharacterClass::Ranger => {
                res = String::from("Рейнджер");
            }
            CharacterClass::Rogue => {
                res = String::from("Плут");
            }
            CharacterClass::Sorcerer => {
                res = String::from("Чародей");
            }
            CharacterClass::Warlok => {
                res = String::from("Колдун");
            }
            CharacterClass::Wizard => {
                res = String::from("Волшебник");
            }
            CharacterClass::Undefined => {
                res = String::from("Неизвестный класс");
            }
        }

        res
    }

}