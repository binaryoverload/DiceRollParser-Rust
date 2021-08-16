use regex::Regex;
use lazy_static::lazy_static;
use crate::token::{Token, Dice};
use crate::token::modifier::{ModifierType, Modifier};
use std::option::Option::Some;
use crate::token::selector::SelectorType;
use crate::token::Token::{Label, ValueOperator, JoiningOperator};
use crate::token::operator::OperatorType;

pub(crate) fn parse(input: &str) -> Vec<Token> {
    lazy_static! {
        static ref DICEROLL: Regex = Regex::new("(?:(?P<die>\\d+d\\d+)|\\((?P<dice>(?:\\d+d\\d+,?)+)\\))(?P<selectors>(?:(?:k|p|mi|ma|rr|ro|ra|e)(?:\\d+)?(?:[><lh]\\d+)?)+)?(?P<operators>(?:[+\\-*/]\\d+)+)?(?:\\[(?P<label>[^\\]]+)])?(?:(?P<roll_operator>[+\\-*/])|$)").unwrap();
    }
    for cap in DICEROLL.captures_iter(input) {
        println!("---- New capture! ----");
        let mut group: u8 = 0;
        let mut tokens: Vec<Token> = vec![];
        for subcap in cap.iter() {
            if let Some(mat) = subcap {
                match group {
                    1 => tokens.push(parse_dice(mat.as_str())),
                    2 => {
                        for die in mat.as_str().split(",") {
                            tokens.push(parse_dice(die))
                        }
                    },
                    3 => tokens.append(&mut parse_modifiers(mat.as_str())),
                    5 => tokens.push(Label(mat.as_str())),
                    6 => tokens.push(JoiningOperator(OperatorType::from(mat.as_str()))),
                    _ => {}
                }
                println!("Group {} [{}] ({}-{}): '{}'", group, group_to_name(group), mat.start(), mat.end(), mat.as_str())
            } else {
                println!("Group {} [{}]: None", group, group_to_name(group))
            }
            group += 1;
        }
        println!("{:?}", tokens);
        // println!("Dice: {:?}, Selectors: {:?}, Operators: {:?}, Label: {:?}, Roll Operator {:?}",
        //          cap.name("die").unwrap_or(cap.name("dice").expect("Dice and die are not present!")), cap.name("selectors"), cap.name("operators"), cap.name("label"), cap.name("roll_operator"));
    }
    vec![]
}

fn parse_dice(dice_input: &str) -> Token {
    lazy_static! {
        static ref DICE: Regex = Regex::new("^(?P<count>\\d+)d(?P<sides>\\d+)$").unwrap();
    }
    if let Some(captures) = DICE.captures(dice_input) {
        let count = captures.name("count").unwrap().as_str();
        let sides = captures.name("sides").unwrap().as_str();
        return Token::Dice(Dice {
            number_of_dice: count.parse().unwrap(),
            number_of_sides: sides.parse().unwrap()
        })
    } else {
        panic!("Dice input '{}' doesn't match dice pattern", dice_input)
    }
}

fn parse_modifiers(modifier_input: &str) -> Vec<Token> {
    lazy_static! {
        static ref MODIFIERS: Regex = Regex::new("(?P<type>k|p|mi|ma|rr|ro|ra|e)(?P<value>\\d+)?(?:(?P<selector>[><lh])(?P<selector_value>\\d+))?").unwrap();
    }
    let mut tokens: Vec<Token> = vec![];
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
        tokens.push(Token::Modifier(Modifier {
            modifier_type,
            modifier_value,
            selector
        }))
    }
    return tokens;
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