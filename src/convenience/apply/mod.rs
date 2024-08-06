use crate::syntax_tree::fol::Formula;

pub trait Apply {
    /// Apply an operation `f` in post-order to each node of a tree
    fn apply(self, f: &mut impl FnMut(Self) -> Self) -> Self
    where
        Self: Sized;

    /// Apply a series of operations `fs` in post-order to each node of a tree
    ///
    /// This function will traverse the tree only once. Whenever a node is visited, the first operation is applied first.
    /// The remaining operations are also applied in this order.
    fn apply_all(self, fs: &mut Vec<Box<dyn FnMut(Self) -> Self>>) -> Self
    where
        Self: Sized,
    {
        let mut f = |mut node: Self| {
            for fi in fs.iter_mut() {
                node = fi(node);
            }
            node
        };
        self.apply(&mut f)
    }
}

impl Apply for Formula {
    fn apply(self, f: &mut impl FnMut(Self) -> Self) -> Self
    where
        Self: Sized,
    {
        let inner = match self {
            x @ Formula::AtomicFormula(_) => x,

            Formula::UnaryFormula {
                connective,
                formula,
            } => Formula::UnaryFormula {
                connective,
                formula: formula.apply(f).into(),
            },

            Formula::BinaryFormula {
                connective,
                lhs,
                rhs,
            } => Formula::BinaryFormula {
                connective,
                lhs: lhs.apply(f).into(),
                rhs: rhs.apply(f).into(),
            },

            Formula::QuantifiedFormula {
                quantification,
                formula,
            } => Formula::QuantifiedFormula {
                quantification,
                formula: formula.apply(f).into(),
            },
        };
        f(inner)
    }
}
