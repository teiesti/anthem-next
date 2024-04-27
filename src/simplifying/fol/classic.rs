use crate::syntax_tree::fol::Formula;

pub fn simplify(formula: Formula) -> Formula {
    crate::simplifying::fol::ht::simplify(formula)
    // TODO: Add classic simplifications
}
