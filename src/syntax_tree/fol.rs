use crate::{
    formatting::fol::default::Format,
    parsing::fol::pest::{
        AtomParser, AtomicFormulaParser, BasicIntegerTermParser, BinaryConnectiveParser,
        BinaryOperatorParser, ComparisonParser, FormulaParser, GeneralTermParser, GuardParser,
        IntegerTermParser, QuantificationParser, QuantifierParser, RelationParser, TheoryParser,
        UnaryConnectiveParser, UnaryOperatorParser, VariableParser,
    },
    syntax_tree::{impl_node, Node},
};

use std::cmp::Ordering;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum BasicIntegerTerm {
    Infimum,
    Supremum,
    Numeral(isize),
    IntegerVariable(String),
}

impl_node!(BasicIntegerTerm, Format, BasicIntegerTermParser);

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum UnaryOperator {
    Negative,
}

impl_node!(UnaryOperator, Format, UnaryOperatorParser);

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum BinaryOperator {
    Add,
    Subtract,
    Multiply,
}

impl_node!(BinaryOperator, Format, BinaryOperatorParser);

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

impl_node!(IntegerTerm, Format, IntegerTermParser);

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum GeneralTerm {
    Symbol(String),
    GeneralVariable(String),
    IntegerTerm(IntegerTerm),
}

impl_node!(GeneralTerm, Format, GeneralTermParser);

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Atom {
    pub predicate: String,
    pub terms: Vec<GeneralTerm>,
}

impl_node!(Atom, Format, AtomParser);

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Relation {
    Equal,
    NotEqual,
    Greater,
    Less,
    GreaterEqual,
    LessEqual,
}

impl_node!(Relation, Format, RelationParser);

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Guard {
    pub relation: Relation,
    pub term: GeneralTerm,
}

impl_node!(Guard, Format, GuardParser);

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Comparison {
    pub term: GeneralTerm,
    pub guards: Vec<Guard>,
}

impl_node!(Comparison, Format, ComparisonParser);

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum AtomicFormula {
    Falsity,
    Atom(Atom),
    Comparison(Comparison),
}

impl_node!(AtomicFormula, Format, AtomicFormulaParser);

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum UnaryConnective {
    Negation,
}

impl_node!(UnaryConnective, Format, UnaryConnectiveParser);

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Quantifier {
    Forall,
    Exists,
}

impl_node!(Quantifier, Format, QuantifierParser);

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Quantification {
    pub quantifier: Quantifier,
    pub variables: Vec<Variable>,
}

impl_node!(Quantification, Format, QuantificationParser);

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Sort {
    Integer,
    General,
}

// TODO: Should Sort be a Node?

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Variable {
    pub name: String,
    pub sort: Sort,
}

impl_node!(Variable, Format, VariableParser);

impl Ord for Variable {
    fn cmp(&self, other: &Self) -> Ordering {
        (&self.name).cmp(&other.name)
    }
}

impl PartialOrd for Variable {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let size1 = self.name.clone();
        let size2 = other.name.clone();
        if size1 < size2 {
            Some(Ordering::Less)
        } else if size1 > size2 {
            Some(Ordering::Greater)
        } else {
            Some(Ordering::Equal)
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum BinaryConnective {
    Conjunction,
    Disjunction,
    Implication,
    ReverseImplication,
    Equivalence,
}

impl_node!(BinaryConnective, Format, BinaryConnectiveParser);

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

impl_node!(Formula, Format, FormulaParser);

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Theory {
    pub formulas: Vec<Formula>,
}

impl_node!(Theory, Format, TheoryParser);
