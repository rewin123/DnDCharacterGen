
#[derive(Clone, Debug)]
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

    pub fn get_by_type(&self, tp : &AbilityType) -> &Ability {
        match tp {
            AbilityType::Charisma => {
                &self.charisma
            }
            AbilityType::Constitution => {
                &self.constitution
            }
            AbilityType::Dexterity => {
                &self.dexterity
            }
            AbilityType::Intelligence => {
                &self.intelligence
            }
            AbilityType::Strength => {
                &self.strength
            }
            AbilityType::Wisdom => {
                &self.wisdom
            }
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
                String::from("???????????? ??????????")
            }
            ExactRace::HillDwarf => {
                String::from("???????????????? ??????????")
            }
            ExactRace::HighElf => {
                String::from("?????????????? ????????")
            }
            ExactRace::ForestElf => {
                String::from("???????????? ????????")
            }
            ExactRace::DarkElf => {
                String::from("???????????? ???????? (????????)")
            }
            ExactRace::LightfootHalfling => {
                String::from("???????????????????? ????????????????????")
            }
            ExactRace::StoutHalfling => {
                String::from("???????????????????? ????????????????????")
            }
            ExactRace::Human => {
                String::from("??????????????")
            }
            ExactRace::Dragonborn => {
                String::from("????????????????????????????????")
            }
            ExactRace::ForestGnome => {
                String::from("???????????? ????????")
            }
            ExactRace::RockGnome => {
                String::from("???????????????? ????????")
            }
            ExactRace::HalfElf => {
                String::from("????????????????")
            }
            ExactRace::HalfOrc => {
                String::from("??????????????")
            }
            ExactRace::Tiefling => {
                String::from("??????????????")
            }
            ExactRace::Undefined => {
                String::from("?????????????????????? ????????")
            }
        }
    }

    pub fn get_class_name(&self) -> String {
        let class = &self.class;
        let mut res = String::from("");
        match class {
            CharacterClass::Barbarian => {
                res = String::from("????????????");
            }
            CharacterClass::Bard => {
                res = String::from("????????");
            }
            CharacterClass::Cleric => {
                res = String::from("????????");
            }
            CharacterClass::Druid => {
                res = String::from("??????????");
            }
            CharacterClass::Fighter => {
                res = String::from("????????");
            }
            CharacterClass::Monk => {
                res = String::from("??????????");
            }
            CharacterClass::Paladin => {
                res = String::from("??????????????");
            }
            CharacterClass::Ranger => {
                res = String::from("????????????????");
            }
            CharacterClass::Rogue => {
                res = String::from("????????");
            }
            CharacterClass::Sorcerer => {
                res = String::from("??????????????");
            }
            CharacterClass::Warlok => {
                res = String::from("????????????");
            }
            CharacterClass::Wizard => {
                res = String::from("??????????????????");
            }
            CharacterClass::Undefined => {
                res = String::from("?????????????????????? ??????????");
            }
        }

        res
    }

}