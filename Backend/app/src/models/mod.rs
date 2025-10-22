// ===========================================
// MODELS MODULE
// ===========================================

// Simple placeholder models for now
// In production, you'd have proper game models here

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Game {
    pub id: String,
    pub name: String,
    pub status: String,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Player {
    pub id: String,
    pub name: String,
    pub score: i32,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Card {
    pub suit: String,
    pub value: String,
}