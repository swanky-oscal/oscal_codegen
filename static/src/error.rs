use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Error)]
pub enum Error {
    #[error("Type Error")]
    TypeError(#[from] oscal_types::Error),
}
