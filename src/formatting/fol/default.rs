use {
    crate::syntax_tree::{
        fol::{
            Atom, AtomicFormula, BasicIntegerTerm, BinaryConnective, BinaryOperator, Comparison,
            Formula, GeneralTerm, Guard, IntegerTerm, Quantification, Quantifier, Relation, Sort,
            UnaryConnective, UnaryOperator, Variable,
        },
        Node,
    },
    std::fmt::{self, Display, Formatter},
};

pub struct Format<'a, N: Node>(pub &'a N);

impl Display for Format<'_, BasicIntegerTerm> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self.0 {
            BasicIntegerTerm::Infimum => write!(f, "#inf"),
            BasicIntegerTerm::Supremum => write!(f, "#sup"),
            BasicIntegerTerm::Numeral(n) => write!(f, "{n}"),
            BasicIntegerTerm::IntegerVariable(s) => write!(f, "{s}$i"),
        }
    }
}

impl Display for Format<'_, UnaryOperator> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self.0 {
            UnaryOperator::Negative => write!(f, "-"),
        }
    }
}

impl Display for Format<'_, BinaryOperator> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self.0 {
            BinaryOperator::Add => write!(f, "+"),
            BinaryOperator::Subtract => write!(f, "-"),
            BinaryOperator::Multiply => write!(f, "*"),
        }
    }
}

impl Format<'_, IntegerTerm> {
    fn precedence(&self) -> usize {
        match self.0 {
            IntegerTerm::BasicIntegerTerm(BasicIntegerTerm::Numeral(1..)) => 1,
            IntegerTerm::UnaryOperation {
                op: UnaryOperator::Negative,
                ..
            }
            | IntegerTerm::BasicIntegerTerm(_) => 0,
            IntegerTerm::BinaryOperation {
                op: BinaryOperator::Multiply,
                ..
            } => 2,
            IntegerTerm::BinaryOperation {
                op: BinaryOperator::Add | BinaryOperator::Subtract,
                ..
            } => 3,
        }
    }
}

impl Display for Format<'_, IntegerTerm> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self.0 {
            IntegerTerm::BasicIntegerTerm(t) => Format(t).fmt(f),
            IntegerTerm::UnaryOperation { op, arg } => {
                let op = Format(op);
                let arg = Format(&**arg);

                write!(f, "{op}")?;
                if self.precedence() < arg.precedence() {
                    write!(f, "({arg})")
                } else {
                    write!(f, "{arg}")
                }
            }
            IntegerTerm::BinaryOperation { op, lhs, rhs } => {
                let op = Format(op);
                let lhs = Format(&**lhs);
                let rhs = Format(&**rhs);

                if self.precedence() < lhs.precedence() {
                    write!(f, "({lhs})")
                } else {
                    write!(f, "{lhs}")
                }?;
                write!(f, " {op} ")?;
                if self.precedence() <= rhs.precedence() {
                    write!(f, "({rhs})")
                } else {
                    write!(f, "{rhs}")
                }
            }
        }
    }
}

impl Display for Format<'_, GeneralTerm> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self.0 {
            GeneralTerm::Symbol(s) => write!(f, "{s}"),
            GeneralTerm::GeneralVariable(v) => write!(f, "{v}$g"),
            GeneralTerm::IntegerTerm(t) => Format(t).fmt(f),
        }
    }
}

impl Display for Format<'_, Atom> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let predicate = &self.0.predicate;
        let terms = &self.0.terms;

        write!(f, "{predicate}")?;

        if !terms.is_empty() {
            let mut iter = terms.iter().map(|t| Format(t));
            write!(f, "({}", iter.next().unwrap())?;
            for term in iter {
                write!(f, ", {term}")?;
            }
            write!(f, ")")?;
        }

        Ok(())
    }
}

