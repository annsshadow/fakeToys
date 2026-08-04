use serde::Serialize;

#[derive(Serialize)]
pub struct Person {
    pub unique: String,
    pub name: String,
    pub mobile: Option<String>,
}

#[derive(Serialize)]
pub struct AuthenticationRequest {
    pub credential: String,
    pub password: String,
}
