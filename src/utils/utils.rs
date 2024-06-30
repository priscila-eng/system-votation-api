use crate::models::models::*;

//get_id function
pub fn get_id(request: &str) -> &str {
    request.split("/").nth(2).unwrap_or_default().split_whitespace().next().unwrap_or_default()
}

//deserialize user from request body with the id
pub fn get_user_request_body(request: &str) -> Result<User, serde_json::Error> {
    serde_json::from_str(request.split("\r\n\r\n").last().unwrap_or_default())
}

pub fn get_votation_request_body(request: &str) -> Result<Votation, serde_json::Error> {
    serde_json::from_str(request.split("\r\n\r\n").last().unwrap_or_default())
}

// Function to deserialize the signup request body
pub fn get_signup_request_body(request: &str) -> Result<SignupData, serde_json::Error> {
    serde_json::from_str(request.split("\r\n\r\n").last().unwrap_or_default())
}

// Password hash function (example)
pub fn hash_password(password: &str) -> String {
    // Here you must implement the actual password hashing logic
    // Simple example:
    bcrypt::hash(password, bcrypt::DEFAULT_COST).unwrap()
}