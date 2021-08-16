mod selector;
mod operator;
mod modifier;

#[derive(Debug)]
pub(crate) struct Dice {
    number_of_dice: u8,
    number_of_sides: u8
}

#[derive(Debug)]
pub(crate) enum Token {
    Selector(selector::SelectorType),
    // Operator, Value
    Operator(operator::OperatorType, u8),
    Modifier(modifier::ModifierType),
    // Dice Count, Number of sides
    Dice(Dice)
}