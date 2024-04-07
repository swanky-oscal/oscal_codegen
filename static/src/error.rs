use thiserror::Error;

#[derive(Debug, Clone, Error)]
pub enum Error {
    #[error("Type Error")]
    TypeError(#[from] oscal_types::Error),
}
