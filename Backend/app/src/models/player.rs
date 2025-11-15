use axum::extract::ws::Message;
use tokio::sync::mpsc::UnboundedSender;

use super::card::Card;

#[derive(Debug)]
pub struct Player {
    pub id: String,
    pub hand: Vec<Card>,
    pub prediction: Option<u8>,
    pub tricks_won: u8,
    pub sender: UnboundedSender<Message>,
}

impl Player {
    pub fn new(id: String, sender: UnboundedSender<Message>) -> Self {
        Self {
            id,
            hand: Vec::new(),
            prediction: None,
            tricks_won: 0,
            sender,
        }
    }
}
