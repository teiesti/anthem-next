use {
    crate::{
        command_line::Decomposition,
        syntax_tree::{asp, fol},
        verifying::{
            problem::{self, AnnotatedFormula, Problem},
            task::Task,
        },
    },
    either::Either,
    thiserror::Error,
};

struct ProofOutline {
    pub forward_basic_lemmas: Vec<fol::AnnotatedFormula>,
    pub backward_basic_lemmas: Vec<fol::AnnotatedFormula>,
}

#[derive(Error, Debug)]
pub enum ExternalEquivalenceTaskError {}

#[derive(Debug)]
pub struct ExternalEquivalenceTask {
    pub specification: Either<asp::Program, fol::Specification>,
    pub program: asp::Program,
    pub user_guide: fol::UserGuide,
    pub proof_outline: fol::Specification,
    pub decomposition: Decomposition,
    pub direction: fol::Direction,
    pub simplify: bool,
    pub break_equivalences: bool,
}

impl Task for ExternalEquivalenceTask {
    type Error = ExternalEquivalenceTaskError;

    fn decompose(self) -> Result<Vec<Problem>, Self::Error> {
        // let task: ValidatedExternalEquivalenceTask = todo!();
        // task.decompose()
        todo!()
    }
}

struct ValidatedExternalEquivalenceTask {
    pub left: Vec<fol::AnnotatedFormula>,
    pub right: Vec<fol::AnnotatedFormula>,
    pub user_guide_assumptions: Vec<fol::AnnotatedFormula>,
    pub proof_outline: ProofOutline,
    pub decomposition: Decomposition,
    pub direction: fol::Direction,
    pub break_equivalences: bool,
}

impl Task for ValidatedExternalEquivalenceTask {
    type Error = ExternalEquivalenceTaskError;

    fn decompose(self) -> Result<Vec<Problem>, Self::Error> {
        let mut stable_premises: Vec<problem::AnnotatedFormula> = Vec::new();
        let mut forward_premises: Vec<problem::AnnotatedFormula> = Vec::new();
        let mut forward_conclusions: Vec<problem::AnnotatedFormula> = Vec::new();
        let mut backward_premises: Vec<problem::AnnotatedFormula> = Vec::new();
        let mut backward_conclusions: Vec<problem::AnnotatedFormula> = Vec::new();

        for assumption in self.user_guide_assumptions {
            stable_premises.push(AnnotatedFormula::from((assumption, problem::Role::Axiom)));
        }

        // S, F |= B
        for formula in self.left {
            match formula {
                fol::AnnotatedFormula {
                    role: fol::Role::Assumption,
                    direction,
                    formula: ref f,
                    ..
                } => match direction {
                    fol::Direction::Universal => stable_premises
                        .push(AnnotatedFormula::from((formula, problem::Role::Axiom))),
                    fol::Direction::Forward => forward_premises
                        .push(AnnotatedFormula::from((formula, problem::Role::Axiom))),
                    fol::Direction::Backward => println!(
                        "A backward assumption has no effect in this context. Ignoring formula {}",
                        f
                    ),
                },

                fol::AnnotatedFormula {
                    role: fol::Role::Spec,
                    direction,
                    ..
                } => match direction {
                    fol::Direction::Universal => {
                        forward_premises.push(AnnotatedFormula::from((
                            formula.clone(),
                            problem::Role::Axiom,
                        )));
                        if self.break_equivalences {
                            let conjectures = formula.break_equivalences();
                            for c in conjectures {
                                backward_conclusions
                                    .push(AnnotatedFormula::from((c, problem::Role::Conjecture)));
                            }
                        } else {
                            backward_conclusions
                                .push(AnnotatedFormula::from((formula, problem::Role::Conjecture)));
                        }
                    }
                    fol::Direction::Forward => {
                        forward_premises
                            .push(AnnotatedFormula::from((formula, problem::Role::Axiom)));
                    }
                    fol::Direction::Backward => {
                        if self.break_equivalences {
                            let conjectures = formula.break_equivalences();
                            for c in conjectures {
                                backward_conclusions
                                    .push(AnnotatedFormula::from((c, problem::Role::Conjecture)));
                            }
                        } else {
                            backward_conclusions
                                .push(AnnotatedFormula::from((formula, problem::Role::Conjecture)));
                        }
                    }
                },

                _ => todo!(), // error
            }
        }

        // S, B |= F
        for formula in self.right {
            match formula {
                fol::AnnotatedFormula {
                    role: fol::Role::Assumption,
                    direction,
                    formula: ref f,
                    ..
                } => match direction {
                    fol::Direction::Universal => stable_premises
                        .push(AnnotatedFormula::from((formula, problem::Role::Axiom))),
                    fol::Direction::Forward => println!(
                        "A forward assumption has no effect in this context. Ignoring formula {}",
                        f
                    ),
                    fol::Direction::Backward => backward_premises
                        .push(AnnotatedFormula::from((formula, problem::Role::Axiom))),
                },

                fol::AnnotatedFormula {
                    role: fol::Role::Spec,
                    direction,
                    ..
                } => match direction {
                    fol::Direction::Universal => {
                        backward_premises.push(AnnotatedFormula::from((
                            formula.clone(),
                            problem::Role::Axiom,
                        )));
                        if self.break_equivalences {
                            let conjectures = formula.break_equivalences();
                            for c in conjectures {
                                forward_conclusions
                                    .push(AnnotatedFormula::from((c, problem::Role::Conjecture)));
                            }
                        } else {
                            forward_conclusions
                                .push(AnnotatedFormula::from((formula, problem::Role::Conjecture)));
                        }
                    }
                    fol::Direction::Forward => {
                        backward_premises
                            .push(AnnotatedFormula::from((formula, problem::Role::Axiom)));
                    }
                    fol::Direction::Backward => {
                        if self.break_equivalences {
                            let conjectures = formula.break_equivalences();
                            for c in conjectures {
                                forward_conclusions
                                    .push(AnnotatedFormula::from((c, problem::Role::Conjecture)));
                            }
                        } else {
                            forward_conclusions
                                .push(AnnotatedFormula::from((formula, problem::Role::Conjecture)));
                        }
                    }
                },

                _ => todo!(), // error
            }
        }

        let task = AssembledExternalEquivalenceTask {
            stable_premises,
            forward_premises,
            forward_conclusions,
            backward_premises,
            backward_conclusions,
            proof_outline: self.proof_outline,
            decomposition: self.decomposition,
            direction: self.direction,
        };
        task.decompose()
    }
}

