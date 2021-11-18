use std::str::FromStr;
use chrono::{Date, DateTime, TimeZone, Utc};

use rocket::serde::json::Json;
use jsonwebtoken::{encode, Header, EncodingKey};


use crate::model::login::{LoginRequest, LoginResponse, User};

const SECRET: &str = dotenv!("JWT_SECRET");
const TOKEN_TEMPO: &str = dotenv!("TOKEN_TEMPO");
const TOKEN_TEMPO_DEFAULT: i64 = 600000;

#[post("/login", format = "application/json", data = "<login_req>")]
pub async fn login(login_req: LoginRequest) -> Json<LoginResponse> {
    let token_tempo: i64 = FromStr::from_str(TOKEN_TEMPO).unwrap_or(TOKEN_TEMPO_DEFAULT);
    let time_to_expire: DateTime<Utc> = Utc.timestamp_millis(Utc::now().timestamp_millis() + token_tempo);

    let user_obj = User{ sub: String::from(login_req.login), exp: time_to_expire.timestamp_millis() as usize };
    
    let token = encode(&Header::default(), &user_obj, &EncodingKey::from_secret(SECRET.as_ref())).unwrap();

    Json(LoginResponse::new(token.to_string(), time_to_expire.to_string()))
}