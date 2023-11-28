use {
    crate::syntax_tree::{asp, fol},
    regex::Regex,
    std::collections::HashSet,
};

/// Choose fresh variants of `Vn` by incrementing `n`
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

/// Choose `arity` variable names by incrementing `variant`, disjoint from `variables`
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
    } else if formulas.len() == 1 {
        formulas.pop().unwrap()
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
    } else if formulas.len() == 1 {
        formulas.pop().unwrap()
    } else {
        fol::Formula::BinaryFormula {
            connective: fol::BinaryConnective::Disjunction,
            rhs: formulas.pop().unwrap().into(),
            lhs: disjoin(formulas).into(),
        }
    }
}

// Z = t
fn construct_equality_formula(term: asp::Term, z: fol::Variable) -> fol::Formula {
    let z_var_term = match z.sort {
        fol::Sort::General => fol::GeneralTerm::GeneralVariable(z.name),
        fol::Sort::Integer => fol::GeneralTerm::IntegerTerm(fol::IntegerTerm::BasicIntegerTerm(
            fol::BasicIntegerTerm::IntegerVariable(z.name),
        )),
    };

    let rhs = match term {
        asp::Term::PrecomputedTerm(t) => match t {
            asp::PrecomputedTerm::Infimum => fol::GeneralTerm::IntegerTerm(
                fol::IntegerTerm::BasicIntegerTerm(fol::BasicIntegerTerm::Infimum),
            ),
            asp::PrecomputedTerm::Supremum => fol::GeneralTerm::IntegerTerm(
                fol::IntegerTerm::BasicIntegerTerm(fol::BasicIntegerTerm::Supremum),
            ),
            asp::PrecomputedTerm::Numeral(i) => fol::GeneralTerm::IntegerTerm(
                fol::IntegerTerm::BasicIntegerTerm(fol::BasicIntegerTerm::Numeral(i)),
            ),
            asp::PrecomputedTerm::Symbol(s) => fol::GeneralTerm::Symbol(s),
        },
        asp::Term::Variable(v) => fol::GeneralTerm::GeneralVariable(v.0),
        _ => panic!(), // Error
    };

    fol::Formula::AtomicFormula(fol::AtomicFormula::Comparison(fol::Comparison {
        term: z_var_term,
        guards: vec![fol::Guard {
            relation: fol::Relation::Equal,
            term: rhs,
        }],
    }))
}

// +,-,*
// exists I J (Z = I op J & val_t1(I) & val_t2(J))
fn construct_total_function_formula(
    valti: fol::Formula,
    valtj: fol::Formula,
    binop: asp::BinaryOperator,
    i_var: fol::Variable,
    j_var: fol::Variable,
    z: fol::Variable,
) -> fol::Formula {
    let i = i_var.name;
    let j = j_var.name;
    let z_var_term = match z.sort {
        fol::Sort::General => fol::GeneralTerm::GeneralVariable(z.name.into()),
        fol::Sort::Integer => fol::GeneralTerm::IntegerTerm(fol::IntegerTerm::BasicIntegerTerm(
            fol::BasicIntegerTerm::IntegerVariable(z.name.into()),
        )),
    };
    let zequals = fol::Formula::AtomicFormula(fol::AtomicFormula::Comparison(fol::Comparison {
        // Z = I binop J
        term: z_var_term,
        guards: vec![fol::Guard {
            relation: fol::Relation::Equal,
            term: fol::GeneralTerm::IntegerTerm(fol::IntegerTerm::BinaryOperation {
                op: match binop {
                    asp::BinaryOperator::Add => fol::BinaryOperator::Add,
                    asp::BinaryOperator::Subtract => fol::BinaryOperator::Subtract,
                    asp::BinaryOperator::Multiply => fol::BinaryOperator::Multiply,
                    _ => panic!(), // More error handling
                },
                lhs: fol::IntegerTerm::BasicIntegerTerm(fol::BasicIntegerTerm::IntegerVariable(
                    i.clone(),
                ))
                .into(),
                rhs: fol::IntegerTerm::BasicIntegerTerm(fol::BasicIntegerTerm::IntegerVariable(
                    j.clone(),
                ))
                .into(),
            }),
        }],
    }));
    fol::Formula::QuantifiedFormula {
        quantification: fol::Quantification {
            quantifier: fol::Quantifier::Exists,
            variables: vec![
                fol::Variable {
                    name: i.into(),
                    sort: fol::Sort::Integer,
                },
                fol::Variable {
                    name: j.into(),
                    sort: fol::Sort::Integer,
                },
            ],
        },
        formula: fol::Formula::BinaryFormula {
            connective: fol::BinaryConnective::Conjunction,
            lhs: fol::Formula::BinaryFormula {
                connective: fol::BinaryConnective::Conjunction,
                lhs: zequals.into(),
                rhs: valti.into(),
            }
            .into(),
            rhs: valtj.into(),
        }
        .into(),
    }
}

