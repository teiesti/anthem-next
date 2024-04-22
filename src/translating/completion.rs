use crate::{
    convenience::unbox::{fol::UnboxedFormula, Unbox},
    syntax_tree::fol,
};

pub fn is_completable_formula(formula: fol::Formula) -> bool {
    split(formula).is_some()
}

fn split(formula: fol::Formula) -> Option<(fol::Formula, fol::AtomicFormula)> {
    if !formula.free_variables().is_empty() {
        return None;
    }

    match formula {
        fol::Formula::QuantifiedFormula {
            quantification:
                fol::Quantification {
                    quantifier: fol::Quantifier::Forall,
                    ..
                },
            formula,
        } => split_implication(*formula),
        formula => split_implication(formula),
    }
}

fn split_implication(formula: fol::Formula) -> Option<(fol::Formula, fol::AtomicFormula)> {
    match formula.unbox() {
        UnboxedFormula::BinaryFormula {
            connective: fol::BinaryConnective::Implication,
            lhs: f,
            rhs: g,
        }
        | UnboxedFormula::BinaryFormula {
            connective: fol::BinaryConnective::ReverseImplication,
            lhs: g,
            rhs: f,
        } => match g {
            fol::Formula::AtomicFormula(
                // TODO: What about fol::AtomicFormula::Truth?
                a @ fol::AtomicFormula::Falsity | a @ fol::AtomicFormula::Atom(_),
            ) => Some((f, a)),
            _ => None,
        },
        _ => None,
    }
}
