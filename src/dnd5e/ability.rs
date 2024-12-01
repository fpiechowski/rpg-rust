use crate::check::{Check, Checkable};
use crate::check::result::{CheckResult, DiceCheckResult};
use crate::dice::D20;
use crate::dnd5e::character::DnD5ECharacter;
use crate::roll::Roll;

#[derive(Hash, Eq, PartialEq, Debug)]
pub enum Ability {
    Strength,
    Dexterity,
    Constitution,
    Wisdom,
    Intelligence,
    Charisma,
}

impl Ability {
    pub fn modifier(score: i64) -> i64 {
        (score - 10) / 2
    }
}

impl Checkable for Ability {
    fn name(&self) -> String {
        format!("{:?}", self)
    }
}

pub trait Abilities {
    fn ability_value(&self, attribute: &Ability) -> i64;
}

impl Check<Ability, D20> for DnD5ECharacter {
    type Difficulty = i64;

    fn check(
        &self,
        attribute: &Ability,
        difficulty: Self::Difficulty,
        roll: D20,
    ) -> DiceCheckResult {
        let value = self.ability_value(attribute);
        let modifier = Ability::modifier(value);
        let roll = roll.roll();
        let total = roll + modifier;

        CheckResult::new(total >= difficulty, total, difficulty)
    }
}
