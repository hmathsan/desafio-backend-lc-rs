use serde::Serialize;

#[derive(Debug)]
pub enum ApiErrors {
    Validation(ValidationError),
    Unauthorized,
    Forbidden,
    Unknown(String),
}

#[derive(Serialize, Debug)]
pub struct ValidationError {
    pub message: String,
}