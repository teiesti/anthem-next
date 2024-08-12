use {
    crate::verifying::{
        problem::Problem,
        proof::{Prover, Report, Status, StatusExtractionError},
    },
    std::{
        fmt::{self, Display},
        io::Write as _,
        process::{Command, Output, Stdio},
    },
    thiserror::Error,
};

#[derive(Error, Debug)]
pub enum VampireError {
    #[error("unable to spawn vampire as a child process")]
    UnableToSpawn(#[source] std::io::Error),
    #[error("unable to write to vampire's stdin")]
    UnableToWrite(#[source] std::io::Error),
    #[error("unable to wait for vampire")]
    UnableToWait(#[source] std::io::Error),
    #[error("unable to convert output")]
    UnableToConvertOutput(#[source] std::string::FromUtf8Error),
}

#[derive(Debug, Clone)]
pub struct VampireOutput {
    pub stdout: String,
    pub stderr: String,
}

impl TryFrom<Output> for VampireOutput {
    type Error = VampireError;

    fn try_from(value: Output) -> Result<Self, Self::Error> {
        // TODO: Should we do something about the exit status?!
        Ok(VampireOutput {
            stdout: String::from_utf8(value.stdout).map_err(VampireError::UnableToConvertOutput)?,
            stderr: String::from_utf8(value.stderr).map_err(VampireError::UnableToConvertOutput)?,
        })
    }
}

#[derive(Debug, Clone)]
pub struct VampireReport {
    pub problem: Problem,
    pub output: VampireOutput,
}

impl Report for VampireReport {
    fn status(&self) -> Result<Status, StatusExtractionError> {
        todo!()
    }
}

impl Display for VampireReport {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

pub struct Vampire;

impl Prover for Vampire {
    type Error = VampireError;
    type Report = VampireReport;

    fn prove(&self, problem: Problem) -> Result<Self::Report, Self::Error> {
        let mut child = Command::new("vampire")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(VampireError::UnableToSpawn)?;

        let mut stdin = child.stdin.take().unwrap();
        write!(stdin, "{problem}").map_err(VampireError::UnableToWrite)?;
        drop(stdin);

        let output = child
            .wait_with_output()
            .map_err(VampireError::UnableToWait)?
            .try_into()?;

        Ok(VampireReport { problem, output })
    }
}
