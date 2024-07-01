mod controllers;
mod utils;
mod models;
mod database;
mod constants;

use actix_cors::Cors;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use database::database::set_database;
use controllers::users::*;
use controllers::votation::*;

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

    HttpServer::new(|| {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);

        App::new()
            .wrap(cors)
            .route("/", web::get().to(hello_world))
            .route("/signup", web::post().to(handle_post_signup))
            .route("/login", web::post().to(handle_login_request))
            .route("/token", web::post().to(handle_auth_request))
            .route("/votation", web::post().to(handle_post_votation))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
