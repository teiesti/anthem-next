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

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Atom {
    pub predicate: String,
    pub terms: Vec<Term>,
}

impl Node for Atom {}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Sign {
    NoSign,
    Negation,
    DoubleNegation,
}

impl Node for Sign {}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Literal {
    pub sign: Sign,
    pub atom: Atom,
}

impl Node for Literal {}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Relation {
    Equal,
    NotEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
}

impl Node for Relation {}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Comparison {
    pub relation: Relation,
    pub lhs: Term,
    pub rhs: Term,
}

impl Node for Comparison {}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum AtomicFormula {
    Literal(Literal),
    Comparison(Comparison),
}

impl Node for AtomicFormula {}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Head {
    Basic(Atom),
    Choice(Atom),
    Constrait,
}

impl Node for Head {}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Body {
    pub formulas: Vec<AtomicFormula>,
}

impl Node for Body {}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Rule {
    pub head: Head,
    pub body: Body,
}

impl Node for Rule {}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Program {
    pub rules: Vec<Rule>,
}

impl Node for Program {}
