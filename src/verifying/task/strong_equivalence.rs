use {
    crate::{
        command_line::Decomposition,
        syntax_tree::{asp, fol},
        translating::{
            gamma::{self, gamma},
            tau_star::tau_star,
        },
        verifying::{
            problem::{AnnotatedFormula, Problem, Role},
            task::Task,
        },
    },
    std::convert::identity,
    thiserror::Error,
};

#[derive(Error, Debug)]
pub enum StrongEquivalenceTaskError {}

pub struct StrongEquivalenceTask {
    pub left: asp::Program,
    pub right: asp::Program,
    pub decomposition: Decomposition,
    pub direction: fol::Direction,
    pub simplify: bool,
    pub break_equivalences: bool,
}

impl StrongEquivalenceTask {
    fn transition_axioms(&self) -> fol::Theory {
        fn transition(p: asp::Predicate) -> fol::Formula {
            let p: fol::Predicate = p.into();

            let hp = gamma::here(p.clone().to_formula());
            let tp = gamma::there(p.to_formula());

            let variables = hp.free_variables();

            fol::Formula::BinaryFormula {
                connective: fol::BinaryConnective::Implication,
                lhs: hp.into(),
                rhs: tp.into(),
            }
            .quantify(fol::Quantifier::Forall, variables.into_iter().collect())
        }

        let mut predicates = self.left.predicates();
        predicates.extend(self.right.predicates());

        fol::Theory {
            formulas: predicates.into_iter().map(transition).collect(),
        }
    }
}

impl Task for StrongEquivalenceTask {
    type Error = StrongEquivalenceTaskError;

    fn decompose(self) -> Result<Vec<Problem>, Self::Error> {
        let transition_axioms = self.transition_axioms(); // These are the "forall X (hp(X) -> tp(X))" axioms.

        let simplify_ht = if self.simplify {
            |theory: fol::Theory| fol::Theory {
                formulas: theory
                    .formulas
                    .into_iter()
                    .map(crate::simplifying::fol::ht::simplify)
                    .collect(),
            }
        } else {
            identity
        };
        let simplify_classic = if self.simplify {
            |theory: fol::Theory| fol::Theory {
                formulas: theory
                    .formulas
                    .into_iter()
                    .map(crate::simplifying::fol::classic::simplify)
                    .collect(),
            }
        } else {
            identity
        };

        // TODO: Break equivalences, if requested

        let left = simplify_classic(gamma(simplify_ht(tau_star(self.left))));
        let right = simplify_classic(gamma(simplify_ht(tau_star(self.right))));

        let mut problems = Vec::new();
        if matches!(
            self.direction,
            fol::Direction::Universal | fol::Direction::Forward
        ) {
            problems.push(
                Problem::default()
                    .add_theory(transition_axioms.clone(), |i, formula| AnnotatedFormula {
                        name: format!("transition_axiom_{i}"),
                        role: Role::Axiom,
                        formula,
                    })
                    .add_theory(left.clone(), |i, formula| AnnotatedFormula {
                        name: format!("left_{i}"),
                        role: Role::Axiom,
                        formula,
                    })
                    .add_theory(right.clone(), |i, formula| AnnotatedFormula {
                        name: format!("right_{i}"),
                        role: Role::Conjecture,
                        formula,
                    }),
            );
        }
        if matches!(
            self.direction,
            fol::Direction::Universal | fol::Direction::Backward
        ) {
            problems.push(
                Problem::default()
                    .add_theory(transition_axioms, |i, formula| AnnotatedFormula {
                        name: format!("transition_axiom_{i}"),
                        role: Role::Axiom,
                        formula,
                    })
                    .add_theory(right, |i, formula| AnnotatedFormula {
                        name: format!("right_{i}"),
                        role: Role::Axiom,
                        formula,
                    })
                    .add_theory(left, |i, formula| AnnotatedFormula {
                        name: format!("left_{i}"),
                        role: Role::Conjecture,
                        formula,
                    }),
            );
        }

        Ok(problems
            .into_iter()
            .flat_map(|p: Problem| match self.decomposition {
                Decomposition::Independent => p.decompose_independent(),
                Decomposition::Sequential => p.decompose_sequential(),
            })
            .collect())
    }
}
