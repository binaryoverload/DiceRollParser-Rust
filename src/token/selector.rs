#[derive(Debug)]
pub(crate) enum SelectorType {
    GreaterThan,
    LessThan,
    Low,
    High
}

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