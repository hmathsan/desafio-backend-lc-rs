use rocket::{http::Status, request::{Request, FromRequest, Outcome}};
use jsonwebtoken::{decode, Validation, DecodingKey};

use crate::model::{errors::ApiErrors, login::User};

const SECRET: &str = dotenv!("JWT_SECRET");

pub struct AuthValidation {
    pub user: User
}

fn is_token_valid(token: &str) -> Option<User> {
    match decode::<User>(token, &DecodingKey::from_secret(SECRET.as_ref()), &Validation::default()) {
        Ok(user) => return Some(user.claims),
        Err(_) => return None,
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AuthValidation {
    type Error = ApiErrors;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        match req.headers().get_one("Authorization") {
            Some(token) => {
                let token: Vec<&str> = token.split(' ').collect();
                match is_token_valid(token[1]) {
                    Some(user) => return Outcome::Success(Self{user}),
                    None => return Outcome::Failure((Status::Forbidden, ApiErrors::Forbidden))
                }
            },
            None => return Outcome::Failure((Status::Unauthorized, ApiErrors::Unauthorized)),
        }
    }
}