struct AssembledExternalEquivalenceTask {
    pub stable_premises: Vec<problem::AnnotatedFormula>,
    pub forward_premises: Vec<problem::AnnotatedFormula>,
    pub forward_conclusions: Vec<problem::AnnotatedFormula>,
    pub backward_premises: Vec<problem::AnnotatedFormula>,
    pub backward_conclusions: Vec<problem::AnnotatedFormula>,
    pub proof_outline: ProofOutline,
    pub decomposition: Decomposition,
    pub direction: fol::Direction,
}

impl Task for AssembledExternalEquivalenceTask {
    type Error = ExternalEquivalenceTaskError;

    fn decompose(self) -> Result<Vec<Problem>, Self::Error> {
        let mut problems = Vec::new();
        if matches!(
            self.direction,
            fol::Direction::Universal | fol::Direction::Forward
        ) {
            let mut forward_sequence = Problem::from_components(
                "forward".to_string(),
                self.stable_premises.clone(),
                self.forward_premises,
                self.forward_conclusions,
                self.proof_outline.forward_basic_lemmas,
            );
            problems.append(&mut forward_sequence);
        }
        if matches!(
            self.direction,
            fol::Direction::Universal | fol::Direction::Backward
        ) {
            let mut backward_sequence = Problem::from_components(
                "backward".to_string(),
                self.stable_premises,
                self.backward_premises,
                self.backward_conclusions,
                self.proof_outline.backward_basic_lemmas,
            );
            problems.append(&mut backward_sequence);
        }

        let result: Vec<Problem> = problems
            .into_iter()
            .flat_map(|p: Problem| match self.decomposition {
                Decomposition::Independent => p.decompose_independent(),
                Decomposition::Sequential => p.decompose_sequential(),
            })
            .collect();

        Ok(result)
    }
}
