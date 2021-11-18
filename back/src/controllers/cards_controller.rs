use rocket::http::Status;

use crate::{
    model::{api_response::ApiResponse, errors::ApiErrors}, 
    repositories::cards_repository::find_all_cards, 
    services::{error_handler::error_handler, guards::auth_guard::AuthValidation}
};

#[get("/")]
pub async fn get_all_cards(auth: Result<AuthValidation, ApiErrors>) -> ApiResponse {
    match auth {
        Ok(_) => {
            let cards = find_all_cards().await;
            return ApiResponse::new(cards, Status::Ok)
        },
        Err(e) => return error_handler(e),
    }
}