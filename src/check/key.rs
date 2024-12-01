use std::hash::{Hash, Hasher};
use num::Num;
use crate::check::{Checkable};

pub struct CheckableKey {
    checkable: &'static dyn Checkable,
}

impl PartialEq<Self> for CheckableKey {
    fn eq(&self, other: &Self) -> bool {
        self.checkable.name() == other.checkable.name()
    }
}

impl Eq for CheckableKey {
}

impl Hash for CheckableKey {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.checkable.name().hash(state)
    }
}

impl CheckableKey {
    pub fn new(value: &'static dyn Checkable) -> Self {
        Self {
            checkable: value
        }
    }
}

pub struct CheckableValue<T: Num> {
    value: T
}

impl <T: Num> Default for CheckableValue<T> {
    fn default() -> Self {
        Self { value: T::zero() }
    }
}