#[derive(Eq, Hash, PartialEq, Debug)]
pub struct Weapon {
    bonus: i64,
}

impl Weapon {
    pub fn bonus(&self) -> i64 {
        self.bonus
    }
}
