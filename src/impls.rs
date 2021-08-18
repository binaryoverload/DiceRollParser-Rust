use crate::types::{SelectorType, OperatorType, ModifierType, Dice, DiceRoll};
use rand::distributions::{Distribution, Uniform};

impl From<&str> for SelectorType {
    fn from(string: &str) -> Self {
        if string.len() != 1 {
            panic!("Selector consists of 1 character only! {} were given", string.len())
        }
        match string {
            ">" => SelectorType::GreaterThan,
            "<" => SelectorType::LessThan,
            "l" => SelectorType::Low,
            "h" => SelectorType::High,
            _ => panic!("Invalid character for selector! Must be one of: < > l h")
        }
    }
}

impl From<&str> for OperatorType {
    fn from(string: &str) -> Self {
        if string.len() != 1 {
            panic!("Operator consists of 1 character only! {} were given", string.len())
        }
        match string {
            "+" => OperatorType::Add,
            "-" => OperatorType::Subtract,
            "*" => OperatorType::Multiply,
            "/" => OperatorType::Divide,
            _ => panic!("Invalid character for operator! Must be one of: + - * /")
        }
    }
}


impl From<&str> for ModifierType {
    fn from(string: &str) -> Self {
        match string {
            "rr" => ModifierType::Reroll,
            "ro" => ModifierType::RerollOne,
            "ra" => ModifierType::RerollAdd,
            "mi" => ModifierType::Minimum,
            "ma" => ModifierType::Maximum,
            "e" => ModifierType::Explode,
            "k" => ModifierType::Keep,
            "d" => ModifierType::Drop,
            _ => panic!("Invalid modifier type! Must be one of: rr ro ra mi ma e k d")
        }
    }
}

impl DiceRoll<'_> {

    fn calculate_dice(dice: Dice) -> u16 {
        let mut rng = rand::thread_rng();
        let mut result: u16 = 0;
        let uniform = Uniform::new(1u16, (dice.number_of_sides + 1) as u16);
        for _ in 0..(dice.number_of_dice) {
            result += uniform.sample(&mut rng);
        }
        result
    }

}