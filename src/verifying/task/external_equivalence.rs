use {
    crate::{
        command_line::Decomposition,
        convenience::with_warnings::{Result, WithWarnings},
        syntax_tree::{asp, fol},
        verifying::{
            problem::{self, Problem},
            task::Task,
        },
    },
    either::Either,
    indexmap::IndexSet,
    std::fmt::Display,
    thiserror::Error,
};

// If all the conjectures are proven,
// then all consequences can be added as axioms to the next proof step
// A basic lemma F has conjectures [F] and consequences [F]
// An inductive lemma F has conjectures [Base, Step] and axioms [F]
#[derive(Clone, Debug, PartialEq)]
struct GeneralLemma {
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

#[derive(Error, Debug)]
enum ProofOutlineError {
    #[error("the following annotated formula has a role that is forbidden in proof outlines: {0}")]
    AnnotatedFormulaWithInvalidRole(fol::AnnotatedFormula),
}

struct ProofOutline {
    pub forward_lemmas: Vec<GeneralLemma>,
    pub backward_lemmas: Vec<GeneralLemma>,
}

impl ProofOutline {
    fn from_specification(
        specification: fol::Specification,
    ) -> std::result::Result<Self, ProofOutlineError> {
        let mut forward_lemmas = Vec::new();
        let mut backward_lemmas = Vec::new();

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
                fol::Role::Definition => todo!(),
                fol::Role::Assumption | fol::Role::Spec => {
                    return Err(ProofOutlineError::AnnotatedFormulaWithInvalidRole(anf))
                }
            }
        }

        Ok(ProofOutline {
            forward_lemmas,
            backward_lemmas,
        })
    }
}

#[derive(Error, Debug)]
pub enum ExternalEquivalenceTaskWarning {}

#[derive(Error, Debug)]
pub enum ExternalEquivalenceTaskError {
    InputOutputPredicatesOverlap(Vec<fol::Predicate>),
    InputPredicateInRuleHead(Vec<fol::Predicate>),
}

impl Display for ExternalEquivalenceTaskError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ExternalEquivalenceTaskError::InputOutputPredicatesOverlap(predicates) => {
                write!(
                    f,
                    "the following predicates are declared as input and output predicates: "
                )?;

                let mut iter = predicates.iter().peekable();
                for predicate in predicates {
                    write!(f, "{predicate}")?;
                    if iter.peek().is_some() {
                        write!(f, ", ")?;
                    }
                }

                writeln!(f)
            }
            ExternalEquivalenceTaskError::InputPredicateInRuleHead(predicates) => {
                write!(f, "the following input predicates occur in rule heads: ")?;

                let mut iter = predicates.iter().peekable();
                for predicate in predicates {
                    write!(f, "{predicate}")?;
                    if iter.peek().is_some() {
                        write!(f, ", ")?;
                    }
                }

                writeln!(f)
            }
        }
    }
}

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

impl ExternalEquivalenceTask {
    fn ensure_input_and_output_predicates_are_disjoint(
        &self,
    ) -> Result<(), ExternalEquivalenceTaskWarning, ExternalEquivalenceTaskError> {
        let input_predicates = self.user_guide.input_predicates();
        let output_predicates = self.user_guide.output_predicates();

        let intersection: Vec<_> = input_predicates
            .intersection(&output_predicates)
            .cloned()
            .collect();

        if intersection.is_empty() {
            Ok(WithWarnings::flawless(()))
        } else {
            Err(ExternalEquivalenceTaskError::InputOutputPredicatesOverlap(
                intersection,
            ))
        }
    }

    fn ensure_program_heads_do_not_contain_input_predicates(
        &self,
    ) -> Result<(), ExternalEquivalenceTaskWarning, ExternalEquivalenceTaskError> {
        let input_predicates = self.user_guide.input_predicates();
        let head_predicates: IndexSet<_> = self
            .program
            .head_predicates()
            .into_iter()
            .map(fol::Predicate::from)
            .collect();

        let intersection: Vec<_> = input_predicates
            .intersection(&head_predicates)
            .cloned()
            .collect();

        if intersection.is_empty() {
            Ok(WithWarnings::flawless(()))
        } else {
            Err(ExternalEquivalenceTaskError::InputPredicateInRuleHead(
                intersection,
            ))
        }
    }
}

impl Task for ExternalEquivalenceTask {
    type Error = ExternalEquivalenceTaskError;
    type Warning = ExternalEquivalenceTaskWarning;

    fn decompose(self) -> Result<Vec<Problem>, Self::Warning, Self::Error> {
        self.ensure_input_and_output_predicates_are_disjoint()?;
        self.ensure_program_heads_do_not_contain_input_predicates()?;
        // TODO: Add more error handing

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
    type Warning = ExternalEquivalenceTaskWarning;

    fn decompose(self) -> Result<Vec<Problem>, Self::Warning, Self::Error> {
        todo!()
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
    type Warning = ExternalEquivalenceTaskWarning;

    fn decompose(self) -> Result<Vec<Problem>, Self::Warning, Self::Error> {
        let mut problems = Vec::new();

        if matches!(
            self.direction,
            fol::Direction::Universal | fol::Direction::Forward
        ) {
            let mut axioms = self.stable_premises.clone();
            axioms.extend(self.forward_premises.clone());

            for (i, lemma) in self.proof_outline.forward_lemmas.iter().enumerate() {
                for (j, conjecture) in lemma.conjectures.iter().enumerate() {
                    problems.push(
                        Problem::with_name(format!("forward_outline_{i}_{j}"))
                            .add_annotated_formulas(axioms.clone())
                            .add_annotated_formulas(std::iter::once(conjecture.clone())),
                    );
                }
                axioms.append(&mut lemma.consequences.clone());
            }

            problems.append(
                &mut Problem::with_name("forward_problem")
                    .add_annotated_formulas(self.stable_premises.clone())
                    .add_annotated_formulas(self.forward_premises)
                    .add_annotated_formulas(
                        self.proof_outline
                            .forward_lemmas
                            .into_iter()
                            .flat_map(|g: GeneralLemma| g.consequences.into_iter()),
                    )
                    .add_annotated_formulas(self.forward_conclusions)
                    .decompose(self.decomposition),
            );
        }

        if matches!(
            self.direction,
            fol::Direction::Universal | fol::Direction::Backward
        ) {
            let mut axioms = self.stable_premises.clone();
            axioms.extend(self.backward_premises.clone());

            for (i, lemma) in self.proof_outline.backward_lemmas.iter().enumerate() {
                for (j, conjecture) in lemma.conjectures.iter().enumerate() {
                    problems.push(
                        Problem::with_name(format!("backward_outline_{i}_{j}"))
                            .add_annotated_formulas(axioms.clone())
                            .add_annotated_formulas(std::iter::once(conjecture.clone())),
                    );
                }
                axioms.append(&mut lemma.consequences.clone());
            }

            problems.append(
                &mut Problem::with_name("backward_problem")
                    .add_annotated_formulas(self.stable_premises)
                    .add_annotated_formulas(self.backward_premises)
                    .add_annotated_formulas(
                        self.proof_outline
                            .backward_lemmas
                            .into_iter()
                            .flat_map(|g: GeneralLemma| g.consequences.into_iter()),
                    )
                    .add_annotated_formulas(self.backward_conclusions)
                    .decompose(self.decomposition),
            );
        }

        Ok(WithWarnings::flawless(problems))
    }
}
