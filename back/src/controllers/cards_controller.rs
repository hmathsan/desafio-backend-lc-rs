use rocket::http::Status;

use crate::{
    repositories::cards_repository::{create_new_card, find_all_cards, update_card, delete_card_by_id}, 
    model::{api_response::ApiResponse, cards::{Card, CardRequest}, errors::ApiErrors}, 
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

#[post("/", format = "application/json", data = "<body>")]
pub async fn post_create_new_card(auth: Result<AuthValidation, ApiErrors>, body: Result<CardRequest, ApiErrors>) -> ApiResponse {
    match auth {
        Ok(_) => {
            match body {
                Ok(card_body) => {
                    match create_new_card(Card::new(card_body.titulo, card_body.conteudo, card_body.lista)).await {
                        Some(card) => ApiResponse::new(card, Status::Created),
                        None => error_handler(ApiErrors::Unknown(format!("An error ocurred while saving the card to database.")))
                    }
                },
                Err(e) => error_handler(e),
            }
        },
        Err(e) => error_handler(e)
    }
}

#[put("/<card_id>", format = "application/json", data = "<body>")]
pub async fn put_update_card(auth: Result<AuthValidation, ApiErrors>, card_id: String, body: Result<CardRequest, ApiErrors>) -> ApiResponse {
    match auth {
        Ok(_) => {
            match body {
                Ok(card_body) => {
                    if card_id.is_empty() { return ApiResponse::new("Card ID must not be empty", Status::BadRequest) }
                    match update_card(Card::from_db(card_id, card_body.titulo, card_body.conteudo, card_body.lista)).await {
                        Ok(card) => {
                            match card {
                                Some(card) => ApiResponse::new(card, Status::Ok),
                                None => error_handler(ApiErrors::Unknown(format!("An error ocurred while saving the card to database.")))
                            }
                        },
                        Err(e) => error_handler(e)
                    }
                },
                Err(e) => error_handler(e)
            }
        },
        Err(e) => error_handler(e)
    }
}

#[delete("/<card_id>")]
pub async fn delete_card(auth: Result<AuthValidation, ApiErrors>, card_id: String) -> ApiResponse {
    match auth {
        Ok(_) => {
            if card_id.is_empty() { return ApiResponse::new("Card ID must not be empty", Status::BadRequest) }
            match delete_card_by_id(card_id).await {
                Ok(cards) => {
                    match cards {
                        Some(cards) => ApiResponse::new(cards, Status::Ok),
                        None => error_handler(ApiErrors::Unknown(format!("An error ocurred while saving the card to database.")))
                    }
                },
                Err(e) => error_handler(e)
            }
        },
        Err(e) => error_handler(e)
    }
}