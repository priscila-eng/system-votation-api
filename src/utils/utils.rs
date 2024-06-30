use serde::de::DeserializeOwned;
use serde_json::Error;

#[derive(Debug)]
pub enum RequestBodyError {
    EmptyBody,
    DeserializationError(Error),
}

//get_id function
pub fn get_id(request: &str) -> &str {
    request.split("/").nth(2).unwrap_or_default().split_whitespace().next().unwrap_or_default()
}

pub fn get_request_body<T: DeserializeOwned>(request: &str) -> Result<T, RequestBodyError> {
    let body = request.split("\r\n\r\n").last().unwrap_or_default();

    if body.is_empty() {
        return Err(RequestBodyError::EmptyBody);
    }

    serde_json::from_str(body).map_err(RequestBodyError::DeserializationError)
}

// Password hash function (example)
pub fn hash_password(password: &str) -> String {
    // Here you must implement the actual password hashing logic
    // Simple example:
    bcrypt::hash(password, bcrypt::DEFAULT_COST).unwrap()
}

pub fn verify_password(password: &str, hash: &str) -> bool {
    bcrypt::verify(password, hash).unwrap()
}

// pub fn generate_uuid() -> String {
    
// }