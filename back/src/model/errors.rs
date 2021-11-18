#[derive(Debug)]
pub enum ApiErrors {
    Validation(String),
    Unauthorized,
    Forbidden,
    Unknown(String),
}