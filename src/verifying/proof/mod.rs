use {
    crate::verifying::problem::Problem,
    std::fmt::{Debug, Display},
};

pub mod vampire;

pub trait Report: Display + Debug + Clone {}

pub trait Prover {
    type Report: Report;
    type Error;

    fn prove(&self, problem: Problem) -> Result<Self::Report, Self::Error>;
}
