use {either::Either, std::path::PathBuf, walkdir::WalkDir};

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
        self.specifications
            .first()
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
