pub mod command_line;
pub mod convenience;
pub mod formatting;
pub mod parsing;
pub mod simplifying;
pub mod syntax_tree;
pub mod translating;
pub mod verifying;

use {
    crate::{
        command_line::{Arguments, Command, Equivalence, Translation},
        syntax_tree::{asp, fol, Node as _},
        translating::{completion::completion, gamma::gamma, tau_star::tau_star},
        verifying::task::{
            external_equivalence::ExternalEquivalenceTask,
            strong_equivalence::StrongEquivalenceTask, Task,
        },
    },
    anyhow::{Context, Result},
    clap::Parser as _,
    either::Either,
    std::ffi::OsStr,
};

fn main() -> Result<()> {
    match Arguments::parse().command {
        Command::Translate { with, input } => {
            match with {
                Translation::Completion => {
                    let theory = fol::Theory::from_file(input)?;
                    let completed_theory =
                        completion(theory).context("the given theory is not completable")?;
                    print!("{completed_theory}")
                }

                Translation::Gamma => {
                    let theory = fol::Theory::from_file(input)?;
                    let gamma_theory = gamma(theory);
                    print!("{gamma_theory}")
                }

                Translation::TauStar => {
                    let program = asp::Program::from_file(input)?;
                    let theory = tau_star(program);
                    print!("{theory}")
                }
            }

            Ok(())
        }

        Command::Verify {
            equivalence,
            decomposition,
            direction,
            no_simplify,
            no_eq_break,
            // no_proof_search,
            out_dir,
            left,
            right,
            aux,
            ..
        } => {
            let problems = match equivalence {
                Equivalence::Strong => {
                    StrongEquivalenceTask {
                        left: asp::Program::from_file(left)?,
                        right: asp::Program::from_file(right)?,
                        decomposition,
                        direction,
                        simplify: !no_simplify,
                        break_equivalences: !no_eq_break,
                    }
                    .decompose()?
                    .data // TODO: Add some kind of unwrap to WithWarnings and use it here
                }

                Equivalence::External => {
                    let specification: Either<asp::Program, fol::Specification> = match left
                        .extension()
                        .map(OsStr::to_str)
                    {
                        Some(Some("lp")) => Either::Left(asp::Program::from_file(left)?),
                        Some(Some("spec")) => Either::Right(fol::Specification::from_file(left)?),
                        Some(Some(_x)) => todo!(),
                        Some(None) => todo!(),
                        None => todo!(),
                    };

                    let program: asp::Program = match right.extension().map(|x| x.to_str()) {
                        Some(Some("lp")) => asp::Program::from_file(right)?,
                        Some(Some(_x)) => todo!(),
                        Some(None) => todo!(),
                        None => todo!(),
                    };

                    let user_guide: fol::UserGuide = match aux
                        .first()
                        .with_context(|| "no user guide was provided")?
                        .extension()
                        .map(OsStr::to_str)
                    {
                        Some(Some("ug")) => fol::UserGuide::from_file(aux.first().unwrap())?,
                        Some(Some(_x)) => todo!(),
                        Some(None) => todo!(),
                        None => todo!(),
                    };

                    let proof_outline = match aux.get(1) {
                        Some(path) => match path.extension().map(OsStr::to_str) {
                            Some(Some("spec")) => {
                                fol::Specification::from_file(aux.first().unwrap())?
                            }
                            Some(Some(_x)) => todo!(),
                            Some(None) => todo!(),
                            None => todo!(),
                        },
                        None => fol::Specification::empty(),
                    };

                    ExternalEquivalenceTask {
                        specification,
                        user_guide,
                        program,
                        proof_outline,
                        decomposition,
                        direction,
                        simplify: !no_simplify,
                        break_equivalences: !no_eq_break,
                    }
                    .decompose()?
                    .data // TODO: Handle warnings
                }
            };

            if let Some(out_dir) = out_dir {
                for problem in problems.into_iter() {
                    let mut path = out_dir.clone();
                    path.push(format!("{}.p", problem.name));
                    problem.to_file(path)?;
                }
            }

            // TODO: Run proof search

            Ok(())
        }
    }
}
