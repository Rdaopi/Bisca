use axum::{
    extract::{Path, State},
    response::sse::{Event, Sse},
    Json,
};
use futures_util::stream::{self, Stream};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    convert::Infallible,
    sync::Arc,
    time::Duration,
};
use tokio::sync::{broadcast, RwLock};
use uuid::Uuid;

use crate::models::game::{GameState, GameStatus, Player};

// ===========================================
// SSE EVENT TYPES
// ===========================================

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum GameEvent {
    PlayerJoined {
        player: Player,
        game_state: GameState,
    },
    PlayerLeft {
        player_id: String,
        game_state: GameState,
    },
    GameStarted {
        game_state: GameState,
    },
    CardPlayed {
        player_id: String,
        card: crate::models::cards::Card,
        game_state: GameState,
    },
    PredictionMade {
        player_id: String,
        prediction: i32,
        game_state: GameState,
    },
    RoundFinished {
        winner_id: String,
        game_state: GameState,
    },
    GameFinished {
        winner_id: String,
        final_scores: HashMap<String, i32>,
        game_state: GameState,
    },
    GameStateUpdate {
        game_state: GameState,
    },
    Error {
        message: String,
    },
}

// ===========================================
// SSE MANAGER
// ===========================================

#[derive(Debug, Clone)]
pub struct SseManager {
    // Map of game_id -> broadcast channel for that game
    game_channels: Arc<RwLock<HashMap<String, broadcast::Sender<GameEvent>>>>,
}

impl SseManager {
    pub fn new() -> Self {
        Self {
            game_channels: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    // Get or create a broadcast channel for a game
    pub async fn get_game_channel(&self, game_id: &str) -> broadcast::Sender<GameEvent> {
        let mut channels = self.game_channels.write().await;
        
        if let Some(sender) = channels.get(game_id) {
            sender.clone()
        } else {
            let (sender, _) = broadcast::channel(100); // Buffer for 100 events
            channels.insert(game_id.to_string(), sender.clone());
            sender
        }
    }

    // Broadcast an event to all players in a game
    pub async fn broadcast_game_event(&self, game_id: &str, event: GameEvent) {
        let channels = self.game_channels.read().await;
        if let Some(sender) = channels.get(game_id) {
            let _ = sender.send(event); // Ignore if no receivers
        }
    }

    // Remove a game channel when game ends
    pub async fn remove_game_channel(&self, game_id: &str) {
        let mut channels = self.game_channels.write().await;
        channels.remove(game_id);
    }
}

// ===========================================
// SSE HANDLERS
// ===========================================

// SSE endpoint for game events
pub async fn game_events_sse(
    Path(game_id): Path<String>,
    State(sse_manager): State<Arc<SseManager>>,
) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    let sender = sse_manager.get_game_channel(&game_id).await;
    let mut receiver = sender.subscribe();

    let stream = async_stream::stream! {
        // Send initial connection event
        yield Ok(Event::default().data("connected"));

        // Listen for game events
        while let Ok(event) = receiver.recv().await {
            let event_data = serde_json::to_string(&event).unwrap_or_else(|_| "{}".to_string());
            yield Ok(Event::default().data(event_data));
        }
    };

    Sse::new(stream)
        .keep_alive(
            axum::response::sse::KeepAlive::new()
                .interval(Duration::from_secs(15))
                .text("keep-alive"),
        )
}

// ===========================================
// HELPER FUNCTIONS
// ===========================================

impl GameEvent {
    // Create events for common game actions
    pub fn player_joined(player: Player, game_state: GameState) -> Self {
        Self::PlayerJoined { player, game_state }
    }

    pub fn player_left(player_id: String, game_state: GameState) -> Self {
        Self::PlayerLeft { player_id, game_state }
    }

    pub fn game_started(game_state: GameState) -> Self {
        Self::GameStarted { game_state }
    }

    pub fn card_played(
        player_id: String,
        card: crate::models::cards::Card,
        game_state: GameState,
    ) -> Self {
        Self::CardPlayed {
            player_id,
            card,
            game_state,
        }
    }

    pub fn prediction_made(player_id: String, prediction: i32, game_state: GameState) -> Self {
        Self::PredictionMade {
            player_id,
            prediction,
            game_state,
        }
    }

    pub fn round_finished(winner_id: String, game_state: GameState) -> Self {
        Self::RoundFinished {
            winner_id,
            game_state,
        }
    }

    pub fn game_finished(
        winner_id: String,
        final_scores: HashMap<String, i32>,
        game_state: GameState,
    ) -> Self {
        Self::GameFinished {
            winner_id,
            final_scores,
            game_state,
        }
    }

    pub fn game_state_update(game_state: GameState) -> Self {
        Self::GameStateUpdate { game_state }
    }

    pub fn error(message: String) -> Self {
        Self::Error { message }
    }
}

// ===========================================
// REQUEST/RESPONSE TYPES
// ===========================================

#[derive(Debug, Deserialize)]
pub struct PlayCardRequest {
    pub card: crate::models::cards::Card,
}

#[derive(Debug, Deserialize)]
pub struct MakePredictionRequest {
    pub prediction: i32,
}

#[derive(Debug, Serialize)]
pub struct GameEventResponse {
    pub success: bool,
    pub message: String,
}

// ===========================================
// GAME ACTION HANDLERS
// ===========================================

// These would be called from your main.rs handlers
// They update the game state and broadcast events

pub async fn handle_play_card(
    game_id: String,
    player_id: String,
    card: crate::models::cards::Card,
    sse_manager: Arc<SseManager>,
    // You'll need access to your game state storage here
) -> Result<GameEventResponse, String> {
    // 1. Update game state (validate move, update game logic)
    // 2. Broadcast the event to all players
    // 3. Return success response

    // Example implementation:
    // let game_state = get_game_state(&game_id).await?;
    // let updated_state = game_state.play_card(&player_id, &card)?;
    // save_game_state(&game_id, &updated_state).await?;
    
    // sse_manager.broadcast_game_event(&game_id, GameEvent::card_played(player_id, card, updated_state)).await;

    Ok(GameEventResponse {
        success: true,
        message: "Card played successfully".to_string(),
    })
}

pub async fn handle_make_prediction(
    game_id: String,
    player_id: String,
    prediction: i32,
    sse_manager: Arc<SseManager>,
) -> Result<GameEventResponse, String> {
    // Similar to handle_play_card but for predictions
    
    Ok(GameEventResponse {
        success: true,
        message: "Prediction made successfully".to_string(),
    })
}
