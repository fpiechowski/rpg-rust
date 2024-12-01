use rand_distr::Normal;
use rand::thread_rng;
use rand::distributions::Distribution;
use crate::roll::Roll;

pub struct NormalDistribution {
    mean: f64,
    std_dev: f64,
}

impl NormalDistribution {
    pub fn new(mean: f64, std_dev: f64) -> Self {
        Self { mean, std_dev }
    }
    
    pub fn mean(&self) -> f64 {
        self.mean.clone()
    }
    
    pub fn with_mean(self, mean: f64) -> Self {
        Self { mean, std_dev: self.std_dev }
    }
}

impl Roll for NormalDistribution {
    type Output = f64;

    fn roll(&self) -> f64 {
        let normal = Normal::new(self.mean, self.std_dev).unwrap();
        normal.sample(&mut thread_rng())
    }
}