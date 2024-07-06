use actix_web::{post, get, web, HttpResponse};
use std::collections::HashSet;

use crate::blockchain::blockchain::SharedBlockchain;

#[derive(Debug, Deserialize)]
pub struct CreateElectionPayload {
    election_id: String,
    vote_options: HashSet<String>,
}

#[post("/create_election")]
async fn handle_post_create_election(
    blockchain: web::Data<SharedBlockchain>,
    web::Json(payload): web::Json<CreateElectionPayload>,
) -> HttpResponse {
    let mut blockchain = blockchain.lock().unwrap();

    match blockchain.create_election(payload.election_id.clone(), payload.vote_options.clone()) {
        Ok(_) => HttpResponse::Ok().json("Election created successfully"),
        Err(err) => HttpResponse::BadRequest().json(err),
    }
}

#[get("/elections")]
async fn handle_get_all_elections(
    blockchain: web::Data<SharedBlockchain>,
) -> HttpResponse {
    let blockchain = blockchain.lock().unwrap();  // Obtenha uma referência imutável à blockchain

    // Recupere as eleições da blockchain
    let elections: Vec<String> = blockchain.elections.keys().cloned().collect();

    HttpResponse::Ok().json(elections)
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg
        .service(handle_post_create_election)
        .service(handle_get_all_elections);
}