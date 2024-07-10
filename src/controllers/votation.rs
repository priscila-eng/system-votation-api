use actix_web::web::service;
use actix_web::{post, get, web, HttpResponse, HttpRequest};
use sha2::digest::typenum::Integer;
use std::collections::HashSet;
use jsonwebtoken::{decode, Validation, DecodingKey};
use std::collections::HashMap;

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

#[derive(Deserialize)]
struct ElectionQuery {
    election_id: Option<String>,
    voter_id: Option<String>,
}

#[derive(Deserialize)]
struct ResultsQuery {
    election_id: Option<String>
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

#[post("/election")]
async fn handle_post_create_election(
    req: HttpRequest,
    blockchain: web::Data<SharedBlockchain>,
    web::Json(payload): web::Json<CreateElectionPayload>,
) -> HttpResponse {
    let creator_id = match extract_user_id_from_token(&req) {
        Ok(id) => id,
        Err(resp) => return resp,
    };

    if payload.vote_options.len() > 20 {
        return HttpResponse::BadRequest().json("Cannot create more than 20 vote options");
    }

    let mut blockchain = blockchain.lock().unwrap();

    match blockchain.create_election(payload.election_id.clone(), payload.vote_options.clone(), creator_id) {
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
    req: HttpRequest,
    blockchain: web::Data<SharedBlockchain>,
    query: web::Query<ElectionQuery>,
) -> HttpResponse {
    println!("Received request to handle_get_all_elections");

    let blockchain = blockchain.lock().unwrap();

    let voter_id_extract = match extract_user_id_from_token(&req) {
        Ok(id) => id,
        Err(resp) => return resp,
    };

    // Buscar todas as eleições em que o usuário participou
    let elections = blockchain.get_elections_by_user(&voter_id_extract);

    // Inicializar vetor de respostas
    let mut responses = Vec::new();

    // Para cada eleição em que o usuário participou, criar a resposta correspondente
    for (election_id, vote_option_id) in elections {
        if let Some(election) = blockchain.elections.get(&election_id) {
            let mut response = serde_json::json!({
                "election_id": election_id,
                "vote_options": election.iter().cloned().collect::<Vec<_>>(), // Assumindo que election é um HashSet de opções de voto
            });

            // Verificar se o vote_option_id está na lista de opções de voto da eleição
            if election.contains(&vote_option_id) {
                response["user_vote"] = serde_json::json!(vote_option_id);
            } else {
                response["user_vote"] = serde_json::Value::Null;
            }

            responses.push(response);
        } else {
            println!("Election not found for id: {:?}", election_id);
        }
    }

    // Retornar a resposta com todas as eleições que o usuário participou
    HttpResponse::Ok().json(responses)
}

#[get("/user/created-elections")]
async fn handle_get_elections_created_by_user(
    req: HttpRequest,
    blockchain: web::Data<SharedBlockchain>,
) -> HttpResponse {
    println!("Received request to handle_get_elections_created_by_user");

    let blockchain = blockchain.lock().unwrap();

    let creator_id_extract = match extract_user_id_from_token(&req) {
        Ok(id) => id,
        Err(resp) => return resp,
    };

    // Buscar todas as eleições em que o usuário participou
    let elections = blockchain.get_elections_created_by_user(&creator_id_extract);

    // Inicializar vetor de respostas
    let mut responses = Vec::new();

    // Para cada eleição em que o usuário participou, criar a resposta correspondente
    for (election_id) in elections {
        if let Some(election) = blockchain.elections.get(&election_id) {
            let mut response = serde_json::json!({
                "election_id": election_id,
                "vote_options": election.iter().cloned().collect::<Vec<_>>(), // Assumindo que election é um HashSet de opções de voto
            });

            // Verificar se o vote_option_id está na lista de opções de voto da eleição
            // if election.contains(&vote_option_id) {
            //     response["user_vote"] = serde_json::json!(vote_option_id);
            // } else {
            //     response["user_vote"] = serde_json::Value::Null;
            // }

            responses.push(response);
        } else {
            println!("Election not found for id: {:?}", election_id);
        }
    }

    // Retornar a resposta com todas as eleições que o usuário participou
    HttpResponse::Ok().json(responses)
}


#[get("/election")]
async fn handle_get_election(
    req: HttpRequest,
    blockchain: web::Data<SharedBlockchain>,
    query: web::Query<ElectionQuery>,
) -> HttpResponse {
    println!("Received request to handle_get_election");

    let blockchain = blockchain.lock().unwrap();

    // println!("elections filter: {:?}", blockchain.get_results_election(&query.election_id));

    // Verifique se o election_id foi fornecido na query
    if let Some(election_id) = &query.election_id {
        println!("Query parameter election_id: {:?}", election_id);

        // Procure a eleição pelo ID
        if let Some(election) = blockchain.elections.get(election_id) {
            println!("Found election: {:?}", election);

            // Inicialize o objeto de resposta
            let mut response = serde_json::json!({
                "election_id": election_id,
                "vote_options": election.iter().cloned().collect::<Vec<_>>(), // Assumindo que election é um HashSet de opções de voto
            });

            // Se voter_id for fornecido, recupere o voto do usuário
            if let Some(voter_id) = &query.voter_id {
                let voter_id_extract = match extract_user_id_from_token(&req) {
                    Ok(id) => id,
                    Err(resp) => return resp,
                };
                println!("Found vote id: {:?}", blockchain.get_votes_by_user(&voter_id_extract, election_id));
                if let Some((_, vote_option_id)) = blockchain.get_votes_by_user(&voter_id_extract, election_id) {
                    // Verificar se o vote_option_id está na lista de opções de voto da eleição
                    if election.contains(&vote_option_id) {
                        response["user_vote"] = serde_json::json!(vote_option_id);
                    } else {
                        response["user_vote"] = serde_json::Value::Null;
                    }
                } else {
                    response["user_vote"] = serde_json::Value::Null;
                }
            } else {
                // Se não houver voter_id na query, definir user_vote como null
                response["user_vote"] = serde_json::Value::Null;
            }

            HttpResponse::Ok().json(response)
        } else {
            println!("Election not found for id: {:?}", election_id);
            HttpResponse::NotFound().json("Election not found")
        }
    } else {
        println!("Missing election_id query parameter");
        HttpResponse::BadRequest().json("Missing election_id query parameter")
    }
}

#[get("/results")]
async fn handle_get_results_election(
    blockchain: web::Data<SharedBlockchain>,
    query: web::Query<ResultsQuery>,
) -> HttpResponse {

    
    let blockchain = blockchain.lock().unwrap();
    
    // Verifique se o election_id foi fornecido na query
    if let Some(election_id) = &query.election_id {
        println!("elections filter: {:?}", blockchain.get_results_election(election_id));
        println!("Query parameter election_id: {:?}", election_id);

        let elections = blockchain.get_results_election(election_id);

        // Inicializar vetor de respostas
        let mut responses = Vec::new();

        let mut votes: HashMap<String, Vec<String>> = HashMap::new();

        for (election_id, vote_option_id) in elections {

            if let Some(election) = blockchain.elections.get(&election_id){
                let mut response = serde_json::json!({
                    "election_id": election_id,
                    "vote_options": election.iter().cloned().collect::<Vec<_>>(), // Assumindo que election é um HashSet de opções de voto
                });
                if !votes.contains_key(&vote_option_id) {
                    for vote_option in election {
                        votes.insert(String::from(vote_option), Vec::new());
                    }
                }
            } else {
                println!("Election not found for id: {:?}", election_id);
            }
            
           
            println!("vote 1: {:?}", votes);
            votes.entry(vote_option_id.clone()).or_insert(Vec::new()).push(String::from(vote_option_id));
            println!("vote 2: {:?}", votes);
        }
        // Retornar a resposta com todas as eleições que o usuário participou
        let mut response = serde_json::json!({
            "election_id": election_id,
        });

        for (key, value) in &votes {
            println!("{}: {}", key, value.len());
            response[key] = serde_json::json!(value.len());
        }
        responses.push(response);
        HttpResponse::Ok().json(responses)
    } else {
        println!("Missing election_id query parameter");
        HttpResponse::BadRequest().json("Missing election_id query parameter")
    }
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg
        .service(handle_post_create_election)
        .service(handle_post_vote)
        .service(handle_get_all_elections)
        .service(handle_get_election)
        .service(handle_get_results_election)
        .service(handle_get_elections_created_by_user);
}
