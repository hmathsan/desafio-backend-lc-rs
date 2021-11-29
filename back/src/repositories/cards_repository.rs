use crate::model::{cards::Card, errors::ApiErrors};

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

pub async fn create_new_card(new_card: Card) -> Option<Card> {
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

pub async fn update_card(card: Card) -> Result<Option<Card>, ApiErrors> {
    let (client, conn) = get_conn().await.unwrap();

    tokio::spawn(async move {
        if let Err(e) = conn.await {
            eprintln!("{}", e);
        }
    });

    if &client.query("SELECT * FROM card WHERE id = $1", &[&card.id]).await.unwrap().len() <= &0 {
        return Err(ApiErrors::Validation(format!("Card id {} doesn't exist.", &card.id)))
    }

    match client.query("UPDATE card
        SET lista = $1, conteudo = $2, titulo = $3
        WHERE id = $4",
        &[&card.lista, &card.conteudo, &card.titulo, &card.id]).await {

        Ok(_) => Ok(Some(card)),
        Err(_) => Ok(None)
    }
}

pub async fn delete_card_by_id(card_id: String) -> Result<Option<Vec<Card>>, ApiErrors> {
    let (client, conn) = get_conn().await.unwrap();

    tokio::spawn(async move {
        if let Err(e) = conn.await {
            eprintln!("{}", e);
        }
    });

    if &client.query("SELECT * FROM card WHERE id = $1", &[&card_id]).await.unwrap().len() <= &0 {
        return Err(ApiErrors::Validation(format!("Card id {} doesn't exist.", &card_id)))
    }

    match client.query("DELETE FROM card WHERE id = $1", &[&card_id]).await {
        Ok(_) => Ok(Some(find_all_cards().await)),
        Err(_) => Ok(None)
    }
}