impl Display for Format<'_, Relation> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self.0 {
            Relation::Equal => write!(f, "="),
            Relation::NotEqual => write!(f, "!="),
            Relation::GreaterEqual => write!(f, ">="),
            Relation::LessEqual => write!(f, "<="),
            Relation::Greater => write!(f, ">"),
            Relation::Less => write!(f, "<"),
        }
    }
}

impl Display for Format<'_, Guard> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", Format(&self.0.relation), Format(&self.0.term))
    }
}

impl Display for Format<'_, Comparison> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let guards = &self.0.guards;

        write!(f, "{}", Format(&self.0.term))?;

        let iter = guards.iter().map(|t| Format(t));
        for guard in iter {
            write!(f, " {guard}")?;
        }

        Ok(())
    }
}

impl Display for Format<'_, AtomicFormula> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self.0 {
            AtomicFormula::Falsity => write!(f, "#false"),
            AtomicFormula::Atom(a) => Format(a).fmt(f),
            AtomicFormula::Comparison(c) => Format(c).fmt(f),
        }
    }
}

impl Display for Format<'_, Quantifier> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self.0 {
            Quantifier::Forall => write!(f, "forall"),
            Quantifier::Exists => write!(f, "exists"),
        }
    }
}

impl Display for Format<'_, Variable> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let name = &self.0.name;
        let sort = &self.0.sort;

        match sort {
            Sort::General => write!(f, "{name}$g"),
            Sort::Integer => write!(f, "{name}$i"),
        }
    }
}

impl Display for Format<'_, Quantification> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let quantifier = &self.0.quantifier;
        let variables = &self.0.variables;

        match quantifier {
            Quantifier::Forall => write!(f, "forall"),
            Quantifier::Exists => write!(f, "exists"),
        }?;

        let iter = variables.iter().map(|t| Format(t));
        for var in iter {
            write!(f, " {var}")?;
        }

        Ok(())
    }
}

impl Display for Format<'_, UnaryConnective> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self.0 {
            UnaryConnective::Negation => write!(f, "not"),
        }
    }
}

impl Display for Format<'_, BinaryConnective> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self.0 {
            BinaryConnective::Equivalence => write!(f, "<=>"),
            BinaryConnective::Implication => write!(f, "=>"),
            BinaryConnective::ReverseImplication => write!(f, "<="),
            BinaryConnective::Conjunction => write!(f, "and"),
            BinaryConnective::Disjunction => write!(f, "or"),
        }
    }
}

impl Display for Format<'_, Formula> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        todo!();
    }
}

// TODO Zach: Continue implementing the default formatting for first-order logic here

#[cfg(test)]
mod tests {
    use crate::{
        formatting::fol::default::Format,
        syntax_tree::fol::{
            Atom, AtomicFormula, BasicIntegerTerm, BinaryOperator, Comparison, GeneralTerm, Guard,
            IntegerTerm, Quantification, Quantifier, Relation, Sort, Variable,
        },
    };

    #[test]
    fn format_basic_integer_term() {
        assert_eq!(Format(&BasicIntegerTerm::Infimum).to_string(), "#inf");
        assert_eq!(Format(&BasicIntegerTerm::Numeral(-1)).to_string(), "-1");
        assert_eq!(Format(&BasicIntegerTerm::Numeral(0)).to_string(), "0");
        assert_eq!(Format(&BasicIntegerTerm::Numeral(42)).to_string(), "42");
        assert_eq!(
            Format(&BasicIntegerTerm::IntegerVariable("A".into())).to_string(),
            "A$i"
        );
        assert_eq!(Format(&BasicIntegerTerm::Supremum).to_string(), "#sup");
    }

