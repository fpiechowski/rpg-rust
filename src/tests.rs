#[cfg(test)]
mod tests {
    use crate::character::Character;
    use crate::check::Check;
    use crate::dnd5e::ability::Ability::{Dexterity, Strength};
    use crate::dnd5e::character::DnD5ECharacter;
    use crate::dnd5e::D20Check;
    use crate::dnd5e::skill::Skill::Acrobatics;
    use crate::probability_distribution::triangle::TriangleDistribution;

    #[test]
    fn test_d20_check() {
        let character = DnD5ECharacter::new([(Strength, 0)].into(), [(Acrobatics, 0)].into());
        let result = character.d20_check(&Dexterity, 10);
    }

    #[test]
    fn test_probability_distribution_check() {
        let character = Character::<f64>::new();
        let result = character.check(&Dexterity, 60.0, TriangleDistribution::new(0.0, 100.0, 50.0));
    }
}