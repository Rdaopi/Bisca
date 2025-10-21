use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Card {
    pub suit: Suit,
    pub value: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum Suit {
    Denari,
    Coppe,
    Spade,
    Bastoni,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Value {
    Due,
    Tre,
    Quattro,
    Cinque,
    Sei,
    Sette,
    Fante,
    Cavallo,
    Re,
    Asso,
}

// Mazzo completo di 40 carte da Briscola
pub const FULL_DECK: [Card; 40] = [
    // Denari
    Card { suit: Suit::Denari, value: Value::Due },
    Card { suit: Suit::Denari, value: Value::Tre },
    Card { suit: Suit::Denari, value: Value::Quattro },
    Card { suit: Suit::Denari, value: Value::Cinque },
    Card { suit: Suit::Denari, value: Value::Sei },
    Card { suit: Suit::Denari, value: Value::Sette },
    Card { suit: Suit::Denari, value: Value::Fante },
    Card { suit: Suit::Denari, value: Value::Cavallo },
    Card { suit: Suit::Denari, value: Value::Re },
    Card { suit: Suit::Denari, value: Value::Asso },
    
    // Coppe
    Card { suit: Suit::Coppe, value: Value::Due },
    Card { suit: Suit::Coppe, value: Value::Tre },
    Card { suit: Suit::Coppe, value: Value::Quattro },
    Card { suit: Suit::Coppe, value: Value::Cinque },
    Card { suit: Suit::Coppe, value: Value::Sei },
    Card { suit: Suit::Coppe, value: Value::Sette },
    Card { suit: Suit::Coppe, value: Value::Fante },
    Card { suit: Suit::Coppe, value: Value::Cavallo },
    Card { suit: Suit::Coppe, value: Value::Re },
    Card { suit: Suit::Coppe, value: Value::Asso },
    
    // Spade
    Card { suit: Suit::Spade, value: Value::Due },
    Card { suit: Suit::Spade, value: Value::Tre },
    Card { suit: Suit::Spade, value: Value::Quattro },
    Card { suit: Suit::Spade, value: Value::Cinque },
    Card { suit: Suit::Spade, value: Value::Sei },
    Card { suit: Suit::Spade, value: Value::Sette },
    Card { suit: Suit::Spade, value: Value::Fante },
    Card { suit: Suit::Spade, value: Value::Cavallo },
    Card { suit: Suit::Spade, value: Value::Re },
    Card { suit: Suit::Spade, value: Value::Asso },
    
    // Bastoni
    Card { suit: Suit::Bastoni, value: Value::Due },
    Card { suit: Suit::Bastoni, value: Value::Tre },
    Card { suit: Suit::Bastoni, value: Value::Quattro },
    Card { suit: Suit::Bastoni, value: Value::Cinque },
    Card { suit: Suit::Bastoni, value: Value::Sei },
    Card { suit: Suit::Bastoni, value: Value::Sette },
    Card { suit: Suit::Bastoni, value: Value::Fante },
    Card { suit: Suit::Bastoni, value: Value::Cavallo },
    Card { suit: Suit::Bastoni, value: Value::Re },
    Card { suit: Suit::Bastoni, value: Value::Asso },
];

impl Card {
    // Funzione per mescolare le carte
    pub fn shuffle_deck() -> Vec<Card> {
        use rand::seq::SliceRandom;
        let mut deck = FULL_DECK.to_vec();
        deck.shuffle(&mut rand::thread_rng());
        deck
    }
    
    // Funzione per confrontare due carte (chi vince)
    pub fn beats(&self, other: &Card, trump_suit: &Suit) -> bool {
        // Se una carta è di briscola e l'altra no, vince la briscola
        if self.suit == *trump_suit && other.suit != *trump_suit {
            return true;
        }
        if other.suit == *trump_suit && self.suit != *trump_suit {
            return false;
        }
        
        // Se entrambe sono briscola o nessuna è briscola
        if self.suit == other.suit {
            // Stesso seme: vince il valore più alto
            self.value > other.value
        } else {
            // Semi diversi e nessuna è briscola: vince la prima giocata
            false
        }
    }
}