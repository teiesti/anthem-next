use {
    crate::{
        command_line::arguments::Decomposition,
        syntax_tree::fol::{Formula, FunctionConstant, Predicate, Sort, Theory},
    },
    anyhow::{Context as _, Result},
    indexmap::IndexSet,
    itertools::Itertools,
    std::{fmt, fs::File, io::Write as _, iter::repeat, path::Path},
};

// Increase problem number by counter
pub fn increment_problem_name(name: &ProblemNameTPTP, counter: usize) -> ProblemNameTPTP {
    let counter: i16 = counter.try_into().unwrap();
    let n: i16 = (100 * name.number[0]) + (10 * name.number[1]) + (name.number[2]) + counter;
    let d1: i16 = (n / 100).try_into().unwrap();
    let r1: i16 = (n % 100).try_into().unwrap();
    let d2: i16 = (r1 / 10).try_into().unwrap();
    let d3: i16 = (r1 % 10).try_into().unwrap();

    ProblemNameTPTP {
        domain: name.domain.clone(),
        number: vec![d1, d2, d3],
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct ProblemNameTPTP {
    pub domain: String,
    pub number: Vec<i16>,
}

impl fmt::Display for ProblemNameTPTP {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}{}{}{}",
            self.domain, self.number[0], self.number[1], self.number[2]
        )
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum Interpretation {
    Standard,
}

impl fmt::Display for Interpretation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, include_str!("standard_interpretation.p"))
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum Role {
    Axiom,
    Conjecture,
}

impl fmt::Display for Role {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Role::Axiom => write!(f, "axiom"),
            Role::Conjecture => write!(f, "conjecture"),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct AnnotatedFormula {
    pub name: String,
    pub role: Role,
    pub formula: Formula,
}

impl AnnotatedFormula {
    pub fn predicates(&self) -> IndexSet<Predicate> {
        self.formula.predicates()
    }

    pub fn symbols(&self) -> IndexSet<String> {
        self.formula.symbols()
    }

    pub fn function_constants(&self) -> IndexSet<FunctionConstant> {
        self.formula.function_constants()
    }

    pub fn rename_conflicting_symbols(self, possible_conflicts: &IndexSet<Predicate>) -> Self {
        AnnotatedFormula {
            name: self.name,
            role: self.role,
            formula: self.formula.rename_conflicting_symbols(possible_conflicts),
        }
    }
}

impl fmt::Display for AnnotatedFormula {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = &self.name;
        let role = &self.role;
        let formula = crate::formatting::fol::tptp::Format(&self.formula);
        writeln!(f, "tff({name}, {role}, {formula}).")
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Problem {
    pub name: ProblemNameTPTP,
    pub interpretation: Interpretation,
    pub formulas: Vec<AnnotatedFormula>,
}

impl Problem {
    pub fn with_name(name: ProblemNameTPTP) -> Problem {
        Problem {
            name,
            interpretation: Interpretation::Standard,
            formulas: vec![],
        }
    }

    pub fn add_annotated_formulas(
        mut self,
        annotated_formulas: impl IntoIterator<Item = AnnotatedFormula>,
    ) -> Self {
        for anf in annotated_formulas {
            if anf.name.is_empty() {
                self.formulas.push(AnnotatedFormula {
                    name: "unnamed_formula".to_string(),
                    role: anf.role,
                    formula: anf.formula,
                });
            } else if anf.name.starts_with('_') {
                self.formulas.push(AnnotatedFormula {
                    name: format!("f{}", anf.name),
                    role: anf.role,
                    formula: anf.formula,
                });
            } else {
                self.formulas.push(anf);
            }
        }
        self
    }

    pub fn add_theory<F>(mut self, theory: Theory, mut annotate: F) -> Self
    where
        F: FnMut(usize, Formula) -> AnnotatedFormula,
    {
        for (i, formula) in theory.formulas.into_iter().enumerate() {
            self.formulas.push(annotate(i, formula))
        }
        self
    }

    pub fn rename_conflicting_symbols(mut self) -> Self {
        let propositional_predicates =
            IndexSet::from_iter(self.predicates().into_iter().filter(|p| p.arity == 0));

        let formulas = self
            .formulas
            .into_iter()
            .map(|f| f.rename_conflicting_symbols(&propositional_predicates))
            .collect();
        self.formulas = formulas;
        self
    }

    // TODO: Improve naming scheme for formulas
    pub fn create_unique_formula_names(mut self) -> Self {
        let mut formulas = vec![];
        for (i, f) in self.formulas.into_iter().enumerate() {
            formulas.push(AnnotatedFormula {
                name: format!("formula_{i}_{}", f.name),
                role: f.role,
                formula: f.formula,
            });
        }
        self.formulas = formulas;
        self
    }