    #[test]
    fn format_general_term() {
        assert_eq!(
            Format(&GeneralTerm::IntegerTerm(
                IntegerTerm::BasicIntegerTerm(BasicIntegerTerm::IntegerVariable("N".into())).into()
            ))
            .to_string(),
            "N$i"
        );
        assert_eq!(
            Format(&GeneralTerm::Symbol("abc".into())).to_string(),
            "abc"
        );
        assert_eq!(
            Format(&GeneralTerm::IntegerTerm(IntegerTerm::BinaryOperation {
                op: BinaryOperator::Multiply,
                lhs: IntegerTerm::BasicIntegerTerm(BasicIntegerTerm::Numeral(1)).into(),
                rhs: IntegerTerm::BasicIntegerTerm(BasicIntegerTerm::Numeral(5)).into(),
            }))
            .to_string(),
            "1 * 5"
        );
    }

    #[test]
    fn format_comparison() {
        assert_eq!(
            Format(&Comparison {
                term: GeneralTerm::IntegerTerm(IntegerTerm::BasicIntegerTerm(
                    BasicIntegerTerm::Numeral(1)
                )),
                guards: vec![Guard {
                    relation: Relation::Less,
                    term: GeneralTerm::IntegerTerm(IntegerTerm::BasicIntegerTerm(
                        BasicIntegerTerm::Numeral(5)
                    )),
                }]
            })
            .to_string(),
            "1 < 5"
        );
        assert_eq!(
            Format(&Comparison {
                term: GeneralTerm::IntegerTerm(IntegerTerm::BasicIntegerTerm(
                    BasicIntegerTerm::IntegerVariable("N".into())
                )),
                guards: vec![
                    Guard {
                        relation: Relation::Less,
                        term: GeneralTerm::IntegerTerm(IntegerTerm::BasicIntegerTerm(
                            BasicIntegerTerm::Numeral(5)
                        )),
                    },
                    Guard {
                        relation: Relation::NotEqual,
                        term: GeneralTerm::IntegerTerm(IntegerTerm::BinaryOperation {
                            op: BinaryOperator::Multiply,
                            lhs: IntegerTerm::BasicIntegerTerm(BasicIntegerTerm::Numeral(7)).into(),
                            rhs: IntegerTerm::BasicIntegerTerm(BasicIntegerTerm::Numeral(2)).into(),
                        }),
                    },
                    Guard {
                        relation: Relation::GreaterEqual,
                        term: GeneralTerm::IntegerTerm(IntegerTerm::BasicIntegerTerm(
                            BasicIntegerTerm::IntegerVariable("Xa".into())
                        )),
                    },
                ]
            })
            .to_string(),
            "N$i < 5 != 7 * 2 >= Xa$i"
        );
    }

    #[test]
    fn format_quantification() {
        assert_eq!(
            Format(&Quantification {
                quantifier: Quantifier::Forall,
                variables: vec![
                    Variable {
                        name: "X".into(),
                        sort: Sort::General,
                    },
                    Variable {
                        name: "Y".into(),
                        sort: Sort::Integer,
                    },
                    Variable {
                        name: "N".into(),
                        sort: Sort::General,
                    },
                ]
            })
            .to_string(),
            "forall X$g Y$i N$g"
        );
    }

    #[test]
    fn format_atomic_formula() {
        assert_eq!(
            Format(&AtomicFormula::Atom(Atom {
                predicate: "p".into(),
                terms: vec![
                    GeneralTerm::Symbol("a".into()),
                    GeneralTerm::IntegerTerm(IntegerTerm::BasicIntegerTerm(
                        BasicIntegerTerm::IntegerVariable("X".into())
                    )),
                ]
            }))
            .to_string(),
            "p(a, X$i)"
        );
        assert_eq!(
            Format(&AtomicFormula::Comparison(Comparison {
                term: GeneralTerm::IntegerTerm(IntegerTerm::BasicIntegerTerm(
                    BasicIntegerTerm::Numeral(5)
                )),
                guards: vec![Guard {
                    relation: Relation::Less,
                    term: GeneralTerm::GeneralVariable("I".into()),
                }]
            }))
            .to_string(),
            "5 < I$g"
        );
        assert_eq!(Format(&AtomicFormula::Falsity).to_string(), "#false");
    }

    // TODO Zach: Add tests for the remaining formatters
}
