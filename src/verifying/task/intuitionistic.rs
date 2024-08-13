use {
    crate::{
        convenience::with_warnings::{Result, WithWarnings},
        syntax_tree::asp,
        translating::shorthand::shorthand,
        verifying::{
            problem::{AnnotatedFormula, Problem, Role},
            task::Task,
        },
    },
    std::convert::Infallible,
    thiserror::Error,
};

#[derive(Error, Debug)]
pub enum IntuitionisticTaskError {}

pub struct IntuitionisticTask {
    pub left: asp::Program,
    pub right: asp::Program,
}

impl Task for IntuitionisticTask {
    type Error = IntuitionisticTaskError;
    type Warning = Infallible;

    fn decompose(self) -> Result<Vec<Problem>, Self::Warning, Self::Error> {
        let mut forward_premises = Vec::new();
        let mut backward_premises = Vec::new();

        let mut forward_conclusions = Vec::new();
        let mut backward_conclusions = Vec::new();

        let f1 = shorthand(self.left);
        let f2 = shorthand(self.right);

        for formula in f1.formulas {
            forward_premises.push(AnnotatedFormula {
                name: "anf".to_string(),
                role: Role::Axiom,
                formula: formula.clone(),
            });
            backward_conclusions.push(AnnotatedFormula {
                name: "anf".to_string(),
                role: Role::Conjecture,
                formula,
            });
        }

        for formula in f2.formulas {
            forward_conclusions.push(AnnotatedFormula {
                name: "anf".to_string(),
                role: Role::Conjecture,
                formula: formula.clone(),
            });
            backward_premises.push(AnnotatedFormula {
                name: "anf".to_string(),
                role: Role::Axiom,
                formula,
            });
        }

        let mut problems = Vec::new();

        problems.push(
            Problem::with_name(format!("forward"))
                .add_annotated_formulas(forward_premises)
                .add_annotated_formulas(forward_conclusions),
        );

        problems.push(
            Problem::with_name(format!("backward"))
                .add_annotated_formulas(backward_premises)
                .add_annotated_formulas(backward_conclusions),
        );

        Ok(WithWarnings::flawless(problems))
    }
}
