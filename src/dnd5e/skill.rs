use crate::check::result::{CheckResult, DiceCheckResult};
use crate::check::{Check, Checkable};
use crate::dice::D20;
use crate::dnd5e::character::DnD5ECharacter;
use crate::roll::Roll;

#[derive(Hash, Eq, PartialEq, Debug)]
pub enum Skill {
    Acrobatics,
    AnimalHandling,
    Arcana,
    Athletics,
    Deception,
    History,
    Insight,
    Intimidation,
    Investigation,
    Medicine,
    Nature,
    Perception,
    Performance,
    Persuasion,
    Religion,
    SleightOfHand,
    Stealth,
    Survival,
}

impl Checkable for Skill {
    fn name(&self) -> String {
        format!("{:?}", self)
    }
}

pub trait Skills {
    fn skill_value(&self, skill: &Skill) -> i64;
}

impl Check<Skill, D20> for DnD5ECharacter {
    type Difficulty = i64;

    fn check(
        &self,
        skill: &Skill,
        difficulty: Self::Difficulty,
        roll: D20,
    ) -> DiceCheckResult {
        let value = self.skill_value(skill);
        let roll = roll.roll();
        let total = roll + value;

        CheckResult::new(total >= difficulty, total, difficulty)
    }
}