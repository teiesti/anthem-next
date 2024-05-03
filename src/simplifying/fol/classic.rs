use crate::syntax_tree::fol::Theory;

pub fn simplify(theory: Theory) -> Theory {
    crate::simplifying::fol::ht::simplify(theory)
    // TODO: Add classic simplifications
}
