use crate::simplifying::fol::ht;
use crate::syntax_tree::fol;

use std::collections::hash_map::Entry;
use std::collections::HashMap;

// If a sentence is completable, it returns the head
pub fn completable_beheader(sentence: fol::Formula) -> Option<fol::AtomicFormula> {
    match sentence {
        fol::Formula::QuantifiedFormula {
            quantification: q,
            formula: f,
        } => match q.quantifier {
            fol::Quantifier::Forall => match *f {
                fol::Formula::BinaryFormula {
                    connective: c,
                    lhs: f1,
                    rhs: f2,
                } => match c {
                    fol::BinaryConnective::Implication => match *f2 {
                        fol::Formula::AtomicFormula(a) => match a {
                            fol::AtomicFormula::Truth | fol::AtomicFormula::Falsity => Some(a),
                            fol::AtomicFormula::Atom(_) => Some(a),
                            fol::AtomicFormula::Comparison(_) => None,
                        },
                        _ => None,
                    },
                    fol::BinaryConnective::ReverseImplication => match *f1 {
                        fol::Formula::AtomicFormula(a) => match a {
                            fol::AtomicFormula::Truth | fol::AtomicFormula::Falsity => Some(a),
                            fol::AtomicFormula::Atom(_) => Some(a),
                            fol::AtomicFormula::Comparison(_) => None,
                        },
                        _ => None,
                    },
                    _ => None,
                },
                _ => None,
            },
            _ => None,
        },
        fol::Formula::BinaryFormula {
            connective: c,
            lhs: f1,
            rhs: f2,
        } => match c {
            fol::BinaryConnective::Implication => match *f2 {
                fol::Formula::AtomicFormula(a) => match a {
                    fol::AtomicFormula::Truth | fol::AtomicFormula::Falsity => Some(a),
                    fol::AtomicFormula::Atom(_) => Some(a),
                    fol::AtomicFormula::Comparison(_) => None,
                },
                _ => None,
            },
            fol::BinaryConnective::ReverseImplication => match *f1 {
                fol::Formula::AtomicFormula(a) => match a {
                    fol::AtomicFormula::Truth | fol::AtomicFormula::Falsity => Some(a),
                    fol::AtomicFormula::Atom(_) => Some(a),
                    fol::AtomicFormula::Comparison(_) => None,
                },
                _ => None,
            },
            _ => None,
        },
        _ => None,
    }
}

// Returns true if s1 and s2 have the same predicate symbol p/n in the head
// but head(s1) != head(s2)
pub fn head_mismatch(s1: &fol::AtomicFormula, s2: &fol::AtomicFormula) -> bool {
    match s1 {
        fol::AtomicFormula::Atom(a1) => {
            let p1 = &a1.predicate;
            let n1 = a1.terms.len();
            match s2 {
                fol::AtomicFormula::Atom(a2) => {
                    let p2 = &a2.predicate;
                    let n2 = a2.terms.len();
                    if p1 == p2 && n1 == n2 {
                        if s1 == s2 {
                            false
                        } else {
                            true
                        }
                    } else {
                        false
                    }
                }
                _ => false,
            }
        }
        _ => false,
    }
}

// Returns a mapping from <theory> formulas to their heads if <theory> is completable
// Otherwise returns None
pub fn completable_theory(
    theory: fol::Theory,
) -> Option<HashMap<fol::Formula, fol::AtomicFormula>> {
    if theory.formulas.len() > 0 {
        let mut formulas = Vec::<fol::Formula>::new();
        let mut rule_heads = Vec::<fol::AtomicFormula>::new();
        //let mut body_vars_u = Vec::<fol::Variable>::new();
        for sentence in theory.formulas.iter() {
            formulas.push(sentence.clone());
            match completable_beheader(sentence.clone()) {
                Some(f) => {
                    rule_heads.push(f.clone());
                }
                None => {
                    return None;
                }
            }
        }
        let mut formula_heads = HashMap::<fol::Formula, fol::AtomicFormula>::new();
        for (i, s) in formulas.iter().enumerate() {
            formula_heads.insert(s.clone(), rule_heads[i].clone());
        }
        for s1 in formulas.iter() {
            let head1 = &formula_heads[s1];
            for (s2, head2) in formula_heads.iter() {
                if s1 != s2 && head_mismatch(head1, head2) {
                    return None;
                }
            }
        }
        return Some(formula_heads);
    } else {
        return None;
    }
}

