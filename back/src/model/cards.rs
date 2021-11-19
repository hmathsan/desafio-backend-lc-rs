use rocket::serde::{Serialize, Deserialize};
use uuid::Uuid;

#[derive(Debug, Serialize)]
pub struct Card {
    pub id: String,
    pub titulo: String,
    pub conteudo: String,
    pub lista: String
}

#[derive(Debug, Deserialize)]
pub struct CardRequest {
    pub titulo: String,
    pub conteudo: String,
    pub lista: String
}

impl Card {
    pub fn new(titulo: String, conteudo: String, lista: String) -> Self {
        let id = Uuid::new_v4().to_string();
        Self{ id, titulo, conteudo, lista }
    }

    pub fn from_db(id: String, titulo: String, conteudo: String, lista: String) -> Self {
        Self{ id, titulo, conteudo, lista }
    }
}