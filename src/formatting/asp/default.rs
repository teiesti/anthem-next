use {
    crate::syntax_tree::{
        asp::{BinaryOperator, Constant, Term, UnaryOperator, Variable},
        Node,
    },
    std::fmt::{self, Display, Formatter},
};

pub struct Format<'a, N: Node>(pub &'a N);

impl Display for Format<'_, Constant> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self.0 {
            Constant::Infimum => write!(f, "#inf"),
            Constant::Integer(n) => write!(f, "{n}"),
            Constant::Symbol(s) => write!(f, "{s}"),
            Constant::Supremum => write!(f, "#sup"),
        }
    }
}

impl Display for Format<'_, Variable> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self.0 {
            Variable::Anonymous => write!(f, "_"),
            Variable::Named(s) => write!(f, "{s}"),
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
            BinaryOperator::Divide => write!(f, "/"),
            BinaryOperator::Modulo => write!(f, "\\"),
            BinaryOperator::Interval => write!(f, ".."),
        }
    }
}

impl Format<'_, Term> {
    fn precedence(&self) -> usize {
        match self.0 {
            Term::Constant(Constant::Integer(1..)) => 1,
            Term::UnaryOperation {
                op: UnaryOperator::Negative,
                ..
            }
            | Term::Constant(_)
            | Term::Variable(_) => 0,
            Term::BinaryOperation {
                op: BinaryOperator::Multiply | BinaryOperator::Divide | BinaryOperator::Modulo,
                ..
            } => 2,
            Term::BinaryOperation {
                op: BinaryOperator::Add | BinaryOperator::Subtract,
                ..
            } => 3,
            Term::BinaryOperation {
                op: BinaryOperator::Interval,
                ..
            } => 4,
        }
    }
}

impl Display for Format<'_, Term> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self.0 {
            Term::Constant(c) => Format(c).fmt(f),
            Term::Variable(v) => Format(v).fmt(f),
            Term::UnaryOperation { op, arg } => {
                let op = Format(op);
                let arg = Format(arg.as_ref());

                op.fmt(f)?;
                if self.precedence() < arg.precedence() {
                    write!(f, "({arg})")
                } else {
                    write!(f, "{arg}")
                }
            }
            Term::BinaryOperation { op, lhs, rhs } => {
                let op = Format(op);
                let lhs = Format(lhs.as_ref());
                let rhs = Format(rhs.as_ref());

                if self.precedence() < lhs.precedence() {
                    write!(f, "({lhs})")
                } else {
                    write!(f, "{lhs}")
                }?;
                if *op.0 == BinaryOperator::Interval {
                    write!(f, "{op}")
                } else {
                    write!(f, " {op} ")
                }?;
                if self.precedence() <= rhs.precedence() {
                    write!(f, "({rhs})")
                } else {
                    write!(f, "{rhs}")
                }
            }
        }
    }
}

// TODO Tobias: Continue implementing the default formatting for ASP here

#[cfg(test)]
mod tests {
    use crate::{
        formatting::asp::default::Format,
        syntax_tree::asp::{BinaryOperator, Constant, Term, UnaryOperator, Variable},
    };

    #[test]
    fn format_constant() {
        assert_eq!(Format(&Constant::Infimum).to_string(), "#inf");
        assert_eq!(Format(&Constant::Integer(-1)).to_string(), "-1");
        assert_eq!(Format(&Constant::Integer(0)).to_string(), "0");
        assert_eq!(Format(&Constant::Integer(42)).to_string(), "42");
        assert_eq!(Format(&Constant::Symbol("a".into())).to_string(), "a");
        assert_eq!(Format(&Constant::Supremum).to_string(), "#sup");
    }

    #[test]
    fn format_variable() {
        assert_eq!(Format(&Variable::Anonymous).to_string(), "_");
        assert_eq!(Format(&Variable::Named("A".into())).to_string(), "A");
    }

    #[test]
    fn format_unary_operator() {
        assert_eq!(Format(&UnaryOperator::Negative).to_string(), "-");
    }

    #[test]
    fn format_binary_operator() {
        assert_eq!(Format(&BinaryOperator::Add).to_string(), "+");
        assert_eq!(Format(&BinaryOperator::Subtract).to_string(), "-");
        assert_eq!(Format(&BinaryOperator::Multiply).to_string(), "*");
        assert_eq!(Format(&BinaryOperator::Divide).to_string(), "/");
        assert_eq!(Format(&BinaryOperator::Modulo).to_string(), "\\");
        assert_eq!(Format(&BinaryOperator::Interval).to_string(), "..");
    }

    #[test]
    fn format_term() {
        assert_eq!(
            Format(&Term::Constant(Constant::Integer(42))).to_string(),
            "42"
        );

        assert_eq!(
            Format(&Term::Variable(Variable::Named("A".into()))).to_string(),
            "A"
        );

        assert_eq!(
            Format(&Term::BinaryOperation {
                op: BinaryOperator::Add,
                lhs: Term::Constant(Constant::Integer(1)).into(),
                rhs: Term::BinaryOperation {
                    op: BinaryOperator::Multiply,
                    lhs: Term::Constant(Constant::Integer(2)).into(),
                    rhs: Term::Constant(Constant::Integer(3)).into(),
                }
                .into(),
            })
            .to_string(),
            "1 + 2 * 3"
        );

        assert_eq!(
            Format(&Term::BinaryOperation {
                op: BinaryOperator::Multiply,
                lhs: Term::Constant(Constant::Integer(1)).into(),
                rhs: Term::BinaryOperation {
                    op: BinaryOperator::Add,
                    lhs: Term::Constant(Constant::Integer(2)).into(),
                    rhs: Term::Constant(Constant::Integer(3)).into(),
                }
                .into(),
            })
            .to_string(),
            "1 * (2 + 3)"
        );

        assert_eq!(
            Format(&Term::BinaryOperation {
                op: BinaryOperator::Add,
                lhs: Term::Constant(Constant::Integer(1)).into(),
                rhs: Term::BinaryOperation {
                    op: BinaryOperator::Add,
                    lhs: Term::Constant(Constant::Integer(2)).into(),
                    rhs: Term::Constant(Constant::Integer(3)).into(),
                }
                .into(),
            })
            .to_string(),
            "1 + (2 + 3)"
        );

        assert_eq!(
            Format(&Term::BinaryOperation {
                op: BinaryOperator::Add,
                lhs: Term::BinaryOperation {
                    op: BinaryOperator::Add,
                    lhs: Term::Constant(Constant::Integer(1)).into(),
                    rhs: Term::Constant(Constant::Integer(2)).into(),
                }
                .into(),
                rhs: Term::Constant(Constant::Integer(3)).into(),
            })
            .to_string(),
            "1 + 2 + 3"
        );
    }
}
