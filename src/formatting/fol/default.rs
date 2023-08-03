use {
    crate::syntax_tree::{
        fol::{
            Atom, AtomicFormula, BasicIntegerTerm, BinaryConnective, BinaryOperator, Comparison,
            GeneralTerm, Guard, IntegerTerm, Quantification, Quantifier, Relation, Sort,
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
        todo!();
    }
}

impl Display for Format<'_, Comparison> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        todo!();
    }
}

impl Display for Format<'_, AtomicFormula> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        todo!();
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
        todo!();
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

// TODO Zach: Continue implementing the default formatting for first-order logic here

#[cfg(test)]
mod tests {
    use crate::{formatting::fol::default::Format, syntax_tree::fol::BasicIntegerTerm};

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

    // TODO Zach: Add tests for the remaining formatters
}
