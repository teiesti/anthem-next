pub mod external_equivalence;
pub mod strong_equivalence;

use crate::verifying::problem::Problem;

pub trait Task {
    type Error;
    fn decompose(self) -> Result<Vec<Problem>, Self::Error>;
}
