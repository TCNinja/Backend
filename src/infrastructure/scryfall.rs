use serde::Deserialize;
use uuid::Uuid;

use crate::{card::Card, infrastructure::InfrastructureError};

const BASE_URL: &str = "https://api.scryfall.com/";

#[derive(Deserialize)]
struct ScryfallCard {
    id: Uuid,
    oracle_id: Uuid,
    name: String,
    lang: String,
    scryfall_uri: String,
    image_uris: Option<ScryfallImageUris>, // `None` when card has multiple faces
    type_line: String,
    scryfall_set_uri: String,
}

#[derive(Deserialize)]
struct ScryfallImageUris {
    png: String,
}

#[derive(Deserialize)]
struct ScryfallSearchResponse {
    data: Vec<ScryfallCard>,
}

#[derive(Clone)]
pub struct ScryfallCardSearchEngine {
    client: reqwest::Client,
    search_url: reqwest::Url,
}

impl ScryfallCardSearchEngine {
    pub fn new() -> Option<Self> {
        let base_url = reqwest::Url::parse(BASE_URL).ok()?;
        let search_url = base_url.join("/cards/search").ok()?;
        Some(Self {
            client: reqwest::ClientBuilder::new().build().ok()?,
            search_url,
        })
    }

    pub async fn search_cards_by_name(&self, name: &str) -> Result<Vec<Card>, InfrastructureError> {
        let response: ScryfallSearchResponse = self
            .client
            .get(self.search_url.clone())
            .query(&[("q", name)])
            .send()
            .await
            .map_err(|e| InfrastructureError::ReqwestError(e))?
            .json()
            .await
            .map_err(|e| InfrastructureError::ReqwestError(e))?;

        let cards = response
            .data
            .iter()
            .map(|card| Card {
                id: card.id,
                oracle_id: card.oracle_id,
                name: card.name.clone(),
                type_line: card.type_line.clone(),
                language: card.lang.clone(),
                image_uri: match &card.image_uris {
                    Some(uris) => uris.png.clone(),
                    None => Default::default(),
                },
                scryfall_uri: card.scryfall_uri.clone(),
                scryfall_set_uri: card.scryfall_set_uri.clone(),
            })
            .collect();

        Ok(cards)
    }
}
