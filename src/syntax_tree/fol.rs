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
use std::collections::HashSet;

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum BasicIntegerTerm {
    Infimum,
    Supremum,
    Numeral(isize),
    IntegerVariable(String),
}

impl_node!(BasicIntegerTerm, Format, BasicIntegerTermParser);

impl BasicIntegerTerm {
    pub fn get_variables(&self) -> HashSet<Variable> {
        let mut vars: HashSet<Variable> = HashSet::<Variable>::new();
        match &self {
            BasicIntegerTerm::IntegerVariable(v) => {
                vars.insert(Variable {
                    name: v.to_string(),
                    sort: Sort::Integer,
                });
            }
            _ => {}
        }
        vars
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum UnaryOperator {
    Negative,
}

impl_node!(UnaryOperator, Format, UnaryOperatorParser);

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum BinaryOperator {
    Add,
    Subtract,
    Multiply,
}

impl_node!(BinaryOperator, Format, BinaryOperatorParser);

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
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

impl IntegerTerm {
    pub fn get_variables(&self) -> HashSet<Variable> {
        match &self {
            IntegerTerm::BasicIntegerTerm(t) => t.get_variables(),
            IntegerTerm::UnaryOperation { op: _, arg: t } => t.get_variables(),
            IntegerTerm::BinaryOperation {
                op: _,
                lhs: t1,
                rhs: t2,
            } => {
                let mut vars: HashSet<Variable> = HashSet::<Variable>::new();
                for var in t1.get_variables().iter() {
                    vars.insert(var.clone());
                }
                for var in t2.get_variables().iter() {
                    vars.insert(var.clone());
                }
                vars
            }
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum GeneralTerm {
    Symbol(String),
    GeneralVariable(String),
    IntegerTerm(IntegerTerm),
}

impl_node!(GeneralTerm, Format, GeneralTermParser);

impl GeneralTerm {
    pub fn get_variables(&self) -> HashSet<Variable> {
        let mut vars: HashSet<Variable> = HashSet::<Variable>::new();
        match &self {
            GeneralTerm::Symbol(_) => vars,
            GeneralTerm::GeneralVariable(v) => {
                vars.insert(Variable {
                    name: v.to_string(),
                    sort: Sort::General,
                });
                vars
            }
            GeneralTerm::IntegerTerm(t) => t.get_variables(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Atom {
    pub predicate: String,
    pub terms: Vec<GeneralTerm>,
}

impl_node!(Atom, Format, AtomParser);

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum Relation {
    Equal,
    NotEqual,
    Greater,
    Less,
    GreaterEqual,
    LessEqual,
}

impl_node!(Relation, Format, RelationParser);

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Guard {
    pub relation: Relation,
    pub term: GeneralTerm,
}

impl_node!(Guard, Format, GuardParser);

impl Guard {
    pub fn get_variables(&self) -> HashSet<Variable> {
        self.term.get_variables()
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Comparison {
    pub term: GeneralTerm,
    pub guards: Vec<Guard>,
}

impl_node!(Comparison, Format, ComparisonParser);

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum AtomicFormula {
    Falsity,
    Atom(Atom),
    Comparison(Comparison),
}

impl_node!(AtomicFormula, Format, AtomicFormulaParser);

impl AtomicFormula {
    pub fn get_variables(&self) -> HashSet<Variable> {
        let mut vars: HashSet<Variable> = HashSet::<Variable>::new();
        match &self {
            AtomicFormula::Falsity => {}
            AtomicFormula::Atom(a) => {
                for t in a.terms.iter() {
                    for var in t.get_variables() {
                        vars.insert(var.clone());
                    }
                }
            }
            AtomicFormula::Comparison(c) => {
                let t1_vars = c.term.get_variables();
                for var in t1_vars.iter() {
                    vars.insert(var.clone());
                }
                for guard in c.guards.iter() {
                    for var in guard.get_variables().iter() {
                        vars.insert(var.clone());
                    }
                }
            }
        }
        vars
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum UnaryConnective {
    Negation,
}

impl_node!(UnaryConnective, Format, UnaryConnectiveParser);

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum Quantifier {
    Forall,
    Exists,
}

impl_node!(Quantifier, Format, QuantifierParser);

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Quantification {
    pub quantifier: Quantifier,
    pub variables: Vec<Variable>,
}

impl_node!(Quantification, Format, QuantificationParser);

#[derive(Clone, Debug, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub enum Sort {
    Integer,
    General,
}

// TODO: Should Sort be a Node?

#[derive(Clone, Debug, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub struct Variable {
    pub name: String,
    pub sort: Sort,
}

impl_node!(Variable, Format, VariableParser);

/*impl Ord for Variable {
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
}*/

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum BinaryConnective {
    Conjunction,
    Disjunction,
    Implication,
    ReverseImplication,
    Equivalence,
}

impl_node!(BinaryConnective, Format, BinaryConnectiveParser);

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
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

impl Formula {
    pub fn get_variables(&self) -> HashSet<Variable> {
        match &self {
            Formula::AtomicFormula(f) => f.get_variables(),
            Formula::UnaryFormula {
                connective: _,
                formula: f,
            } => f.get_variables(),
            Formula::BinaryFormula {
                connective: _,
                lhs: f1,
                rhs: f2,
            } => {
                let mut vars: HashSet<Variable> = HashSet::<Variable>::new();
                for var in f1.get_variables().iter() {
                    vars.insert(var.clone());
                }
                for var in f2.get_variables().iter() {
                    vars.insert(var.clone());
                }
                vars
            }
            Formula::QuantifiedFormula {
                quantification: _,
                formula: f,
            } => f.get_variables(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Theory {
    pub formulas: Vec<Formula>,
}

impl_node!(Theory, Format, TheoryParser);
