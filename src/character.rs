
#[derive(Clone, Debug)]
pub struct Ability {
    value : i32
}

#[derive(Clone, Debug)]
pub struct AbilityBase {
    strength : Ability,
    dexterity : Ability,
    constitution : Ability,
    wisdom : Ability,
    intelligence: Ability,
    charisma : Ability
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
pub struct Character {
    pub abilities :  AbilityBase,
    pub class : CharacterClass,
    pub race : Race,
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
        }
    }

    pub fn new_from_class(class : CharacterClass) -> Self {
        Self {
            abilities : AbilityBase::new(),
            class,
            race : Race::Undefined,
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