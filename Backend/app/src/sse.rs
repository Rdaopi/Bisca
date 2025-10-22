use axum::{
    extract::State,
    response::sse::{Event, KeepAlive, Sse},
};
use serde_json::json;
use std::convert::Infallible;
use std::sync::Arc;
use tokio_stream::{Stream, StreamExt};

// ===========================================
// SSE MANAGER
// ===========================================

#[derive(Clone)]
pub struct SseManager {
    // Simple in-memory storage for now
    // In production, you'd use Redis or similar
}

impl SseManager {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn broadcast_game_event(&self, game_id: &str, event: &str) {
        println!("📡 Broadcasting event to game {}: {}", game_id, event);
        // In a real implementation, you'd store this event and notify all connected clients
    }
}

// ===========================================
// SSE HANDLERS
// ===========================================

pub async fn game_events_sse(
    State(_sse_manager): State<Arc<SseManager>>,
) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    let stream = tokio_stream::iter(vec![
        Ok(Event::default().data("Game events SSE connected")),
        Ok(Event::default().data("Waiting for game events...")),
    ]);

    Sse::new(stream).keep_alive(KeepAlive::default())
}