// Are these definitions correct?
// Division: exists I J Q R (I = J * Q + R & val_t1(I) & val_t2(J) & J != 0 & R >= 0 & R < Q & Z = Q)
// Modulo:   exists I J Q R (I = J * Q + R & val_t1(I) & val_t2(J) & J != 0 & R >= 0 & R < Q & Z = R)
fn construct_partial_function_formula(
    valti: fol::Formula,
    valtj: fol::Formula,
    binop: asp::BinaryOperator,
    i_var: fol::Variable,
    j_var: fol::Variable,
    z: fol::Variable,
) -> fol::Formula {
    let i = i_var.name;
    let j = j_var.name;

    let mut taken_vars = HashSet::<fol::Variable>::new();
    for var in valti.variables().iter() {
        taken_vars.insert(fol::Variable {
            name: var.to_string(),
            sort: fol::Sort::General,
        });
    }
    for var in valtj.variables().iter() {
        taken_vars.insert(fol::Variable {
            name: var.to_string(),
            sort: fol::Sort::General,
        });
    }

    let z_var_term = match z.sort {
        fol::Sort::General => fol::GeneralTerm::GeneralVariable(z.name.into()),
        fol::Sort::Integer => fol::GeneralTerm::IntegerTerm(fol::IntegerTerm::BasicIntegerTerm(
            fol::BasicIntegerTerm::IntegerVariable(z.name.into()),
        )),
    };

    // I = J * Q + R
    let qvar = choose_fresh_variable_names_v(&taken_vars, "Q", 1)
        .pop()
        .unwrap();
    let rvar = choose_fresh_variable_names_v(&taken_vars, "R", 1)
        .pop()
        .unwrap();
    let iequals = fol::Formula::AtomicFormula(fol::AtomicFormula::Comparison(fol::Comparison {
        term: fol::GeneralTerm::IntegerTerm(fol::IntegerTerm::BasicIntegerTerm(
            fol::BasicIntegerTerm::IntegerVariable(i.clone()),
        )),
        guards: vec![fol::Guard {
            relation: fol::Relation::Equal,
            term: fol::GeneralTerm::IntegerTerm(fol::IntegerTerm::BinaryOperation {
                op: fol::BinaryOperator::Add,
                lhs: fol::IntegerTerm::BinaryOperation {
                    op: fol::BinaryOperator::Multiply,
                    lhs: fol::IntegerTerm::BasicIntegerTerm(
                        fol::BasicIntegerTerm::IntegerVariable(j.clone()),
                    )
                    .into(),
                    rhs: fol::IntegerTerm::BasicIntegerTerm(
                        fol::BasicIntegerTerm::IntegerVariable(qvar.clone().into()),
                    )
                    .into(),
                }
                .into(),
                rhs: fol::IntegerTerm::BasicIntegerTerm(fol::BasicIntegerTerm::IntegerVariable(
                    rvar.clone().into(),
                ))
                .into(),
            }),
        }],
    }));

    // J != 0 & R >= 0 & R < Q
    let conditions = fol::Formula::BinaryFormula {
        connective: fol::BinaryConnective::Conjunction,
        lhs: fol::Formula::BinaryFormula {
            connective: fol::BinaryConnective::Conjunction,
            lhs: fol::Formula::AtomicFormula(fol::AtomicFormula::Comparison(fol::Comparison {
                term: fol::GeneralTerm::IntegerTerm(fol::IntegerTerm::BasicIntegerTerm(
                    fol::BasicIntegerTerm::IntegerVariable(j.clone()),
                )),
                guards: vec![fol::Guard {
                    relation: fol::Relation::NotEqual,
                    term: fol::GeneralTerm::IntegerTerm(fol::IntegerTerm::BasicIntegerTerm(
                        fol::BasicIntegerTerm::Numeral(0),
                    )),
                }],
            }))
            .into(),
            rhs: fol::Formula::AtomicFormula(fol::AtomicFormula::Comparison(fol::Comparison {
                term: fol::GeneralTerm::IntegerTerm(fol::IntegerTerm::BasicIntegerTerm(
                    fol::BasicIntegerTerm::IntegerVariable(rvar.clone().into()),
                )),
                guards: vec![fol::Guard {
                    relation: fol::Relation::GreaterEqual,
                    term: fol::GeneralTerm::IntegerTerm(fol::IntegerTerm::BasicIntegerTerm(
                        fol::BasicIntegerTerm::Numeral(0),
                    )),
                }],
            }))
            .into(),
        }
        .into(),
        rhs: fol::Formula::AtomicFormula(fol::AtomicFormula::Comparison(fol::Comparison {
            term: fol::GeneralTerm::IntegerTerm(fol::IntegerTerm::BasicIntegerTerm(
                fol::BasicIntegerTerm::IntegerVariable(rvar.clone().into()),
            )),
            guards: vec![fol::Guard {
                relation: fol::Relation::Less,
                term: fol::GeneralTerm::IntegerTerm(fol::IntegerTerm::BasicIntegerTerm(
                    fol::BasicIntegerTerm::IntegerVariable(qvar.clone().into()),
                )),
            }],
        }))
        .into(),
    };

    // val_t1(I) & val_t2(J)
    let inner_vals = fol::Formula::BinaryFormula {
        connective: fol::BinaryConnective::Conjunction,
        lhs: valti.into(),
        rhs: valtj.into(),
    };

    // (( I = J * Q + R ) & ( val_t1(I) & val_t2(J) )) & ( J != 0 & R >= 0 & R < Q )
    let subformula = {
        fol::Formula::BinaryFormula {
            connective: fol::BinaryConnective::Conjunction,
            lhs: fol::Formula::BinaryFormula {
                connective: fol::BinaryConnective::Conjunction,
                lhs: iequals.into(),
                rhs: inner_vals.into(),
            }
            .into(),
            rhs: conditions.into(),
        }
    };

    // Z = Q or Z = R
    let zequals = match binop {
        asp::BinaryOperator::Divide => {
            fol::Formula::AtomicFormula(fol::AtomicFormula::Comparison(fol::Comparison {
                term: z_var_term,
                guards: vec![fol::Guard {
                    relation: fol::Relation::Equal,
                    term: fol::GeneralTerm::IntegerTerm(fol::IntegerTerm::BasicIntegerTerm(
                        fol::BasicIntegerTerm::IntegerVariable(qvar.clone().into()),
                    )),
                }],
            }))
        }
        asp::BinaryOperator::Modulo => {
            fol::Formula::AtomicFormula(fol::AtomicFormula::Comparison(fol::Comparison {
                term: z_var_term,
                guards: vec![fol::Guard {
                    relation: fol::Relation::Equal,
                    term: fol::GeneralTerm::IntegerTerm(fol::IntegerTerm::BasicIntegerTerm(
                        fol::BasicIntegerTerm::IntegerVariable(rvar.clone().into()),
                    )),
                }],
            }))
        }
        _ => panic!(), // Error
    };

    fol::Formula::QuantifiedFormula {
        quantification: fol::Quantification {
            quantifier: fol::Quantifier::Exists,
            variables: vec![
                fol::Variable {
                    name: i.into(),
                    sort: fol::Sort::Integer,
                },
                fol::Variable {
                    name: j.into(),
                    sort: fol::Sort::Integer,
                },
                fol::Variable {
                    name: qvar.into(),
                    sort: fol::Sort::Integer,
                },
                fol::Variable {
                    name: rvar.into(),
                    sort: fol::Sort::Integer,
                },
            ],
        },
        formula: fol::Formula::BinaryFormula {
            connective: fol::BinaryConnective::Conjunction,
            lhs: subformula.into(),
            rhs: zequals.into(),
        }
        .into(),
    }
}

