use crate::syntax_tree::fol::Formula;

pub trait Apply {
    fn apply(self, f: &mut impl FnMut(Self) -> Self) -> Self
    where
        Self: Sized;
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
