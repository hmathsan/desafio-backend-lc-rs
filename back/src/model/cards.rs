use rocket::serde::{Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize)]
pub struct Card {
    id: String,
    titulo: String,
    conteudo: String,
    lista: String
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