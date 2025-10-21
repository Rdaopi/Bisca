use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use std::collections::HashMap;
use crate::models::cards::Card;

// ===========================================
// GAME_RULES (Customizable Game Rules)
// ===========================================
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameRules {
    pub max_players: i32,
    pub min_players: i32,
    pub cards_per_player: Option<i32>, // None = auto-calcolato
    pub time_per_turn: Option<i32>, // secondi, None = no limit
    pub time_per_prediction: Option<i32>, // secondi per predizioni
    pub allow_spectators: bool,
    pub auto_start: bool, // auto-start quando piena
    pub friendly_mode: bool, // meno punitivo per errori
    pub show_trump_card: bool, // mostra carta briscola a tutti
    pub prediction_required: bool, // predizioni obbligatorie
    pub max_predictions: Option<i32>, // limite predizioni per giocatore
    pub victory_conditions: VictoryConditions,
    pub scoring_system: ScoringSystem,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum VictoryConditions {
    Standard, // Vince chi raggiunge il numero predetto
    FirstToTarget(i32), // Primo a raggiungere X vittorie
    BestOf(i32), // Migliore di N partite
    TimeBased(i32), // Partita più lunga di X minuti
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ScoringSystem {
    Standard, // +1 per vittoria esatta, -1 per errore
    Progressive, // Punti crescenti per vittorie consecutive
    Bonus, // Bonus per vittorie difficili
    Custom(Vec<ScoringRule>), // Regole personalizzate
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScoringRule {
    pub condition: String, // "exact_prediction", "consecutive_wins", etc.
    pub points: i32,
    pub description: String,
}

// ===========================================
// GAME_INFO (Join Phase - Fast Query)
// ===========================================
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameInfo {
    pub game_id: String,
    pub game_name: String,
    pub host_id: String,
    pub host_username: String,
    pub game_status: GameStatus,
    pub max_players: i32,
    pub current_players_count: i32,
    pub password_protected: bool,
    pub password_hash: Option<String>,
    pub rules: GameRules, // Regole personalizzate
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
}

// ===========================================
// GAME_STATE (Game Phase - Complete Data)
// ===========================================
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameState {
    pub game_id: String,
    pub game_name: String,
    pub host_id: String,
    pub players: Vec<Player>,
    pub current_turn: String, // player_id
    pub cards_played: Vec<PlayedCard>,
    pub player_hands: HashMap<String, Vec<Card>>, // player_id -> carte in mano
    pub trump_card: Option<Card>,
    pub game_status: GameStatus,
    pub current_round: i32,
    pub total_rounds: i32,
    pub max_players: i32,
    pub password_protected: bool,
    pub rules: GameRules, // Regole personalizzate
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Player {
    pub player_id: String,
    pub username: String,
    pub prediction: Option<i32>, // numero vittorie predetto
    pub actual_wins: i32,
    pub is_ready: bool,
    pub joined_at: DateTime<Utc>,
    
    // Informazioni aggiuntive per utenti registrati
    pub user_id: Option<String>, // None per guest, Some per utenti registrati
    pub is_guest: bool,          // true = guest, false = utente registrato
    pub profile_picture: Option<String>, // Solo per utenti registrati
    pub user_stats: Option<PlayerStats>, // Solo per utenti registrati
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerStats {
    pub victories: i32,
    pub defeats: i32,
    pub total_games: i32,
    pub win_rate: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayedCard {
    pub card: Card,
    pub player_id: String,
    pub round: i32,
    pub played_at: DateTime<Utc>,
    pub won_round: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum GameStatus {
    Waiting,    // In attesa di giocatori
    Predicting, // Fase predizioni
    Playing,    // Gioco in corso
    Paused,     // Partita in pausa
    Finished,   // Partita terminata
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameLobby {
    pub lobby_id: String,
    pub host_id: String,
    pub max_players: i32,
    pub current_players: Vec<String>, // player_ids
    pub is_public: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameInvite {
    pub invite_id: String,
    pub game_id: String,
    pub from_player_id: String,
    pub to_player_id: String,
    pub status: InviteStatus,
    pub created_at: DateTime<Utc>,
    pub responded_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum InviteStatus {
    Pending,
    Accepted,
    Declined,
    Expired,
}

// ===========================================
// HELPER FUNCTIONS
// ===========================================

impl GameRules {
    // Regole predefinite per Briscola standard
    pub fn default_briscola() -> Self {
        Self {
            max_players: 4,
            min_players: 2,
            cards_per_player: None, // Auto-calcolato
            time_per_turn: None, // No limit
            time_per_prediction: Some(30), // 30 secondi per predizioni
            allow_spectators: false,
            auto_start: false,
            friendly_mode: false,
            show_trump_card: true,
            prediction_required: true,
            max_predictions: None,
            victory_conditions: VictoryConditions::Standard,
            scoring_system: ScoringSystem::Standard,
        }
    }
    
    // Regole per partita veloce
    pub fn fast_game() -> Self {
        Self {
            max_players: 2,
            min_players: 2,
            cards_per_player: Some(10), // 10 carte per giocatore
            time_per_turn: Some(15), // 15 secondi per turno
            time_per_prediction: Some(10), // 10 secondi per predizioni
            allow_spectators: true,
            auto_start: true,
            friendly_mode: true,
            show_trump_card: true,
            prediction_required: false,
            max_predictions: Some(3),
            victory_conditions: VictoryConditions::FirstToTarget(5),
            scoring_system: ScoringSystem::Progressive,
        }
    }
    
    // Regole per torneo
    pub fn tournament() -> Self {
        Self {
            max_players: 4,
            min_players: 4,
            cards_per_player: None,
            time_per_turn: Some(30),
            time_per_prediction: Some(20),
            allow_spectators: true,
            auto_start: false,
            friendly_mode: false,
            show_trump_card: false,
            prediction_required: true,
            max_predictions: None,
            victory_conditions: VictoryConditions::BestOf(3),
            scoring_system: ScoringSystem::Bonus,
        }
    }
    
    // Verifica se le regole sono valide
    pub fn is_valid(&self) -> bool {
        self.max_players >= self.min_players
            && self.min_players >= 2
            && self.max_players <= 8
            && (self.time_per_turn.is_none() || self.time_per_turn.unwrap() > 0)
            && (self.time_per_prediction.is_none() || self.time_per_prediction.unwrap() > 0)
    }
    
    // Calcola carte per giocatore in base al numero di giocatori
    pub fn calculate_cards_per_player(&self, num_players: i32) -> i32 {
        match self.cards_per_player {
            Some(cards) => cards,
            None => {
                // Distribuzione standard: 40 carte totali
                let total_cards = 40;
                total_cards / num_players
            }
        }
    }
}

impl GameInfo {
    // Crea info partita per join phase
    pub fn new(game_id: String, game_name: String, host_id: String, host_username: String, rules: GameRules, password: Option<String>) -> Self {
        let password_hash = password.map(|p| bcrypt::hash(p, 12).unwrap_or_default());
        
        Self {
            game_id,
            game_name,
            host_id,
            host_username,
            game_status: GameStatus::Waiting,
            max_players: rules.max_players,
            current_players_count: 0,
            password_protected: password.is_some(),
            password_hash,
            rules,
            created_at: chrono::Utc::now(),
            last_updated: chrono::Utc::now(),
        }
    }
    
    // Verifica password partita
    pub fn verify_password(&self, password: &str) -> bool {
        match &self.password_hash {
            Some(hash) => bcrypt::verify(password, hash).unwrap_or(false),
            None => !self.password_protected,
        }
    }
    
    // Aggiorna conteggio giocatori
    pub fn update_player_count(&mut self, count: i32) {
        self.current_players_count = count;
        self.last_updated = chrono::Utc::now();
    }
    
    // Verifica se la partita è piena
    pub fn is_full(&self) -> bool {
        self.current_players_count >= self.max_players
    }
    
    // Verifica se la partita è joinable
    pub fn can_join(&self) -> bool {
        !self.is_full() && self.game_status == GameStatus::Waiting
    }
}

impl GameState {
    // Crea stato partita completo da GameInfo
    pub fn from_game_info(info: GameInfo, players: Vec<Player>) -> Self {
        let current_players_count = players.len() as i32;
        
        Self {
            game_id: info.game_id,
            game_name: info.game_name,
            host_id: info.host_id,
            players,
            current_turn: String::new(), // Sarà impostato quando inizia il gioco
            cards_played: Vec::new(),
            player_hands: HashMap::new(),
            trump_card: None,
            game_status: info.game_status,
            current_round: 0,
            total_rounds: 0, // Sarà calcolato in base al numero di giocatori
            max_players: info.max_players,
            password_protected: info.password_protected,
            created_at: info.created_at,
            last_updated: chrono::Utc::now(),
        }
    }
    
    // Ottieni dati partita per giocatore specifico
    pub fn get_player_data(&self, player_id: &str) -> Option<PlayerGameData> {
        let player = self.players.iter().find(|p| p.player_id == player_id)?;
        let cards = self.player_hands.get(player_id).cloned().unwrap_or_default();
        
        Some(PlayerGameData {
            player: player.clone(),
            cards,
            is_my_turn: self.current_turn == player_id,
            game_status: self.game_status.clone(),
            current_round: self.current_round,
            total_rounds: self.total_rounds,
            trump_card: self.trump_card.clone(),
            cards_played_this_round: self.get_cards_played_this_round(),
        })
    }
    
    // Ottieni carte giocate nel round corrente
    fn get_cards_played_this_round(&self) -> Vec<PlayedCard> {
        self.cards_played
            .iter()
            .filter(|card| card.round == self.current_round)
            .cloned()
            .collect()
    }
    
    // Inizia una nuova partita (distribuzione carte)
    pub fn start_game(&mut self) -> Result<(), String> {
        // Verifica numero minimo di giocatori
        if self.players.len() < self.rules.min_players as usize {
            return Err(format!("Not enough players. Minimum: {}", self.rules.min_players));
        }
        
        if self.players.len() > self.rules.max_players as usize {
            return Err(format!("Too many players. Maximum: {}", self.rules.max_players));
        }
        
        // Verifica se tutti sono pronti (se auto_start è false)
        if !self.rules.auto_start && !self.players.iter().all(|p| p.is_ready) {
            return Err("Not all players are ready".to_string());
        }
        
        // Mescola e distribuisce le carte
        let mut deck = Card::shuffle_deck();
        let cards_per_player = self.rules.calculate_cards_per_player(self.players.len() as i32) as usize;
        
        // Verifica che abbiamo abbastanza carte
        if cards_per_player * self.players.len() > deck.len() {
            return Err("Not enough cards for all players".to_string());
        }
        
        // Distribuisci carte ai giocatori
        for player in &self.players {
            let player_cards: Vec<Card> = deck.drain(0..cards_per_player).collect();
            self.player_hands.insert(player.player_id.clone(), player_cards);
        }
        
        // Imposta carta di briscola (ultima carta del mazzo)
        self.trump_card = deck.pop();
        
        // Calcola numero totale di round
        self.total_rounds = cards_per_player as i32;
        
        // Inizia il gioco in base alle regole
        if self.rules.prediction_required {
            self.game_status = GameStatus::Predicting;
        } else {
            self.game_status = GameStatus::Playing;
            // Se non servono predizioni, inizia subito il primo turno
            if let Some(first_player) = self.players.first() {
                self.current_turn = first_player.player_id.clone();
            }
        }
        
        self.current_round = 1;
        self.last_updated = chrono::Utc::now();
        
        Ok(())
    }
    
    // Aggiungi giocatore alla partita
    pub fn add_player(&mut self, player: Player) -> Result<(), String> {
        if self.players.len() >= self.max_players as usize {
            return Err("Game is full".to_string());
        }
        
        if self.game_status != GameStatus::Waiting {
            return Err("Game has already started".to_string());
        }
        
        self.players.push(player);
        self.last_updated = chrono::Utc::now();
        
        Ok(())
    }
    
    // Rimuovi giocatore dalla partita
    pub fn remove_player(&mut self, player_id: &str) -> Result<(), String> {
        if self.game_status == GameStatus::Playing {
            return Err("Cannot remove player during active game".to_string());
        }
        
        self.players.retain(|p| p.player_id != player_id);
        self.player_hands.remove(player_id);
        self.last_updated = chrono::Utc::now();
        
        Ok(())
    }
}

// ===========================================
// PLAYER GAME DATA (Response per giocatore)
// ===========================================
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerGameData {
    pub player: Player,
    pub cards: Vec<Card>,
    pub is_my_turn: bool,
    pub game_status: GameStatus,
    pub current_round: i32,
    pub total_rounds: i32,
    pub trump_card: Option<Card>,
    pub cards_played_this_round: Vec<PlayedCard>,
}

impl Player {
    // Crea un giocatore guest
    pub fn new_guest(username: String) -> Self {
        Self {
            player_id: uuid::Uuid::new_v4().to_string(),
            username,
            prediction: None,
            actual_wins: 0,
            is_ready: false,
            joined_at: chrono::Utc::now(),
            user_id: None,
            is_guest: true,
            profile_picture: None,
            user_stats: None,
        }
    }
    
    // Crea un giocatore registrato
    pub fn new_registered(user_id: String, username: String, profile_picture: Option<String>, stats: Option<PlayerStats>) -> Self {
        Self {
            player_id: uuid::Uuid::new_v4().to_string(),
            username,
            prediction: None,
            actual_wins: 0,
            is_ready: false,
            joined_at: chrono::Utc::now(),
            user_id: Some(user_id),
            is_guest: false,
            profile_picture,
            user_stats: stats,
        }
    }
    
    // Verifica se è un utente registrato
    pub fn is_registered(&self) -> bool {
        !self.is_guest && self.user_id.is_some()
    }
    
    // Ottieni l'ID utente (solo per registrati)
    pub fn get_user_id(&self) -> Option<&String> {
        self.user_id.as_ref()
    }
}
