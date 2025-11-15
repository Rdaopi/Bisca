use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

// ===========================================
// USER (Frequent read/write - Authentication)
// ===========================================
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// ===========================================
// USER_PROFILE (Read-heavy, Write-rare)
// ===========================================
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserProfile {
    pub user_id: String, // Reference to User.id
    pub name: String,
    pub surname: String,
    pub bio: String,
    pub profile_picture: Option<String>,
    pub updated_at: DateTime<Utc>,
}

// ===========================================
// USER_STATISTICS (Write-heavy, Read-medium)
// ===========================================
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserStatistics {
    pub user_id: String, // Reference to User.id
    pub victories: i32,
    pub defeats: i32,
    pub draws: i32,
    pub total_games: i32,
    pub win_rate: f32,
    pub lose_rate: f32,
    pub draw_rate: f32,
    pub total_rate: f32,
    pub total_rate_games: i32,
    pub last_updated: DateTime<Utc>,
}

// ===========================================
// USER_FRIENDS (Many-to-many relationship)
// ===========================================
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserFriends {
    pub user_id: String, // Reference to User.id
    pub friend_id: String, // Reference to User.id
    pub status: FriendshipStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum FriendshipStatus {
    Pending,
    Accepted,
    Blocked,
}

// ===========================================
// USER_FRIEND_REQUESTS (Temporary data)
// ===========================================
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserFriendRequest {
    pub request_id: String,
    pub from_user_id: String, // Reference to User.id
    pub to_user_id: String,   // Reference to User.id
    pub status: RequestStatus,
    pub created_at: DateTime<Utc>,
    pub responded_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum RequestStatus {
    Pending,
    Accepted,
    Declined,
    Expired,
}

// ===========================================
// HELPER FUNCTIONS
// ===========================================

impl UserStatistics {
    // Crea statistiche iniziali per nuovo utente
    pub fn new(user_id: String) -> Self {
        Self {
            user_id,
            victories: 0,
            defeats: 0,
            draws: 0,
            total_games: 0,
            win_rate: 0.0,
            lose_rate: 0.0,
            draw_rate: 0.0,
            total_rate: 0.0,
            total_rate_games: 0,
            last_updated: chrono::Utc::now(),
        }
    }
    
    // Aggiorna statistiche dopo una partita
    pub fn update_after_game(&mut self, result: GameResult) {
        self.total_games += 1;
        
        match result {
            GameResult::Victory => {
                self.victories += 1;
                self.win_rate = self.victories as f32 / self.total_games as f32;
            }
            GameResult::Defeat => {
                self.defeats += 1;
                self.lose_rate = self.defeats as f32 / self.total_games as f32;
            }
            GameResult::Draw => {
                self.draws += 1;
                self.draw_rate = self.draws as f32 / self.total_games as f32;
            }
        }
        
        self.total_rate = (self.victories as f32 + 0.5 * self.draws as f32) / self.total_games as f32;
        self.last_updated = chrono::Utc::now();
    }
}

#[derive(Debug, Clone)]
pub enum GameResult {
    Victory,
    Defeat,
    Draw,
}

impl UserProfile {
    // Crea profilo iniziale per nuovo utente
    pub fn new(user_id: String, name: String, surname: String) -> Self {
        Self {
            user_id,
            name,
            surname,
            bio: String::new(),
            profile_picture: None,
            updated_at: chrono::Utc::now(),
        }
    }
}