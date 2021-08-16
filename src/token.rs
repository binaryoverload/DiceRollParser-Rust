use modifier::Modifier;
use operator::OperatorType;

pub(crate) mod selector;
pub(crate) mod operator;
pub(crate) mod modifier;

#[derive(Debug)]
pub(crate) struct Dice {
    pub(crate) number_of_dice: u8,
    pub(crate) number_of_sides: u8
}

#[derive(Debug)]
pub(crate) enum Token<'a> {
    // The operator that comes at the end of the dice roll (between dice rolls)
    JoiningOperator(OperatorType),
    // Operator, Value
    ValueOperator(OperatorType, u8),
    Modifier(Modifier),
    // Dice Count, Number of sides
    Dice(Dice),
    Label(&'a str)
}