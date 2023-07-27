use {
    crate::syntax_tree::{asp::Constant, Node},
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

// TODO Tobias: Continue implementing the default formatting for ASP here
