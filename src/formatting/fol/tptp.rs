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

#[cfg(test)]
mod tests {
    use crate::{
        formatting::fol::tptp::Format,
        syntax_tree::fol::{
            Atom, AtomicFormula, BasicIntegerTerm, BinaryOperator, Comparison, GeneralTerm, Guard,
            IntegerTerm, Quantification, Quantifier, Relation, Sort, UnaryOperator, Variable,
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
}

// TODO Zach: Implement the TPTP formatting for first-order logic here
