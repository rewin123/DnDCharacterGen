
pub struct Ability {
    value : i32
}

pub struct AbilityBase {
    strength : Ability,
    dexterity : Ability,
    constitution : Ability,
    wisdom : Ability,
    intelligence: Ability,
    charisma : Ability

}

pub struct Character {
    abilities : AbilityBase
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
            abilities : AbilityBase::new()
        }
    }

}