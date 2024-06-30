//Model: USer struct with id, name, email
#[derive(Serialize, Deserialize)]
pub struct User {
    pub id: Option<i32>,
    pub name: String,
    pub email: String,
}

//Model: Votation struct with id, title, description, end_date
#[derive(Serialize, Deserialize)]
pub struct Votation {
    pub id: Option<i32>,
    pub title: String,
    pub description: String,
    pub end_date: i64
}