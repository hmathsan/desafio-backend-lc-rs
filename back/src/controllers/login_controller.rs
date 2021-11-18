use std::str::FromStr;

use rocket::http::Status;
use chrono::{DateTime, TimeZone, Utc};
use jsonwebtoken::{encode, Header, EncodingKey};


use crate::{model::{api_response::ApiResponse, errors::ApiErrors, login::{LoginRequest, LoginResponse, User}}, services::error_handler::error_handler};

const SECRET: &str = dotenv!("JWT_SECRET");
const TOKEN_TEMPO: &str = dotenv!("TOKEN_TEMPO");
const TOKEN_TEMPO_DEFAULT: i64 = 600000;

#[post("/", format = "application/json", data = "<login_req>")]
pub async fn login(login_req: Result<LoginRequest, ApiErrors>) -> ApiResponse {
    match login_req {
        Ok(login) => {
            let token_tempo: i64 = FromStr::from_str(TOKEN_TEMPO).unwrap_or(TOKEN_TEMPO_DEFAULT);
            let time_to_expire: DateTime<Utc> = Utc.timestamp_millis(Utc::now().timestamp_millis() + token_tempo);
        
            let user_obj = User{ sub: String::from(login.login), exp: time_to_expire.timestamp_millis() as usize };
        
            let token = encode(&Header::default(), &user_obj, &EncodingKey::from_secret(SECRET.as_ref())).unwrap();
        
            return ApiResponse::new(
                LoginResponse::new(
                    token.to_string(), time_to_expire.format("%d/%m/%Y %T %Z").to_string()
                ), Status::Ok)
        },
        Err(e) => {
            return error_handler(e);
        },
    }
}