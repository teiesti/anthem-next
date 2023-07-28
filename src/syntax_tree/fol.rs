use super::Node;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UnaryOperator {
    UnaryMinus: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum BinaryOperator {
    Add,
    Subtract,
    Multiply,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum BasicIntegerTerm {
    Infimum,
    Supremum,
    Numeral(isize),
    IntegerVariable(String),
}

impl Node for BasicIntegerTerm {}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum IntegerTerm {
    BinaryOperation {
        op: BinaryOperator,
        lhs: Box<IntegerTerm>,
        rhs: Box<IntegerTerm>,
    },
    UnaryOperation {
        op: UnaryOperator,
        arg: Box<IntegerTerm>,
    },
    BasicIntegerTerm,
}

impl Node for IntegerTerm {}

// TODO Zach: Continue implementing the abstract syntax tree for first-order logic here