// t1..t2
// exists I J K (val_t1(I) & val_t2(J) & I <= K <= J & Z = K)
fn construct_interval_formula(
    valti: fol::Formula,
    valtj: fol::Formula,
    i_var: fol::Variable,
    j_var: fol::Variable,
    k_var: fol::Variable,
    z: fol::Variable,
) -> fol::Formula {
    let z_var_term = match z.sort {
        fol::Sort::General => fol::GeneralTerm::GeneralVariable(z.name.into()),
        fol::Sort::Integer => fol::GeneralTerm::IntegerTerm(fol::IntegerTerm::BasicIntegerTerm(
            fol::BasicIntegerTerm::IntegerVariable(z.name.into()),
        )),
    };

    // I <= K <= J
    let range = fol::Formula::AtomicFormula(fol::AtomicFormula::Comparison(fol::Comparison {
        term: fol::GeneralTerm::IntegerTerm(fol::IntegerTerm::BasicIntegerTerm(
            fol::BasicIntegerTerm::IntegerVariable(i_var.name.clone().into()),
        )),
        guards: vec![
            fol::Guard {
                relation: fol::Relation::LessEqual,
                term: fol::GeneralTerm::IntegerTerm(fol::IntegerTerm::BasicIntegerTerm(
                    fol::BasicIntegerTerm::IntegerVariable(k_var.name.clone().into()),
                )),
            },
            fol::Guard {
                relation: fol::Relation::LessEqual,
                term: fol::GeneralTerm::IntegerTerm(fol::IntegerTerm::BasicIntegerTerm(
                    fol::BasicIntegerTerm::IntegerVariable(j_var.name.clone().into()),
                )),
            },
        ],
    }));

    // val_t1(I) & val_t2(J) & Z = k
    let subformula = fol::Formula::BinaryFormula {
        connective: fol::BinaryConnective::Conjunction,
        lhs: fol::Formula::BinaryFormula {
            connective: fol::BinaryConnective::Conjunction,
            lhs: valti.into(),
            rhs: valtj.into(),
        }
        .into(),
        rhs: fol::Formula::AtomicFormula(fol::AtomicFormula::Comparison(fol::Comparison {
            term: z_var_term,
            guards: vec![fol::Guard {
                relation: fol::Relation::Equal,
                term: fol::GeneralTerm::IntegerTerm(fol::IntegerTerm::BasicIntegerTerm(
                    fol::BasicIntegerTerm::IntegerVariable(k_var.name.clone().into()),
                )),
            }],
        }))
        .into(),
    };

    fol::Formula::QuantifiedFormula {
        quantification: fol::Quantification {
            quantifier: fol::Quantifier::Exists,
            variables: vec![i_var.clone(), j_var.clone(), k_var.clone()],
        },
        formula: fol::Formula::BinaryFormula {
            connective: fol::BinaryConnective::Conjunction,
            lhs: subformula.into(),
            rhs: range.into(),
        }
        .into(),
    }
}

