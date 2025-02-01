use crate::{
    convenience::{apply::Apply as _, compose::Compose as _},
    simplifying::fol::intuitionistic::INTUITIONISTIC,
    syntax_tree::fol::{Formula, Theory},
};

pub fn simplify(theory: Theory) -> Theory {
    Theory {
        formulas: theory.formulas.into_iter().map(simplify_formula).collect(),
    }
}

pub fn simplify_formula(formula: Formula) -> Formula {
    formula.apply(&mut INTUITIONISTIC.iter().chain(HT).compose())
}

pub const HT: &[fn(Formula) -> Formula] = &[];
