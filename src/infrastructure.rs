mod scryfall;

pub use scryfall::ScryfallCardSearchEngine;

#[derive(Debug)]
pub enum InfrastructureError {
    ReqwestError(reqwest::Error),
}

impl std::fmt::Display for InfrastructureError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InfrastructureError::ReqwestError(e) => e.fmt(f),
        }
    }
}

impl std::error::Error for InfrastructureError{}
