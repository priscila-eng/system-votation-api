use crate::utils::utils::get_votation_request_body;
use crate::responses::responses::*;

pub fn handle_post_votation(request:&str) -> (String, String) {

    match get_votation_request_body(&request) {
        Ok(votation) => {
            ("Blockchain created".to_string(), "Votation created".to_string())
        }
        _ => (INTERNAL_SERVER_ERROR.to_string(), "Error".to_string()),
    }
    

} 