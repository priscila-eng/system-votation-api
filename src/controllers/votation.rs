use actix_web::{post, get, web, HttpResponse, HttpRequest};
use std::collections::HashSet;
use jsonwebtoken::{decode, Validation, DecodingKey};

use crate::blockchain::blockchain::SharedBlockchain;
use crate::models::models::Claims;
use crate::constants::constants::SECRET_KEY;

#[derive(Debug, Deserialize)]
pub struct CreateElectionPayload {
    election_id: String,
    vote_options: HashSet<String>,
}

#[derive(Debug, Deserialize)]
pub struct VotePayload {
    election_id: String,
    vote_option_id: String,
}

fn extract_user_id_from_token(req: &HttpRequest) -> Result<String, HttpResponse> {
    if let Some(auth_header) = req.headers().get("Authorization") {
        if let Ok(auth_str) = auth_header.to_str() {
            if auth_str.starts_with("Bearer ") {
                let token = &auth_str[7..];
                let token_data = decode::<Claims>(
                    token,
                    &DecodingKey::from_secret(SECRET_KEY.as_ref()),
                    &Validation::default(),
                );

                return match token_data {
                    Ok(data) => Ok(data.claims.sub),
                    Err(_) => Err(HttpResponse::Unauthorized().body("Invalid token")),
                };
            }
        }
    }
    Err(HttpResponse::Unauthorized().body("Missing or malformed Authorization header"))
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

#[post("/vote")]
async fn handle_post_vote(
    req: HttpRequest,
    blockchain: web::Data<SharedBlockchain>,
    web::Json(payload): web::Json<VotePayload>,
) -> HttpResponse {
    let voter_id = match extract_user_id_from_token(&req) {
        Ok(id) => id,
        Err(resp) => return resp,
    };

    let mut blockchain = blockchain.lock().unwrap();

    match blockchain.add_vote_operation(voter_id, payload.election_id.clone(), payload.vote_option_id.clone()) {
        Ok(_) => HttpResponse::Ok().json("Vote added successfully"),
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
        .service(handle_post_vote)
        .service(handle_get_all_elections);
}
