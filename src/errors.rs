use thiserror::Error;

#[derive(Debug, Error)]
pub enum LitError {
    #[error("DIMACS numbers cannot be 0")]
    InvalidDimacs,

    #[error("Index out of range, cannot be greater than Var::max().index()")]
    IndexTooLarge,
}
