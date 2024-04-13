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
        command_line::{Arguments, Command, Decomposition, Translation},
        syntax_tree::{asp, fol},
        translating::tau_star::tau_star,
        verifying::task::Task,
    },
    anyhow::{Context, Result},
    clap::Parser as _,
    command_line::Equivalence,
    either::Either,
    std::{ffi::OsStr, fs::read_to_string},
    translating::gamma::gamma,
    verifying::task::external_equivalence::ExternalEquivalenceTask,
};

fn main() -> Result<()> {
    match Arguments::parse().command {
        Command::Translate { with, input } => {
            let content = read_to_string(&input)
                .with_context(|| format!("could not read file `{}`", input.display()))?;

            match with {
                Translation::Gamma => {
                    let theory: fol::Theory = content
                        .parse()
                        .with_context(|| format!("could not parse file `{}`", input.display()))?;

                    let theory = gamma(theory);

                    print!("{theory}")
                }

                Translation::TauStar => {
                    let program: asp::Program = content
                        .parse()
                        .with_context(|| format!("could not parse file `{}`", input.display()))?;

                    let theory = tau_star(program);

                    print!("{theory}")
                }
            }

            Ok(())
        }

        Command::Verify {
            equivalence: Equivalence::External,
            decomposition,
            direction,
            no_simplify,
            no_eq_break,
            no_proof_search,
            out_dir,
            left,
            right,
            aux,
        } => {
            let specification: Either<asp::Program, fol::Specification> = match left
                .extension()
                .map(OsStr::to_str)
            {
                Some(Some("lp")) => Either::Left(
                    read_to_string(&left)
                        .with_context(|| format!("could not read file `{}`", left.display()))?
                        .parse()
                        .with_context(|| format!("could not parse file `{}`", left.display()))?,
                ),
                Some(Some("spec")) => Either::Right(
                    read_to_string(&left)
                        .with_context(|| format!("could not read file `{}`", left.display()))?
                        .parse()
                        .with_context(|| format!("could not parse file `{}`", left.display()))?,
                ),
                Some(Some(x)) => todo!(),
                Some(None) => todo!(),
                None => todo!(),
            };

            let program: asp::Program = match right.extension().map(|x| x.to_str()) {
                Some(Some("lp")) => read_to_string(&right)
                    .with_context(|| format!("could not read file `{}`", right.display()))?
                    .parse()
                    .with_context(|| format!("could not parse file `{}`", right.display()))?,
                Some(Some(x)) => todo!(),
                Some(None) => todo!(),
                None => todo!(),
            };

            let user_guide: fol::UserGuide = match aux
                .first()
                .with_context(|| "no user guide was provided")?
                .extension()
                .map(OsStr::to_str)
            {
                Some(Some("ug")) => {
                    let path = aux.first().unwrap();
                    read_to_string(path)
                        .with_context(|| format!("could not read file `{}`", path.display()))?
                        .parse()
                        .with_context(|| format!("could not parse file `{}`", path.display()))?
                }
                Some(Some(x)) => todo!(),
                Some(None) => todo!(),
                None => todo!(),
            };

            let proof_outline = match aux.get(1) {
                Some(path) => match path.extension().map(OsStr::to_str) {
                    Some(Some("spec")) => {
                        let path = aux.first().unwrap();
                        read_to_string(path)
                            .with_context(|| format!("could not read file `{}`", path.display()))?
                            .parse()
                            .with_context(|| format!("could not parse file `{}`", path.display()))?
                    }
                    Some(Some(x)) => todo!(),
                    Some(None) => todo!(),
                    None => todo!(),
                },
                None => fol::Specification::empty(),
            };

            let task = ExternalEquivalenceTask {
                specification,
                user_guide,
                program,
                proof_outline,
                decomposition,
                direction,
                simplify: !no_simplify,
                break_equivalences: !no_eq_break,
            };

            let problems = task.decompose()?;

            todo!()
        }

        Command::Verify {
            equivalence: Equivalence::Strong,
            ..
        } => todo!(),
    }
}
