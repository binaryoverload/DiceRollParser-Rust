use modifier::Modifier;
use operator::OperatorType;

mod selector;
mod operator;
mod modifier;

#[derive(Debug)]
pub(crate) struct Dice {
    pub(crate) number_of_dice: u8,
    pub(crate) number_of_sides: u8
}

#[derive(Debug)]
pub(crate) enum Token {
    // Operator, Value
    Operator(OperatorType, u8),
    Modifier(Modifier),
    // Dice Count, Number of sides
    Dice(Dice)
}