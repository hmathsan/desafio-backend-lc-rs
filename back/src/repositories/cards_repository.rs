use crate::model::cards::{Card, CardRequest};

use super::get_conn;

pub async fn find_all_cards() -> Vec<Card> {
    let (client, conn) = get_conn().await.unwrap();

    let mut cards: Vec<Card> = vec![];

    tokio::spawn(async move {
        if let Err(e) = conn.await {
            eprintln!("{}", e);
        }
    });

    for row in &client.query("SELECT * FROM card", &[]).await.unwrap() {
        cards.push(Card::from_db(row.get(0), row.get(1), row.get(2), row.get(3)));
    }

    cards
}

pub async fn crete_new_card(new_card: Card) -> Option<Card> {
    let (client, conn) = get_conn().await.unwrap();

    tokio::spawn(async move {
        if let Err(e) = conn.await {
            eprintln!("{}", e);
        }
    });

    match client.query("INSERT INTO card 
        (id, titulo, conteudo, lista) 
        VALUES ($1, $2, $3, $4);",
    &[&new_card.id, &new_card.titulo, &new_card.conteudo, &new_card.lista]).await {
        Ok(_) => Some(new_card),
        Err(_) => None,
    }


}