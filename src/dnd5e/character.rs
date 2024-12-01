use std::collections::HashMap;
use crate::check::{Check, Checkable};
use crate::check::result::DiceCheckResult;
use crate::dice::D20;
use crate::dnd5e::ability::{Abilities, Ability};
use crate::dnd5e::D20Check;
use crate::dnd5e::skill::{Skill, Skills};
use crate::roll::Roll;

pub struct DnD5ECharacter {
    attribute_values: HashMap<Ability, i64>,
    skill_values: HashMap<Skill, i64>,
}

impl DnD5ECharacter {
    pub fn new(attribute_values: HashMap<Ability, i64>, skill_values: HashMap<Skill, i64>) -> Self {
        Self {
            attribute_values,
            skill_values,
        }
    }
}

impl Abilities for DnD5ECharacter {
    fn ability_value(&self, attribute: &Ability) -> i64 {
        *self.attribute_values.get(attribute).unwrap_or(&0)
    }
}

impl Skills for DnD5ECharacter {
    fn skill_value(&self, skill: &Skill) -> i64 {
        *self.skill_values.get(skill).unwrap_or(&0)
    }
}

impl<T: Checkable> D20Check<T> for DnD5ECharacter
where Self: Check<T, D20, Difficulty = i64>
{
     fn d20_check(&self, checkable: &'static T, difficulty: i64) -> DiceCheckResult {
        <Self as Check<T, D20>>::check(&self, checkable, difficulty, D20)
    }
}