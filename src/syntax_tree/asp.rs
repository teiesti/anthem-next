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

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Term {
    Constant(Constant),
    Variable(Variable),
    UnaryOperation {
        op: UnaryOperator,
        arg: Box<Term>,
    },
    BinaryOperation {
        op: BinaryOperator,
        lhs: Box<Term>,
        rhs: Box<Term>,
    },
}

impl Node for Term {}

// TODO Tobias: Continue implementing the abstract syntax tree for ASP here
