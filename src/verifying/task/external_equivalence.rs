use {
    crate::{
        command_line::Decomposition,
        convenience::{
            apply::Apply as _,
            with_warnings::{Result, WithWarnings},
        },
        syntax_tree::{asp, fol},
        translating::{completion::completion, tau_star::tau_star},
        verifying::{
            problem::{self, Problem},
            task::Task,
        },
    },
    either::Either,
    indexmap::{IndexMap, IndexSet},
    std::fmt::Display,
    thiserror::Error,
};

// TODO: The following could be much easier with an enum over all types of nodes which implements the apply trait
trait ReplacePlaceholders {
    fn replace_placeholders(self, mapping: &IndexMap<String, fol::FunctionConstant>) -> Self;
}

impl ReplacePlaceholders for fol::Specification {
    fn replace_placeholders(self, mapping: &IndexMap<String, fol::FunctionConstant>) -> Self {
        fol::Specification {
            formulas: self
                .formulas
                .into_iter()
                .map(|f| f.replace_placeholders(mapping))
                .collect(),
        }
    }
}

impl ReplacePlaceholders for fol::AnnotatedFormula {
    fn replace_placeholders(mut self, mapping: &IndexMap<String, fol::FunctionConstant>) -> Self {
        self.formula = self.formula.replace_placeholders(mapping);
        self
    }
}

impl ReplacePlaceholders for fol::Theory {
    fn replace_placeholders(self, mapping: &IndexMap<String, fol::FunctionConstant>) -> Self {
        fol::Theory {
            formulas: self
                .formulas
                .into_iter()
                .map(|f| f.replace_placeholders(mapping))
                .collect(),
        }
    }
}

impl ReplacePlaceholders for fol::Formula {
    fn replace_placeholders(self, mapping: &IndexMap<String, fol::FunctionConstant>) -> Self {
        self.apply(&mut |formula| match formula {
            fol::Formula::AtomicFormula(a) => {
                fol::Formula::AtomicFormula(a.replace_placeholders(mapping))
            }
            x => x,
        })
    }
}

impl ReplacePlaceholders for fol::AtomicFormula {
    fn replace_placeholders(self, mapping: &IndexMap<String, fol::FunctionConstant>) -> Self {
        match self {
            fol::AtomicFormula::Atom(a) => {
                fol::AtomicFormula::Atom(a.replace_placeholders(mapping))
            }
            fol::AtomicFormula::Comparison(c) => {
                fol::AtomicFormula::Comparison(c.replace_placeholders(mapping))
            }
            x => x,
        }
    }
}

impl ReplacePlaceholders for fol::Atom {
    fn replace_placeholders(self, mapping: &IndexMap<String, fol::FunctionConstant>) -> Self {
        fol::Atom {
            predicate_symbol: self.predicate_symbol,
            terms: self
                .terms
                .into_iter()
                .map(|t| t.replace_placeholders(mapping))
                .collect(),
        }
    }
}

impl ReplacePlaceholders for fol::Comparison {
    fn replace_placeholders(self, mapping: &IndexMap<String, fol::FunctionConstant>) -> Self {
        fol::Comparison {
            term: self.term.replace_placeholders(mapping),
            guards: self
                .guards
                .into_iter()
                .map(|g| g.replace_placeholders(mapping))
                .collect(),
        }
    }
}

impl ReplacePlaceholders for fol::Guard {
    fn replace_placeholders(self, mapping: &IndexMap<String, fol::FunctionConstant>) -> Self {
        fol::Guard {
            relation: self.relation,
            term: self.term.replace_placeholders(mapping),
        }
    }
}

impl ReplacePlaceholders for fol::GeneralTerm {
    fn replace_placeholders(self, mapping: &IndexMap<String, fol::FunctionConstant>) -> Self {
        match self {
            fol::GeneralTerm::SymbolicTerm(fol::SymbolicTerm::Symbol(s)) => {
                if let Some(fc) = mapping.get(&s) {
                    match fc.sort {
                        fol::Sort::General => fol::GeneralTerm::FunctionConstant(s),
                        fol::Sort::Integer => {
                            fol::GeneralTerm::IntegerTerm(fol::IntegerTerm::FunctionConstant(s))
                        }
                        fol::Sort::Symbol => {
                            fol::GeneralTerm::SymbolicTerm(fol::SymbolicTerm::FunctionConstant(s))
                        }
                    }
                } else {
                    fol::GeneralTerm::SymbolicTerm(fol::SymbolicTerm::Symbol(s))
                }
            }
            x => x,
        }
    }
}

