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

impl Display for Format<'_, Term> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

// TODO Tobias: Continue implementing the default formatting for ASP here

#[cfg(test)]
mod tests {
    use crate::{
        formatting::asp::default::Format,
        syntax_tree::asp::{BinaryOperator, Constant, UnaryOperator, Variable},
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
}
