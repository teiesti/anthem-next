use {
    crate::{
        convenience::{
            unbox::{fol::UnboxedFormula, Unbox as _},
            with_warnings::{Result, WithWarnings},
        },
        syntax_tree::fol,
        verifying::problem,
    },
    indexmap::IndexSet,
    std::fmt::Display,
    thiserror::Error,
};

// If all the conjectures are proven,
// then all consequences can be added as axioms to the next proof step
// A basic lemma F has conjectures [F] and consequences [F]
// An inductive lemma F has conjectures [Base, Step] and axioms [F]
#[derive(Clone, Debug, PartialEq)]
pub struct GeneralLemma {
    pub conjectures: Vec<problem::AnnotatedFormula>,
    pub consequences: Vec<problem::AnnotatedFormula>,
}

impl TryFrom<fol::AnnotatedFormula> for GeneralLemma {
    type Error = fol::AnnotatedFormula;

    fn try_from(
        annotated_formula: fol::AnnotatedFormula,
    ) -> std::result::Result<Self, Self::Error> {
        match annotated_formula.role {
            fol::Role::Lemma => Ok(GeneralLemma {
                conjectures: vec![annotated_formula
                    .clone()
                    .into_problem_formula(problem::Role::Conjecture)],
                consequences: vec![annotated_formula.into_problem_formula(problem::Role::Axiom)],
            }),
            // TODO: Add inductive lemmas!
            fol::Role::Assumption | fol::Role::Spec | fol::Role::Definition => {
                Err(annotated_formula)
            }
        }
    }
}

// TODO: Think about the name
trait CheckInternal {
    // Returns the predicate defined in the LHS of the formula if it is a valid definition, else returns an error
    fn definition(
        &self,
        taken_predicates: &IndexSet<fol::Predicate>,
    ) -> Result<fol::Predicate, ProofOutlineWarning, ProofOutlineError>;
}

impl CheckInternal for fol::Formula {
    fn definition(
        &self,
        taken_predicates: &IndexSet<fol::Predicate>,
    ) -> Result<fol::Predicate, ProofOutlineWarning, ProofOutlineError> {
        match self.clone().unbox() {
            UnboxedFormula::QuantifiedFormula {
                quantification:
                    fol::Quantification {
                        quantifier: fol::Quantifier::Forall,
                        variables,
                    },
                formula:
                    fol::Formula::BinaryFormula {
                        connective: fol::BinaryConnective::Equivalence,
                        lhs,
                        rhs,
                    },
            } => match lhs.unbox() {
                UnboxedFormula::AtomicFormula(fol::AtomicFormula::Atom(a)) => {
                    let mut warnings = Vec::new();

                    // check variables has no duplicates
                    let len = variables.len();
                    let uniques: IndexSet<fol::Variable> = IndexSet::from_iter(variables);
                    if uniques.len() < len {
                        return Err(ProofOutlineError::DuplicatedVariables(self.clone()));
                    }

                    // TODO: Check variables in quantifications are the same as the terms in the atom

                    // check predicate is totally fresh
                    let predicate = a.predicate();
                    if taken_predicates.contains(&predicate) {
                        return Err(ProofOutlineError::TakenPredicate(predicate));
                    }

                    // check RHS has no free variables other than those in uniques
                    if rhs.free_variables().difference(&uniques).next().is_some() {
                        return Err(ProofOutlineError::FreeRhsVariables(self.clone()));
                    }

                    // warn the user if the RHS is missing some variable from the quantification
                    if uniques.difference(&rhs.free_variables()).next().is_some() {
                        warnings.push(ProofOutlineWarning::ExcessQuantifiedVariables(self.clone()));
                    }

                    // check RHS has no predicates other than taken predicates
                    // this should ensure no recursion through definition sequence
                    if let Some(predicate) = rhs.predicates().difference(taken_predicates).next() {
                        return Err(ProofOutlineError::UndefinedRhsPredicate {
                            definition: self.clone(),
                            predicate: predicate.clone(),
                        });
                    }

                    Ok(WithWarnings::flawless(predicate).preface_warnings(warnings))
                }
                _ => Err(ProofOutlineError::MalformedDefinition(self.clone())),
            },

            _ => Err(ProofOutlineError::MalformedDefinition(self.clone())),
        }
    }
}

#[derive(Error, Debug, PartialEq)]
pub enum ProofOutlineError {
    #[error("the following annotated formula has a role that is forbidden in proof outlines: {0}")]
    AnnotatedFormulaWithInvalidRole(fol::AnnotatedFormula),
    #[error(
        "the following definiton contains duplicated variables in outermost quantification: {0}"
    )]
    DuplicatedVariables(fol::Formula),
    #[error("definitions require fresh predicates but the following predicate is taken: {0}")]
    TakenPredicate(fol::Predicate),
    #[error("the following definition contains free variables in the RHS: {0}")]
    FreeRhsVariables(fol::Formula),
    #[error("undefined predicate -- `{predicate}` occurs for the first time in the RHS of definition `{definition}`")]
    UndefinedRhsPredicate {
        definition: fol::Formula,
        predicate: fol::Predicate,
    },
    #[error("the follwing definition is malformed: {0}")]
    MalformedDefinition(fol::Formula),
}

