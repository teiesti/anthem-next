use crate::syntax_tree::fol::Theory;

pub fn simplify(theory: Theory, full: bool) -> Theory {
    crate::simplifying::fol::ht::simplify_theory(theory, full)
    // TODO: Add classic simplifications
}
