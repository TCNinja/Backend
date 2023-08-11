use serde::Deserialize;
use uuid::Uuid;

use crate::{
    card::Card, infrastructure::InfrastructureError, infrastructure::InfrastructureResult,
};

const BASE_URL: &str = "https://api.scryfall.com/";

#[derive(Deserialize)]
struct ScryfallCard {
    id: Uuid,
    oracle_id: Uuid,
    name: String,
    lang: String,
    scryfall_uri: String,
    type_line: String,
    scryfall_set_uri: String,
    #[serde(flatten)]
    card_face_kind: ScryfallCardFaceKind,
}

#[derive(Deserialize)]
enum ScryfallCardFaceKind {
    #[serde(rename = "image_uris")]
    SingleFace(ScryfallImageUris),
    #[serde(rename = "card_faces")]
    MultipleFace([ScryfallCardFace; 2]),
}

#[derive(Deserialize)]
struct ScryfallImageUris {
    png: String,
}

#[derive(Deserialize)]
struct ScryfallCardFace {
    image_uris: ScryfallImageUris,
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
    pub fn new() -> InfrastructureResult<Self> {
        let base_url =
            reqwest::Url::parse(BASE_URL).map_err(|e| InfrastructureError::Unknown(e.into()))?;
        Ok(Self {
            client: reqwest::ClientBuilder::new()
                .build()
                .map_err(|e| InfrastructureError::Unknown(e.into()))?,
            search_url: base_url
                .join("/cards/search")
                .map_err(|e| InfrastructureError::Unknown(e.into()))?,
        })
    }

    pub async fn search_cards_by_name(&self, name: &str) -> InfrastructureResult<Vec<Card>> {
        let response: ScryfallSearchResponse = self
            .client
            .get(self.search_url.clone())
            .query(&[("q", name)])
            .send()
            .await
            .map_err(|e| InfrastructureError::Unknown(e.into()))?
            .json()
            .await
            .map_err(|e| InfrastructureError::Parse(e.to_string()))?;

        let cards = response
            .data
            .iter()
            .map(|card| Card {
                id: card.id,
                oracle_id: card.oracle_id,
                name: card.name.clone(),
                type_line: card.type_line.clone(),
                language: card.lang.clone(),
                image_uri: match &card.card_face_kind {
                    ScryfallCardFaceKind::SingleFace(image_uris) => image_uris.png.clone(),
                    ScryfallCardFaceKind::MultipleFace(faces) => faces[0].image_uris.png.clone(),
                },
                scryfall_uri: card.scryfall_uri.clone(),
                scryfall_set_uri: card.scryfall_set_uri.clone(),
            })
            .collect();

        Ok(cards)
    }
}
