use crate::syntax_tree::{asp, fol};
use regex::Regex;
use std::collections::HashSet;

use crate::syntax_tree;

// Choose fresh variants of V# by incrementing #
pub fn choose_fresh_global_variables(program: &asp::Program) -> Vec<String> {
    let mut max_arity = 0;
    let mut head_arity;
    for rule in program.rules.iter() {
        head_arity = rule.head.arity();
        if head_arity > max_arity {
            max_arity = head_arity;
        }
    }
    let mut max_taken_var = 0;
    let taken_vars = program.variables();
    let re = Regex::new(r"^V(?<number>[0-9]*)$").unwrap();
    for var in taken_vars {
        match re.captures(&var.0) {
            Some(caps) => {
                let taken: usize = (&caps["number"]).parse().unwrap_or_else(|_| 0);
                if taken > max_taken_var {
                    max_taken_var = taken;
                }
            }
            None => {}
        }
    }
    let mut globals = Vec::<String>::new();
    for i in 1..max_arity + 1 {
        let mut v: String = "V".to_owned();
        let counter: &str = &(max_taken_var + i).to_string();
        v.push_str(counter);
        globals.push(v);
    }
    globals
}

// Choose <arity> variable names by incrementing <variant>, disjoint from <variables>
pub fn choose_fresh_variable_names_v(
    variables: &HashSet<fol::Variable>,
    variant: &str,
    arity: usize,
) -> Vec<String> {
    let mut taken_vars = Vec::<String>::new();
    for var in variables.iter() {
        taken_vars.push(var.name.to_string());
    }
    let mut fresh_vars = Vec::<String>::new();
    let arity_bound = match taken_vars.contains(&variant.to_string()) {
        true => arity + 1,
        false => {
            fresh_vars.push(variant.to_string());
            arity
        }
    };
    for n in 1..arity_bound {
        let mut candidate: String = variant.to_owned();
        let number: &str = &n.to_string();
        candidate.push_str(number);
        let mut m = n;
        while taken_vars.contains(&candidate) || fresh_vars.contains(&candidate) {
            candidate = variant.to_owned();
            m += 1;
            let number = &m.to_string();
            candidate.push_str(number);
        }
        fresh_vars.push(candidate.to_string());
    }
    fresh_vars
}

// Recursively turn a list of formulas into a conjunction tree
pub fn conjoin(mut formulas: Vec<fol::Formula>) -> fol::Formula {
    if formulas.len() == 0 {
        fol::Formula::AtomicFormula(fol::AtomicFormula::Truth)
    } else {
        fol::Formula::BinaryFormula {
            connective: fol::BinaryConnective::Conjunction,
            rhs: formulas.pop().unwrap().into(),
            lhs: conjoin(formulas).into(),
        }
    }
}

// Recursively turn a list of formulas into a tree of disjunctions
pub fn disjoin(mut formulas: Vec<fol::Formula>) -> fol::Formula {
    if formulas.len() == 0 {
        fol::Formula::AtomicFormula(fol::AtomicFormula::Falsity)
    } else {
        fol::Formula::BinaryFormula {
            connective: fol::BinaryConnective::Disjunction,
            rhs: formulas.pop().unwrap().into(),
            lhs: disjoin(formulas).into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::formatting;
    use crate::{syntax_tree::asp, syntax_tree::fol};

    #[test]
    pub fn conjoin_test1() {
        let f1: fol::Formula = "X = Y".parse().unwrap();
        let f2: fol::Formula = "p(a)".parse().unwrap();
        let f3: fol::Formula = "q(X)".parse().unwrap();
        let formulas = vec![f1, f2, f3];

        let src = super::conjoin(formulas);
        let dest: fol::Formula = "#true and X = Y and p(a) and q(X)".parse().unwrap();
        assert_eq!(
            format!("{}", formatting::fol::default::Format(&src)),
            format!("{}", formatting::fol::default::Format(&dest))
        );
    }

    #[test]
    pub fn conjoin_test2() {
        let formulas = vec![];

        let src = super::conjoin(formulas);
        let dest: fol::Formula = "#true".parse().unwrap();
        assert_eq!(
            format!("{}", formatting::fol::default::Format(&src)),
            format!("{}", formatting::fol::default::Format(&dest))
        );
    }

    #[test]
    pub fn disjoin_test1() {
        let f1: fol::Formula = "X = Y".parse().unwrap();
        let f2: fol::Formula = "p(a)".parse().unwrap();
        let f3: fol::Formula = "q(X)".parse().unwrap();
        let formulas = vec![f1, f2, f3];

        let src = super::disjoin(formulas);
        let dest: fol::Formula = "#false or X = Y or p(a) or q(X)".parse().unwrap();
        assert_eq!(
            format!("{}", formatting::fol::default::Format(&src)),
            format!("{}", formatting::fol::default::Format(&dest))
        );
    }

    #[test]
    pub fn disjoin_test2() {
        let formulas = vec![];

        let src = super::disjoin(formulas);
        let dest: fol::Formula = "#false".parse().unwrap();
        assert_eq!(
            format!("{}", formatting::fol::default::Format(&src)),
            format!("{}", formatting::fol::default::Format(&dest))
        );
    }
}