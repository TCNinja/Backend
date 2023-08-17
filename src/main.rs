use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use actix_web::{
    get,
    web::{Data, Query},
    App, HttpResponse, HttpServer, Responder,
};
use infrastructure::ScryfallSearchEngine;
use serde::{Deserialize, Serialize};

use crate::card::Card;

mod card;
mod infrastructure;

#[derive(Deserialize)]
struct CardSearchParamaters {
    card_name: String,
}

#[derive(Serialize)]
struct CardSearchResponse {
    results: Vec<Card>,
}

#[get("/cards/search")]
async fn search_cards(
    search_engine: Data<ScryfallSearchEngine>,
    search_parameters: Query<CardSearchParamaters>,
) -> impl Responder {
    match search_engine
        .search_cards_by_name(&search_parameters.card_name)
        .await
    {
        Ok(cards) => HttpResponse::Ok().json(CardSearchResponse { results: cards }),
        Err(e) => {
            eprintln!("{e}");
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    let address = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), 8080);
    let search_engine = ScryfallSearchEngine::new()?;

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(search_engine.clone()))
            .service(search_cards)
    })
    .bind(address)?
    .run()
    .await?;

    Ok(())
}