pub struct ProofOutline {
    pub forward_lemmas: Vec<GeneralLemma>,
    pub backward_lemmas: Vec<GeneralLemma>,
    pub forward_definitions: Vec<fol::AnnotatedFormula>,
    pub backward_definitions: Vec<fol::AnnotatedFormula>,
}

#[derive(Error, Debug, Eq, PartialEq)]
pub enum ProofOutlineWarning {
    ExcessQuantifiedVariables(fol::Formula),
}

impl Display for ProofOutlineWarning {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProofOutlineWarning::ExcessQuantifiedVariables(formula) => {
                writeln!(f, "the universally quantified list of variables contains members which do not occur in the RHS of {formula}")
            }
        }
    }
}

impl ProofOutline {
    pub fn from_specification(
        specification: fol::Specification,
        mut taken_predicates: IndexSet<fol::Predicate>,
    ) -> Result<Self, ProofOutlineWarning, ProofOutlineError> {
        let mut warnings = Vec::new();

        let mut forward_lemmas = Vec::new();
        let mut backward_lemmas = Vec::new();
        let mut forward_definitions = Vec::new();
        let mut backward_definitions = Vec::new();

        for anf in specification.formulas {
            match anf.role {
                fol::Role::Lemma => match anf.direction {
                    // TODO: Revisit the unwraps when implementing inductive lemmas
                    fol::Direction::Universal => {
                        forward_lemmas.push(anf.clone().try_into().unwrap());
                        backward_lemmas.push(anf.try_into().unwrap());
                    }
                    fol::Direction::Forward => forward_lemmas.push(anf.try_into().unwrap()),
                    fol::Direction::Backward => backward_lemmas.push(anf.try_into().unwrap()),
                },
                fol::Role::Definition => {
                    let predicate = anf.formula.definition(&taken_predicates)?;
                    taken_predicates.insert(predicate.data);
                    warnings.extend(predicate.warnings);
                    match anf.direction {
                        fol::Direction::Forward => {
                            forward_definitions.push(anf);
                        }
                        fol::Direction::Backward => {
                            backward_definitions.push(anf);
                        }
                        fol::Direction::Universal => {
                            forward_definitions.push(anf.clone());
                            backward_definitions.push(anf);
                        }
                    }
                }
                fol::Role::Assumption | fol::Role::Spec => {
                    return Err(ProofOutlineError::AnnotatedFormulaWithInvalidRole(anf))
                }
            }
        }

        Ok(WithWarnings::flawless(ProofOutline {
            forward_lemmas,
            backward_lemmas,
            forward_definitions,
            backward_definitions,
        })
        .preface_warnings(warnings))
    }
}

#[cfg(test)]
mod tests {
    use {
        super::{CheckInternal, ProofOutlineError},
        crate::syntax_tree::fol,
        indexmap::IndexSet,
    };

    #[test]
    fn check_correct_definition() {
        for (src, target) in [
            (
                "forall X ( p(X) <-> 1 < 2 )",
                fol::Predicate {
                    symbol: "p".to_string(),
                    arity: 1,
                },
            ),
            (
                "forall X Y$i ( pred(X, Y$i) <-> exists N$i (X = N$i and t(X) or t(Y$i)) )",
                fol::Predicate {
                    symbol: "pred".to_string(),
                    arity: 2,
                },
            ),
        ] {
            let taken_predicates: IndexSet<fol::Predicate> =
                IndexSet::from_iter(vec![fol::Predicate {
                    symbol: "t".to_string(),
                    arity: 1,
                }]);
            let formula: fol::Formula = src.parse().unwrap();
            assert_eq!(formula.definition(&taken_predicates).unwrap().data, target)
        }
    }

    #[test]
    fn check_incorrect_definition() {
        for (src, target) in [
            (
                "forall X Y X ( p(X) <-> 1 < 2 )",
                ProofOutlineError::DuplicatedVariables(
                    "forall X Y X ( p(X) <-> 1 < 2 )".parse().unwrap(),
                ),
            ),
            (
                "forall X ( t(X) <-> 1 < 2 )",
                ProofOutlineError::TakenPredicate(fol::Predicate {
                    symbol: "t".to_string(),
                    arity: 1,
                }),
            ),
            (
                "forall Z1 Z2 ( ancestor(Z1, Z2) <-> t(X) and t(Z2) )",
                ProofOutlineError::FreeRhsVariables(
                    "forall Z1 Z2 ( ancestor(Z1, Z2) <-> t(X) and t(Z2) )"
                        .parse()
                        .unwrap(),
                ),
            ),
            (
                "forall Z1 Z2 ( ancestor(Z1, Z2) <-> ancestor(Z1, Z2) )",
                ProofOutlineError::UndefinedRhsPredicate {
                    definition: "forall Z1 Z2 ( ancestor(Z1, Z2) <-> ancestor(Z1, Z2) )"
                        .parse()
                        .unwrap(),
                    predicate: fol::Predicate {
                        symbol: "ancestor".to_string(),
                        arity: 2,
                    },
                },
            ),
        ] {
            let taken_predicates: IndexSet<fol::Predicate> =
                IndexSet::from_iter(vec![fol::Predicate {
                    symbol: "t".to_string(),
                    arity: 1,
                }]);
            let formula: fol::Formula = src.parse().unwrap();
            assert_eq!(formula.definition(&taken_predicates), Err(target))
        }
    }
}
