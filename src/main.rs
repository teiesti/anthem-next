pub mod formatting;
pub mod parsing;
pub mod syntax_tree;
pub mod translating;

use crate::syntax_tree::{asp, fol};
//use std::collections::HashSet;
//use std::collections::hash_map::Entry;

// If a sentence is completable, it returns the head
/*pub fn completable_beheader(sentence: fol::Formula) -> Option<fol::AtomicFormula> {
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


// What about facts? p(5) is actually an abbreviation for p(5) :- .
// <theory> must be a completable theory, so we know it has the form forall V (body -> head)
// <formula_heads[formula] = head(formula)>
pub fn completion(theory: fol::Theory, formula_heads: HashMap::<fol::Formula, fol::AtomicFormula>) -> fol::Theory {
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
    // By this point, we should have a map from each unique head to a vector of F_i formula bodies (definitions)
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
                let full_body = disjoin((bodies, f1));
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
*/

fn main() {
    /*let rule1: asp::Rule = "p(a).".parse().unwrap();
    let rule2: asp::Rule = "p(b).".parse().unwrap();
    let rule3: asp::Rule = "q(X,Y) :- p(X), p(Y).".parse().unwrap();
    let program = asp::Program {
        rules: vec![rule1, rule2, rule3],
    };

    let f1 = translating::tau_star::tau_star_program(program);
    let thing = formatting::fol::default::Format(&f1);
    println!("{thing}");

    let form1: fol::Formula = "forall V1 (V1 = a -> p(V1))".parse().unwrap();
    let form2: fol::Formula = "forall V1 (V1 = b -> p(V1))".parse().unwrap();
    let form3: fol::Formula = "forall X Y V1 V2 (V1 = X and V2 = Y and (exists Z1 (Z1 = X and p(Z1)) and exists Z1 (Z1 = Y and p(Z1))) -> q(V1,V2))".parse().unwrap();
    let theory = fol::Theory {
        formulas: vec![form1, form2, form3],
    };
    println!("{theory}");*/

    /*let x = fol::Variable {
        name: "X".to_string(),
        sort: fol::Sort::General,
    };
    let y = fol::Variable {
        name: "Y".to_string(),
        sort: fol::Sort::General,
    };
    let q = fol::Quantification {
        quantifier: fol::Quantifier::Forall,
        variables: vec![y, x],
    };

    println!("{q}");*/

    /*let target: fol::Formula =
                "not p(a)"
                    .parse()
                    .unwrap();

        println!("{target}");
    */
    /*let atomic: asp::AtomicFormula = "X < 1..5".parse().unwrap();
    let target: fol::Formula =
            "exists Z1$g Z2$g (Z1$g = 1 and (exists I1$i J1$i K1$i (I1$i = 1 and J1$i = 5 and Z2$g = K1$i and I1$i <= K1$i <= J1$i)) and Z1$g < Z2$g)"
                .parse()
                .unwrap();

    println!("{target}");*/

    let rule1: asp::Rule = "q :- not not p.".parse().unwrap();
    let program = asp::Program { rules: vec![rule1] };
    println!("{program}");

    let form1: fol::Formula = "not p -> q".parse().unwrap();
    let theory = fol::Theory {
        formulas: vec![form1],
    };
    let theory = translating::tau_star::tau_star_program(program);
    println!("{theory}");

    /*let completable = completable_theory(f1.clone());
    match completable {
        Some(m) => {
            for (key, val) in m.iter() {
                println!("{}", formatting::fol::default::Format(val));
            }
            println!("");
            let completion = completion(f1.clone(), m);
            for c in completion.formulas.iter() {
                println!("{}", formatting::fol::default::Format(c));
            }
        },
        None => {println!("uh oh");},
    }*/
    //println!("{f1}");

    //let formula: fol::Formula = "a -> b".parse().unwrap();
    //println!("{formula}");
}
