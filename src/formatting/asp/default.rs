use {
    crate::syntax_tree::{
        asp::{Constant, Variable, UnaryOperator, BinaryOperator, Term},
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
            BinaryOperator::Interval => write!(f, "*"),
        }
    }
}

impl Display for Format<'_, Term> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

// TODO Tobias: Continue implementing the default formatting for ASP here
