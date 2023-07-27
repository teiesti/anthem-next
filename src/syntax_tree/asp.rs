use super::Node;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Constant {
    Infimum,
    Integer(isize),
    Symbol(String),
    Supremum,
}

impl Node for Constant {}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Variable {
    Anonymous,
    Named(String),
}

impl Node for Variable {}

// TODO Tobias: Continue implementing the abstract syntax tree for ASP here