// val_t(Z)
pub fn val(t: asp::Term, z: fol::Variable) -> fol::Formula {
    let mut taken_vars = HashSet::<fol::Variable>::new();
    for var in t.variables().iter() {
        taken_vars.insert(fol::Variable {
            name: var.to_string(),
            sort: fol::Sort::General,
        });
    }
    taken_vars.insert(z.clone());

    let mut fresh_ivar = choose_fresh_variable_names_v(&taken_vars, "I", 1);
    let mut fresh_jvar = choose_fresh_variable_names_v(&taken_vars, "J", 1);
    let mut fresh_kvar = choose_fresh_variable_names_v(&taken_vars, "K", 1);

    // Fresh integer variables
    let var1 = fol::Variable {
        name: fresh_ivar.pop().unwrap().into(),
        sort: fol::Sort::Integer,
    };
    let var2 = fol::Variable {
        name: fresh_jvar.pop().unwrap().into(),
        sort: fol::Sort::Integer,
    };
    let var3 = fol::Variable {
        name: fresh_kvar.pop().unwrap().into(),
        sort: fol::Sort::Integer,
    };
    match t {
        asp::Term::PrecomputedTerm(_) | asp::Term::Variable(_) => construct_equality_formula(t, z),
        asp::Term::UnaryOperation { op, arg } => {
            match op {
                asp::UnaryOperator::Negative => {
                    let lhs = asp::Term::PrecomputedTerm(asp::PrecomputedTerm::Numeral(0)); // Shorthand for 0 - t
                    let valti = val(lhs, var1.clone()); // val_t1(I)
                    let valtj = val(*arg, var2.clone()); // val_t2(J)
                    construct_total_function_formula(
                        valti,
                        valtj,
                        asp::BinaryOperator::Subtract,
                        var1.clone(),
                        var2.clone(),
                        z,
                    )
                }
            }
        }
        asp::Term::BinaryOperation { op, lhs, rhs } => {
            let valti = val(*lhs, var1.clone()); // val_t1(I)
            let valtj = val(*rhs, var2.clone()); // val_t2(J)
            match op {
                asp::BinaryOperator::Add => construct_total_function_formula(
                    valti,
                    valtj,
                    asp::BinaryOperator::Add,
                    var1.clone(),
                    var2.clone(),
                    z,
                ),
                asp::BinaryOperator::Subtract => construct_total_function_formula(
                    valti,
                    valtj,
                    asp::BinaryOperator::Subtract,
                    var1.clone(),
                    var2.clone(),
                    z,
                ),
                asp::BinaryOperator::Multiply => construct_total_function_formula(
                    valti,
                    valtj,
                    asp::BinaryOperator::Multiply,
                    var1.clone(),
                    var2.clone(),
                    z,
                ),
                asp::BinaryOperator::Divide => construct_partial_function_formula(
                    valti,
                    valtj,
                    asp::BinaryOperator::Divide,
                    var1.clone(),
                    var2.clone(),
                    z,
                ),
                asp::BinaryOperator::Modulo => construct_partial_function_formula(
                    valti,
                    valtj,
                    asp::BinaryOperator::Modulo,
                    var1.clone(),
                    var2.clone(),
                    z,
                ),
                asp::BinaryOperator::Interval => construct_interval_formula(
                    valti,
                    valtj,
                    var1.clone(),
                    var2.clone(),
                    var3.clone(),
                    z,
                ),
            }
        }
    }
}

