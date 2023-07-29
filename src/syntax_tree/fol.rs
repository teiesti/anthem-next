use super::Node;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum UnaryOperator {
    Negative,
}

impl Node for UnaryOperator {}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum BinaryOperator {
    Add,
    Subtract,
    Multiply,
}

impl Node for BinaryOperator {}

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
    BasicIntegerTerm(BasicIntegerTerm),
    BinaryOperation {
        op: BinaryOperator,
        lhs: Box<IntegerTerm>,
        rhs: Box<IntegerTerm>,
    },
    UnaryOperation {
        op: UnaryOperator,
        arg: Box<IntegerTerm>,
    },
}

impl Node for IntegerTerm {}

// TODO Zach: Continue implementing the abstract syntax tree for first-order logic here
