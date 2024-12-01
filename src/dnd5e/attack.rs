use crate::check::result::DiceCheckResult;
use crate::check::{Check, Checkable};
use crate::dice::D20;
use crate::dnd5e::ability::Ability::{Dexterity, Strength};
use crate::dnd5e::ability::{Abilities, Ability};
use crate::dnd5e::character::DnD5ECharacter;
use crate::dnd5e::weapon::Weapon;
use crate::roll::Roll;

#[derive(Eq, Hash, PartialEq, Debug)]
pub enum Attack {
    Melee {
        weapon: Weapon,
    },
    Ranged {
        weapon: Weapon,
    },
}

impl Checkable for Attack {
    fn name(&self) -> String {
        format!("{:?}", self)
    }
}

impl Check<Attack, D20> for DnD5ECharacter {
    type Difficulty = i64;

    fn check(
        &self,
        checkable: &Attack,
        difficulty: Self::Difficulty,
        roll: D20,
    ) -> DiceCheckResult {
        let bonus;
        let ability;

        match checkable {
            Attack::Melee { weapon } => {
                bonus = weapon.bonus();
                ability = self.ability_value(&Strength)
            }
            Attack::Ranged { weapon } => {
                bonus = weapon.bonus();
                ability = self.ability_value(&Dexterity)
            }
        };

        let modifier = Ability::modifier(ability);
        let roll = roll.roll();
        let total = roll + modifier + bonus;

        DiceCheckResult::new(total >= difficulty, total, difficulty)
    }
}