// val_t1(Z1) & val_t2(Z2) & ... & val_tn(Zn)
pub fn valtz(terms: Vec<asp::Term>, variables: Vec<fol::Variable>) -> fol::Formula {
    let mut formulas: Vec<fol::Formula> = Vec::with_capacity(terms.len() as usize);
    for (i, t) in terms.iter().enumerate() {
        let val_ti_zi = val(t.clone(), variables[i].clone());
        formulas.push(val_ti_zi);
    }
    conjoin(formulas)
}

// Translate a first-order body literal
pub fn tau_b_first_order_literal(
    l: asp::Literal,
    taken_vars: HashSet<fol::Variable>,
) -> fol::Formula {
    let atom = l.atom;
    let terms = atom.terms;
    let arity = terms.len();
    let varnames = choose_fresh_variable_names_v(&taken_vars, "Z", arity);

    // Compute val_t1(Z1) & val_t2(Z2) & ... & val_tk(Zk)
    let mut var_terms: Vec<fol::GeneralTerm> = Vec::with_capacity(arity as usize);
    let mut var_vars: Vec<fol::Variable> = Vec::with_capacity(arity as usize);
    let mut valtz_vec: Vec<fol::Formula> = Vec::with_capacity(arity as usize);
    for (i, t) in terms.iter().enumerate() {
        let var = fol::Variable {
            sort: fol::Sort::General,
            name: varnames[i].clone(),
        };
        valtz_vec.push(val(t.clone(), var.clone()));
        var_terms.push(fol::GeneralTerm::GeneralVariable(varnames[i].clone()));
        var_vars.push(var);
    }
    let valtz = conjoin(valtz_vec);

    // Compute p(Z1, Z2, ..., Zk)
    let p_zk = fol::Formula::AtomicFormula(fol::AtomicFormula::Atom(fol::Atom {
        predicate_symbol: atom.predicate_symbol,
        terms: var_terms,
    }));

    // Compute tau^b(B)
    match l.sign {
        asp::Sign::NoSign => fol::Formula::QuantifiedFormula {
            quantification: fol::Quantification {
                quantifier: fol::Quantifier::Exists,
                variables: var_vars,
            },
            formula: fol::Formula::BinaryFormula {
                connective: fol::BinaryConnective::Conjunction,
                lhs: valtz.into(),
                rhs: p_zk.into(),
            }
            .into(),
        },
        asp::Sign::Negation => fol::Formula::QuantifiedFormula {
            quantification: fol::Quantification {
                quantifier: fol::Quantifier::Exists,
                variables: var_vars,
            },
            formula: fol::Formula::BinaryFormula {
                connective: fol::BinaryConnective::Conjunction,
                lhs: valtz.into(),
                rhs: fol::Formula::UnaryFormula {
                    connective: fol::UnaryConnective::Negation,
                    formula: p_zk.into(),
                }
                .into(),
            }
            .into(),
        },
        asp::Sign::DoubleNegation => fol::Formula::QuantifiedFormula {
            quantification: fol::Quantification {
                quantifier: fol::Quantifier::Exists,
                variables: var_vars,
            },
            formula: fol::Formula::BinaryFormula {
                connective: fol::BinaryConnective::Conjunction,
                lhs: valtz.into(),
                rhs: fol::Formula::UnaryFormula {
                    connective: fol::UnaryConnective::Negation,
                    formula: fol::Formula::UnaryFormula {
                        connective: fol::UnaryConnective::Negation,
                        formula: p_zk.into(),
                    }
                    .into(),
                }
                .into(),
            }
            .into(),
        },
    }
}

