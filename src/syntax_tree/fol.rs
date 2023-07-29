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
    UnaryOperation {
        op: UnaryOperator,
        arg: Box<IntegerTerm>,
    },
    BinaryOperation {
        op: BinaryOperator,
        lhs: Box<IntegerTerm>,
        rhs: Box<IntegerTerm>,
    },
}

impl Node for IntegerTerm {}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum GeneralTerm {
    Symbol(String),
    GeneralVariable(String),
    IntegerTerm(IntegerTerm),
}

impl Node for GeneralTerm {}

// TODO Zach: Continue implementing the abstract syntax tree for first-order logic here
