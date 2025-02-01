use crate::syntax_tree::fol::Theory;

pub fn simplify(theory: Theory) -> Theory {
    crate::simplifying::fol::intuitionistic::simplify(theory)
    // TODO: Add ht simplifications
}
