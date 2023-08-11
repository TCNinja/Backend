mod scryfall;

pub use scryfall::ScryfallCardSearchEngine;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum InfrastructureError {
    #[error("Failed to parse data from infrastructure ({0})")]
    Parse(String),
    #[error("Unknown infrastructure error")]
    Unknown(anyhow::Error),
}
