enum SelectorValue {
    GreaterThan,
    LessThan,
    Low,
    High
}

impl From<&str> for SelectorValue {
    fn from(string: &str) -> Self {
        if string.len() != 1 {
            panic!("Selector consists of 1 character only! {} were given", string.len())
        }
        match string.chars().nth(0) {
            None => {
                panic!("Selector consists of at least 1 character!")
            }
            Some(char) => {
                match char {
                    '>' => SelectorValue::GreaterThan,
                    '<' => SelectorValue::LessThan,
                    'l' => SelectorValue::Low,
                    'h' => SelectorValue::High,
                    _ => panic!("Invalid character for selector! Must be one of: < > l h")
                }
            }
        }
    }
}