// Create a map from each unique head to a vector of F_i formula bodies (definitions)
pub fn definitions(theory: &fol::Theory) -> HashMap<fol::AtomicFormula, Vec<fol::Formula>> {
    let mut definitions = HashMap::<fol::AtomicFormula, Vec<fol::Formula>>::new();
    for sentence in theory.formulas.iter() {
        match sentence {
            fol::Formula::QuantifiedFormula {
                quantification: _,
                formula: f,
            } => match *f.clone() {
                fol::Formula::BinaryFormula {
                    connective: c,
                    lhs: f1,
                    rhs: f2,
                } => match c {
                    fol::BinaryConnective::Implication => match *f2 {
                        fol::Formula::AtomicFormula(head) => match definitions.entry(head) {
                            Entry::Occupied(mut entry) => {
                                entry.get_mut().push(*f1);
                            }
                            Entry::Vacant(entry) => {
                                entry.insert(vec![*f1]);
                            }
                        },
                        _ => {}
                    },
                    fol::BinaryConnective::ReverseImplication => match *f1 {
                        fol::Formula::AtomicFormula(head) => match definitions.entry(head) {
                            Entry::Occupied(mut entry) => {
                                entry.get_mut().push(*f2);
                            }
                            Entry::Vacant(entry) => {
                                entry.insert(vec![*f2]);
                            }
                        },
                        _ => {}
                    },
                    _ => {}
                },
                _ => {}
            },
            fol::Formula::BinaryFormula {
                connective: c,
                lhs: f1,
                rhs: f2,
            } => match c {
                fol::BinaryConnective::Implication => match *(*f2).clone() {
                    fol::Formula::AtomicFormula(head) => match definitions.entry(head) {
                        Entry::Occupied(mut entry) => {
                            entry.get_mut().push((**f1).clone());
                        }
                        Entry::Vacant(entry) => {
                            entry.insert(vec![(**f1).clone()]);
                        }
                    },
                    _ => {}
                },
                fol::BinaryConnective::ReverseImplication => match *(*f1).clone() {
                    fol::Formula::AtomicFormula(head) => match definitions.entry(head) {
                        Entry::Occupied(mut entry) => {
                            entry.get_mut().push((**f2).clone());
                        }
                        Entry::Vacant(entry) => {
                            entry.insert(vec![(**f2).clone()]);
                        }
                    },
                    _ => {}
                },
                _ => {}
            },
            _ => {}
        }
    }
    definitions
}

// <theory> must be a completable theory, so we know it has the form forall V (body -> head) OR body -> head
// <formula_heads[formula] = head(formula)>
pub fn completion(
    theory: fol::Theory,
    formula_heads: HashMap<fol::Formula, fol::AtomicFormula>,
) -> fol::Theory {
    let definitions = definitions(&theory);
    let mut completions = Vec::<fol::Formula>::new(); // Now we need the completed definitions of each unique head
    for (formula, head) in formula_heads.iter() {
        match head {
            fol::AtomicFormula::Falsity => {
                // Every constraint gets its own "completed definition"
                completions.push(formula.clone());
            }
            _ => {}
        }
    }
    for (head, body_vec) in definitions.iter() {
        // p(V), { Fi }
        match head {
            fol::AtomicFormula::Falsity => {}
            _ => {
                // TODO distinguish between intensional and extensional predicate symbols
                let head_vars = head.variables();
                let mut bodies = Vec::<fol::Formula>::new();
                for body in body_vec.iter() {
                    let mut free_vars = Vec::<fol::Variable>::new();
                    for var in body.variables().iter() {
                        // Ui (Ui are free variables in Fi that do not belong to V)
                        if body.contains_free_variable(&var) && !head_vars.contains(&var) {
                            free_vars.push(var.clone());
                        }
                    }
                    if free_vars.len() > 0 {
                        let qbod = fol::Formula::QuantifiedFormula {
                            quantification: fol::Quantification {
                                quantifier: fol::Quantifier::Exists,
                                variables: free_vars,
                            },
                            formula: body.clone().into(),
                        };
                        bodies.push(qbod);
                    } else {
                        bodies.push(body.clone());
                    }
                }
                //let f1 = bodies.pop().unwrap();
                let full_body = ht::simplify(super::tau_star::disjoin(bodies));
                let comp = match head_vars.len() {
                    0 => fol::Formula::BinaryFormula {
                        connective: fol::BinaryConnective::Equivalence,
                        lhs: fol::Formula::AtomicFormula(head.clone()).into(),
                        rhs: full_body.into(),
                    },
                    _ => fol::Formula::QuantifiedFormula {
                        quantification: fol::Quantification {
                            quantifier: fol::Quantifier::Forall,
                            variables: Vec::from_iter(head_vars),
                        },
                        formula: fol::Formula::BinaryFormula {
                            connective: fol::BinaryConnective::Equivalence,
                            lhs: Box::new(fol::Formula::AtomicFormula(head.clone())),
                            rhs: full_body.into(),
                        }
                        .into(),
                    },
                };
                completions.push(comp);
            }
        }
    }
    fol::Theory {
        formulas: completions,
    }
}

#[cfg(test)]
mod tests {
    use crate::formatting;
    use crate::simplifying::fol::ht;
    use crate::translating::completion;
    use crate::{syntax_tree::asp, syntax_tree::fol};

    #[test]
    pub fn simplify_test1() {
        let f1: fol::Formula = "forall X (q(X) or (p or #true))".parse().unwrap();
        let src = ht::simplify(f1);
        let dest: fol::Formula = "forall X #true".parse().unwrap();
        assert_eq!(
            format!("{}", formatting::fol::default::Format(&src)),
            format!("{}", formatting::fol::default::Format(&dest))
        );
    }

