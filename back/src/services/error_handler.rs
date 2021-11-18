use rocket::http::Status;

use crate::model::{api_response::ApiResponse, errors::ApiErrors};

pub fn error_handler(api_errors: ApiErrors) -> ApiResponse {
    match api_errors {
        ApiErrors::Validation(message) => return ApiResponse::new(message, Status::BadRequest),
        ApiErrors::Unknown(message) => return ApiResponse::new(message, Status::InternalServerError),
        ApiErrors::Unauthorized => return ApiResponse::new_without_body(Status::Unauthorized),
        ApiErrors::Forbidden => return ApiResponse::new_without_body(Status::Forbidden),
    }
}