mod controllers;
mod responses;
mod utils;
mod models;

use postgres::{ Client, NoTls };
use postgres::Error as PostgresError;
use std::net::{ TcpListener, TcpStream };
use std::io::{ Read, Write };
use responses::responses::*;
use controllers::users::*;
use controllers::votation::*;

// use std::env;
// use chrono::NaiveDate;

#[macro_use]
extern crate serde_derive;

//DATABASE_URL
// const DB_URL: &str = env!("DATABASE_URL");
const DB_URL: &str = "postgres://postgres:postgres@db:5432/postgres";

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

//handle_client function
fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    let mut request = String::new();

    match stream.read(&mut buffer) {
        Ok(size) => {
            request.push_str(String::from_utf8_lossy(&buffer[..size]).as_ref());

            let (status_line, content) = match &*request {
                r if r.starts_with("POST /users") => handle_post_request(r),
                r if r.starts_with("GET /users/") => handle_get_request(r),
                r if r.starts_with("GET /users") => handle_get_all_request(r),
                r if r.starts_with("PUT /users/") => handle_put_request(r),
                r if r.starts_with("DELETE /users/") => handle_delete_request(r),
                r if r.starts_with("POST /votation") => handle_post_votation(r),
                _ => (NOT_FOUND.to_string(), "404 Not Found".to_string()),
            };

            stream.write_all(format!("{}{}", status_line, content).as_bytes()).unwrap();
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }
}

//set_database function
fn set_database() -> Result<(), PostgresError> {
    //Connect to database
    let mut client = Client::connect(DB_URL, NoTls)?;

    //Create table
    client.batch_execute(
        "CREATE TABLE IF NOT EXISTS users (
            id SERIAL PRIMARY KEY,
            name VARCHAR NOT NULL,
            email VARCHAR NOT NULL
        )"
    )?;
    Ok(())
}