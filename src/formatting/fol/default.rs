use {
    crate::syntax_tree::{fol::Primitive, Node},
    std::fmt::{self, Display, Formatter},
};

pub struct Format<'a, N: Node>(pub &'a N);

impl Display for Format<'_, Primitive> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self.0 {
            Primitive::Infimum => write!(f, "#inf"),
            Primitive::Supremum => write!(f, "#sup"),
        }
    }
}

// TODO Zach: Implement the default formatting for first-order logic here
