mod scryfall;

use thiserror::Error;

pub use scryfall::ScryfallSearchEngine;

pub type InfrastructureResult<T> = Result<T, InfrastructureError>;

#[derive(Error, Debug)]
pub enum InfrastructureError {
    #[error("Failed to parse data from infrastructure ({0})")]
    Parse(String),
    #[error(transparent)]
    Unknown(anyhow::Error),
}
