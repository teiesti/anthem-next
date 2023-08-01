use {
    crate::syntax_tree::{
        fol::{BasicIntegerTerm, BinaryOperator, GeneralTerm, IntegerTerm, UnaryOperator},
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

// TODO Zach: Implement the default formatting for first-order logic here

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
