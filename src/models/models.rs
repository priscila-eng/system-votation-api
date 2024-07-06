#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub iat: usize, // Timestamp de emissão
    pub exp: usize, // Timestamp de expiração
}

//Model: USer struct with id, name, email
#[derive(Serialize, Deserialize)]
pub struct User {
    pub id: Option<i32>,
    pub name: String,
    pub email: String,
}

//Model: SignupData struct with name, email, password
#[derive(Serialize, Deserialize)]
pub struct SignupData {
    pub name: String,
    pub email: String,
    pub password: String, // Add password field for signup
}

//Model: SignupData struct with name, email, password
#[derive(Serialize, Deserialize)]
pub struct LoginData {
    pub email: String,
    pub password: String,
}

//Model: SignupData struct with name, email, password
#[derive(Serialize, Deserialize)]
pub struct AuthData {
    pub token: String,
}