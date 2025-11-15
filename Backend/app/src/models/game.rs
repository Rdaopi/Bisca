use axum::extract::ws::Message;
use serde_json::Value;

use super::{
    card::{deal_round, Card, Suit},
    player::Player,
};

#[derive(Debug)]
pub struct GameState {
    pub players: Vec<Player>,
    pub round_number: usize,
    pub starting_cards: usize,
    pub deck: Vec<Card>,
    pub current_turn_cards: Vec<(String, Card)>, // (player_id, card)
    pub leading_suit: Option<Suit>,
}

impl GameState {
    pub fn new_game(num_players: usize, starting_cards: usize) -> Self {
        GameState {
            players: Vec::with_capacity(num_players),
            round_number: 1,
            starting_cards,
            deck: Card::shuffle_deck(),
            current_turn_cards: Vec::new(),
            leading_suit: None,
        }
    }

    pub fn start_game(&mut self) {
        self.round_number = 1;
        self.deck = Card::shuffle_deck();
        self.deal_round();
    }

    pub fn add_player(&mut self, player: Player) -> Result<(), String> {
        if self.players.iter().any(|p| p.id == player.id) {
            return Err("Giocatore gia presente".to_string());
        }
        self.players.push(player);
        Ok(())
    }

    pub fn remove_player(&mut self, player_id: &str) {
        self.players.retain(|p| p.id != player_id);
        self.current_turn_cards
            .retain(|(id, _)| id != player_id);
    }

    pub fn broadcast(&self, event: &str, data: Value) {
        if self.players.is_empty() {
            return;
        }
        let payload = serde_json::json!({ "event": event, "data": data }).to_string();
        for player in &self.players {
            let _ = player.sender.send(Message::Text(payload.clone()));
        }
    }

    pub fn deal_round(&mut self) {
        if self.players.is_empty() {
            return;
        }
        let hands = deal_round(
            &mut self.deck,
            self.players.len(),
            self.round_number,
            self.starting_cards,
        );
        for (player, hand) in self.players.iter_mut().zip(hands) {
            player.hand = hand;
            player.prediction = None;
            player.tricks_won = 0;
        }
        self.current_turn_cards.clear();
        self.leading_suit = None;
    }

    pub fn make_prediction(&mut self, player_id: &str, prediction: u8) -> Result<(), String> {
        let player_index = self
            .player_index(player_id)
            .ok_or_else(|| "Giocatore inesistente".to_string())?;

        if self.players.is_empty() {
            return Err("Nessun giocatore in partita".to_string());
        }

        let predictions_so_far: Vec<u8> = self
            .players
            .iter()
            .filter_map(|p| p.prediction)
            .collect();
        let cards_in_hand = self.players[player_index].hand.len() as u8;

        if predictions_so_far.len() + 1 == self.players.len() {
            let total: u8 = predictions_so_far.iter().copied().sum();
            if total + prediction == cards_in_hand {
                return Err("Ultimo giocatore non puo completare la somma esatta".to_string());
            }
        }

        self.players[player_index].prediction = Some(prediction);
        Ok(())
    }

    pub fn play_card(&mut self, player_id: &str, card: Card) -> Result<(), String> {
        let player_index = self
            .player_index(player_id)
            .ok_or_else(|| "Giocatore inesistente".to_string())?;

        let player = &mut self.players[player_index];
        let card_position = player
            .hand
            .iter()
            .position(|c| c == &card)
            .ok_or_else(|| "Carta non trovata nella mano del giocatore".to_string())?;

        let played_card = player.hand.remove(card_position);

        if self.leading_suit.is_none() {
            self.leading_suit = Some(played_card.suit.clone());
        }

        self.current_turn_cards
            .push((player.id.clone(), played_card));
        Ok(())
    }

    pub fn end_turn(&mut self) -> Option<String> {
        if self.current_turn_cards.is_empty() {
            return None;
        }
        let leading = match self.leading_suit.as_ref() {
            Some(leading) => leading,
            None => return None,
        };

        let mut best_index = 0;
        for i in 1..self.current_turn_cards.len() {
            let (_, ref card_i) = self.current_turn_cards[i];
            let (_, ref card_best) = self.current_turn_cards[best_index];
            if card_i.beats_custom(card_best, leading) {
                best_index = i;
            }
        }

        let winner_id = self.current_turn_cards[best_index].0.clone();
        if let Some(player) = self.players.iter_mut().find(|p| p.id == winner_id) {
            player.tricks_won += 1;
        }

        self.current_turn_cards.clear();
        self.leading_suit = None;
        Some(winner_id)
    }

    pub fn end_round(&self) -> Vec<(String, bool)> {
        self.players
            .iter()
            .map(|p| {
                let success = p.prediction.unwrap_or(0) == p.tricks_won;
                (p.id.clone(), success)
            })
            .collect()
    }

    pub fn next_round(&mut self) {
        self.round_number += 1;
        self.deck = Card::shuffle_deck();
        self.deal_round();
    }

    pub fn is_round_over(&self) -> bool {
        self.players.iter().all(|p| p.hand.is_empty())
    }

    pub fn is_game_over(&self) -> bool {
        self.round_number > self.starting_cards
    }

    fn player_index(&self, player_id: &str) -> Option<usize> {
        self.players.iter().position(|p| p.id == player_id)
    }
}
