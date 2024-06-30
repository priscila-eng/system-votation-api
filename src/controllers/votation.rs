use crate::utils::utils::get_request_body;
use crate::responses::responses::*;
use crate::models::models::Votation;

pub fn handle_post_votation(request:&str) -> (String, String) {
    match get_request_body::<Votation>(&request) {
        Ok(votation) => {
            ("Blockchain created".to_string(), "Votation created".to_string())
        }
        _ => (INTERNAL_SERVER_ERROR.to_string(), "Error".to_string()),
    }
} 