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
