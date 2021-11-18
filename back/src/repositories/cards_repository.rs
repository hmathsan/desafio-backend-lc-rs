use crate::model::cards::Card;

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