use std::ops::Add;
use num::{Num, ToPrimitive};
use rand::{thread_rng, Rng};
use rand::distributions::{Distribution, Standard};
use crate::check::{Check, Checkable, Checkables};
use crate::roll::Roll;

mod character;

pub struct TriangleDistribution {
    min: f64,
    max: f64,
    mode: f64,
}

impl TriangleDistribution {
    pub fn new(min: f64, max: f64, mode: f64) -> Self {
        assert!(min < max, "min must be less than max");
        assert!(
            min <= mode && mode <= max,
            "mode must be between min and max"
        );
        Self { min, max, mode }
    }
    
    pub fn mode(&self) -> f64 {
        self.mode.clone()
    }

    pub fn with_mode(&self, mode: f64) -> Self {
        Self {
            min: self.min.clone(),
            max: self.max.clone(),
            mode,
        }
    }
}

impl Roll for TriangleDistribution {
    type Output = f64;

    fn roll(&self) -> f64 {
        let mut rng = thread_rng();
        let u: f64 = rng.gen();
        let (min, max, mode) = (self.min.clone(), self.max.clone(), self.mode.clone());
        
        if u < (mode - min) / (max - min) {
            min +(u * (mode - min) * (max - min))
        } else {
            max - f64::sqrt((1.0 - u) * (max - mode) * (max - min))
        }
    }
}

