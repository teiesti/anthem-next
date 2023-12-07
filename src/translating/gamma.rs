use crate::{
    convenience::apply::Apply as _,
    syntax_tree::fol::{AtomicFormula, BinaryConnective, Formula, UnaryConnective},
};

pub fn gamma(formula: Formula) -> Formula {
    match formula {
        x @ Formula::AtomicFormula(_) => here(x),

        Formula::UnaryFormula {
            connective: connective @ UnaryConnective::Negation,
            formula,
        } => Formula::UnaryFormula {
            connective,
            formula: there(*formula).into(),
        },

        Formula::BinaryFormula {
            connective:
                connective @ BinaryConnective::Conjunction | connective @ BinaryConnective::Disjunction,
            lhs,
            rhs,
        } => Formula::BinaryFormula {
            connective,
            lhs: gamma(*lhs).into(),
            rhs: gamma(*rhs).into(),
        },

        Formula::BinaryFormula {
            connective: BinaryConnective::Implication,
            lhs,
            rhs,
        } => Formula::BinaryFormula {
            connective: BinaryConnective::Conjunction,
            lhs: Formula::BinaryFormula {
                connective: BinaryConnective::Implication,
                lhs: gamma(*lhs.clone()).into(),
                rhs: gamma(*rhs.clone()).into(),
            }
            .into(),
            rhs: Formula::BinaryFormula {
                connective: BinaryConnective::Implication,
                lhs: there(*lhs).into(),
                rhs: there(*rhs).into(),
            }
            .into(),
        },

        Formula::QuantifiedFormula {
            quantification,
            formula,
        } => Formula::QuantifiedFormula {
            quantification,
            formula: gamma(*formula).into(),
        },

        // TODO: Support reverse implication and equivalence
        _ => todo!(),
    }
}

fn here(formula: Formula) -> Formula {
    prepend_predicate(formula, "h")
}

fn there(formula: Formula) -> Formula {
    prepend_predicate(formula, "t")
}

fn prepend_predicate(formula: Formula, prefix: &'static str) -> Formula {
    formula.apply(&mut |formula| match formula {
        Formula::AtomicFormula(AtomicFormula::Atom(mut a)) => {
            a.predicate.insert_str(0, prefix);
            Formula::AtomicFormula(AtomicFormula::Atom(a))
        }
        x => x,
    })
}

#[cfg(test)]
mod tests {
    use super::gamma;

    #[test]
    fn test_simplify() {
        for (src, target) in [
            ("#true", "#true"),
            ("a", "ha"),
            ("X > 1", "X > 1"), // TODO: Is this correct?
            ("not a", "not ta"),
            ("a and not b", "ha and not tb"),
            ("a or not b", "ha or not tb"),
            ("a -> b", "(ha -> hb) and (ta -> tb)"),
            ("forall X p(X)", "forall X hp(X)"),
            ("exists X p(X)", "exists X hp(X)"),
        ] {
            assert_eq!(gamma(src.parse().unwrap()), target.parse().unwrap())
        }
    }
}
