use {
    crate::syntax_tree::{
        asp::Program,
        fol::{Specification, UserGuide},
    },
    either::Either,
};

pub struct ExternalEquivalenceTask {
    pub specification: Either<Program, Specification>,
    pub program: Program,
    pub user_guide: UserGuide,
    pub proof_outline: Specification,
    // TODO: Add more fields
}
