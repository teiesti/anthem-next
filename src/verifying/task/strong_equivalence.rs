use {
    crate::{
        command_line::Decomposition,
        syntax_tree::{asp, fol},
        translating::{gamma::gamma, tau_star::tau_star},
        verifying::{
            problem::{AnnotatedFormula, Problem, Role},
            task::Task,
        },
    },
    std::fmt,
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

impl Task for StrongEquivalenceTask {
    type Error = StrongEquivalenceTaskError;

    fn decompose(&self) -> Result<Vec<Problem>, Self::Error> {
        // TODO: Apply simplifications, if requested
        // TODO: Break equivalences, if requested
        // TODO: Avoid cloning the programs
        // TODO: Add "forall X (hp(X) -> tp(X))" axioms

        let left = gamma(tau_star(self.left.clone()));
        let right = gamma(tau_star(self.right.clone()));

        let mut problems = Vec::new();
        if matches!(
            self.direction,
            fol::Direction::Universal | fol::Direction::Forward
        ) {
            problems.push(
                Problem::default()
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
            fol::Direction::Universal | fol::Direction::Forward
        ) {
            problems.push(
                Problem::default()
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
            .map(|p: Problem| match self.decomposition {
                Decomposition::Independent => p.decompose_independent(),
                Decomposition::Sequential => p.decompose_sequential(),
            })
            .flatten()
            .collect())
    }
}

impl fmt::Display for StrongEquivalenceTask {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}
