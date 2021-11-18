use rocket::{data::{Data, ToByteUnit, FromData, Outcome}, http::Status, request::Request, tokio::io::AsyncReadExt};

use crate::model::{errors::{ApiErrors}, login::LoginRequest};

const DEFAULT_LOGIN: &str = dotenv!("DEFAULT_LOGIN");
const DEFAULT_SENHA: &str = dotenv!("DEFAULT_SENHA");

#[rocket::async_trait]
impl<'r> FromData<'r> for LoginRequest {
    type Error = ApiErrors;

    async fn from_data(_req: &'r Request<'_>, data: Data<'r>) -> Outcome<'r, Self, Self::Error> {
        let mut body = String::new();
        if let Err(e) = data.open(1.mebibytes()).read_to_string(&mut body).await {
            Outcome::Failure((Status::InternalServerError, ApiErrors::Unknown(format!("{:?}", e))))
        } else {
            match serde_json::from_str(&body[..]) {
                Ok(b) => {
                    let login_req: LoginRequest = LoginRequest::from(b);
                    if login_req.login == DEFAULT_LOGIN && login_req.senha == DEFAULT_SENHA {
                        return Outcome::Success(login_req)
                    } else {
                        return Outcome::Failure((Status::Forbidden, ApiErrors::Forbidden))
                    }
                },
                Err(e) => {
                    return Outcome::Failure((Status::InternalServerError, ApiErrors::Validation(format!("{}", e))))
                },
            }
        }
    }
}