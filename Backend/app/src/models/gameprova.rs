use crate::cards::{Card, Suit, deal_round, beats_custom, shuffle_deck};

#[derive(Debug)]
pub struct Player {
    pub id: usize,
    pub hand: Vec<Card>,
    pub prediction: Option<u8>,
    pub tricks_won: u8,
}

#[derive(Debug)]
pub struct GameState {
    pub players: Vec<Player>,
    pub round_number: usize,
    pub starting_cards: usize,
    pub deck: Vec<Card>,
    pub current_turn_cards: Vec<(usize, Card)>, // (player_index, card)
    pub leading_suit: Option<Suit>,
}

// ========================
// Funzioni principali
// ========================

impl GameState {
    //  Crea una nuova partita
    pub fn new_game(num_players: usize, starting_cards: usize) -> Self {
        let mut deck = Card::shuffle_deck();
        let players = (0..num_players).map(|i| Player {
            id: i,
            hand: Vec::new(),
            prediction: None,
            tricks_won: 0,
        }).collect();

        GameState {
            players,
            round_number: 1,
            starting_cards,
            deck,
            current_turn_cards: Vec::new(),
            leading_suit: None,
        }
    }

    //  Distribuisce le carte per il round corrente
    pub fn deal_round(&mut self) {
        let hands = deal_round(
            &mut self.deck,
            self.players.len(),
            self.round_number,
            self.starting_cards,
        );
        for (i, hand) in hands.into_iter().enumerate() {
            self.players[i].hand = hand;
        }
        self.current_turn_cards.clear();
        self.leading_suit = None;
        for player in &mut self.players {
            player.prediction = None;
            player.tricks_won = 0;
        }
    }

    //  Registrare la predizione di un giocatore
    pub fn make_prediction(&mut self, player_index: usize, prediction: u8) -> Result<(), String> {
        // Controllo regola ultimo giocatore
        let predictions_so_far: Vec<u8> = self.players.iter()
            .filter_map(|p| p.prediction)
            .collect();
        let cards_in_hand = self.players[player_index].hand.len() as u8;
        if predictions_so_far.len() as u8 == (self.players.len() as u8 - 1) {
            let total: u8 = predictions_so_far.iter().sum();
            if total + prediction == cards_in_hand as u8 {
                return Err("Ultimo giocatore non può completare la somma esatta".to_string());
            }
        }
        self.players[player_index].prediction = Some(prediction);
        Ok(())
    }

    //  Giocare una carta
    pub fn play_card(&mut self, player_index: usize, card_index: usize) -> Result<(), String> {
        if player_index >= self.players.len() {
            return Err("Giocatore inesistente".to_string());
        }
        let card = self.players[player_index].hand.remove(card_index);
        if self.leading_suit.is_none() {
            self.leading_suit = Some(card.suit.clone());
        }
        self.current_turn_cards.push((player_index, card));
        Ok(())
    }


    //  Determina il vincitore del turno
    pub fn end_turn(&mut self) -> usize {
        let leading = self.leading_suit.as_ref().unwrap();
        let mut best_index = 0;
        for i in 1..self.current_turn_cards.len() {
            let (_, ref card_i) = self.current_turn_cards[i];
            let (_, ref card_best) = self.current_turn_cards[best_index];
            if card_i.beats_custom(card_best, leading) {
                best_index = i;
            }
        }
        let winner_index = self.current_turn_cards[best_index].0;
        self.players[winner_index].tricks_won += 1;
        self.current_turn_cards.clear();
        self.leading_suit = None;
        winner_index
    }

    //  Fine round: restituisce un vettore di risultati
    pub fn end_round(&self) -> Vec<(usize, bool)> {
        self.players.iter().map(|p| {
            let success = p.prediction.unwrap_or(0) == p.tricks_won;
            (p.id, success)
        }).collect()
    }

    //  Avanza al round successivo
    pub fn next_round(&mut self) {
        self.round_number += 1;
        self.deck = Card::shuffle_deck();
        self.deal_round();
    }

    //  Controlla se il round è finito
    pub fn is_round_over(&self) -> bool {
        self.players.iter().all(|p| p.hand.is_empty())
    }

    //  Controlla se la partita è finita
    pub fn is_game_over(&self) -> bool {
        self.round_number > self.starting_cards
    }
}
