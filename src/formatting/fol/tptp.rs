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
            BasicIntegerTerm::Infimum => write!(f, "c__infimum__"),
            BasicIntegerTerm::Supremum => write!(f, "c__supremum__"),
            BasicIntegerTerm::Numeral(n) => {
                if *n < 0 as isize {
                    let m = (*n).abs();
                    write!(f, "$uminus({m})")?;
                } else {
                    write!(f, "{n}")?;
                }

                Ok(())
            }
            BasicIntegerTerm::IntegerVariable(v) => write!(f, "{v}"),
        }
    }
}

impl Display for Format<'_, UnaryOperator> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self.0 {
            UnaryOperator::Negative => write!(f, "$uminus"),
        }
    }
}

impl Display for Format<'_, BinaryOperator> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self.0 {
            BinaryOperator::Add => write!(f, "$sum"),
            BinaryOperator::Subtract => write!(f, "$difference"),
            BinaryOperator::Multiply => write!(f, "$product"),
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

                write!(f, "{op}(")?;
                if self.precedence() < arg.precedence() {
                    write!(f, "({arg})")
                } else {
                    write!(f, "{arg}")
                }?;
                write!(f, ")")?;

                Ok(())
            }
            IntegerTerm::BinaryOperation { op, lhs, rhs } => {
                let op = Format(op);
                let lhs = Format(&**lhs);
                let rhs = Format(&**rhs);

                write!(f, "{op}(")?;
                if self.precedence() < lhs.precedence() {
                    write!(f, "({lhs}), ")
                } else {
                    write!(f, "{lhs}, ")
                }?;
                if self.precedence() <= rhs.precedence() {
                    write!(f, "({rhs})")
                } else {
                    write!(f, "{rhs}")
                }?;
                write!(f, ")")?;

                Ok(())
            }
        }
    }
}

impl Display for Format<'_, GeneralTerm> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self.0 {
            GeneralTerm::Symbol(s) => write!(f, "{s}"),
            GeneralTerm::GeneralVariable(v) => write!(f, "{v}"),
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
            Relation::GreaterEqual => write!(f, "$greatereq"),
            Relation::LessEqual => write!(f, "$lesseq"),
            Relation::Greater => write!(f, "$greater"),
            Relation::Less => write!(f, "$less"),
        }
    }
}

impl Display for Format<'_, Comparison> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let guards = &self.0.guards;

        let mut counter = 0;
        let mut previous_term = &self.0.term;
        for g in guards.iter() {
            if counter > 0 {
                write!(f, " and ")?;
            }
            match g.relation {
                Relation::Equal | Relation::NotEqual => write!(
                    f,
                    "{} {} {}",
                    Format(previous_term),
                    Format(&g.relation),
                    Format(&g.term)
                ),
                _ => write!(
                    f,
                    "{}({}, {})",
                    Format(&g.relation),
                    Format(previous_term),
                    Format(&g.term)
                ),
            }?;
            previous_term = &g.term;
            counter += 1;
        }

        Ok(())
    }
}

impl Display for Format<'_, AtomicFormula> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self.0 {
            AtomicFormula::Falsity => write!(f, "$false"),
            AtomicFormula::Atom(a) => Format(a).fmt(f),
            AtomicFormula::Comparison(c) => Format(c).fmt(f),
        }
    }
}

impl Display for Format<'_, Quantifier> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self.0 {
            Quantifier::Forall => write!(f, "!"),
            Quantifier::Exists => write!(f, "?"),
        }
    }
}

impl Display for Format<'_, Variable> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", &self.0.name)
    }
}

impl Display for Format<'_, Quantification> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let variables = &self.0.variables;

        write!(f, "{}[", Format(&self.0.quantifier))?;

        let mut counter = 0;
        for var in variables.iter() {
            if counter > 0 {
                write!(f, ", ")?;
            }
            match var.sort {
                Sort::Integer => write!(f, "{}: $int", var.name),
                Sort::General => write!(f, "{}: object", var.name),
            }?;
            counter += 1;
        }

        write!(f, "]: ")?;

        Ok(())
    }
}

impl Display for Format<'_, UnaryConnective> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self.0 {
            UnaryConnective::Negation => write!(f, "~"),
        }
    }
}

impl Display for Format<'_, BinaryConnective> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self.0 {
            BinaryConnective::Equivalence => write!(f, "<=>"),
            BinaryConnective::Implication => write!(f, "=>"),
            BinaryConnective::ReverseImplication => write!(f, "<="),    // TODO: Decide how this should be converted to TFF
            BinaryConnective::Conjunction => write!(f, "&"),
            BinaryConnective::Disjunction => write!(f, "|"),
        }
    }
}

