use rocket::{
    serde::{Deserialize, Serialize},
};

#[derive(Deserialize, Debug)]
pub struct LoginRequest {
    pub login: String,
    pub senha: String
}

#[derive(Serialize, Debug)]
pub struct LoginResponse {
    token: String,
    expires_in: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub sub: String,
    pub exp: usize,
}

impl LoginResponse {
    pub fn new(token: String, expires_in: String) -> Self {
        Self { token, expires_in }
    }
}