    pub fn axioms(&self) -> Vec<AnnotatedFormula> {
        self.formulas
            .iter()
            .filter(|f| f.role == Role::Axiom)
            .cloned()
            .collect_vec()
    }

    pub fn conjectures(&self) -> Vec<AnnotatedFormula> {
        self.formulas
            .iter()
            .filter(|f| f.role == Role::Conjecture)
            .cloned()
            .collect_vec()
    }

    pub fn predicates(&self) -> IndexSet<Predicate> {
        let mut result = IndexSet::new();
        for formula in &self.formulas {
            result.extend(formula.predicates())
        }
        result
    }

    pub fn symbols(&self) -> IndexSet<String> {
        let mut result = IndexSet::new();
        for formula in &self.formulas {
            result.extend(formula.symbols())
        }
        result
    }

    pub fn function_constants(&self) -> IndexSet<FunctionConstant> {
        let mut result = IndexSet::new();
        for formula in &self.formulas {
            result.extend(formula.function_constants())
        }
        result
    }

    pub fn decompose(&self, strategy: Decomposition) -> Vec<Self> {
        match strategy {
            Decomposition::Independent => self.decompose_independent(),
            Decomposition::Sequential => self.decompose_sequential(),
        }
    }

    pub fn decompose_independent(&self) -> Vec<Self> {
        let axioms = self.axioms();
        self.conjectures()
            .into_iter()
            .enumerate()
            .map(|(i, c)| {
                let mut formulas = axioms.clone();
                formulas.push(c);
                Problem {
                    name: increment_problem_name(&self.name, i),
                    interpretation: self.interpretation.clone(),
                    formulas,
                }
            })
            .collect_vec()
    }

    pub fn decompose_sequential(&self) -> Vec<Self> {
        let mut formulas = self.axioms();
        self.conjectures()
            .into_iter()
            .enumerate()
            .map(|(i, c)| {
                if let Some(last) = formulas.last_mut() {
                    last.role = Role::Axiom;
                }

                formulas.push(c);

                Problem {
                    name: increment_problem_name(&self.name, i),
                    interpretation: self.interpretation.clone(),
                    formulas: formulas.clone(),
                }
            })
            .collect_vec()
    }

