pub mod classic;
pub mod ht;

use crate::{
    convenience::{
        choose_fresh_variable_names,
        unbox::{fol::UnboxedFormula, Unbox as _},
    },
    syntax_tree::fol::{
        AtomicFormula, BinaryConnective, Comparison, Formula, GeneralTerm, Guard, IntegerTerm,
        Quantification, Relation, Sort, SymbolicTerm, Variable,
    },
};

// ASSUMES ivar is an integer variable and ovar is a general variable
// This function checks if the comparison `ivar = ovar` or `ovar = ivar` matches the comparison `comp`
// If so, it replaces ovar with a fresh integer variable within `formula`
// Otherwise it returns `formula`
fn replacement_helper(
    ivar: &Variable,
    ovar: &Variable,
    comp: &Comparison,
    formula: &Formula,
) -> (Formula, bool) {
    let mut simplified_formula = formula.clone();
    let ivar_term = GeneralTerm::IntegerTerm(IntegerTerm::Variable(ivar.name.clone()));
    let candidate = Comparison {
        term: GeneralTerm::Variable(ovar.name.clone()),
        guards: vec![Guard {
            relation: Relation::Equal,
            term: ivar_term.clone(),
        }],
    };
    let mut replace = false;
    if comp == &candidate {
        replace = true;
    } else {
        let candidate = Comparison {
            term: ivar_term.clone(),
            guards: vec![Guard {
                relation: Relation::Equal,
                term: GeneralTerm::Variable(ovar.name.clone()),
            }],
        };
        if comp == &candidate {
            replace = true;
        }
    }

    if replace {
        let varnames = choose_fresh_variable_names(
            &formula.variables(),
            &ivar.name.chars().next().unwrap().to_string(),
            1,
        );
        let fvar = varnames[0].clone();
        let fvar_term = GeneralTerm::IntegerTerm(IntegerTerm::Variable(fvar.clone()));
        match formula.clone() {
            Formula::QuantifiedFormula {
                quantification:
                    Quantification {
                        quantifier,
                        mut variables,
                    },
                formula: f,
            } => {
                variables.retain(|x| x != ovar); // Drop ovar from the outer quantification, leaving ovar free within formula
                variables.push(Variable {
                    // Add the new integer variable to replace ovar
                    name: fvar,
                    sort: Sort::Integer,
                });
                simplified_formula = Formula::QuantifiedFormula {
                    quantification: Quantification {
                        quantifier: quantifier.clone(),
                        variables,
                    },
                    formula: f.substitute(ovar.clone(), fvar_term).into(), // Replace all free occurences of ovar with fvar_term
                };
            }

            _ => panic!("You are using the replacement helper function wrong"),
        }
    }
    (simplified_formula, replace)
}

// ASSUMES formula has the form:
// F(var) & var = term  OR
// F(var) & term = var
// If var is a variable of sort S and term is a term of a subsort of S,
// and term doesn't contain variables quantified within F, return `F(term)`
// Otherwise, return the original formula
fn subsort_equality(var: Variable, term: GeneralTerm, formula: Formula) -> (Formula, bool) {
    let mut modified = false;
    let mut simplified_formula = formula.clone();
    match formula.clone().unbox() {
        UnboxedFormula::BinaryFormula {
            connective: BinaryConnective::Conjunction,
            lhs,
            ..
        } => {
            let term_vars = term.variables(); // term must not contain var
            match var.sort {
                Sort::General => {
                    if !term_vars.contains(&var) && !lhs.clone().unsafe_substitution(&var, &term) {
                        modified = true;
                        simplified_formula = lhs.substitute(var, term);
                    }
                }
                Sort::Integer => match term.clone() {
                    GeneralTerm::IntegerTerm(_) => {
                        if !term_vars.contains(&var)
                            && !lhs.clone().unsafe_substitution(&var, &term)
                        {
                            modified = true;
                            simplified_formula = lhs.substitute(var, term);
                        }
                    }
                    _ => {
                        simplified_formula = formula;
                    }
                },
                Sort::Symbol => match term.clone() {
                    GeneralTerm::SymbolicTerm(_) => {
                        if !term_vars.contains(&var)
                            && !lhs.clone().unsafe_substitution(&var, &term)
                        {
                            modified = true;
                            simplified_formula = lhs.substitute(var, term);
                        }
                    }
                    _ => {
                        simplified_formula = formula;
                    }
                },
            }
        }

        _ => panic!("you're using the subsort equality fn wrong"),
    }
    (simplified_formula, modified)
}

