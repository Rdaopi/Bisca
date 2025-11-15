use rand::seq::SliceRandom;
use rand::thread_rng;
use serde::{Serialize, Deserialize};

//  I quattro semi del mazzo italiano
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum Suit {
    Denari,
    Coppe,
    Spade,
    Bastoni,
}

pub fn suit_strength(suit: &Suit) -> u8 {
    match suit {
        Suit::Denari => 4,
        Suit::Coppe => 3,
        Suit::Spade => 2,
        Suit::Bastoni => 1,
    }
}


//  I valori delle carte (1–10)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Value {
    Asso = 11,  
    Re = 10,
    Cavallo = 9,
    Fante = 8,
    Sette = 7,
    Sei = 6,
    Cinque = 5,
    Quattro = 4,
    Tre = 3,
    Due = 2,
}

//  Struttura principale della carta
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Card {
    pub suit: Suit,
    pub value: Value,
}

impl Card {
    //  Crea e mescola un mazzo completo (40 carte)
    pub fn shuffle_deck() -> Vec<Card> {
        let mut deck = Vec::new();

        let suits = vec![
            Suit::Denari,
            Suit::Coppe,
            Suit::Spade,
            Suit::Bastoni,
        ];

        let values = vec![
            Value::Asso,
            Value::Tre,
            Value::Re,
            Value::Cavallo,
            Value::Fante,
            Value::Sette,
            Value::Sei,
            Value::Cinque,
            Value::Quattro,
            Value::Due,
        ];

        for suit in suits {
            for value in &values {
                deck.push(Card {
                    suit: suit.clone(),
                    value: value.clone(),
                });
            }
        }

        let mut rng = thread_rng();
        deck.shuffle(&mut rng);
        deck
    }
}

//  Distribuisce carte a ciascun giocatore
pub fn deal_cards(deck: &mut Vec<Card>, num_players: usize, cards_per_player: usize) -> Vec<Vec<Card>> {
    let mut hands = vec![Vec::new(); num_players];
    
    for _ in 0..cards_per_player {
        for player in 0..num_players {
            if let Some(card) = deck.pop() {
                hands[player].push(card);
            }
        }
    }
    hands
}

//  Distribuisce le carte per round (ogni round diminuisce di una carta)
pub fn deal_round(deck: &mut Vec<Card>, num_players: usize, round_number: usize, starting_cards: usize) -> Vec<Vec<Card>> {
    let cards_per_player = starting_cards - (round_number - 1);
    deal_cards(deck, num_players, cards_per_player)
}

impl Card {
    pub fn beats_custom(&self, other: &Card, leading_suit: &Suit) -> bool {
        // Se self è Denari e l'altra no → vince self
        if suit_strength(&self.suit) > suit_strength(&other.suit) {
            return true;
        }
        // Se stessa forza e stesso seme → confronto per valore
        if self.suit == other.suit {
            return self.value > other.value;
        }
        // Se semi diversi ma self ha il seme dominante
        if self.suit == *leading_suit && other.suit != *leading_suit {
            return true;
        }
        false
    }
}
