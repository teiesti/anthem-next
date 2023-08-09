use crate::{
    formatting::asp::default::Format,
    parsing::asp::pest::{
        AtomParser, AtomicFormulaParser, BinaryOperatorParser, BodyParser, ComparisonParser,
        HeadParser, LiteralParser, PrecomputedTermParser, ProgramParser, RelationParser,
        RuleParser, SignParser, TermParser, UnaryOperatorParser, VariableParser,
    },
    syntax_tree::{impl_node, Node},
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PrecomputedTerm {
    Infimum,
    Numeral(isize),
    Symbol(String),
    Supremum,
}

impl_node!(PrecomputedTerm, Format, PrecomputedTermParser);

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Variable(pub String);

impl_node!(Variable, Format, VariableParser);

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum UnaryOperator {
    Negative,
}

impl_node!(UnaryOperator, Format, UnaryOperatorParser);

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum BinaryOperator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    Interval,
}

impl_node!(BinaryOperator, Format, BinaryOperatorParser);

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Term {
    PrecomputedTerm(PrecomputedTerm),
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

impl_node!(Term, Format, TermParser);

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Atom {
    pub predicate: String,
    pub terms: Vec<Term>,
}

impl_node!(Atom, Format, AtomParser);

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Sign {
    NoSign,
    Negation,
    DoubleNegation,
}

impl_node!(Sign, Format, SignParser);

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Literal {
    pub sign: Sign,
    pub atom: Atom,
}

impl_node!(Literal, Format, LiteralParser);

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Relation {
    Equal,
    NotEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
}

impl_node!(Relation, Format, RelationParser);

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Comparison {
    pub relation: Relation,
    pub lhs: Term,
    pub rhs: Term,
}

impl_node!(Comparison, Format, ComparisonParser);

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum AtomicFormula {
    Literal(Literal),
    Comparison(Comparison),
}

impl_node!(AtomicFormula, Format, AtomicFormulaParser);

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Head {
    Basic(Atom),
    Choice(Atom),
    Falsity,
}

impl_node!(Head, Format, HeadParser);

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Body {
    pub formulas: Vec<AtomicFormula>,
}

impl_node!(Body, Format, BodyParser);

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Rule {
    pub head: Head,
    pub body: Body,
}

impl_node!(Rule, Format, RuleParser);

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Program {
    pub rules: Vec<Rule>,
}

impl_node!(Program, Format, ProgramParser);
