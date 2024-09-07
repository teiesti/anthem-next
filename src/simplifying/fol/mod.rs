pub mod classic;
pub mod ht;

use crate::syntax_tree::fol::{
    Comparison, Formula, GeneralTerm, Guard, IntegerTerm, Quantification, Relation, Sort, Variable,
};

use crate::convenience::choose_fresh_variable_names;

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
