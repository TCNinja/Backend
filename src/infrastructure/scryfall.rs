use anyhow::anyhow;
use reqwest::Response;
use serde::Deserialize;
use uuid::Uuid;

use crate::{
    card::Card,
    infrastructure::{InfrastructureError, InfrastructureResult},
};

const BASE_URL: &str = "https://api.scryfall.com/";

#[derive(Deserialize)]
#[serde(tag = "object")]
enum ScryfallObject {
    #[serde(rename = "list")]
    List { data: Vec<ScryfallCard> },
    #[serde(rename = "error")]
    Error { details: String },
}

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

#[derive(Clone)]
pub struct ScryfallSearchEngine {
    client: reqwest::Client,
    search_url: reqwest::Url,
}

impl From<&ScryfallCard> for Card {
    fn from(value: &ScryfallCard) -> Self {
        Self {
            id: value.id,
            oracle_id: value.oracle_id,
            name: value.name.clone(),
            type_line: value.type_line.clone(),
            language: value.lang.clone(),
            image_uri: match &value.card_face_kind {
                ScryfallCardFaceKind::SingleFace(image_uris) => image_uris.png.clone(),
                ScryfallCardFaceKind::MultipleFace(faces) => faces[0].image_uris.png.clone(),
            },
            scryfall_uri: value.scryfall_uri.clone(),
            scryfall_set_uri: value.scryfall_set_uri.clone(),
        }
    }
}

impl ScryfallSearchEngine {
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
        let query = format!("\"{name}\"");
        let response = self.search_cards(&query).await?;

        if response.status() == reqwest::StatusCode::NOT_FOUND {
            return Ok(Vec::new());
        }

        let object = self.parse_response(response).await?;

        let cards = match object {
            ScryfallObject::List { data } => data.iter().map(Card::from).collect(),
            ScryfallObject::Error { details } => {
                return Err(InfrastructureError::Unknown(anyhow!(
                    "Received error from Scryfall '{details}'"
                )))
            }
        };

        Ok(cards)
    }

    /// Sends a query to Scryfall's `/cards/search` endpoint.
    /// Scryfall documentation at https://scryfall.com/docs/api/cards/search.
    ///
    /// The query is sent using `self.client` and `self.search_url`.
    /// Query parameters are URL encoded and formatted into the URL
    /// before the request is sent.
    ///
    /// * `query` - The raw fulltext query to be sent to the Scryfall API.
    /// Syntax is specified by Scryfall at https://scryfall.com/docs/syntax.
    async fn search_cards(&self, query: &str) -> InfrastructureResult<Response> {
        self.client
            .get(self.search_url.clone())
            .query(&[("q", query)])
            .send()
            .await
            .map_err(|e| InfrastructureError::Unknown(e.into()))
    }

    async fn parse_response(&self, response: Response) -> InfrastructureResult<ScryfallObject> {
        response
            .json()
            .await
            .map_err(|e| InfrastructureError::Parse(e.to_string()))
    }
}
