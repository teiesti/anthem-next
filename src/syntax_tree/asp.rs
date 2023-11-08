use crate::{
    formatting::asp::default::Format,
    parsing::asp::pest::{
        AtomParser, AtomicFormulaParser, BinaryOperatorParser, BodyParser, ComparisonParser,
        HeadParser, LiteralParser, PrecomputedTermParser, ProgramParser, RelationParser,
        RuleParser, SignParser, TermParser, UnaryOperatorParser, VariableParser,
    },
    syntax_tree::{impl_node, Node},
};

use std::collections::HashSet;

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

impl Term {
    pub fn get_variables(&self) -> HashSet<String> {
        let mut vars: HashSet<String> = HashSet::<String>::new();
        match &self {
            Term::PrecomputedTerm(_) => (),
            Term::Variable(v) => {
                vars.insert(v.0.clone());
            }
            Term::UnaryOperation { op: _, arg: term1 } => {
                let term1_vars = term1.get_variables();
                for t in term1_vars.iter() {
                    vars.insert(t.to_string());
                }
            }
            Term::BinaryOperation {
                op: _,
                lhs: term1,
                rhs: term2,
            } => {
                let term1_vars = term1.get_variables();
                for t in term1_vars.iter() {
                    vars.insert(t.to_string());
                }
                let term2_vars = term2.get_variables();
                for t in term2_vars.iter() {
                    vars.insert(t.to_string());
                }
            }
        }
        vars
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Atom {
    pub predicate: String,
    pub terms: Vec<Term>,
}

impl_node!(Atom, Format, AtomParser);

impl Atom {
    pub fn get_predicate_symbol(&self) -> String {
        let predicate_symbol: String = self.predicate.clone().to_owned();
        /*let predicate_arity: &str = &self.terms.len().to_string();
        predicate_symbol.push_str("/");
        predicate_symbol.push_str(predicate_arity);*/
        predicate_symbol
    }

    pub fn get_variables(&self) -> HashSet<String> {
        let mut vars: HashSet<String> = HashSet::<String>::new();
        for term in self.terms.iter() {
            let term_vars = term.get_variables();
            for t in term_vars.iter() {
                vars.insert(t.to_string());
            }
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
    pub fn get_variables(&self) -> HashSet<String> {
        self.atom.get_variables()
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
    pub fn get_variables(&self) -> HashSet<String> {
        let mut vars: HashSet<String> = HashSet::<String>::new();
        for var in self.lhs.get_variables().iter() {
            vars.insert(var.to_string());
        }
        for var in self.rhs.get_variables().iter() {
            vars.insert(var.to_string());
        }
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
    pub fn get_variables(&self) -> HashSet<String> {
        match &self {
            AtomicFormula::Literal(l) => l.get_variables(),
            AtomicFormula::Comparison(c) => c.get_variables(),
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
    pub fn get_predicate(&self) -> Option<String> {
        match self {
            Head::Basic(a) => Some(a.predicate.clone()),
            Head::Choice(a) => Some(a.predicate.clone()),
            Head::Falsity => None,
        }
    }

    pub fn get_terms(&self) -> Option<Vec<Term>> {
        match self {
            Head::Basic(a) => Some(a.terms.clone()),
            Head::Choice(a) => Some(a.terms.clone()),
            Head::Falsity => None,
        }
    }

    pub fn get_arity(&self) -> usize {
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

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Rule {
    pub head: Head,
    pub body: Body,
}

impl_node!(Rule, Format, RuleParser);

impl Rule {
    pub fn is_propositional_fact(&self) -> bool {
        if *(&self.body.formulas.len()) == 0 {
            match &self.head {
                Head::Basic(Atom {
                    predicate: _,
                    terms: t,
                }) => {
                    if t.len() == 0 {
                        true
                    } else {
                        false
                    }
                }
                Head::Choice(Atom {
                    predicate: _,
                    terms: t,
                }) => {
                    if t.len() == 0 {
                        true
                    } else {
                        false
                    }
                }
                Head::Falsity => todo!(), // technically, someone could write "#false." as a fact, making the program immediately unsat
            }
        } else {
            false
        }
    }

    pub fn is_constraint(&self) -> bool {
        match &self.head {
            Head::Basic(_) | Head::Choice(_) => false,
            Head::Falsity => true,
        }
    }

    pub fn get_head_symbol(&self) -> Option<String> {
        match &self.head {
            Head::Basic(a) => Some(a.get_predicate_symbol()),
            Head::Choice(a) => Some(a.get_predicate_symbol()),
            Head::Falsity => None,
        }
    }

    pub fn get_variables(&self) -> HashSet<String> {
        let mut vars: HashSet<String> = HashSet::new();
        for var in self.get_body_variables().iter() {
            vars.insert(var.to_string());
        }
        for var in self.get_head_variables().iter() {
            vars.insert(var.to_string());
        }
        vars
    }

    pub fn get_head_variables(&self) -> HashSet<String> {
        let mut vars: HashSet<String> = HashSet::new();
        match &self.head {
            Head::Basic(a) => {
                for var in a.get_variables().iter() {
                    vars.insert(var.to_string());
                }
            }
            Head::Choice(a) => {
                for var in a.get_variables().iter() {
                    vars.insert(var.to_string());
                }
            }
            Head::Falsity => (),
        };
        vars
    }

    pub fn get_body_variables(&self) -> HashSet<String> {
        let mut vars: HashSet<String> = HashSet::new();
        for formula in self.body.formulas.iter() {
            for var in formula.get_variables() {
                vars.insert(var.to_string());
            }
        }
        vars
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Program {
    pub rules: Vec<Rule>,
}

impl_node!(Program, Format, ProgramParser);

impl Program {
    pub fn get_variables(&self) -> HashSet<String> {
        let mut vars: HashSet<String> = HashSet::new();
        for rule in self.rules.iter() {
            for var in rule.get_variables().iter() {
                vars.insert(var.to_string());
            }
        }
        vars
    }
}
