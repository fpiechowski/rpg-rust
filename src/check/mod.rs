use std::fmt::Debug;
use std::hash::{Hash, Hasher};
use std::ops::Add;
use num::Num;
use result::CheckResult;
use crate::roll::Roll;

pub mod key;
pub mod result;

pub trait Checkable : Debug {
    fn name(&self) -> String;
}

pub trait Checkables {
    type Value;

    fn get_value(&self, checkable: &'static dyn Checkable) -> Self::Value;
}

pub trait Check<T: Checkable, R: Roll<Output=Self::Difficulty>> {
    type Difficulty: Num;

    fn check(&self, checkable: &'static T, difficulty: Self::Difficulty, roll: R) -> CheckResult<Self::Difficulty>;
}