trait RenamePredicates {
    fn rename_predicates(self, mapping: &IndexMap<fol::Predicate, String>) -> Self;
}

impl RenamePredicates for fol::Specification {
    fn rename_predicates(self, mapping: &IndexMap<fol::Predicate, String>) -> Self {
        fol::Specification {
            formulas: self
                .formulas
                .into_iter()
                .map(|f| f.rename_predicates(mapping))
                .collect(),
        }
    }
}

impl RenamePredicates for fol::AnnotatedFormula {
    fn rename_predicates(mut self, mapping: &IndexMap<fol::Predicate, String>) -> Self {
        self.formula = self.formula.rename_predicates(mapping);
        self
    }
}

impl RenamePredicates for fol::Formula {
    fn rename_predicates(self, mapping: &IndexMap<fol::Predicate, String>) -> Self {
        self.apply(&mut |formula| match formula {
            fol::Formula::AtomicFormula(a) => {
                fol::Formula::AtomicFormula(a.rename_predicates(mapping))
            }
            x => x,
        })
    }
}

impl RenamePredicates for fol::AtomicFormula {
    fn rename_predicates(self, mapping: &IndexMap<fol::Predicate, String>) -> Self {
        match self {
            fol::AtomicFormula::Atom(a) => fol::AtomicFormula::Atom(a.rename_predicates(mapping)),
            x => x,
        }
    }
}

impl RenamePredicates for fol::Atom {
    fn rename_predicates(self, mapping: &IndexMap<fol::Predicate, String>) -> Self {
        match mapping.get(&self.predicate()) {
            Some(name_extension) => fol::Atom {
                predicate_symbol: format!("{}_{}", self.predicate_symbol, name_extension),
                terms: self.terms,
            },
            None => self,
        }
    }
}

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
pub enum ProofOutlineError {
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
pub enum ExternalEquivalenceTaskWarning {
    InconsistentDirectionAnnotation(fol::AnnotatedFormula),
    InvalidRoleWithinUserGuide(fol::AnnotatedFormula),
}

impl Display for ExternalEquivalenceTaskWarning {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ExternalEquivalenceTaskWarning::InconsistentDirectionAnnotation(formula) => {
                let proof_direction = match formula.direction {
                    fol::Direction::Forward => fol::Direction::Backward,
                    fol::Direction::Backward => fol::Direction::Forward,
                    fol::Direction::Universal => unreachable!(),
                };

                writeln!(
                    f,
                    "the following assumption is ignored in the {proof_direction} direction of the proof due its annotated direction: {formula}"
                )
            },
            ExternalEquivalenceTaskWarning::InvalidRoleWithinUserGuide(formula) => writeln!(
                f, "the following formula is ignored because user guides only permit assumptions: {formula}"
            ),
        }
    }
}