// Translate a propositional body literal
pub fn tau_b_propositional_literal(l: asp::Literal) -> fol::Formula {
    let atom = l.atom;
    match l.sign {
        asp::Sign::NoSign => fol::Formula::AtomicFormula(fol::AtomicFormula::Atom(fol::Atom {
            predicate_symbol: atom.predicate_symbol,

            terms: vec![],
        })),
        asp::Sign::Negation => fol::Formula::UnaryFormula {
            connective: fol::UnaryConnective::Negation,
            formula: fol::Formula::AtomicFormula(fol::AtomicFormula::Atom(fol::Atom {
                predicate_symbol: atom.predicate_symbol,
                terms: vec![],
            }))
            .into(),
        },
        asp::Sign::DoubleNegation => fol::Formula::UnaryFormula {
            connective: fol::UnaryConnective::Negation,
            formula: fol::Formula::UnaryFormula {
                connective: fol::UnaryConnective::Negation,
                formula: fol::Formula::AtomicFormula(fol::AtomicFormula::Atom(fol::Atom {
                    predicate_symbol: atom.predicate_symbol,
                    terms: vec![],
                }))
                .into(),
            }
            .into(),
        },
    }
}

// Translate a body comparison
pub fn tau_b_comparison(c: asp::Comparison, taken_vars: HashSet<fol::Variable>) -> fol::Formula {
    let varnames = choose_fresh_variable_names_v(&taken_vars, "Z", 2);

    // Compute val_t1(Z1) & val_t2(Z2)
    let term_z1 = fol::GeneralTerm::GeneralVariable(varnames[0].clone());
    let term_z2 = fol::GeneralTerm::GeneralVariable(varnames[1].clone());
    let var_z1 = fol::Variable {
        sort: fol::Sort::General,
        name: varnames[0].clone(),
    };
    let var_z2 = fol::Variable {
        sort: fol::Sort::General,
        name: varnames[1].clone(),
    };
    let valtz = conjoin(vec![val(c.lhs, var_z1.clone()), val(c.rhs, var_z2.clone())]);

    // Compute Z1 rel Z2
    let z1_rel_z2 = fol::Formula::AtomicFormula(fol::AtomicFormula::Comparison(fol::Comparison {
        term: term_z1,
        guards: vec![fol::Guard {
            relation: match c.relation {
                asp::Relation::Equal => fol::Relation::Equal,
                asp::Relation::NotEqual => fol::Relation::NotEqual,
                asp::Relation::Greater => fol::Relation::Greater,
                asp::Relation::Less => fol::Relation::Less,
                asp::Relation::GreaterEqual => fol::Relation::GreaterEqual,
                asp::Relation::LessEqual => fol::Relation::LessEqual,
            },
            term: term_z2,
        }],
    }));

    fol::Formula::QuantifiedFormula {
        quantification: fol::Quantification {
            quantifier: fol::Quantifier::Exists,
            variables: vec![var_z1, var_z2],
        },
        formula: fol::Formula::BinaryFormula {
            connective: fol::BinaryConnective::Conjunction,
            lhs: valtz.into(),
            rhs: z1_rel_z2.into(),
        }
        .into(),
    }
}

