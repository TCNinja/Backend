use uuid::Uuid;

#[derive(Clone, Eq, PartialEq, Debug, Default)]
pub enum CardFinish {
    #[default]
    NonFoil,
    #[allow(dead_code)]
    Foil,
    #[allow(dead_code)]
    Etched,
}

#[derive(Clone, Eq, PartialEq, Debug, Default)]
pub struct Card {
    pub id: Uuid,
    pub oracle_id: Uuid,
    pub name: String,
    pub type_line: String,
    pub language: String,
    pub finish: CardFinish,
    pub image_uri: String,
    pub scryfall_uri: String,
    pub scryfall_set_uri: String,
}
