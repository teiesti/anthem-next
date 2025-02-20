use crate::{
    syntax_tree::fol,
    translating::completion::{components, has_head_mismatches},
};

pub fn ordered_completion(theory: fol::Theory) -> Option<fol::Theory> {
    let (definitions, constraints) = components(theory)?;

    if has_head_mismatches(&definitions) {
        return None;
    }

    // rule translations for each p, i.e.
    // forall X (p(X) <- disjoin(rule bodies of p(X)) )
    // this is just the normal completion but instead of equivalences using <-
    let rule_translations = definitions.clone().into_iter().map(|(p, bodies)| {
        let v = p.variables();
        fol::Formula::BinaryFormula {
            connective: fol::BinaryConnective::ReverseImplication,
            lhs: fol::Formula::AtomicFormula(p).into(),
            rhs: fol::Formula::disjoin(bodies.into_iter().map(|f_i| {
                let u_i = f_i.free_variables().difference(&v).cloned().collect();
                f_i.quantify(fol::Quantifier::Exists, u_i)
            }))
            .into(),
        }
        .quantify(fol::Quantifier::Forall, v.into_iter().collect())
    });

    // definition parts for each p, i.e.
    // forall X (disjoin(rule bodies of p(X) with order constraint) -> p(X))
    // this is the -> part of normal completion modified to include the order constraints
    let definitions_with_order = definitions.into_iter().map(|(p, bodies)| {
        let v = p.variables();
        match p {
            fol::AtomicFormula::Atom(ref head_atom) => fol::Formula::BinaryFormula {
                connective: fol::BinaryConnective::Implication,
                rhs: fol::Formula::disjoin(bodies.into_iter().map(|f_i| {
                    let u_i = f_i.free_variables().difference(&v).cloned().collect();
                    let f_i_with_order = conjoin_order_atoms(f_i, head_atom.clone());
                    f_i_with_order.quantify(fol::Quantifier::Exists, u_i)
                }))
                .into(),
                lhs: fol::Formula::AtomicFormula(p).into(),
            }
            .quantify(fol::Quantifier::Forall, v.into_iter().collect()),
            _ => unreachable!("definitions should only contain atoms as first component"),
        }
    });

    let mut formulas: Vec<_> = constraints
        .into_iter()
        .map(fol::Formula::universal_closure)
        .collect();
    formulas.extend(rule_translations);
    formulas.extend(definitions_with_order);

    Some(fol::Theory { formulas })
}

fn create_order_formula(p: fol::Atom, q: fol::Atom) -> fol::Formula {
    // creates the atomic formula less_p_q(xs, ys)
    // where p(xs) and q(ys) are the given atoms
    fol::Formula::AtomicFormula(fol::AtomicFormula::Atom(fol::Atom {
        predicate_symbol: format!("less_{}_{}", p.predicate_symbol, q.predicate_symbol),
        terms: p.terms.into_iter().chain(q.terms).collect(),
    }))
}

fn conjoin_order_atoms(formula: fol::Formula, head_atom: fol::Atom) -> fol::Formula {
    // replaces all positive atoms q(zs) in formula
    // (i.e. all q(zs) not in the scope of any negation) by
    //   q(zs) and less_q_p(zs, xs)
    // where p(xs) is head_atom
    match formula {
        fol::Formula::AtomicFormula(fol::AtomicFormula::Atom(ref q)) => {
            fol::Formula::BinaryFormula {
                connective: fol::BinaryConnective::Conjunction,
                rhs: create_order_formula(q.clone(), head_atom).into(),
                lhs: formula.into(),
            }
        }
        fol::Formula::AtomicFormula(_) => formula,
        fol::Formula::UnaryFormula {
            connective: fol::UnaryConnective::Negation,
            ..
        } => formula,
        fol::Formula::BinaryFormula {
            connective,
            lhs,
            rhs,
        } => fol::Formula::BinaryFormula {
            connective,
            lhs: conjoin_order_atoms(*lhs, head_atom.clone()).into(),
            rhs: conjoin_order_atoms(*rhs, head_atom).into(),
        },
        fol::Formula::QuantifiedFormula {
            quantification,
            formula,
        } => fol::Formula::QuantifiedFormula {
            quantification,
            formula: conjoin_order_atoms(*formula, head_atom).into(),
        },
    }
}

