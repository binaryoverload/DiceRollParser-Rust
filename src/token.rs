mod selector;
mod operator;
mod modifier;

pub(crate) enum Token {
    Selector(selector::SelectorType),
    Operator(operator::OperatorType),
    Modifier(modifier::ModifierType)
}