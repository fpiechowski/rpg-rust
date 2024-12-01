use std::ops::Add;
use num::Num;

pub struct CheckResult<T: Num> {
    success: bool,
    total: T,
    difficulty: T,
}

impl<T: Num> CheckResult<T> {
    pub fn new(success: bool, total: T, difficulty: T) -> Self {
        Self {
            success,
            total,
            difficulty,
        }
    }

    pub fn succeeded(&self) -> bool {
        self.success
    }
}

pub type DiceCheckResult = CheckResult<i64>;