use std::hash::Hash;

use {
    crate::{
        formatting::asp::default::Format,
        parsing::asp::pest::{
            AtomParser, AtomicFormulaParser, BinaryOperatorParser, BodyParser, ComparisonParser,
            HeadParser, LiteralParser, PrecomputedTermParser, ProgramParser, RelationParser,
            RuleParser, SignParser, TermParser, UnaryOperatorParser, VariableParser,
        },
        syntax_tree::{impl_node, Node},
    },
    std::collections::HashSet,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PrecomputedTerm {
    Infimum,
    Numeral(isize),
    Symbol(String),
    Supremum,
}

impl_node!(PrecomputedTerm, Format, PrecomputedTermParser);

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
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

impl Term {
    pub fn variables(&self) -> HashSet<Variable> {
        match &self {
            Term::PrecomputedTerm(_) => HashSet::new(),
            Term::Variable(v) => HashSet::from([v.clone()]),
            Term::UnaryOperation { arg, .. } => arg.variables(),
            Term::BinaryOperation { lhs, rhs, .. } => {
                let mut vars = lhs.variables();
                vars.extend(rhs.variables());
                vars
            }
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Predicate {
    pub symbol: String,
    pub arity: usize,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Atom {
    pub predicate: Predicate,
    pub terms: Vec<Term>,
}

impl_node!(Atom, Format, AtomParser);

impl Atom {
    pub fn predicate(&self) -> Predicate {
        self.predicate.clone()
    }

    pub fn variables(&self) -> HashSet<Variable> {
        let mut vars = HashSet::new();
        for term in self.terms.iter() {
            vars.extend(term.variables())
        }
        vars
    }
}

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

impl Literal {
    pub fn variables(&self) -> HashSet<Variable> {
        self.atom.variables()
    }

    pub fn predicates(&self) -> HashSet<Predicate> {
        HashSet::from([self.atom.predicate()])
    }
}

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

impl Comparison {
    pub fn variables(&self) -> HashSet<Variable> {
        let mut vars = self.lhs.variables();
        vars.extend(self.rhs.variables());
        vars
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum AtomicFormula {
    Literal(Literal),
    Comparison(Comparison),
}

impl_node!(AtomicFormula, Format, AtomicFormulaParser);

impl AtomicFormula {
    pub fn variables(&self) -> HashSet<Variable> {
        match &self {
            AtomicFormula::Literal(l) => l.variables(),
            AtomicFormula::Comparison(c) => c.variables(),
        }
    }

    pub fn predicates(&self) -> HashSet<Predicate> {
        match &self {
            AtomicFormula::Literal(l) => l.predicates(),
            AtomicFormula::Comparison(_) => HashSet::new(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Head {
    Basic(Atom),
    Choice(Atom),
    Falsity,
}

impl_node!(Head, Format, HeadParser);

impl Head {
    pub fn predicate(&self) -> Option<&Predicate> {
        match self {
            Head::Basic(a) => Some(&a.predicate),
            Head::Choice(a) => Some(&a.predicate),
            Head::Falsity => None,
        }
    }

    pub fn predicates(&self) -> HashSet<Predicate> {
        match self {
            Head::Basic(a) | Head::Choice(a) => HashSet::from([a.predicate()]),
            Head::Falsity => HashSet::new(),
        }
    }

    pub fn terms(&self) -> Option<&[Term]> {
        match self {
            Head::Basic(a) => Some(&a.terms),
            Head::Choice(a) => Some(&a.terms),
            Head::Falsity => None,
        }
    }

    pub fn arity(&self) -> usize {
        match self {
            Head::Basic(a) => a.terms.len(),
            Head::Choice(a) => a.terms.len(),
            Head::Falsity => 0,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Body {
    pub formulas: Vec<AtomicFormula>,
}

impl_node!(Body, Format, BodyParser);

impl Body {
    pub fn predicates(&self) -> HashSet<Predicate> {
        let mut preds = HashSet::<Predicate>::new();
        for form in self.formulas.iter() {
            preds.extend(form.predicates());
        }
        preds
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Rule {
    pub head: Head,
    pub body: Body,
}

impl_node!(Rule, Format, RuleParser);

impl Rule {
    // TODO: Drop?
    pub fn is_constraint(&self) -> bool {
        match &self.head {
            Head::Basic(_) | Head::Choice(_) => false,
            Head::Falsity => true,
        }
    }

    pub fn head_symbol(&self) -> Option<Predicate> {
        match &self.head {
            Head::Basic(a) => Some(a.predicate().clone()),
            Head::Choice(a) => Some(a.predicate().clone()),
            Head::Falsity => None,
        }
    }

    pub fn variables(&self) -> HashSet<Variable> {
        let mut vars = self.head_variables();
        vars.extend(self.body_variables());
        vars
    }

    pub fn head_variables(&self) -> HashSet<Variable> {
        match &self.head {
            Head::Basic(a) | Head::Choice(a) => a.variables(),
            Head::Falsity => HashSet::new(),
        }
    }

    pub fn body_variables(&self) -> HashSet<Variable> {
        let mut vars = HashSet::new();
        for formula in self.body.formulas.iter() {
            vars.extend(formula.variables())
        }
        vars
    }

    pub fn predicates(&self) -> HashSet<Predicate> {
        let mut preds = self.body.predicates();
        preds.extend(self.head.predicates());
        preds
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Program {
    pub rules: Vec<Rule>,
}

impl_node!(Program, Format, ProgramParser);

impl Program {
    pub fn variables(&self) -> HashSet<Variable> {
        let mut vars = HashSet::new();
        for rule in self.rules.iter() {
            vars.extend(rule.variables())
        }
        vars
    }

    pub fn predicates(&self) -> HashSet<Predicate> {
        let mut preds = HashSet::new();
        for rule in self.rules.iter() {
            preds.extend(rule.predicates())
        }
        preds
    }
}

