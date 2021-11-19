use rocket::http::Status;

use crate::model::{api_response::ApiResponse, errors::{ApiErrors, DefaultErrorReturn}};

pub fn error_handler(api_errors: ApiErrors) -> ApiResponse {
    match api_errors {
        ApiErrors::Validation(message) => ApiResponse::new(DefaultErrorReturn::new(message), Status::BadRequest),
        ApiErrors::Unknown(message) => ApiResponse::new(DefaultErrorReturn::new(message), Status::InternalServerError),
        ApiErrors::Unauthorized => ApiResponse::new_without_body(Status::Unauthorized),
        ApiErrors::Forbidden => ApiResponse::new_without_body(Status::Forbidden),
    }
}