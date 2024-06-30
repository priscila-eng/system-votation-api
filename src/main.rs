mod controllers;
mod responses;
mod utils;
mod models;
mod database;
mod constants;
mod routes;

use std::net::TcpListener;
use responses::responses::*;
use database::database::set_database;
use routes::routes::handle_client;
// use std::env;
// use chrono::NaiveDate;

#[macro_use]
extern crate serde_derive;

//main function
fn main() {
    //Set database
    if let Err(e) = set_database() {
        println!("Error: {}", e);
        return;
    }

    //start server and print port
    let listener = TcpListener::bind(format!("0.0.0.0:8080")).unwrap();
    println!("Server started at port 8080");

    //handle the client
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                handle_client(stream);
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
}