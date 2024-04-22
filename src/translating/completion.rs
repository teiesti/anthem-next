use {
    crate::{
        convenience::unbox::{fol::UnboxedFormula, Unbox},
        syntax_tree::fol,
    },
    indexmap::{map::Entry, IndexMap},
};

pub fn completion(theory: fol::Theory) -> Option<fol::Theory> {
    let definitions = definitions(theory)?;

    // TODO: Take care for constraints
    // TODO: Confirm there are not head mismatches

    let formulas = definitions
        .into_iter()
        .map(|(g, f)| {
            let v = g.variables();
            fol::Formula::BinaryFormula {
                connective: fol::BinaryConnective::Equivalence,
                lhs: fol::Formula::AtomicFormula(g).into(),
                rhs: fol::Formula::disjoin(f.into_iter().map(|f_i| {
                    let variables = f_i.free_variables().difference(&v).cloned().collect();
                    f_i.quantify(fol::Quantifier::Exists, variables)
                }))
                .into(),
            }
            .quantify(fol::Quantifier::Forall, v.into_iter().collect())
        })
        .collect();

    Some(fol::Theory { formulas })
}

fn definitions(theory: fol::Theory) -> Option<IndexMap<fol::AtomicFormula, Vec<fol::Formula>>> {
    let mut result: IndexMap<_, Vec<fol::Formula>> = IndexMap::new();
    for formula in theory.formulas {
        let (f, g) = split(formula)?;
        match result.entry(g) {
            Entry::Occupied(mut e) => {
                e.get_mut().push(f);
            }
            Entry::Vacant(e) => {
                e.insert(vec![f]);
            }
        };
    }
    Some(result)
}

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
