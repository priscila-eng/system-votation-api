use postgres::{ Client, NoTls };
use postgres::error::SqlState;
use jsonwebtoken::{encode, Header, EncodingKey}; 
use std::time::{SystemTime, UNIX_EPOCH};

use crate::responses::responses::*;
use crate::models::models::*;
use crate::utils::utils::*;
use crate::constants::constants::{ DB_URL, SECRET_KEY };

pub fn handle_post_request(request: &str) -> (String, String) {
    match (get_request_body::<User>(&request), Client::connect(DB_URL, NoTls)) {
        (Ok(user), Ok(mut client)) => {
            client
                .execute(
                    "INSERT INTO users (name, email) VALUES ($1, $2)",
                    &[&user.name, &user.email]
                )
                .unwrap();

            (OK_RESPONSE.to_string(), "User created".to_string())
        }
        _ => (INTERNAL_SERVER_ERROR.to_string(), "Error".to_string()),
    }
}

//handle_get_request function
pub fn handle_get_request(request: &str) -> (String, String) {
    match (get_id(&request).parse::<i32>(), Client::connect(DB_URL, NoTls)) {
        (Ok(id), Ok(mut client)) =>
            match client.query_one("SELECT * FROM users WHERE id = $1", &[&id]) {
                Ok(row) => {
                    let user = User {
                        id: row.get(0),
                        name: row.get(1),
                        email: row.get(2),
                    };

                    (OK_RESPONSE.to_string(), serde_json::to_string(&user).unwrap())
                }
                _ => (NOT_FOUND.to_string(), "User not found".to_string()),
            }

        _ => (INTERNAL_SERVER_ERROR.to_string(), "Error".to_string()),
    }
}

//handle_get_all_request function
pub fn handle_get_all_request(request: &str) -> (String, String) {
    match Client::connect(DB_URL, NoTls) {
        Ok(mut client) => {
            let mut users = Vec::new();

            for row in client.query("SELECT * FROM users", &[]).unwrap() {
                users.push(User {
                    id: row.get(0),
                    name: row.get(1),
                    email: row.get(2),
                });
            }

            (OK_RESPONSE.to_string(), serde_json::to_string(&users).unwrap())
        }
        _ => (INTERNAL_SERVER_ERROR.to_string(), "Error".to_string()),
    }
}


//handle_put_request function
pub fn handle_put_request(request: &str) -> (String, String) {
    match
        (
            get_id(&request).parse::<i32>(),
            get_request_body::<User>(&request),
            Client::connect(DB_URL, NoTls),
        )
    {
        (Ok(id), Ok(user), Ok(mut client)) => {
            client
                .execute(
                    "UPDATE users SET name = $1, email = $2 WHERE id = $3",
                    &[&user.name, &user.email, &id]
                )
                .unwrap();

            (OK_RESPONSE.to_string(), "User updated".to_string())
        }
        _ => (INTERNAL_SERVER_ERROR.to_string(), "Error".to_string()),
    }
}

//handle_delete_request function
pub fn handle_delete_request(request: &str) -> (String, String) {
    match (get_id(&request).parse::<i32>(), Client::connect(DB_URL, NoTls)) {
        (Ok(id), Ok(mut client)) => {
            let rows_affected = client.execute("DELETE FROM users WHERE id = $1", &[&id]).unwrap();

            if rows_affected == 0 {
                return (NOT_FOUND.to_string(), "User not found".to_string());
            }

            (OK_RESPONSE.to_string(), "User deleted".to_string())
        }
        _ => (INTERNAL_SERVER_ERROR.to_string(), "Error".to_string()),
    }
}