pub fn ordered_completion_axioms(theory: fol::Theory) -> fol::Theory {
    fn get_general_variables(l: usize, u: usize) -> Vec<fol::GeneralTerm> {
        // l and u are lower and upper bound for index of general variables
        // i.e. return general variables Xl, ..., Xu
        (l..=u)
            .map(|i| fol::GeneralTerm::Variable(format!("X{i}")))
            .collect()
    }

    fn irreflexivity_axiom(p: fol::Predicate) -> fol::Formula {
        // turn predicate p into atom p(xs)
        let p_atom = fol::Atom {
            predicate_symbol: p.symbol,
            terms: get_general_variables(1, p.arity),
        };

        // not less_p_p(xs, xs)
        let formula = fol::Formula::UnaryFormula {
            connective: fol::UnaryConnective::Negation,
            formula: create_order_formula(p_atom.clone(), p_atom).into(),
        };
        let variables = formula.free_variables().into_iter().collect();

        formula.quantify(fol::Quantifier::Forall, variables)
    }

    fn transitivity_axiom(p: fol::Predicate, q: fol::Predicate, r: fol::Predicate) -> fol::Formula {
        // turn p, q, r into atoms
        // variables of the three atoms need to distinct
        // to do so the variable index goes from 1 to p.arity + q.arity + r.arity
        let p_atom = fol::Atom {
            predicate_symbol: p.symbol,
            terms: get_general_variables(1, p.arity),
        };
        let q_atom = fol::Atom {
            predicate_symbol: q.symbol,
            terms: get_general_variables(p.arity + 1, p.arity + q.arity),
        };
        let r_atom = fol::Atom {
            predicate_symbol: r.symbol,
            terms: get_general_variables(p.arity + q.arity + 1, p.arity + q.arity + r.arity),
        };

        // (less_p_q(xs, ys) and less_q_r(ys, zs)) -> less_p_r(xs, zs)
        let formula = fol::Formula::BinaryFormula {
            connective: fol::BinaryConnective::Implication,
            lhs: Box::new(fol::Formula::BinaryFormula {
                connective: fol::BinaryConnective::Conjunction,
                lhs: create_order_formula(p_atom.clone(), q_atom.clone()).into(),
                rhs: create_order_formula(q_atom, r_atom.clone()).into(),
            }),
            rhs: create_order_formula(p_atom, r_atom).into(),
        };
        let variables = formula.free_variables().into_iter().collect();

        formula.quantify(fol::Quantifier::Forall, variables)
    }

    // reflexivity for each predicate
    let mut axioms: Vec<_> = theory
        .predicates()
        .into_iter()
        .map(irreflexivity_axiom)
        .collect();

    // transitivity for each tuple (p, q, r)
    for p in theory.predicates() {
        for q in theory.predicates() {
            for r in theory.predicates() {
                axioms.push(transitivity_axiom(p.clone(), q.clone(), r))
            }
        }
    }

    fol::Theory { formulas: axioms }
}

#[cfg(test)]
mod tests {
    use super::{ordered_completion, ordered_completion_axioms};
    use crate::{syntax_tree::fol, translating::tau_star::tau_star};

