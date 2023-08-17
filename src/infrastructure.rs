mod scryfall;

pub use scryfall::ScryfallSearchEngine;
use thiserror::Error;

pub type InfrastructureResult<T> = Result<T, InfrastructureError>;

#[derive(Error, Debug)]
pub enum InfrastructureError {
    #[error("Failed to parse data from infrastructure ({0})")]
    Parse(String),
    #[error(transparent)]
    Unknown(anyhow::Error),
}
