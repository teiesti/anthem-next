use super::Node;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum BasicIntegerTerm {
    Infimum,
    Supremum,
    Numeral(isize),
    IntegerVariable(String),
}

impl Node for BasicIntegerTerm {}

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

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Atom {
    pub predicate: String,
    pub terms: Vec<GeneralTerm>,
}

impl Node for Atom {}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Relation {
    Equal,
    NotEqual,
    Greater,
    Less,
    GreaterEqual,
    LessEqual,
}

impl Node for Relation {}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Guard {
    pub relation: Relation,
    pub term: GeneralTerm,
}

impl Node for Guard {}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Comparison {
    pub term: GeneralTerm,
    pub guards: Vec<Guard>,
}

impl Node for Comparison {}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum AtomicFormula {
    Falsity,
    Atom(Atom),
    Comparison(Comparison),
}

impl Node for AtomicFormula {}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum UnaryConnective {
    Negation,
}

impl Node for UnaryConnective {}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Quantifier {
    Forall,
    Exists,
}

impl Node for Quantifier {}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Quantification {
    pub quantifier: Quantifier,
    pub variables: Vec<Variable>,
}

impl Node for Quantification {}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Sort {
    Integer,
    General,
}

impl Node for Sort {}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Variable {
    pub name: String,
    pub sort: Sort,
}

impl Node for Variable {}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum BinaryConnective {
    Conjunction,
    Disjunction,
    Implication,
    ReverseImplication,
    Equivalence,
}

impl Node for BinaryConnective {}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Formula {
    AtomicFormula(AtomicFormula),
    UnaryFormula {
        connective: UnaryConnective,
        formula: Box<Formula>,
    },
    BinaryFormula {
        connective: BinaryConnective,
        lhs: Box<Formula>,
        rhs: Box<Formula>,
    },
    QuantifiedFormula {
        quantification: Quantification,
        formula: Box<Formula>,
    },
}

impl Node for Formula {}
