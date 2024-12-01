use crate::check::Checkable;
use crate::check::result::DiceCheckResult;

pub(crate) mod ability;
pub(crate) mod skill;
pub(crate) mod character;
pub(crate) mod attack;
pub(crate) mod weapon;

pub trait D20Check<T: Checkable> {
    fn d20_check(&self, checkable: &'static T, difficulty: i64) -> DiceCheckResult;
}
