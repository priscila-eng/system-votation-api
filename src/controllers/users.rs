use actix_web::{web, HttpResponse, Responder};
use tokio_postgres::NoTls;
use jsonwebtoken::{encode, Header, EncodingKey};
use std::time::{SystemTime, UNIX_EPOCH};
use serde::Serialize;

use crate::utils::utils::{ hash_password, verify_password };
use crate::models::models::{SignupData, Claims, LoginData, AuthData};
use crate::constants::constants::{ DB_URL, SECRET_KEY };

#[derive(Serialize)]
struct TokenResponse {
    token: String,
}

pub async fn handle_post_signup(body: web::Json<SignupData>) -> impl Responder {
    let signup_data = body.into_inner();

    // Conectar ao banco de dados
    let (client, connection) = match tokio_postgres::connect(DB_URL, NoTls).await {
        Ok(result) => result,
        Err(_) => return HttpResponse::InternalServerError().body("Error connecting to database"),
    };

    // Executa a conexão em uma tarefa separada
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("Database connection error: {:?}", e);
        }
    });

    // Hash da senha
    let hashed_password = hash_password(&signup_data.password); 


    // Inserir o usuário no banco de dados
    let result = client.execute(
        "INSERT INTO users (name, email, password) VALUES ($1, $2, $3)",
        &[&signup_data.name, &signup_data.email, &hashed_password]
    ).await;

    match result {
        Ok(_) => HttpResponse::Ok().body("New user created"),
        Err(err) => {
            // Tratar erros de banco de dados
            if let Some(pq_err) = err.as_db_error() {
                if pq_err.code() == &tokio_postgres::error::SqlState::UNIQUE_VIOLATION {
                    HttpResponse::BadRequest().body("Email already exists")
                } else {
                    HttpResponse::InternalServerError().body(format!("Database error: {}", pq_err))
                }
            } else {
                HttpResponse::InternalServerError().body("Unknown database error")
            }
        }
    }
}

pub async fn handle_login_request(body: web::Json<LoginData>) -> impl Responder {
    let login_data = body.into_inner();

    // Conectar ao banco de dados
    let (client, connection) = match tokio_postgres::connect(DB_URL, NoTls).await {
        Ok(result) => result,
        Err(_) => return HttpResponse::InternalServerError().body("Error connecting to database"),
    };

    // Executa a conexão em uma tarefa separada
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("Database connection error: {:?}", e);
        }
    });

    // Consultar o usuário
    let row = match client.query_one(
        "SELECT id, password FROM users WHERE email = $1",
        &[&login_data.email],
    ).await {
        Ok(row) => row,
        Err(_) => return HttpResponse::Unauthorized().body("Invalid credentials"),
    };

    let stored_password: String = row.get(1);
    let user_id: i32 = row.get(0);

    if !verify_password(&login_data.password, &stored_password) {
        return HttpResponse::Unauthorized().body("Invalid credentials");
    }

    // Deletar sessões anteriores
    if let Err(e) = client.execute(
        "DELETE FROM sessions WHERE user_id = $1",
        &[&user_id],
    ).await {
        return HttpResponse::InternalServerError().body(format!("Database error: {}", e));
    }

    // Gerar token JWT
    let now = SystemTime::now().duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs() as usize;
    let expiration = now + 60 * 60; // 1 hora de validade
    let claims = Claims {
        sub: login_data.email.clone(),
        iat: now,
        exp: expiration,
    };

    let token = match encode(&Header::default(), &claims, &EncodingKey::from_secret(SECRET_KEY)) {
        Ok(t) => t,
        Err(_) => return HttpResponse::InternalServerError().body("Token generation error"),
    };

    // Inserir nova sessão
    if let Err(e) = client.execute(
        "INSERT INTO sessions (user_id, token) VALUES ($1, $2)",
        &[&user_id, &token],
    ).await {
        return HttpResponse::InternalServerError().body(format!("Database error: {}", e));
    }

    HttpResponse::Ok().json(TokenResponse { token })
}

pub async fn handle_auth_request(body: web::Json<AuthData>) -> impl Responder {
    let auth_data = body.into_inner();

    // Conectar ao banco de dados
    let (client, connection) = match tokio_postgres::connect(DB_URL, NoTls).await {
        Ok(result) => result,
        Err(_) => return HttpResponse::InternalServerError().body("Error connecting to database"),
    };

    // Executa a conexão em uma tarefa separada
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("Database connection error: {:?}", e);
        }
    });

    // Consultar a sessão
    let row = match client.query_one(
        "SELECT user_id FROM sessions WHERE token = $1",
        &[&auth_data.token],
    ).await {
        Ok(row) => row,
        Err(_) => return HttpResponse::Unauthorized().body("Invalid token"),
    };

    let user_id: i32 = row.get(0);

    // Consultar o usuário
    match client.query_one(
        "SELECT 1 FROM users WHERE id = $1",
        &[&user_id],
    ).await {
        Ok(_) => HttpResponse::Ok().body("Token valid"),
        Err(_) => HttpResponse::NotFound().body("Invalid token"),
    }
}