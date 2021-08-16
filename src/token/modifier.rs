pub(crate) enum ModifierType {
    Reroll,
    RerollOne,
    RerollAdd,
    Minimum,
    Maximum,
    Explode,
    Keep,
    Drop
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