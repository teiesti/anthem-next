use crate::{
    convenience::{
        apply::Apply as _,
        unbox::{fol::UnboxedFormula, Unbox},
    },
    syntax_tree::fol::{Formula, Theory, UnaryConnective},
};

pub fn simplify(theory: Theory) -> Theory {
    simplify_classic(crate::simplifying::fol::ht::simplify(theory))
    // TODO: Add classic simplifications
}

pub fn simplify_classic(theory: Theory) -> Theory {
    Theory {
        formulas: theory.formulas.into_iter().map(simplify_formula).collect(),
    }
}

fn simplify_formula(formula: Formula) -> Formula {
    formula.apply_all(&mut vec![Box::new(eliminate_double_negation)])
}

fn eliminate_double_negation(formula: Formula) -> Formula {
    // Remove double negation
    // e.g. not not F => F

    match formula.unbox() {
        UnboxedFormula::UnaryFormula {
            connective: UnaryConnective::Negation,
            formula:
                Formula::UnaryFormula {
                    connective: UnaryConnective::Negation,
                    formula: inner,
                },
        } => *inner,

        x => x.rebox(),
    }
}

#[cfg(test)]
mod tests {
    use {
        super::{eliminate_double_negation, simplify_formula},
        crate::{convenience::apply::Apply as _, syntax_tree::fol::Formula},
    };

    #[test]
    fn test_simplify() {
        for (src, target) in [("not not forall X p(X)", "forall X p(X)")] {
            assert_eq!(
                simplify_formula(src.parse().unwrap()),
                target.parse().unwrap()
            )
        }
    }

    #[test]
    fn test_eliminate_double_negation() {
        for (src, target) in [("not not a", "a")] {
            assert_eq!(
                src.parse::<Formula>()
                    .unwrap()
                    .apply(&mut eliminate_double_negation),
                target.parse().unwrap()
            )
        }
    }
}