// Translate a body literal or comparison
pub fn tau_b(f: asp::AtomicFormula) -> fol::Formula {
    let mut taken_vars = HashSet::<fol::Variable>::new();
    for var in f.variables().iter() {
        taken_vars.insert(fol::Variable {
            name: var.to_string(),
            sort: fol::Sort::General,
        });
    }
    match f {
        asp::AtomicFormula::Literal(l) => {
            let arity = l.atom.terms.len();
            if arity > 0 {
                tau_b_first_order_literal(l, taken_vars)
            } else {
                tau_b_propositional_literal(l)
            }
        }
        asp::AtomicFormula::Comparison(c) => tau_b_comparison(c, taken_vars),
    }
}

#[cfg(test)]
mod tests {
    use super::{conjoin, disjoin, tau_b, val};

    #[test]
    fn test_conjoin() {
        for (src, target) in [
            (vec![], "#true"),
            (vec!["X = Y"], "X = Y"),
            (vec!["X = Y", "p(a)", "q(X)"], "X = Y and p(a) and q(X)"),
        ] {
            assert_eq!(
                conjoin(src.iter().map(|x| x.parse().unwrap()).collect()),
                target.parse().unwrap(),
            )
        }
    }

    #[test]
    fn test_disjoin() {
        for (src, target) in [
            (vec![], "#false"),
            (vec!["X = Y"], "X = Y"),
            (vec!["X = Y", "p(a)", "q(X)"], "X = Y or p(a) or q(X)"),
        ] {
            assert_eq!(
                disjoin(src.iter().map(|x| x.parse().unwrap()).collect()),
                target.parse().unwrap(),
            )
        }
    }

    #[test]
    fn test_val() {
        for (term, var, target) in [
            ("X + 1", "Z1", "exists I$i J$i (Z1$g = I$i + J$i and I$i = X and J$i = 1)"),
            ("3 - 5", "Z1", "exists I$i J$i (Z1$g = I$i - J$i and I$i = 3 and J$i = 5)"),
            ("Xanadu/Yak", "Z1", "exists I$i J$i Q$i R$i (I$i = J$i * Q$i + R$i and (I$i = Xanadu and J$i = Yak) and (J$i != 0 and R$i >= 0 and R$i < Q$i) and Z1$g = Q$i)"),
            ("X \\ 3", "Z1", "exists I$i J$i Q$i R$i (I$i = J$i * Q$i + R$i and (I$i = X and J$i = 3) and (J$i != 0 and R$i >= 0 and R$i < Q$i) and Z1$g = R$i)"),
            ("X..Y", "Z", "exists I$i J$i K$i (I$i = X and J$i = Y and Z$g = K$i and I$i <= K$i <= J$i)"),
            ("X+1..Y", "Z1", "exists I$i J$i K$i ((exists I1$i J$i (I$i = I1$i + J$i and I1$i = X and J$i = 1)) and J$i = Y and Z1 = K$i and I$i <= K$i <= J$i)"),
        ] {
            assert_eq!(
                val(term.parse().unwrap(), var.parse().unwrap()),
                target.parse().unwrap()
            )
        }
    }

    #[test]
    fn test_tau_b() {
        for (src, target) in [
            ("p(t)", "exists Z (Z = t and p(Z))"),
            ("not p(t)", "exists Z (Z = t and not p(Z))"),
            ("X < 1..5", "exists Z Z1 (Z = X and exists I$i J$i K$i (I$i = 1 and J$i = 5 and Z1 = K$i and I$i <= K$i <= J$i) and Z < Z1)"),
            ("not not p(t)", "exists Z (Z = t and not not p(Z))"),
            ("not not x", "not not x"),
            ("not p(X,5)", "exists Z Z1 (Z = X and Z1 = 5 and not p(Z,Z1))"),
            ("not p(X,0-5)", "exists Z Z1 (Z = X and exists I$i J$i (Z1 = I$i - J$i and I$i = 0 and J$i = 5) and not p(Z,Z1))"),
            ("p(X,-1..5)", "exists Z Z1 (Z = X and exists I$i J$i K$i (I$i = -1 and J$i = 5 and Z1 = K$i and I$i <= K$i <= J$i) and p(Z,Z1))"),
            ("p(X,-(1..5))", "exists Z Z1 (Z = X and exists I$i J$i (Z1 = I$i - J$i and I$i = 0 and exists I$i J1$i K$i (I$i = 1 and J1$i = 5  and J$i = K$i and I$i <= K$i <= J1$i)) and p(Z,Z1))")
        ] {
            assert_eq!(
                tau_b(src.parse().unwrap()),
                target.parse().unwrap(),
            )
        }
    }
}
