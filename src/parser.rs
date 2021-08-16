use regex::Regex;
use lazy_static::lazy_static;
use crate::token::{Token, Dice};
use std::option::Option::Some;
use std::borrow::Borrow;

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
                    }
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

fn parse_modifiers(modifier_input: &str) -> Token {
    lazy_static! {
        static ref MODIFIERS: Regex = Regex::new("(k|p|mi|ma|rr|ro|ra|e)(\\d+)?(?:([><lh])(\\d+))?")
    }
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