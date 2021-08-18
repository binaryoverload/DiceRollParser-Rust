use modifier::Modifier;
use operator::OperatorType;

#[derive(Debug)]
pub(crate) struct Dice {
    pub(crate) number_of_dice: u8,
    pub(crate) number_of_sides: u8
}

#[derive(Debug)]
pub(crate) struct ValueOperator {
    pub(crate) operator_type: OperatorType,
    pub(crate) value: u8
}

#[derive(Debug)]
pub(crate) enum SelectorType {
    GreaterThan,
    LessThan,
    Low,
    High
}

#[derive(Debug)]
pub(crate) enum OperatorType {
    Add,
    Subtract,
    Multiply,
    Divide
}

#[derive(Debug)]
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

#[derive(Debug)]
pub(crate) struct Modifier {
    pub(crate) modifier_type: ModifierType,
    pub(crate) modifier_value: Option<u8>,
    pub(crate) selector: Option<(SelectorType, u8)>
}

#[derive(Debug)]
pub(crate) struct DiceRoll<'a> {
    pub(crate) dice: Vec<Dice>,
    pub(crate) modifiers: Vec<Modifier>,
    pub(crate) value_operators: Vec<ValueOperator>,
    pub(crate) label: Option<&'a str>,
    pub(crate) joining_operator: Option<OperatorType>
}