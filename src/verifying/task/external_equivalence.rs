use {
    crate::{
        command_line::Decomposition,
        syntax_tree::{
            asp::Program,
            fol::{Direction, Specification, UserGuide},
        },
    },
    either::Either,
};

pub struct ExternalEquivalenceTask {
    pub specification: Either<Program, Specification>,
    pub program: Program,
    pub user_guide: UserGuide,
    pub proof_outline: Specification,
    pub decomposition: Decomposition,
    pub direction: Direction,
    pub simplify: bool,
    pub break_equivalences: bool,
}
