use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize, Clone, Eq, PartialEq, Debug, Default)]
pub enum CardFinish {
    #[default]
    NonFoil,
    #[allow(dead_code)]
    Foil,
    #[allow(dead_code)]
    Etched,
}

#[derive(Serialize, Clone, Eq, PartialEq, Debug, Default)]
pub struct Card {
    pub id: Uuid,
    pub oracle_id: Uuid,
    pub name: String,
    pub type_line: String,
    pub language: String,
    pub image_uri: String,
    pub scryfall_uri: String,
    pub scryfall_set_uri: String,
}

#[derive(Serialize, Clone, Eq, PartialEq, Debug, Default)]
pub struct CardInstance {
    pub card: Card,
    pub finish: CardFinish,
}
