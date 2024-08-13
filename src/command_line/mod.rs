use {
    clap::{Parser, Subcommand, ValueEnum},
    either::Either,
    std::path::PathBuf,
    walkdir::WalkDir,
};

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
pub struct Arguments {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Translate a given answer set program or first-order theory
    Translate {
        /// The translation to use
        #[arg(long, value_enum)]
        with: Translation,

        /// The file to translate
        input: Option<PathBuf>,
    },

    /// Create and optionally verify a set of problem files from a claim about answer set programs or first-order theories
    Verify {
        /// The equivalence theory used to proof the claim
        #[arg(long, value_enum)]
        equivalence: Equivalence,

        /// The decomposition strategy to use
        #[arg(long, value_enum, default_value_t)]
        decomposition: Decomposition,

        /// The direction of the proof
        #[arg(long, value_enum, default_value_t)]
        direction: Direction,

        /// Omit simplifications
        #[arg(long, action)]
        no_simplify: bool,

        /// Omit breaking equivalences
        #[arg(long, action)]
        no_eq_break: bool,

        /// Omit proof search and just create problem files
        #[arg(long, action)]
        no_proof_search: bool,

        /// The destination directory for the problem files
        #[arg(long)]
        out_dir: Option<PathBuf>,

        /// A set of files from which to construct the claim, including
        ///
        ///   - a specification of intended behavior,
        ///   - a program about which the claim is constructed, and
        ///   - additional knowledge used to construct the claim (e.g., user guide, proof outline).
        #[arg(verbatim_doc_comment)]
        files: Vec<PathBuf>,
    },
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum Translation {
    Completion,
    Gamma,
    TauStar,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum Equivalence {
    Strong,
    External,
}

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum Decomposition {
    Independent,
    #[default]
    Sequential,
}

pub use crate::syntax_tree::fol::Direction;

#[derive(Debug)]
pub struct Files {
    pub specifications: Vec<PathBuf>,
    pub programs: Vec<PathBuf>,
    pub user_guides: Vec<PathBuf>,
    pub proof_outlines: Vec<PathBuf>,
    pub other: Vec<PathBuf>,
}

impl Files {
    pub fn empty() -> Self {
        Files {
            specifications: vec![],
            programs: vec![],
            user_guides: vec![],
            proof_outlines: vec![],
            other: vec![],
        }
    }

    pub fn sort(paths: impl IntoIterator<Item = PathBuf>) -> Result<Self, walkdir::Error> {
        let mut result = Files::empty();

        for entry in paths
            .into_iter()
            .map(WalkDir::new)
            .flat_map(WalkDir::sort_by_file_name)
        {
            let entry = entry?;
            if entry.file_type().is_file() {
                let path = entry.into_path();
                let name = path
                    .file_name()
                    .expect("a file should have a name")
                    .to_str()
                    .expect("the name of a file should be UTF-8");

                if name.ends_with(".lp") {
                    &mut result.programs
                } else if name.ends_with(".help.spec") {
                    &mut result.proof_outlines
                } else if name.ends_with(".spec") {
                    &mut result.specifications
                } else if name.ends_with(".ug") {
                    &mut result.user_guides
                } else {
                    &mut result.other
                }
                .push(path);
            }
        }

        Ok(result)
    }

    pub fn left(&self) -> Option<&PathBuf> {
        self.programs.first()
    }

    pub fn right(&self) -> Option<&PathBuf> {
        self.programs.get(1)
    }

    pub fn specification(&self) -> Option<Either<&PathBuf, &PathBuf>> {
        self.specifications.first()
            .map(Either::Right)
            .or_else(|| self.programs.first().map(Either::Left))
    }

    pub fn program(&self) -> Option<&PathBuf> {
        if self.specifications.is_empty() {
            self.programs.get(1)
        } else {
            self.programs.first()
        }
    }

    pub fn user_guide(&self) -> Option<&PathBuf> {
        self.user_guides.first()
    }

    pub fn proof_outline(&self) -> Option<&PathBuf> {
        self.proof_outlines.first()
    }
}

#[cfg(test)]
mod tests {
    use super::Arguments;

    #[test]
    fn verify() {
        use clap::CommandFactory as _;
        Arguments::command().debug_assert()
    }
}
