use {
    crate::verifying::problem::Problem,
    lazy_static::lazy_static,
    regex::Regex,
    std::{
        fmt::{Debug, Display},
        str::FromStr,
    },
    thiserror::Error,
};

pub mod vampire;

lazy_static! {
    static ref STATUS: Regex =
        Regex::new(r"^% SZS status (?<status>[[:word:]]+) for (?<problem>[[:word:]]+)$").unwrap();
}

#[derive(Debug, Error)]
pub enum StatusExtractionError {
    #[error("the status of verifying this problem is missing")]
    Missing,
    #[error("the status of verifying this problem is not recognized: `{0}`")]
    Unknown(String),
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Success {
    Theorem,
    CounterSatisfiable,
    ContradictoryAxioms,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Failure {
    TimeOut,
    MemoryOut,
    GaveUp,
    // UserTerminated,
    Error,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Status {
    Success(Success),
    Failure(Failure),
}

impl FromStr for Status {
    type Err = StatusExtractionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (_line, [status, _problem]) = STATUS
            .captures(s)
            .ok_or_else(|| StatusExtractionError::Missing)?
            .extract();

        match status {
            "Theorem" => Ok(Self::Success(Success::Theorem)),
            "CounterSatisfiable" => Ok(Self::Success(Success::CounterSatisfiable)),
            "ContradictoryAxioms" => Ok(Self::Success(Success::ContradictoryAxioms)),
            "Timeout" => Ok(Self::Failure(Failure::TimeOut)),
            "MemoryOut" => Ok(Self::Failure(Failure::MemoryOut)),
            "GaveUp" => Ok(Self::Failure(Failure::GaveUp)),
            "Error" => Ok(Self::Failure(Failure::Error)),
            x => Err(StatusExtractionError::Unknown(x.to_string())),
        }
    }
}

pub trait Report: Display + Debug + Clone {
    fn status(&self) -> Result<Status, StatusExtractionError>;
}

pub trait Prover {
    type Report: Report;
    type Error;

    fn prove(&self, problem: Problem) -> Result<Self::Report, Self::Error>;
}
