use postgres::{ Client, NoTls };

use crate::responses::responses::NOT_FOUND;
use crate::responses::responses::INTERNAL_SERVER_ERROR;
use crate::responses::responses::OK_RESPONSE;
use crate::models::models::User;
use crate::utils::utils::*;

const DB_URL: &str = "postgres://postgres:postgres@db:5432/postgres";

pub fn handle_post_request(request: &str) -> (String, String) {
    match (get_user_request_body(&request), Client::connect(DB_URL, NoTls)) {
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
            get_user_request_body(&request),
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