#[derive(Error, Debug)]
pub enum ExternalEquivalenceTaskError {
    InputOutputPredicatesOverlap(Vec<fol::Predicate>),
    InputPredicateInRuleHead(Vec<fol::Predicate>),
    OutputPredicateInUserGuideAssumption(Vec<fol::Predicate>),
    OutputPredicateInSpecificationAssumption(Vec<fol::Predicate>),
    ProofOutlineError(#[from] ProofOutlineError),
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
            ExternalEquivalenceTaskError::OutputPredicateInUserGuideAssumption(predicates) => {
                write!(
                    f,
                    "the following output predicates occur in user guide assumptions: "
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
            ExternalEquivalenceTaskError::OutputPredicateInSpecificationAssumption(predicates) => {
                write!(
                    f,
                    "the following output predicates occur in specification assumptions: "
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
            ExternalEquivalenceTaskError::ProofOutlineError(_) => {
                writeln!(f, "the given proof outline contains errors")
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

    fn ensure_rule_heads_do_not_contain_input_predicates(
        &self,
        program: &asp::Program,
    ) -> Result<(), ExternalEquivalenceTaskWarning, ExternalEquivalenceTaskError> {
        let input_predicates = self.user_guide.input_predicates();
        let head_predicates: IndexSet<_> = program
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

    fn ensure_specification_assumptions_do_not_contain_output_predicates(
        &self,
        specification: &fol::Specification,
    ) -> Result<(), ExternalEquivalenceTaskWarning, ExternalEquivalenceTaskError> {
        let output_predicates = self.user_guide.output_predicates();

        for formula in &specification.formulas {
            if matches!(formula.role, fol::Role::Assumption) {
                let overlap: Vec<_> = formula
                    .predicates()
                    .into_iter()
                    .filter(|p| output_predicates.contains(p))
                    .collect();

                if !overlap.is_empty() {
                    return Err(
                        ExternalEquivalenceTaskError::OutputPredicateInSpecificationAssumption(
                            overlap,
                        ),
                    );
                }
            }
        }

        Ok(WithWarnings::flawless(()))
    }
}

impl Task for ExternalEquivalenceTask {
    type Error = ExternalEquivalenceTaskError;
    type Warning = ExternalEquivalenceTaskWarning;

    fn decompose(self) -> Result<Vec<Problem>, Self::Warning, Self::Error> {
        self.ensure_input_and_output_predicates_are_disjoint()?;
        self.ensure_rule_heads_do_not_contain_input_predicates(&self.program)?;

        match self.specification {
            Either::Left(ref program) => {
                self.ensure_rule_heads_do_not_contain_input_predicates(program)?;
            }
            Either::Right(ref specification) => {
                self.ensure_specification_assumptions_do_not_contain_output_predicates(
                    specification,
                )?;
            }
        }

        // TODO: Ensure assumption in user guides and first-order specification only contain input symbols
        // TODO: Ensure placeholder name uniqueness?
        // TODO: Add more error handing

        let mut warnings = Vec::new();

        let placeholders = self
            .user_guide
            .placeholders()
            .into_iter()
            .map(|p| (p.name.clone(), p))
            .collect();

        let public_predicates = self.user_guide.public_predicates();

        let specification_private_predicates: IndexSet<_> = match self.specification {
            Either::Left(ref program) => program
                .predicates()
                .into_iter()
                .map(fol::Predicate::from)
                .filter(|p| public_predicates.contains(p))
                .collect(),
            Either::Right(ref specification) => specification
                .predicates()
                .into_iter()
                .filter(|p| public_predicates.contains(p))
                .collect(),
        };

        let program_private_predicates: IndexSet<_> = self
            .program
            .predicates()
            .into_iter()
            .map(fol::Predicate::from)
            .filter(|p| public_predicates.contains(p))
            .collect();

        fn head_predicate(formula: &fol::Formula) -> Option<fol::Predicate> {
            match formula {
                fol::Formula::BinaryFormula {
                    connective: fol::BinaryConnective::Equivalence,
                    lhs,
                    rhs: _,
                } => match **lhs {
                    fol::Formula::AtomicFormula(fol::AtomicFormula::Atom(ref a)) => {
                        Some(a.predicate())
                    }
                    _ => None,
                },
                fol::Formula::QuantifiedFormula {
                    quantification:
                        fol::Quantification {
                            quantifier: fol::Quantifier::Forall,
                            variables: _,
                        },
                    formula,
                } => head_predicate(formula),
                _ => None,
            }
        }

        let control_translate = |theory: fol::Theory| {
            let mut constraint_counter = 0..;
            let formulas = theory
                .formulas
                .into_iter()
                .map(|formula| match head_predicate(&formula) {
                    Some(p) if public_predicates.contains(&p) => fol::AnnotatedFormula {
                        role: fol::Role::Spec,
                        direction: fol::Direction::Universal,
                        name: format!("completed_definition_of_{}_{}", p.symbol, p.arity),
                        formula,
                    },
                    Some(p) => fol::AnnotatedFormula {
                        role: fol::Role::Assumption,
                        direction: fol::Direction::Universal,
                        name: format!("completed_definition_of_{}_{}", p.symbol, p.arity),
                        formula,
                    },
                    None => fol::AnnotatedFormula {
                        role: fol::Role::Spec,
                        direction: fol::Direction::Universal,
                        name: format!("constraint_{}", constraint_counter.next().unwrap()),
                        formula,
                    },
                })
                .collect();
            fol::Specification { formulas }
        };

        let left = match self.specification {
            Either::Left(program) => control_translate(
                completion(tau_star(program).replace_placeholders(&placeholders))
                    .expect("tau_star did not create a completable theory"),
            ),
            Either::Right(specification) => specification.replace_placeholders(&placeholders),
        };

        let right = control_translate(
            completion(tau_star(self.program).replace_placeholders(&placeholders))
                .expect("tau_star did not create a completable theory"),
        );

        // TODO: Warn when a conflict between private predicates is encountered
        // TODO: Check if renaming creates new conflicts
        let right = right.rename_predicates(
            &specification_private_predicates
                .intersection(&program_private_predicates)
                .map(|p| (p.clone(), "p".to_string()))
                .collect(),
        );

        let mut user_guide_assumptions = Vec::new();
        for formula in self.user_guide.formulas() {
            match formula.role {
                fol::Role::Assumption => {
                    let overlap: Vec<_> = formula
                        .predicates()
                        .into_iter()
                        .filter(|p| self.user_guide.output_predicates().contains(p))
                        .collect();
                    if overlap.is_empty() {
                        user_guide_assumptions.push(formula.replace_placeholders(&placeholders));
                    } else {
                        return Err(
                            ExternalEquivalenceTaskError::OutputPredicateInUserGuideAssumption(
                                overlap,
                            ),
                        );
                    }
                }
                _ => warnings.push(ExternalEquivalenceTaskWarning::InvalidRoleWithinUserGuide(
                    formula,
                )),
            }
        }

        Ok(ValidatedExternalEquivalenceTask {
            left: left.formulas,
            right: right.formulas,
            user_guide_assumptions,
            proof_outline: ProofOutline::from_specification(self.proof_outline)?,
            decomposition: self.decomposition,
            direction: self.direction,
            break_equivalences: self.break_equivalences,
        }
        .decompose()?
        .preface_warnings(warnings))
    }
}

struct ValidatedExternalEquivalenceTask {
    pub left: Vec<fol::AnnotatedFormula>, // TODO: Use fol::Specification?
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
        use crate::{
            syntax_tree::fol::{Direction::*, Role::*},
            verifying::problem::Role::*,
        };

        let mut stable_premises: Vec<_> = self
            .user_guide_assumptions
            .into_iter()
            .map(|a| a.into_problem_formula(problem::Role::Axiom))
            .collect();

        let mut forward_premises = Vec::new();
        let mut forward_conclusions = Vec::new();
        let mut backward_premises = Vec::new();
        let mut backward_conclusions = Vec::new();

        let mut warnings = Vec::new();

        for formula in self.left {
            match formula.role {
                Assumption => match formula.direction {
                    Universal => stable_premises.push(formula.into_problem_formula(Axiom)),
                    Forward => forward_premises.push(formula.into_problem_formula(Axiom)),
                    Backward => warnings.push(
                        ExternalEquivalenceTaskWarning::InconsistentDirectionAnnotation(formula),
                    ),
                },
                Spec => {
                    if matches!(formula.direction, Universal | Forward) {
                        forward_premises.push(formula.clone().into_problem_formula(Axiom))
                    }
                    if matches!(formula.direction, Universal | Backward) {
                        if self.break_equivalences {
                            // TODO: Apply symmetry breaking
                            todo!("Symmetry breaking is not yet implemented")
                        }
                        backward_conclusions.push(formula.into_problem_formula(Conjecture))
                    }
                }
                Lemma | Definition => unreachable!(),
            }
        }

        for formula in self.right {
            match formula.role {
                Assumption => match formula.direction {
                    Universal => stable_premises.push(formula.into_problem_formula(Axiom)),
                    Forward => warnings.push(
                        ExternalEquivalenceTaskWarning::InconsistentDirectionAnnotation(formula),
                    ),
                    Backward => backward_premises.push(formula.into_problem_formula(Axiom)),
                },
                Spec => {
                    if matches!(formula.direction, Universal | Backward) {
                        backward_premises.push(formula.clone().into_problem_formula(Axiom))
                    }
                    if matches!(formula.direction, Universal | Forward) {
                        if self.break_equivalences {
                            // TODO: Apply symmetry breaking
                            todo!("Symmetry breaking is not yet implemented")
                        }
                        forward_conclusions.push(formula.into_problem_formula(Conjecture))
                    }
                }
                Lemma | Definition => unreachable!(),
            }
        }

        Ok(AssembledExternalEquivalenceTask {
            stable_premises,
            forward_premises,
            forward_conclusions,
            backward_premises,
            backward_conclusions,
            proof_outline: self.proof_outline,
            decomposition: self.decomposition,
            direction: self.direction,
        }
        .decompose()?
        .preface_warnings(warnings))
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
