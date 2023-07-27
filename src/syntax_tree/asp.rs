use super::Node;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Constant {
    Infimum,
    Integer(isize),
    Symbol(String),
    Supremum,
}

impl Node for Constant {}

// TODO Tobias: Continue implementing the abstract syntax tree for ASP here
