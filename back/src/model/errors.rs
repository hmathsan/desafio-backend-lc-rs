use rocket::serde::Serialize;

#[derive(Debug)]
pub enum ApiErrors {
    Validation(String),
    Unauthorized,
    Forbidden,
    Unknown(String),
}

#[derive(Debug, Serialize)]
pub struct DefaultErrorReturn {
    message: String
}

impl DefaultErrorReturn {
    pub fn new(message: String) -> Self {
        Self{ message }
    }
}