    #[test]
    pub fn simplify_test2() {
        let f1: fol::Formula = "forall X ((#true and p and q(X)) or (p or #true))"
            .parse()
            .unwrap();
        let src = ht::simplify(f1);
        let dest: fol::Formula = "forall X #true".parse().unwrap();
        assert_eq!(
            format!("{}", formatting::fol::default::Format(&src)),
            format!("{}", formatting::fol::default::Format(&dest))
        );
    }

    #[test]
    pub fn completion_test1() {
        let program: asp::Program = "p(X) :- q(X).".parse().unwrap();
        let theory = crate::translating::tau_star::tau_star_program(program);
        let target: fol::Theory =
            "forall V1 (p(V1) <-> exists X (V1 = X and exists Z (Z = X and q(Z))))."
                .parse()
                .unwrap();

        let completable = completion::completable_theory(theory.clone());
        match completable {
            Some(m) => {
                let completion = completion::completion(theory.clone(), m);
                assert_eq!(
                    format!("{}", formatting::fol::default::Format(&completion)),
                    format!("{}", formatting::fol::default::Format(&target))
                )
            }
            None => {
                assert!(false)
            }
        }
        assert!(true)
    }

    #[test]
    pub fn completion_test2() {
        let program: asp::Program = "p(a). p(b). q(X,Y) :- p(X), p(Y).".parse().unwrap();
        let theory = crate::translating::tau_star::tau_star_program(program);

        let f1: fol::Formula = "forall V1 (p(V1) <-> V1 = a or V1 = b)".parse().unwrap();
        let f2: fol::Formula = "forall V1 V2 (q(V1, V2) <-> exists X Y (V1 = X and V2 = Y and (exists Z (Z = X and p(Z)) and exists Z (Z = Y and p(Z)))))".parse().unwrap();

        let target = fol::Theory {
            formulas: vec![f1, f2],
        };

        let completable = completion::completable_theory(theory.clone());
        match completable {
            Some(m) => {
                let completion = completion::completion(theory.clone(), m);
                assert!(completion.identical(&target))
            }
            None => {
                assert!(false)
            }
        }
    }

    #[test]
    pub fn completion_test3() {
        let program: asp::Program = "{p(X+1)} :- q(X).".parse().unwrap();
        let theory = crate::translating::tau_star::tau_star_program(program);
        let target: fol::Theory = "forall V1 (p(V1) <-> exists X (exists I$i J$i (V1 = I$i + J$i and I$i = X and J$i = 1) and exists Z (Z = X and q(Z)) and not not p(V1))).".parse().unwrap();

        let completable = completion::completable_theory(theory.clone());
        match completable {
            Some(m) => {
                let completion = completion::completion(theory.clone(), m);
                assert!(completion.identical(&target))
            }
            None => {
                assert!(false)
            }
        }
    }

    #[test]
    pub fn completion_test4() {
        let program: asp::Program = "r(X) :- q(X). r(G,Y) :- G < Y. r(a).".parse().unwrap();
        let theory = crate::translating::tau_star::tau_star_program(program);
        let f1: fol::Formula =
            "forall V1 (r(V1) <-> exists X (V1 = X and exists Z (Z = X and q(Z))) or V1 = a)"
                .parse()
                .unwrap();
        let f2: fol::Formula = "forall V1 V2 (r(V1,V2) <-> exists G Y (V1 = G and V2 = Y and exists Z Z1 (Z = G and Z1 = Y and Z < Z1) ) )".parse().unwrap();
        let target = fol::Theory {
            formulas: vec![f1, f2],
        };

        let completable = completion::completable_theory(theory.clone());
        match completable {
            Some(m) => {
                let completion = completion::completion(theory.clone(), m);
                assert!(completion.identical(&target))
            }
            None => {
                assert!(false)
            }
        }
    }

    #[test]
    pub fn completion_test5() {
        let program: asp::Program =
            "composite(I*J) :- I>1, J>1. prime(I) :- I = 2..n, not composite(I)."
                .parse()
                .unwrap();
        let theory = crate::translating::tau_star::tau_star_program(program);
        let f1: fol::Formula = "forall V1 (composite(V1) <-> exists I J (exists I1$i J1$i (V1 = I1$i * J1$i and I1$i = I and J1$i = J) and (exists Z Z1 (Z = I and Z1 = 1 and Z > Z1) and exists Z Z1 (Z = J and Z1 = 1 and Z > Z1))))".parse().unwrap();
        let f2: fol::Formula = "forall V1 (prime(V1) <-> exists I (V1 = I and (exists Z Z1 (Z = I and exists I$i J$i K$i (I$i = 2 and J$i = n and Z1 = K$i and I$i <= K$i <= J$i) and Z = Z1) and exists Z (Z = I and not composite(Z)))))".parse().unwrap();
        let target = fol::Theory {
            formulas: vec![f1, f2],
        };

        let completable = completion::completable_theory(theory.clone());
        match completable {
            Some(m) => {
                let completion = completion::completion(theory.clone(), m);
                assert!(completion.identical(&target))
            }
            None => {
                assert!(false)
            }
        }
    }
}
