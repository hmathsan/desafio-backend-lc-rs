use rocket::{data::{Data, ToByteUnit, FromData, Outcome}, http::Status, request::Request, tokio::io::AsyncReadExt};

use crate::model::{cards::CardRequest, errors::ApiErrors};

#[rocket::async_trait]
impl<'r> FromData<'r> for CardRequest {
    type Error = ApiErrors;

    async fn from_data(_req: &'r Request<'_>, data: Data<'r>) -> Outcome<'r, Self, Self::Error> {
        let mut body = String::new();
        if let Err(e) = data.open(1.mebibytes()).read_to_string(&mut body).await {
            Outcome::Failure((
                Status::InternalServerError, 
                ApiErrors::Unknown(format!("An unknown error ocurred while converting body data: {:?}", e))
            ))
        } else {
            match serde_json::from_str(&body[..]) {
                Ok(body) => {
                    let card_body: CardRequest = CardRequest::from(body);
                    if card_body.titulo.is_empty() || card_body.conteudo.is_empty() || card_body.lista.is_empty() { 
                        Outcome::Failure((Status::BadGateway, ApiErrors::Validation(String::from("Os campos não podem estar vazios."))))
                    } else {
                        Outcome::Success(card_body)
                    }
                },
                Err(e) => Outcome::Failure((Status::BadGateway, ApiErrors::Validation(format!("{}", e)))),
            }
        }
    }
}