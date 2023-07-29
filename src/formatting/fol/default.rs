use {
    crate::syntax_tree::{Node, fol::BasicIntegerTerm, fol::IntegerTerm},
    std::fmt::{self, Display, Formatter},
};

pub struct Format<'a, N: Node>(pub &'a N);

impl Display for Format<'_, BasicIntegerTerm> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self.0 {
            BasicIntegerTerm::Infimum => write!(f, "#inf"),
            BasicIntegerTerm::Supremum => write!(f, "#sup"),
            BasicIntegerTerm::Numeral(n) => write!(f, "{n}"),
            BasicIntegerTerm::IntegerVariable(s) => write!(f, "{s}"),
        }
    }
}

impl Display for Format<'_, IntegerTerm> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "todo\n");
        todo!();
    }
}

// TODO Zach: Implement the default formatting for first-order logic here