// Given a tree of conjunctions, F1, .. Fi, .. Fn, identify an equality formula Fi: X = t or t = X
// If possible, substitute t for X within the tree and drop Fi
// Return the original formula and None if not possible
// Otherwise, return the simplified formula and the (X, t) substitution pair
fn simplify_conjunction_tree_with_equality(
    formula: Formula,
    enclosing_variables: Vec<Variable>,
) -> (Formula, Option<(Variable, GeneralTerm)>) {
    let mut result = (formula.clone(), None);
    let conjunctive_terms = Formula::conjoin_invert(formula.clone());
    for ct in conjunctive_terms.iter() {
        // Search for an equality formula
        if let Formula::AtomicFormula(AtomicFormula::Comparison(comp)) = ct {
            if comp.equality_comparison() {
                let term = &comp.term;
                let g = comp.guards[0].clone();
                let lhs_is_var = match term.clone() {
                    GeneralTerm::Variable(v) => Some(Variable {
                        sort: Sort::General,
                        name: v,
                    }),
                    GeneralTerm::IntegerTerm(IntegerTerm::Variable(v)) => Some(Variable {
                        sort: Sort::Integer,
                        name: v,
                    }),
                    GeneralTerm::SymbolicTerm(SymbolicTerm::Variable(v)) => Some(Variable {
                        sort: Sort::Symbol,
                        name: v,
                    }),
                    _ => None,
                };
                let rhs_is_var = match g.term.clone() {
                    GeneralTerm::Variable(v) => Some(Variable {
                        sort: Sort::General,
                        name: v,
                    }),
                    GeneralTerm::IntegerTerm(IntegerTerm::Variable(v)) => Some(Variable {
                        sort: Sort::Integer,
                        name: v,
                    }),
                    GeneralTerm::SymbolicTerm(SymbolicTerm::Variable(v)) => Some(Variable {
                        sort: Sort::Symbol,
                        name: v,
                    }),
                    _ => None,
                };

                let mut safety = true; // Simplify var = term or term = var but not both
                                       // Don't restructure the conjunction tree unless simplification occurs
                let mut restructured = vec![]; // Make the equality formula the top rhs leaf of a new conjunction tree
                                               // for i in 0..conjunctive_terms.len() {
                                               //     if conjunctive_terms[i] != *ct {
                                               //         restructured.push(conjunctive_terms[i].clone());
                                               //     }
                                               // }
                for alt_ct in conjunctive_terms.clone() {
                    if alt_ct != *ct {
                        restructured.push(alt_ct);
                    }
                }
                restructured.push(ct.clone());

                let simplified = Formula::conjoin(restructured);

                if let Some(v1) = lhs_is_var {
                    if enclosing_variables.contains(&v1) {
                        let simplification_result =
                            subsort_equality(v1.clone(), g.term.clone(), simplified.clone());
                        if simplification_result.1 {
                            result = (simplification_result.0, Some((v1, g.term)));
                            safety = false;
                        }
                    }
                }
                if let Some(v2) = rhs_is_var {
                    if enclosing_variables.contains(&v2) && safety {
                        let simplification_result =
                            subsort_equality(v2.clone(), term.clone(), simplified);
                        if simplification_result.1 {
                            result = (simplification_result.0, Some((v2, term.clone())));
                            safety = false;
                        }
                    }
                }
                if !safety {
                    break;
                }
            }
        }
    }
    result
}

#[test]
fn test_simplify_conjunction_tree() {
    for (src, target) in [(
        (
            "X = Z and not q(X)",
            vec![
                Variable {
                    name: "X".to_string(),
                    sort: Sort::General,
                },
                Variable {
                    name: "Y".to_string(),
                    sort: Sort::General,
                },
            ],
        ),
        "not q(Z)",
    )] {
        let result = simplify_conjunction_tree_with_equality(src.0.parse().unwrap(), src.1).0;
        let target = target.parse().unwrap();
        assert_eq!(result, target, "{result} != {target}")
    }
}
