#[derive(Debug)]
pub(crate) enum OperatorType {
    Add,
    Subtract,
    Multiply,
    Divide
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