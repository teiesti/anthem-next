pub mod formatting;
pub mod parsing;
pub mod syntax_tree;
pub mod translating;

use crate::syntax_tree::{asp, fol};
use std::collections::HashSet;
use std::collections::HashMap;
use std::collections::hash_map::Entry;

// If a sentence is completable, it returns the head
pub fn completable_beheader(sentence: fol::Formula) -> Option<fol::AtomicFormula> {
    match sentence {
        fol::Formula::QuantifiedFormula {
            quantification: q,
            formula: f
        } => {
            match q.quantifier {
                fol::Quantifier::Forall => {
                    match *f {
                        fol::Formula::BinaryFormula {
                            connective: c, lhs: _, rhs: f2 } => {
                                match c {
                                    fol::BinaryConnective::Implication => {
                                        match *f2 {
                                            fol::Formula::AtomicFormula(a) => {
                                                match a {
                                                    fol::AtomicFormula::Falsity => Some(a),
                                                    fol::AtomicFormula::Atom(_) => Some(a),
                                                    fol::AtomicFormula::Comparison(_) => None,
                                                }
                                            },
                                            _ => None,
                                        }
                                    },
                                    _ => None,
                                }
                            },
                        _ => None,
                    }
                },
                _ => None,
            }
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
                },
                _ => false,
            }
        },
        _ => false,
    }
}

// Returns a mapping from <theory> formulas to their heads if <theory> is completable
// Otherwise returns None
pub fn completable_theory(theory: fol::Theory) -> Option<HashMap::<fol::Formula, fol::AtomicFormula>> {
    if theory.formulas.len() > 0 {
        let mut formulas = Vec::<fol::Formula>::new();
        let mut rule_heads = Vec::<fol::AtomicFormula>::new();
        let mut body_vars_u = Vec::<fol::Variable>::new();
        for sentence in theory.formulas.iter() {
            formulas.push(sentence.clone());
            match completable_beheader(sentence.clone()) {
                Some(f) => {
                    rule_heads.push(f.clone());
                },
                None => {
                    return None;
                }
            }
        }
        let mut formula_heads = HashMap::<fol::Formula, fol::AtomicFormula>::new();
        for (i, s) in formulas.iter().enumerate() {
            formula_heads.insert(s.clone(), rule_heads[i].clone());
        }
        for (i, s1) in formulas.iter().enumerate() {
            let head1 = &formula_heads[s1];
            for (s2, head2) in formula_heads.iter() {
                if s1 != s2 && head_mismatch(head1, head2) {
                    return None
                }
            }
        }
        return Some(formula_heads);
    } else {
        return None;
    }
}

// Produces a map from each unique head to a vector of F_i formula bodies (definitions)
pub fn definitions(theory: &fol::Theory, formula_heads: &HashMap::<fol::Formula, fol::AtomicFormula>) -> HashMap::<fol::AtomicFormula, Vec<fol::Formula>> {
    let mut definitions = HashMap::<fol::AtomicFormula, Vec<fol::Formula>>::new();
    for sentence in theory.formulas.iter() {
        match sentence {
            fol::Formula::QuantifiedFormula { quantification: q, formula: f } => {
                match *f.clone() {
                    fol::Formula::BinaryFormula{ connective: c, lhs: body, rhs: head_formula } => {
                        match c {
                            fol::BinaryConnective::Implication => {
                                match *head_formula {
                                    fol::Formula::AtomicFormula(head) => {
                                        match definitions.entry(head) {
                                            Entry::Occupied(mut entry)  => { entry.get_mut().push(*body); },
                                            Entry::Vacant(entry)        => { entry.insert(vec!(*body)); },
                                        }
                                    },
                                    _ => {},
                                }
                            },
                            _ => {},
                        }
                    },
                    _ => {},
                }
            },
            _ => {},
        }
    }
    definitions
}

// What about facts? p(5) is actually an abbreviation for p(5) :- .
// <theory> must be a completable theory, so we know it has the form forall V (body -> head)
// <formula_heads[formula] = head(formula)>
pub fn completion(theory: fol::Theory, formula_heads: HashMap::<fol::Formula, fol::AtomicFormula>) -> fol::Theory {
    let definitions = definitions(&theory, &formula_heads);
    // Now we need the completed definitions of each unique head
    let mut completions = Vec::<fol::Formula>::new();
    for (formula, head) in formula_heads.iter() {
        match head {
            fol::AtomicFormula::Falsity => {                        // Every constraint gets its own "completed definition"
                completions.push(formula.clone());
            },
            _ => {},
        }
    }
    for (head, body_vec) in definitions.iter() {
        match head {
            fol::AtomicFormula::Falsity => {},
            _ => {                                                  // TODO distinguish between intensional and extensional predicate symbols
                let mut bodies = Vec::<fol::Formula>::new();
                for body in body_vec.iter() {
                    let body_vars = body.get_variables();
                    println!("{:?}", body_vars);
                    let qbod = fol::Formula::QuantifiedFormula{
                        quantification: fol::Quantification{
                            quantifier: fol::Quantifier::Exists,
                            variables: vec![fol::Variable{sort: fol::Sort::General, name: "TEMP".to_string(),}]  //body.get_free_variables(),
                        },
                        formula: body.clone().into(),
                    };
                    bodies.push(qbod);
                }
                let f1: fol::Formula = bodies.pop().unwrap();
                let full_body = translating::tau_star::disjoin((bodies, f1));
                let head_var_names = head.get_variables();
                let mut head_vars = Vec::<fol::Variable>::new();
                for name in head_var_names.iter() {
                    head_vars.push(fol::Variable{ sort: fol::Sort::General, name: name.to_string() });
                }
                let comp = fol::Formula::QuantifiedFormula{
                    quantification: fol::Quantification{
                        quantifier: fol::Quantifier::Forall,
                        variables: head_vars,
                    },
                    formula: fol::Formula::BinaryFormula{
                        connective: fol::BinaryConnective::Equivalence,
                        lhs: Box::new(fol::Formula::AtomicFormula(head.clone())),
                        rhs: full_body.into(),
                    }.into(),
                };
                completions.push(comp);
            },
        }
    }
    fol::Theory{ formulas: completions }
}

fn main() {
    let program: asp::Program = "q(X,a) :- p(X,3). q(X,Y). q(X,Y) :- t(X), t(Y).".parse().unwrap();
    //let program: asp::Program = "".parse().unwrap();

    println!("{program}");

    let theory: fol::Theory = translating::tau_star::tau_star_program(program);
    println!("{theory}");

    let completable = completable_theory(theory.clone());
    match completable {
        Some(m) => {
            for (key, val) in m.iter() {
                println!("{}", formatting::fol::default::Format(val));
            }
            println!("");
            let completion = completion(theory.clone(), m);
            for c in completion.formulas.iter() {
                println!("{}", formatting::fol::default::Format(c));
            }
        },
        None => {
            println!("uh oh");
        },
    }
}