impl Format<'_, Formula> {
    fn precedence(&self) -> usize {
        match self.0 {
            Formula::AtomicFormula(_) => 0,
            Formula::UnaryFormula { .. } | Formula::QuantifiedFormula { .. } => 1,
            Formula::BinaryFormula {
                connective: BinaryConnective::Conjunction,
                ..
            } => 2,
            Formula::BinaryFormula {
                connective: BinaryConnective::Disjunction,
                ..
            } => 3,
            Formula::BinaryFormula {
                connective:
                    BinaryConnective::Equivalence
                    | BinaryConnective::Implication
                    | BinaryConnective::ReverseImplication,
                ..
            } => 4,
        }
    }

    fn is_left_associative(&self) -> bool {
        match self.0 {
            Formula::BinaryFormula {
                connective:
                    BinaryConnective::Conjunction
                    | BinaryConnective::Disjunction
                    | BinaryConnective::ReverseImplication,
                ..
            } => true,
            Formula::BinaryFormula {
                connective: BinaryConnective::Equivalence | BinaryConnective::Implication,
                ..
            } => false,
            _ => unreachable!(), // TODO
        }
    }
}

impl Display for Format<'_, Formula> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self.0 {
            Formula::AtomicFormula(a) => Format(a).fmt(f),
            Formula::UnaryFormula {
                connective,
                formula,
            } => {
                let connective = Format(connective);
                let formula = Format(formula.as_ref());

                connective.fmt(f)?;
                if self.precedence() < formula.precedence() {
                    write!(f, "({formula})")
                } else {
                    write!(f, "{formula}")
                }
            }
            Formula::QuantifiedFormula {
                quantification,
                formula,
            } => {
                let connective = Format(quantification);
                let formula = Format(formula.as_ref());

                connective.fmt(f)?;
                if 0 == 0 {
                    write!(f, "({formula})")
                } else {
                    write!(f, "({formula})")
                }

                //if self.precedence() < formula.precedence() {
                //    write!(f, "({formula})")
                //} else {
                //    write!(f, "{formula}")
                //}
            }
            Formula::BinaryFormula {
                connective,
                lhs,
                rhs,
            } => {
                let op = Format(connective);
                let lhs = Format(lhs.as_ref());
                let rhs = Format(rhs.as_ref());

                if self.precedence() < lhs.precedence()
                    || self.precedence() == lhs.precedence() && !lhs.is_left_associative()
                {
                    write!(f, "({lhs})")
                } else {
                    write!(f, "{lhs}")
                }?;

                write!(f, " {op} ")?;
                if self.precedence() < rhs.precedence()
                    || self.precedence() == rhs.precedence() && self.is_left_associative()
                {
                    write!(f, "({rhs})")
                } else {
                    write!(f, "{rhs}")
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        formatting::fol::tptp::Format,
        syntax_tree::fol::{
            Atom, AtomicFormula, BasicIntegerTerm, BinaryOperator, Comparison, GeneralTerm, Guard,
            IntegerTerm, Quantification, Quantifier, Relation, Sort, UnaryOperator, Variable, Formula,
            BinaryConnective,
        },
    };

    #[test]
    fn format_basic_integer_term() {
        assert_eq!(
            Format(&BasicIntegerTerm::Infimum).to_string(),
            "c__infimum__"
        );
        assert_eq!(
            Format(&BasicIntegerTerm::Numeral(-1)).to_string(),
            "$uminus(1)"
        );
        assert_eq!(Format(&BasicIntegerTerm::Numeral(0)).to_string(), "0");
        assert_eq!(Format(&BasicIntegerTerm::Numeral(42)).to_string(), "42");
        assert_eq!(
            Format(&BasicIntegerTerm::IntegerVariable("A".into())).to_string(),
            "A"
        );
        assert_eq!(
            Format(&BasicIntegerTerm::Supremum).to_string(),
            "c__supremum__"
        );
    }

    #[test]
    fn format_integer_term() {
        assert_eq!(
            Format(&IntegerTerm::BasicIntegerTerm(BasicIntegerTerm::Infimum)).to_string(),
            "c__infimum__"
        );
        assert_eq!(
            Format(&IntegerTerm::BasicIntegerTerm(BasicIntegerTerm::Supremum)).to_string(),
            "c__supremum__"
        );
        assert_eq!(
            Format(&IntegerTerm::BasicIntegerTerm(BasicIntegerTerm::Numeral(3))).to_string(),
            "3"
        );
        assert_eq!(
            Format(&IntegerTerm::BasicIntegerTerm(BasicIntegerTerm::Numeral(
                -3
            )))
            .to_string(),
            "$uminus(3)"
        );
        assert_eq!(
            Format(&IntegerTerm::BinaryOperation {
                op: BinaryOperator::Multiply,
                lhs: IntegerTerm::BasicIntegerTerm(BasicIntegerTerm::Numeral(1)).into(),
                rhs: IntegerTerm::BasicIntegerTerm(BasicIntegerTerm::Numeral(5)).into(),
            })
            .to_string(),
            "$product(1, 5)"
        );
        assert_eq!(
            Format(&IntegerTerm::BinaryOperation {
                op: BinaryOperator::Add,
                lhs: IntegerTerm::BasicIntegerTerm(BasicIntegerTerm::Numeral(10)).into(),
                rhs: IntegerTerm::BasicIntegerTerm(BasicIntegerTerm::IntegerVariable("N".into()))
                    .into(),
            })
            .to_string(),
            "$sum(10, N)"
        );
        assert_eq!(
            Format(&IntegerTerm::BinaryOperation {
                op: BinaryOperator::Subtract,
                lhs: IntegerTerm::BasicIntegerTerm(BasicIntegerTerm::Numeral(-195)).into(),
                rhs: IntegerTerm::UnaryOperation {
                    op: UnaryOperator::Negative,
                    arg: IntegerTerm::BasicIntegerTerm(BasicIntegerTerm::IntegerVariable(
                        "N".into()
                    ))
                    .into(),
                }
                .into(),
            })
            .to_string(),
            "$difference($uminus(195), $uminus(N))"
        );
    }

    #[test]
    fn format_general_term() {
        assert_eq!(
            Format(&GeneralTerm::IntegerTerm(IntegerTerm::BasicIntegerTerm(
                BasicIntegerTerm::Infimum
            )))
            .to_string(),
            "c__infimum__"
        );
        assert_eq!(Format(&GeneralTerm::Symbol("p".into())).to_string(), "p");
        assert_eq!(
            Format(&GeneralTerm::GeneralVariable("N1".into())).to_string(),
            "N1"
        );
    }

    #[test]
    fn format_atom() {
        assert_eq!(
            Format(&Atom {
                predicate: "prime".into(),
                terms: vec![
                    GeneralTerm::IntegerTerm(IntegerTerm::BinaryOperation {
                        op: BinaryOperator::Add,
                        lhs: IntegerTerm::BasicIntegerTerm(BasicIntegerTerm::IntegerVariable(
                            "N1".into()
                        ))
                        .into(),
                        rhs: IntegerTerm::BasicIntegerTerm(BasicIntegerTerm::Numeral(3)).into(),
                    }),
                    GeneralTerm::IntegerTerm(IntegerTerm::BasicIntegerTerm(
                        BasicIntegerTerm::Numeral(5)
                    )),
                ]
            })
            .to_string(),
            "prime($sum(N1, 3), 5)"
        )
    }

    #[test]
    fn format_comparison() {
        assert_eq!(
            Format(&Comparison {
                term: GeneralTerm::IntegerTerm(IntegerTerm::BasicIntegerTerm(
                    BasicIntegerTerm::Numeral(5)
                )),
                guards: vec![Guard {
                    relation: Relation::Equal,
                    term: GeneralTerm::IntegerTerm(IntegerTerm::BasicIntegerTerm(
                        BasicIntegerTerm::Numeral(3)
                    )),
                }]
            })
            .to_string(),
            "5 = 3"
        );
        assert_eq!(
            Format(&Comparison {
                term: GeneralTerm::IntegerTerm(IntegerTerm::BasicIntegerTerm(
                    BasicIntegerTerm::Numeral(5)
                )),
                guards: vec![Guard {
                    relation: Relation::NotEqual,
                    term: GeneralTerm::IntegerTerm(IntegerTerm::BasicIntegerTerm(
                        BasicIntegerTerm::Numeral(3)
                    )),
                }]
            })
            .to_string(),
            "5 != 3"
        );
        assert_eq!(
            Format(&Comparison {
                term: GeneralTerm::IntegerTerm(IntegerTerm::BasicIntegerTerm(
                    BasicIntegerTerm::Numeral(5)
                )),
                guards: vec![Guard {
                    relation: Relation::LessEqual,
                    term: GeneralTerm::IntegerTerm(IntegerTerm::BasicIntegerTerm(
                        BasicIntegerTerm::Numeral(3)
                    )),
                }]
            })
            .to_string(),
            "$lesseq(5, 3)"
        );
        assert_eq!(
            Format(&Comparison {
                term: GeneralTerm::IntegerTerm(IntegerTerm::BasicIntegerTerm(
                    BasicIntegerTerm::Numeral(5)
                )),
                guards: vec![
                    Guard {
                        relation: Relation::LessEqual,
                        term: GeneralTerm::IntegerTerm(IntegerTerm::BasicIntegerTerm(
                            BasicIntegerTerm::Numeral(3)
                        )),
                    },
                    Guard {
                        relation: Relation::Equal,
                        term: GeneralTerm::IntegerTerm(IntegerTerm::BasicIntegerTerm(
                            BasicIntegerTerm::Numeral(4)
                        )),
                    }
                ]
            })
            .to_string(),
            "$lesseq(5, 3) and 3 = 4"
        );
        assert_eq!(
            Format(&Comparison {
                term: GeneralTerm::IntegerTerm(IntegerTerm::BasicIntegerTerm(
                    BasicIntegerTerm::Numeral(5)
                )),
                guards: vec![
                    Guard {
                        relation: Relation::LessEqual,
                        term: GeneralTerm::IntegerTerm(IntegerTerm::BasicIntegerTerm(
                            BasicIntegerTerm::Numeral(3)
                        )),
                    },
                    Guard {
                        relation: Relation::Less,
                        term: GeneralTerm::IntegerTerm(IntegerTerm::BasicIntegerTerm(
                            BasicIntegerTerm::Numeral(6)
                        )),
                    },
                    Guard {
                        relation: Relation::NotEqual,
                        term: GeneralTerm::IntegerTerm(IntegerTerm::BasicIntegerTerm(
                            BasicIntegerTerm::Numeral(5)
                        )),
                    }
                ]
            })
            .to_string(),
            "$lesseq(5, 3) and $less(3, 6) and 6 != 5"
        );
    }

    #[test]
    fn format_quantification() {
        assert_eq!(
            Format(&Quantification {
                quantifier: Quantifier::Forall,
                variables: vec![
                    Variable {
                        name: "X1".into(),
                        sort: Sort::Integer,
                    },
                    Variable {
                        name: "N2".into(),
                        sort: Sort::General,
                    },
                ]
            })
            .to_string(),
            "![X1: $int, N2: object]: "
        );
        assert_eq!(
            Format(&Quantification {
                quantifier: Quantifier::Exists,
                variables: vec![
                    Variable {
                        name: "X1".into(),
                        sort: Sort::Integer,
                    },
                ]
            })
            .to_string(),
            "?[X1: $int]: "
        );
    }

    #[test]
    fn format_formula() {
        assert_eq!(
            Format(&Formula::AtomicFormula(AtomicFormula::Atom(Atom {
                predicate: "p".into(),
                terms: vec![]
            })))
            .to_string(),
            "p"
        );
        assert_eq!(
            Format(&Formula::BinaryFormula {
                connective: BinaryConnective::Implication,
                lhs: Formula::BinaryFormula {
                    connective: BinaryConnective::Implication,
                    lhs: Formula::AtomicFormula(AtomicFormula::Atom(Atom {
                        predicate: "p".into(),
                        terms: vec![]
                    }))
                    .into(),
                    rhs: Formula::AtomicFormula(AtomicFormula::Atom(Atom {
                        predicate: "q".into(),
                        terms: vec![]
                    }))
                    .into()
                }
                .into(),
                rhs: Formula::AtomicFormula(AtomicFormula::Atom(Atom {
                    predicate: "r".into(),
                    terms: vec![]
                }))
                .into(),
            })
            .to_string(),
            "(p => q) => r"
        );
        assert_eq!(Format(&Formula::QuantifiedFormula {
            quantification: Quantification {
                quantifier: Quantifier::Forall,
                variables: vec![
                    Variable {
                        name: "X".into(),
                        sort: Sort::Integer,
                    },
                    Variable {
                        name: "Y1".into(),
                        sort: Sort::General,
                    },
                ]
            },
            formula: Formula::BinaryFormula {
                connective: BinaryConnective::Conjunction,
                lhs: Formula::AtomicFormula(AtomicFormula::Atom(Atom {
                    predicate: "p".into(),
                    terms: vec![],
                })).into(),
                rhs: Formula::AtomicFormula(AtomicFormula::Atom(Atom {
                    predicate: "q".into(),
                    terms: vec![],
                })).into(),
            }.into()
        }).to_string(), "![X: $int, Y1: object]: (p & q)");
    }
}

// TODO Zach: Implement the TPTP formatting for first-order logic here
