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