pub fn handle_post_signup(request: &str) -> (String, String) {
    match get_request_body::<SignupData>(request) {
        Ok(signup_data) => {
            match Client::connect(DB_URL, NoTls) {
                Ok(mut client) => {
                    let hashed_password = hash_password(&signup_data.password); // Função para hash da senha

                    let result = client.execute(
                        "INSERT INTO users (name, email, password) VALUES ($1, $2, $3)",
                        &[&signup_data.name, &signup_data.email, &hashed_password]
                    );

                    match result {
                        Ok(_) => (OK_RESPONSE.to_string(), "New user created".to_string()),
                        Err(err) => {
                            if let Some(db_err) = err.as_db_error() {
                                if db_err.code() == &SqlState::UNIQUE_VIOLATION {
                                    (BAD_REQUEST.to_string(), "Email already exists".to_string())
                                } else {
                                    (INTERNAL_SERVER_ERROR.to_string(), format!("Database error: {}", db_err))
                                }
                            } else {
                                (INTERNAL_SERVER_ERROR.to_string(), "Unknown database error".to_string())
                            }
                        }
                    }
                }
                _ => (INTERNAL_SERVER_ERROR.to_string(), "Error connecting to database".to_string()),
            }
        }
        _ => (INTERNAL_SERVER_ERROR.to_string(), "Error parsing signup data".to_string()),
    }
}

pub fn handle_login_request(request: &str) -> (String, String) {
    match (get_request_body::<LoginData>(&request), Client::connect(DB_URL, NoTls)) {
        (Ok(login_data), Ok(mut client)) => {
            match client.query_one(
                "SELECT id, password FROM users WHERE email = $1",
                &[&login_data.email],
            ) {
                Ok(row) => {
                    let stored_password: String = row.get(1);
                    let user_id: i32 = row.get(0);
                    if verify_password(&login_data.password, &stored_password) {
                        if let Err(e) = client.execute(
                            "DELETE FROM sessions WHERE user_id = $1",
                            &[&user_id],
                        ) {
                            return (INTERNAL_SERVER_ERROR.to_string(), format!("Database error: {}", e));
                        }
                        
                        // Credenciais válidas, gerar token JWT
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
                            Err(_) => return (INTERNAL_SERVER_ERROR.to_string(), "Token generation error".to_string()),
                        };

                        if let Err(e) = client.execute(
                            "INSERT INTO sessions (user_id, token) VALUES ($1, $2)",
                            &[&user_id, &token],
                        ) {
                            return (INTERNAL_SERVER_ERROR.to_string(), format!("Database error: {}", e));
                        }

                        (OK_RESPONSE.to_string(), format!("{{\"token\": \"{}\"}}", token))
                    } else {
                        (UNAUTHORIZED.to_string(), "Invalid credentials".to_string())
                    }
                }
                Err(_) => {
                    // Usuário não encontrado ou erro no banco de dados
                    (UNAUTHORIZED.to_string(), "Invalid credentials".to_string())
                }
            }
        }
        _ => (INTERNAL_SERVER_ERROR.to_string(), "Error".to_string()),
    }
}

pub fn handle_auth_request(request: &str) -> (String, String) {
    match (get_request_body::<AuthData>(&request), Client::connect(DB_URL, NoTls)) {
        (Ok(auth_data), Ok(mut client)) => {
            match client.query_one(
                "SELECT user_id FROM sessions WHERE token = $1",
                &[&auth_data.token],
            ) {
                Ok(row) => {
                    let user_id: i32 = row.get(0);
                    match client.query_one(
                        "SELECT * FROM users WHERE id = $1",
                        &[&user_id],
                    ) {
                        Ok(row) => {
                            (OK_RESPONSE.to_string(), "Token valid".to_string())
                        }
                        _ => (NOT_FOUND.to_string(), "Invalid token".to_string()),
                    
                    }
                }
                Err(_) => {
                    // Usuário não encontrado ou erro no banco de dados
                    (UNAUTHORIZED.to_string(), "Invalid token".to_string())
                }
            }
        }
        _ => (INTERNAL_SERVER_ERROR.to_string(), "Error".to_string()),
    }
}