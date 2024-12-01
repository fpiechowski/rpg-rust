use std::collections::HashMap;
use crate::check::{Check, Checkable, Checkables};
use crate::check::key::CheckableKey;
use crate::check::result::CheckResult;
use crate::probability_distribution::triangle::TriangleDistribution;
use crate::roll::Roll;

pub struct TriangleProbabilityDistributionCharacter {
    checkables: HashMap<CheckableKey, f64>,
}

impl TriangleProbabilityDistributionCharacter {
    pub fn new() -> Self {
        Self { checkables: HashMap::new() }
    }
}

impl Checkables for TriangleProbabilityDistributionCharacter {
    type Value = f64;

    fn get_value(&self, checkable: &'static dyn Checkable) -> Self::Value {
        *self.checkables.get(&CheckableKey::new(checkable)).unwrap_or(&0.0)
    }
}

impl<T: Checkable> Check<T, TriangleDistribution> for TriangleProbabilityDistributionCharacter
where
    Self: Checkables,
{
    type Difficulty = f64;

    fn check(&self, checkable: &'static T, difficulty: Self::Difficulty, roll: TriangleDistribution) -> CheckResult<Self::Difficulty> {
        let value = *self.checkables.get(&CheckableKey::new(checkable)).unwrap_or(&0.0);
        let roll = roll.with_mode(roll.mode() + value).roll();

        CheckResult::new(
            roll < difficulty,
            roll,
            difficulty,
        )
    }
}