use {
    crate::{
        analyzing::tightness::Tightness,
        command_line::{
            arguments::{Arguments, Command, Equivalence, Property, Translation},
            files::Files,
        },
        syntax_tree::{asp, fol, Node as _},
        translating::{completion::completion, gamma::gamma, tau_star::tau_star},
        verifying::{
            problem::{increment_problem_name, ProblemNameTPTP},
            prover::{vampire::Vampire, Prover, Report, Status, Success},
            task::{
                external_equivalence::ExternalEquivalenceTask,
                strong_equivalence::StrongEquivalenceTask, Task,
            },
        },
    },
    anyhow::{anyhow, Context, Result},
    clap::Parser as _,
    either::Either,
    lazy_static::lazy_static,
    regex::Regex,
    std::time::Instant,
};

lazy_static! {
    static ref RE: Regex = Regex::new(r"^([A-Z]{3})([0-9])([0-9])([0-9])$").unwrap();
}

pub fn main() -> Result<()> {
    match Arguments::parse().command {
        Command::Analyze { property, input } => {
            match property {
                Property::Tightness => {
                    let program =
                        input.map_or_else(asp::Program::from_stdin, asp::Program::from_file)?;
                    let is_tight = program.is_tight();
                    println!("{is_tight}");
                }
            }

            Ok(())
        }

        Command::Translate { with, input } => {
            match with {
                Translation::Completion => {
                    let theory =
                        input.map_or_else(fol::Theory::from_stdin, fol::Theory::from_file)?;
                    let completed_theory =
                        completion(theory).context("the given theory is not completable")?;
                    print!("{completed_theory}")
                }

                Translation::Gamma => {
                    let theory =
                        input.map_or_else(fol::Theory::from_stdin, fol::Theory::from_file)?;
                    let gamma_theory = gamma(theory);
                    print!("{gamma_theory}")
                }

                Translation::TauStar => {
                    let program =
                        input.map_or_else(asp::Program::from_stdin, asp::Program::from_file)?;
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
            bypass_tightness,
            no_simplify,
            no_eq_break,
            no_proof_search,
            no_timing,
            time_limit,
            prover_instances,
            prover_cores,
            save_problems: out_dir,
            files,
            problem_name,
        } => {
            let start_time = Instant::now();

            let files =
                Files::sort(files).context("unable to sort the given files by their function")?;

            let problem_name = match problem_name {
                Some(s) => {
                    let captive = RE.captures(&s).ok_or(anyhow!("invalid tptp name"))?;
                    Some(ProblemNameTPTP {
                        domain: captive.get(1).unwrap().as_str().to_string(),
                        number: vec![
                            captive.get(2).unwrap().as_str().parse().unwrap(),
                            captive.get(3).unwrap().as_str().parse().unwrap(),
                            captive.get(4).unwrap().as_str().parse().unwrap(),
                        ],
                    })
                }
                None => None,
            };

            let problems = match equivalence {
                Equivalence::Strong => StrongEquivalenceTask {
                    left: asp::Program::from_file(
                        files
                            .left()
                            .ok_or(anyhow!("no left program was provided"))?,
                    )?,
                    right: asp::Program::from_file(
                        files
                            .right()
                            .ok_or(anyhow!("no right program was provided"))?,
                    )?,
                    decomposition,
                    direction,
                    problem_name,
                    simplify: !no_simplify,
                    break_equivalences: !no_eq_break,
                }
                .decompose()?
                .report_warnings(),
                Equivalence::External => ExternalEquivalenceTask {
                    specification: match files
                        .specification()
                        .ok_or(anyhow!("no specification was provided"))?
                    {
                        Either::Left(program) => Either::Left(asp::Program::from_file(program)?),
                        Either::Right(specification) => {
                            Either::Right(fol::Specification::from_file(specification)?)
                        }
                    },
                    program: asp::Program::from_file(
                        files.program().ok_or(anyhow!("no program was provided"))?,
                    )?,
                    user_guide: fol::UserGuide::from_file(
                        files
                            .user_guide()
                            .ok_or(anyhow!("no user guide was provided"))?,
                    )?,
                    proof_outline: files
                        .proof_outline()
                        .map(fol::Specification::from_file)
                        .unwrap_or_else(|| Ok(fol::Specification::empty()))?,
                    decomposition,
                    direction,
                    bypass_tightness,
                    problem_name,
                    simplify: !no_simplify,
                    break_equivalences: !no_eq_break,
                }
                .decompose()?
                .report_warnings(),
            };

            if let Some(out_dir) = out_dir {
                for problem in &problems {
                    let mut path = out_dir.clone();
                    if no_simplify {
                        path.push(format!("{}=2.p", problem.name));
                    } else {
                        path.push(format!("{}=1.p", problem.name));
                    }
                    problem.to_file(path)?;
                }
            }

            if !no_proof_search {
                let prover = Vampire {
                    time_limit,
                    instances: prover_instances,
                    cores: prover_cores,
                };

                let problems = problems.into_iter().inspect(|problem| {
                    println!("> Proving {}...", problem.name);
                    println!("Axioms:");
                    for axiom in problem.axioms() {
                        println!("    {}", axiom.formula);
                    }
                    println!();
                    println!("Conjectures:");
                    for conjecture in problem.conjectures() {
                        println!("    {}", conjecture.formula);
                    }
                    println!();
                });

                let mut success = true;
                for result in prover.prove_all(problems) {
                    match result {
                        Ok(report) => match report.status() {
                            Ok(status) => {
                                println!(
                                    "> Proving {} ended with a SZS status",
                                    report.problem.name
                                );
                                print!("Status: {status}");
                                if !no_timing {
                                    print!(" ({} ms)", report.elapsed_time.as_millis())
                                }
                                println!();
                                if !matches!(status, Status::Success(Success::Theorem)) {
                                    success = false;
                                }
                            }
                            Err(error) => {
                                println!(
                                    "> Proving {} ended without a SZS status",
                                    report.problem.name
                                );
                                println!("Output/stdout:");
                                println!("{}", report.output.stdout);
                                println!("Output/stderr:");
                                println!("{}", report.output.stderr);
                                println!("Error: {error}");
                                success = false;
                            }
                        },
                        Err(error) => {
                            println!("> Proving <a problem> ended with an error"); // TODO: Get the name of the problem
                            println!("Error: {error}");
                            success = false;
                        }
                    }
                    println!();
                }

                if success {
                    print!("> Success! Anthem found a proof of equivalence.")
                } else {
                    print!("> Failure! Anthem was unable to find a proof of equivalence.")
                }

                if !no_timing {
                    print!(" ({} ms)", start_time.elapsed().as_millis())
                }

                println!()
            }

            Ok(())
        }
    
        // Command::Test { problem_name } => {
        //     let problem_name = match problem_name {
        //         Some(s) => {
        //             let captive = RE.captures(&s).ok_or(anyhow!("invalid tptp name"))?;
                    
        //             let temp = captive.get(1).unwrap().as_str().to_string();

        //             Some(ProblemNameTPTP {
        //                 domain: temp,
        //                 number: vec![
        //                     captive.get(2).unwrap().as_str().parse().unwrap(),
        //                     captive.get(3).unwrap().as_str().parse().unwrap(),
        //                     captive.get(4).unwrap().as_str().parse().unwrap(),
        //                 ],
        //             })
        //         }
        //         None => None,
        //     };

        //     //println!("{}", problem_name.unwrap());

        //     let newname = increment_problem_name(&problem_name.unwrap(), 5);

        //     println!("{}", newname);

        //     Ok(())
        // }
    }
}
