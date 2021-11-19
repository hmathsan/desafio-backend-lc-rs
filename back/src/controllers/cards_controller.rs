use rocket::http::Status;

use crate::{model::{api_response::ApiResponse, cards::{Card, CardRequest}, errors::ApiErrors}, repositories::cards_repository::{crete_new_card, find_all_cards}, services::{error_handler::error_handler, guards::auth_guard::AuthValidation}};

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

#[post("/", format = "application/json", data = "<body>")]
pub async fn create_new_card(auth: Result<AuthValidation, ApiErrors>, body: Result<CardRequest, ApiErrors>) -> ApiResponse {
    match auth {
        Ok(_) => {
            match body {
                Ok(card_body) => {
                    match crete_new_card(Card::new(card_body.titulo, card_body.conteudo, card_body.lista)).await {
                        Some(card) => ApiResponse::new(card, Status::Created),
                        None => ApiResponse::new(
                            "An error ocurred while saving the card to database.", 
                            Status::InternalServerError)
                    }
                },
                Err(e) => error_handler(e),
            }
        },
        Err(e) => error_handler(e)
    }
}