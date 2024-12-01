use crate::roll::Roll;
use rand::Rng;

pub struct Dice {
    sides: u8,
    count: u8,
}

impl Dice {
    pub fn new(sides: u8, count: u8) -> Dice {
        Dice { sides, count }
    }

    pub const D20: Dice = Dice {
        sides: 20,
        count: 1,
    };
}

impl Roll for Dice {
    type Output = i64;

    fn roll(&self) -> Self::Output {
        let mut rng = rand::thread_rng();
        (0..self.count)
            .map(|_| rng.gen_range(1..=self.sides) as Self::Output)
            .sum()
    }
}

pub struct D20;

pub trait DiceType {
    fn dice() -> Dice;
}

impl DiceType for D20 {
    fn dice() -> Dice {
        Dice::D20
    }
}

impl Roll for D20 {
    type Output = i64;

    fn roll(&self) -> Self::Output {
        D20::dice().roll()
    }
}