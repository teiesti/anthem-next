use {
    crate::{
        convenience::unbox::{fol::UnboxedFormula, Unbox},
        syntax_tree::fol,
    },
    indexmap::{map::Entry, IndexMap},
};

pub fn completion(theory: fol::Theory) -> Option<fol::Theory> {
    let (definitions, constraints) = components(theory)?;
    // TODO: Confirm there are not head mismatches

    let completed_definitions = definitions.into_iter().map(|(g, a)| {
        let v = g.variables();
        fol::Formula::BinaryFormula {
            connective: fol::BinaryConnective::Equivalence,
            lhs: fol::Formula::AtomicFormula(g).into(),
            rhs: fol::Formula::disjoin(a.into_iter().map(|f_i| {
                let u_i = f_i.free_variables().difference(&v).cloned().collect();
                f_i.quantify(fol::Quantifier::Exists, u_i)
            }))
            .into(),
        }
        .quantify(fol::Quantifier::Forall, v.into_iter().collect())
    });

    let mut formulas: Vec<_> = constraints;
    formulas.extend(completed_definitions);

    Some(fol::Theory { formulas })
}

fn components(theory: fol::Theory) -> Option<(Definitions, Constraints)> {
    let mut definitions: Definitions = IndexMap::new();
    let mut constraints = Vec::new();

    for formula in theory.formulas {
        match split(formula)? {
            Component::Constraint(c) => constraints.push(c),
            Component::PartialDefinition { f, a } => match definitions.entry(a) {
                Entry::Occupied(mut e) => {
                    e.get_mut().push(f);
                }
                Entry::Vacant(e) => {
                    e.insert(vec![f]);
                }
            },
        }
    }

    Some((definitions, constraints))
}

type Definitions = IndexMap<fol::AtomicFormula, Vec<fol::Formula>>;
type Constraints = Vec<fol::Formula>;

fn split(formula: fol::Formula) -> Option<Component> {
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

fn split_implication(formula: fol::Formula) -> Option<Component> {
    match formula.clone().unbox() {
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
            // TODO: What about fol::AtomicFormula::Truth?
            fol::Formula::AtomicFormula(fol::AtomicFormula::Falsity) => {
                Some(Component::Constraint(formula))
            }
            fol::Formula::AtomicFormula(a @ fol::AtomicFormula::Atom(_)) => {
                Some(Component::PartialDefinition { f, a })
            }
            _ => None,
        },
        _ => None,
    }
}

enum Component {
    PartialDefinition {
        f: fol::Formula,
        a: fol::AtomicFormula,
    },
    Constraint(fol::Formula),
}

#[cfg(test)]
mod tests {
    use crate::translating::{completion::completion, tau_star::tau_star};

    #[test]
    fn test_completion() {
        for (src, target) in [
            ("p(X) :- q(X).", "forall V1 (p(V1) <-> exists X (V1 = X and exists Z (Z = X and q(Z))))."),
            ("p(a). p(b). q(X,Y) :- p(X), p(Y).", "forall V1 (p(V1) <-> V1 = a and #true or V1 = b and #true). forall V1 V2 (q(V1, V2) <-> exists X Y (V1 = X and V2 = Y and (exists Z (Z = X and p(Z)) and exists Z (Z = Y and p(Z)))))."),
            ("{p(X+1)} :- q(X).", "forall V1 (p(V1) <-> exists X (exists I$i J$i (V1 = I$i + J$i and I$i = X and J$i = 1) and exists Z (Z = X and q(Z)) and not not p(V1)))."),
            ("r(X) :- q(X). r(G,Y) :- G < Y. r(a).", "forall V1 (r(V1) <-> exists X (V1 = X and exists Z (Z = X and q(Z))) or V1 = a and #true). forall V1 V2 (r(V1,V2) <-> exists G Y (V1 = G and V2 = Y and exists Z Z1 (Z = G and Z1 = Y and Z < Z1) ) )."),
            ("composite(I*J) :- I>1, J>1. prime(I) :- I = 2..n, not composite(I).", "forall V1 (composite(V1) <-> exists I J (exists I1$i J1$i (V1 = I1$i * J1$i and I1$i = I and J1$i = J) and (exists Z Z1 (Z = I and Z1 = 1 and Z > Z1) and exists Z Z1 (Z = J and Z1 = 1 and Z > Z1)))). forall V1 (prime(V1) <-> exists I (V1 = I and (exists Z Z1 (Z = I and exists I$i J$i K$i (I$i = 2 and J$i = n and Z1 = K$i and I$i <= K$i <= J$i) and Z = Z1) and exists Z (Z = I and not composite(Z)))))."),
            ("p :- q, not t. p :- r. r :- t.", "p <-> (q and not t) or (r). r <-> t."),
            ("p. p(a). :- q.", "q -> #false. p <-> #true. forall V1 (p(V1) <-> V1 = a and #true)."),
        ] {
            let left = completion(tau_star(src.parse().unwrap())).unwrap();
            let right = target.parse().unwrap();

            assert!(
                left == right,
                "assertion `left == right` failed:\n left:\n{left}\n right:\n{right}"
            );
        }
    }
}
