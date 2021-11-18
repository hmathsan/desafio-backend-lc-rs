use rocket::{
    http::Status,
    request::Request,
    http::ContentType,
    serde::{json::Value, Serialize},
    response::{Responder, Response},

};
use serde_json::json;

pub struct ApiResponse {
    pub json_value: Value,
    pub status: Status
}

impl ApiResponse {
    pub fn new<T: Serialize>(response: T, status: Status) -> Self {
        let json_value = json!(response);
        Self { json_value, status }
    }

    pub fn new_without_body(status: Status) -> Self {
        Self { json_value: Value::Null, status }
    }
}

impl<'r, 'o: 'r> Responder<'r, 'o> for ApiResponse {
    fn respond_to(self, request: &'r Request<'_>) -> rocket::response::Result<'o> {
        match self.json_value {
            Value::Null => {
                Response::build().status(self.status).ok()
            },
            _ => {
                Response::build_from(self.json_value.respond_to(request).unwrap())
                    .status(self.status)
                    .header(ContentType::JSON)
                    .ok()
            }
        }
        
    }
}