    #[test]
    fn test_ordered_completion() {
        for (src, target) in [
            ("p :- q.", "p <- q. p -> q and less_q_p."),
            ("p(X) :- q.", "forall V1 (p(V1) <- exists X (V1 = X and q)). forall V1 (p(V1) -> exists X (V1 = X and (q and less_q_p(V1))))."),
            ("p(X) :- q(X).", "forall V1 (p(V1) <- exists X (V1 = X and exists Z (Z = X and q(Z)))). forall V1 (p(V1) -> exists X (V1 = X and exists Z (Z = X and (q(Z) and less_q_p(Z, V1)))))."),
            ("p(X) :- q(X). p(X) :- not r(X).", "forall V1 (p(V1) <- exists X (V1 = X and exists Z (Z = X and q(Z))) or exists X (V1 = X and exists Z (Z = X and not r(Z)))). forall V1 (p(V1) -> exists X (V1 = X and exists Z (Z = X and (q(Z) and less_q_p(Z, V1)))) or exists X (V1 = X and exists Z (Z = X and not r(Z))))."),
            ("p(X) :- q(X-1). {p(X)} :- r(X,Y).", "forall V1 (p(V1) <- exists X (V1 = X and exists Z (exists I$i J$i (Z = I$i - J$i and I$i = X and J$i = 1) and q(Z))) or exists X Y (V1 = X and exists Z Z1 (Z = X and Z1 = Y and r(Z, Z1)) and not not p(V1))). forall V1 (p(V1) -> exists X (V1 = X and exists Z (exists I$i J$i (Z = I$i - J$i and I$i = X and J$i = 1) and (q(Z) and less_q_p(Z, V1)))) or exists X Y (V1 = X and exists Z Z1 (Z = X and Z1 = Y and (r(Z, Z1) and less_r_p(Z, Z1, V1))) and not not p(V1)))."),
            ("t(X,Y) :- e(X,Y). t(X,Z) :- e(X,Y), t(Y,Z).", "forall V1 V2 (t(V1, V2) <- exists X Y (V1 = X and V2 = Y and exists Z Z1 (Z = X and Z1 = Y and e(Z, Z1))) or exists X Z Y (V1 = X and V2 = Z and (exists Z Z1 (Z = X and Z1 = Y and e(Z, Z1)) and exists Z1 Z2 (Z1 = Y and Z2 = Z and t(Z1, Z2))))). forall V1 V2 (t(V1, V2) -> exists X Y (V1 = X and V2 = Y and exists Z Z1 (Z = X and Z1 = Y and (e(Z, Z1) and less_e_t(Z, Z1, V1, V2)))) or exists X Z Y (V1 = X and V2 = Z and (exists Z Z1 (Z = X and Z1 = Y and (e(Z, Z1) and less_e_t(Z, Z1, V1, V2))) and exists Z1 Z2 (Z1 = Y and Z2 = Z and (t(Z1, Z2) and less_t_t(Z1, Z2, V1, V2))))))."),
        ] {
            let left = ordered_completion(tau_star(src.parse().unwrap())).unwrap();
            let right = target.parse().unwrap();

            assert!(
                left == right,
                "assertion `left == right` failed:\n left:\n{left}\n right:\n{right}"
            );
        }
    }

    #[test]
    fn test_ordered_completion_incompletable() {
        for theory in [
            "forall X (p(X, a) <- q(X)).",
            "forall X (p(X, X) <- q(X)).",
            "forall X (p(X) <- q(X,Y)).",
            "forall V1 V2 (p(V1, V2) <- t). forall V1 X (p(V1, X) <- q).",
        ] {
            let theory: fol::Theory = theory.parse().unwrap();
            assert!(
                ordered_completion(theory.clone()).is_none(),
                "`{theory}` should not be completable"
            );
        }
    }

    #[test]
    fn test_ordered_completion_axioms() {
        for (src, target) in [
            ("p :- p.", "not less_p_p. less_p_p and less_p_p -> less_p_p."),
            (
                "p(X) :- q.",
                "not less_q_q. forall X1 not less_p_p(X1, X1). less_q_q and less_q_q -> less_q_q. forall X1 (less_q_q and less_q_p(X1) -> less_q_p(X1)). forall X1 (less_q_p(X1) and less_p_q(X1) -> less_q_q). forall X1 X2 (less_q_p(X1) and less_p_p(X1, X2) -> less_q_p(X2)). forall X1 (less_p_q(X1) and less_q_q -> less_p_q(X1)). forall X1 X2 (less_p_q(X1) and less_q_p(X2) -> less_p_p(X1, X2)). forall X1 X2 (less_p_p(X1, X2) and less_p_q(X2) -> less_p_q(X1)). forall X1 X2 X3 (less_p_p(X1, X2) and less_p_p(X2, X3) -> less_p_p(X1, X3)).",
            ),
        ] {
            let left = ordered_completion_axioms(tau_star(src.parse().unwrap()));
            let right = target.parse().unwrap();

            assert!(
                left == right,
                "assertion `left == right` failed:\n left:\n{left}\n right:\n{right}"
            );
        }
    }
}
