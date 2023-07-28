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

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum UnaryOperator {
    Negative,
}

impl Node for UnaryOperator {}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum BinaryOperator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    Interval,
}

impl Node for BinaryOperator {}

// TODO Tobias: Continue implementing the abstract syntax tree for ASP here
