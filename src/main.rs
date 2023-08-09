use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use actix_web::{get, web::Query, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};

use crate::card::Card;

mod card;

#[derive(Deserialize)]
struct CardSearchParamaters {
    card_name: String,
}

#[derive(Serialize)]
struct CardSearchResponse {
    results: Vec<Card>,
}

#[get("/cards/search")]
async fn search_cards(search_parameters: Query<CardSearchParamaters>) -> impl Responder {
    let card = Card {
        name: search_parameters.card_name.clone(),
        ..Default::default()
    };

    HttpResponse::Ok().json(CardSearchResponse {
        results: vec![card],
    })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let address = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), 8080);
    HttpServer::new(|| App::new().service(search_cards))
        .bind(address)?
        .run()
        .await
}
