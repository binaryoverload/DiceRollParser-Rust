use regex::Regex;
use lazy_static::lazy_static;
use crate::types::{Dice, Modifier, SelectorType, ModifierType, ValueOperator, OperatorType, DiceRoll};
use std::option::Option::Some;

pub(crate) fn parse(input: &str) -> Vec<DiceRoll> {
    lazy_static! {
        // Original pattern with comments: https://github.com/binaryoverload/DiceRollParser/blob/29485fdd78/src/main/java/uk/co/binaryoverload/dicerollparser/Parser.java#L25-L40
        static ref DICEROLL: Regex = Regex::new("(?:(?P<die>\\d+d\\d+)|\\((?P<dice>(?:\\d+d\\d+,?)+)\\))(?P<selectors>(?:(?:k|p|mi|ma|rr|ro|ra|e)(?:\\d+)?(?:[><lh]\\d+)?)+)?(?P<operators>(?:[+\\-*/]\\d+)+)?(?:\\[(?P<label>[^\\]]+)])?(?:(?P<roll_operator>[+\\-*/])|$)").unwrap();
    }
    let mut dice_rolls: Vec<DiceRoll> = vec![];
    for cap in DICEROLL.captures_iter(input) {
        println!("---- New capture! ----");

        let mut dice: Vec<Dice> = vec![];
        let mut modifiers: Vec<Modifier> = vec![];
        let mut value_operators: Vec<ValueOperator> = vec![];
        let mut label: Option<&str> = None;
        let mut joining_operator: Option<OperatorType> = None;

        if let Some(_die) = cap.name("die") {
            dice.push(parse_dice(_die.as_str()))
        } else if let Some(_dice) = cap.name("dice") {
            for die in _dice.as_str().split(",") {
                dice.push(parse_dice(die));
            }
        } else {
            panic!("There must be either 1 dice or multiple specified!");
        }

        if let Some(_modifiers) = cap.name("modifiers") {
            modifiers = parse_modifiers(_modifiers.as_str());
        }

        if let Some(_value_operators) = cap.name("operators") {
            value_operators = parse_operators(_value_operators.as_str());
        }

        if let Some(_label) = cap.name("label") {
            label = Some(_label.as_str());
        }

        if let Some(_joining_operator) = cap.name("joining_operator") {
            joining_operator = Some(OperatorType::from(_joining_operator.as_str()));
        }

        let dice_roll = DiceRoll {
            dice,
            modifiers,
            value_operators,
            label,
            joining_operator
        };
        println!("{:?}", dice_roll);
        dice_rolls.push(dice_roll);
    }
    dice_rolls
}

fn parse_dice(dice_input: &str) -> Dice {
    lazy_static! {
        static ref DICE: Regex = Regex::new("^(?P<count>\\d+)d(?P<sides>\\d+)$").unwrap();
    }
    if let Some(captures) = DICE.captures(dice_input) {
        let count = captures.name("count").unwrap().as_str();
        let sides = captures.name("sides").unwrap().as_str();
        return Dice {
            number_of_dice: count.parse().unwrap(),
            number_of_sides: sides.parse().unwrap()
        }
    } else {
        panic!("Dice input '{}' doesn't match dice pattern", dice_input)
    }
}

fn parse_modifiers(modifier_input: &str) -> Vec<Modifier> {
    lazy_static! {
        static ref MODIFIERS: Regex = Regex::new("(?P<type>k|p|mi|ma|rr|ro|ra|e)(?P<value>\\d+)?(?:(?P<selector>[><lh])(?P<selector_value>\\d+))?").unwrap();
    }
    let mut modifiers: Vec<Modifier> = vec![];
    for capture in MODIFIERS.captures_iter(modifier_input) {
        let modifier_type = ModifierType::from(capture.name("type").unwrap().as_str());
        let modifier_value: Option<u8> = capture.name("value").map_or(None, |value| { Some(value.as_str().parse().unwrap()) });
        let selector: Option<(SelectorType, u8)> = if let Some(selector) = capture.name("selector") {
            let selector_type = SelectorType::from(selector.as_str());
            if let Some(selector_value) = capture.name("selector_value") {
                Some((selector_type, selector_value.as_str().parse().unwrap()))
            } else {
                panic!("Selector value is missing from specified selector type")
            }
        } else {
            None
        };
        modifiers.push(Modifier {
            modifier_type,
            modifier_value,
            selector
        })
    }
    return modifiers;
}

fn parse_operators(operators_input: &str) -> Vec<ValueOperator> {
    lazy_static! {
        static ref OPERATORS: Regex = Regex::new("(?P<operator>[+\\-*/])(?P<value>\\d+)").unwrap();
    }
    let mut operators: Vec<ValueOperator> = vec![];
    for capture in OPERATORS.captures_iter(operators_input) {
        // There is no error checking here due to the regex checking which will ensure the correct outcome
        let operator: OperatorType = OperatorType::from(capture.name("operator").unwrap().as_str());
        let value: u8 = capture.name("value").unwrap().as_str().parse().expect("The number is too large to be parsed! Expected a number of size u8");
        operators.push(ValueOperator{ operator, value })
    }
    return operators;
}

fn group_to_name(group: u8) -> &'static str {
    match group {
        0 => "full match",
        1 => "die",
        2 => "dice",
        3 => "selectors",
        4 => "operators",
        5 => "label",
        6 => "roll operator",
        _ => panic!("Invalid group {}!", group)
    }
}