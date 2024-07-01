
// use std::net::TcpStream;
// use crate::responses::responses::NOT_FOUND;
// use crate::controllers::users::*;
// use crate::controllers::votation::*;
// use std::io::{ Read, Write };

// //handle_client function
// pub fn handle_client(mut stream: TcpStream) {
//     let mut buffer = [0; 1024];
//     let mut request = String::new();

//     match stream.read(&mut buffer) {
//         Ok(size) => {
//             request.push_str(String::from_utf8_lossy(&buffer[..size]).as_ref());

//             let (status_line, content) = match &*request {
//                 r if r.starts_with("POST /users") => handle_post_request(r),
//                 r if r.starts_with("GET /users/") => handle_get_request(r),
//                 r if r.starts_with("GET /users") => handle_get_all_request(r),
//                 r if r.starts_with("PUT /users/") => handle_put_request(r),
//                 r if r.starts_with("DELETE /users/") => handle_delete_request(r),
//                 r if r.starts_with("POST /votation") => handle_post_votation(r), // create a votation
//                 r if r.starts_with("POST /signup") => handle_post_signup(r), // register an user
//                 r if r.starts_with("POST /login") => handle_login_request(r), // do the login
//                 r if r.starts_with("POST /token") => handle_auth_request(r), // checks if a token is valid
//                 _ => (NOT_FOUND.to_string(), "404 Not Found".to_string()),
//             };

//             stream.write_all(format!("{}{}", status_line, content).as_bytes()).unwrap();
//         }
//         Err(e) => {
//             println!("Error: {}", e);
//         }
//     }
// }