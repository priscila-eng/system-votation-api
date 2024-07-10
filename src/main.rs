mod controllers;
mod utils;
mod models;
mod database;
mod constants;
mod blockchain;

use actix_cors::Cors;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use std::sync::{Arc, Mutex};
use database::database::set_database;
use controllers::users::*;
use controllers::votation::configure as votation_configure;
use blockchain::blockchain::{Blockchain, SharedBlockchain};

#[macro_use]
extern crate serde_derive;

async fn hello_world() -> impl Responder {
    HttpResponse::Ok().body("Hello, world!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Set up the database
    if let Err(e) = set_database().await {
        eprintln!("Error setting up the database: {:?}", e);
        return Err(std::io::Error::new(std::io::ErrorKind::Other, "Database setup failed"));
    }

    let blockchain = Arc::new(Mutex::new(Blockchain::new()));

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);

        App::new()
            .app_data(web::Data::new(blockchain.clone())) 
            .wrap(cors)
            .route("/", web::get().to(hello_world))
            .route("/signup", web::post().to(handle_post_signup))
            .route("/login", web::post().to(handle_login_request))
            .route("/token", web::post().to(handle_auth_request))
            .configure(votation_configure)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
