use crate::check::key::CheckableKey;
use crate::check::{Check, Checkable, Checkables};
use num::Num;
use std::collections::HashMap;
use crate::check::result::CheckResult;
use crate::dice::{Dice, D20};
use crate::probability_distribution::normal::NormalDistribution;
use crate::probability_distribution::triangle::TriangleDistribution;
use crate::roll::Roll;

pub struct Character<T: Num> {
    checkables: HashMap<CheckableKey, T>,
}

impl<T: Num> Character<T> {
    pub fn new() -> Self {
        Self {
            checkables: HashMap::<CheckableKey, T>::new()
        }
    }
}

impl<T: Num + Clone> Checkables for Character<T> {
    type Value = T;

    fn get_value(&self, checkable: &'static dyn Checkable) -> Self::Value {
        self.checkables.get(&CheckableKey::new(checkable)).cloned().unwrap_or_else(T::zero)
    }
}

impl <T: Checkable> Check<T, TriangleDistribution> for Character<f64> 
where {
    type Difficulty = f64;

    fn check(&self, checkable: &'static T, difficulty: Self::Difficulty, roll: TriangleDistribution) -> CheckResult<Self::Difficulty> {
        let value = self.get_value(checkable);
        let triangle = roll.with_mode(roll.mode() + value);
        let roll = triangle.roll();
        CheckResult::new(roll >= difficulty, roll, difficulty)
    }
}

impl <T: Checkable> Check<T, NormalDistribution> for Character<f64> {
    type Difficulty = f64;

    fn check(&self, checkable: &'static T, difficulty: Self::Difficulty, roll: NormalDistribution) -> CheckResult<Self::Difficulty> {
        let value = self.get_value(checkable);
        let mean = roll.mean();
        let normal = roll.with_mean(mean + value);
        let roll = normal.roll();
        CheckResult::new(roll >= difficulty, roll, difficulty)
    }
}

impl <T: Checkable> Check<T, Dice> for Character<i64> {
    type Difficulty = i64;

    fn check(&self, checkable: &'static T, difficulty: Self::Difficulty, roll: Dice) -> CheckResult<Self::Difficulty> {
        let value = self.get_value(checkable);
        let roll = roll.roll();
        CheckResult::new(roll >= difficulty, roll + value, difficulty)
    }
}