    pub fn to_file<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let path = path.as_ref();
        let mut file = File::create(path)
            .with_context(|| format!("could not create file `{}`", path.display()))?;
        write!(file, "{self}").with_context(|| format!("could not write file `{}`", path.display()))
    }
}

impl fmt::Display for Problem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "include('standard_preamble.ax').")?;

        for (i, predicate) in self.predicates().into_iter().enumerate() {
            let symbol = predicate.symbol;
            // let input: String = repeat("general")
            //     .take(predicate.arity)
            //     .intersperse(" * ")
            //     .collect();
            let input: String =
                Itertools::intersperse(repeat("general").take(predicate.arity), " * ").collect();
            if predicate.arity > 0 {
                writeln!(f, "tff(predicate_{i}, type, {symbol}: ({input}) > $o).")?
            } else {
                writeln!(f, "tff(predicate_{i}, type, {symbol}: $o).")?
            }
        }

        for (i, symbol) in self.symbols().into_iter().enumerate() {
            writeln!(f, "tff(type_symbol_{i}, type, {symbol}: symbol).")?
        }

        for (i, constant) in self.function_constants().into_iter().enumerate() {
            let name = crate::formatting::fol::tptp::Format(&constant);
            let sort = match constant.sort {
                Sort::General => "general",
                Sort::Integer => "$int",
                Sort::Symbol => "symbol",
            };
            writeln!(f, "tff(type_function_constant_{i}, type, {name}: {sort}).")?
        }

        let mut symbols = Vec::from_iter(self.symbols());
        symbols.sort_unstable();
        for (i, s) in symbols.windows(2).enumerate() {
            writeln!(
                f,
                "tff(symbol_order_{i}, axiom, p__less__(f__symbolic__({}), f__symbolic__({}))).",
                s[0], s[1]
            )?
        }

        for formula in &self.formulas {
            formula.fmt(f)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use {
        super::{AnnotatedFormula, Interpretation, Problem, ProblemNameTPTP, Role, increment_problem_name},
        std::vec,
    };

    #[test]
    fn test_increment_problem_number() {
        let name = ProblemNameTPTP {
            domain: "SWV".into(),
            number: vec![0, 3, 5],
        };
        assert_eq!(
            increment_problem_name(&name, 1),
            ProblemNameTPTP {
                domain: "SWV".into(),
                number: vec![0, 3, 6],
            }
        );
        assert_eq!(
            increment_problem_name(&name, 5),
            ProblemNameTPTP {
                domain: "SWV".into(),
                number: vec![0, 4, 0],
            }
        );
        assert_eq!(
            increment_problem_name(&name, 182),
            ProblemNameTPTP {
                domain: "SWV".into(),
                number: vec![2, 1, 7],
            }
        );
    }

    #[test]
    fn test_decomposition() {
        let problem = Problem {
            name: ProblemNameTPTP {
                domain: "SWV".into(),
                number: vec![0, 0, 0],
            },
            interpretation: Interpretation::Standard,
            formulas: vec![
                AnnotatedFormula {
                    name: "axiom_0".into(),
                    role: Role::Axiom,
                    formula: "p(a)".parse().unwrap(),
                },
                AnnotatedFormula {
                    name: "axiom_1".into(),
                    role: Role::Axiom,
                    formula: "forall X p(X) -> q(X)".parse().unwrap(),
                },
                AnnotatedFormula {
                    name: "conjecture_0".into(),
                    role: Role::Conjecture,
                    formula: "p(a)".parse().unwrap(),
                },
                AnnotatedFormula {
                    name: "conjecture_1".into(),
                    role: Role::Conjecture,
                    formula: "q(a)".parse().unwrap(),
                },
            ],
        };

        assert_eq!(
            problem.decompose_independent(),
            vec![
                Problem {
                    name: ProblemNameTPTP {
                        domain: "SWV".into(),
                        number: vec![0, 0, 0],
                    },
                    interpretation: Interpretation::Standard,
                    formulas: vec![
                        AnnotatedFormula {
                            name: "axiom_0".into(),
                            role: Role::Axiom,
                            formula: "p(a)".parse().unwrap(),
                        },
                        AnnotatedFormula {
                            name: "axiom_1".into(),
                            role: Role::Axiom,
                            formula: "forall X p(X) -> q(X)".parse().unwrap(),
                        },
                        AnnotatedFormula {
                            name: "conjecture_0".into(),
                            role: Role::Conjecture,
                            formula: "p(a)".parse().unwrap(),
                        },
                    ],
                },
                Problem {
                    name: ProblemNameTPTP {
                        domain: "SWV".into(),
                        number: vec![0, 0, 1],
                    },
                    interpretation: Interpretation::Standard,
                    formulas: vec![
                        AnnotatedFormula {
                            name: "axiom_0".into(),
                            role: Role::Axiom,
                            formula: "p(a)".parse().unwrap(),
                        },
                        AnnotatedFormula {
                            name: "axiom_1".into(),
                            role: Role::Axiom,
                            formula: "forall X p(X) -> q(X)".parse().unwrap(),
                        },
                        AnnotatedFormula {
                            name: "conjecture_1".into(),
                            role: Role::Conjecture,
                            formula: "q(a)".parse().unwrap(),
                        },
                    ],
                }
            ]
        );

        assert_eq!(
            problem.decompose_sequential(),
            vec![
                Problem {
                    name: ProblemNameTPTP {
                        domain: "SWV".into(),
                        number: vec![0, 0, 0],
                    },
                    interpretation: Interpretation::Standard,
                    formulas: vec![
                        AnnotatedFormula {
                            name: "axiom_0".into(),
                            role: Role::Axiom,
                            formula: "p(a)".parse().unwrap(),
                        },
                        AnnotatedFormula {
                            name: "axiom_1".into(),
                            role: Role::Axiom,
                            formula: "forall X p(X) -> q(X)".parse().unwrap(),
                        },
                        AnnotatedFormula {
                            name: "conjecture_0".into(),
                            role: Role::Conjecture,
                            formula: "p(a)".parse().unwrap(),
                        },
                    ],
                },
                Problem {
                    name: ProblemNameTPTP {
                        domain: "SWV".into(),
                        number: vec![0, 0, 1],
                    },
                    interpretation: Interpretation::Standard,
                    formulas: vec![
                        AnnotatedFormula {
                            name: "axiom_0".into(),
                            role: Role::Axiom,
                            formula: "p(a)".parse().unwrap(),
                        },
                        AnnotatedFormula {
                            name: "axiom_1".into(),
                            role: Role::Axiom,
                            formula: "forall X p(X) -> q(X)".parse().unwrap(),
                        },
                        AnnotatedFormula {
                            name: "conjecture_0".into(),
                            role: Role::Axiom,
                            formula: "p(a)".parse().unwrap(),
                        },
                        AnnotatedFormula {
                            name: "conjecture_1".into(),
                            role: Role::Conjecture,
                            formula: "q(a)".parse().unwrap(),
                        },
                    ],
                }
            ]
        );
    }
}
