use axum::{
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    response::IntoResponse,
    routing::get,
    Router,
};
use futures::{SinkExt, StreamExt};
use std::sync::Arc;
use tokio::sync::{Mutex, mpsc};
use uuid::Uuid;
use serde_json::json;

use crate::models::{Card, GameState, Player};

//creazione routes partendo dallo stato del gioco condiviso
pub fn websocket_routes(game_state: Arc<Mutex<GameState>>) -> Router {
    Router::new().route("/game", get(move |ws: WebSocketUpgrade| {
        let state = game_state.clone();
        async move { ws.on_upgrade(move |socket| handle_socket(socket, state)) }
    }))
}

//
async fn handle_socket(socket: WebSocket, state: Arc<Mutex<GameState>>) {
    let (mut sender, mut receiver) = socket.split();
    let (tx, mut rx) = mpsc::unbounded_channel();
    let player_id = Uuid::new_v4().to_string();

    let player = Player::new(player_id.clone(), tx.clone());
    {
        let mut game = state.lock().await;
        if let Err(err) = game.add_player(player) {
            eprintln!("Impossibile aggiungere il giocatore: {}", err);
            return;
        }
        let players_list: Vec<String> = game.players.iter().map(|p| p.id.clone()).collect();
        let current_turn: Vec<serde_json::Value> = game
            .current_turn_cards
            .iter()
            .map(|(id, card)| json!({ "player_id": id, "card": card }))
            .collect();
        let hand = game
            .players
            .iter()
            .find(|p| p.id == player_id)
            .map(|p| p.hand.clone())
            .unwrap_or_default();
        let welcome_msg = json!({
            "event": "welcome",
            "data": {
                "player_id": player_id.clone(),
                "round_number": game.round_number,
                "starting_cards": game.starting_cards,
                "hand": hand,
                "players": players_list,
                "turn": current_turn
            }
        })
        .to_string();
        let _ = tx.send(Message::Text(welcome_msg));

        game.broadcast("player_joined", json!({ "id": player_id.clone() }));
    }

    // TASK 1: invia messaggi al client
    let mut send_task = tokio::spawn(async move {
        while let Some(msg) = rx.recv().await {
            if sender.send(msg).await.is_err() {
                break;
            }
        }
    });

    // TASK 2: riceve messaggi dal client
    let state_clone = state.clone();
    let pid = player_id.clone();
    let mut recv_task = tokio::spawn(async move {
        while let Some(Ok(Message::Text(text))) = receiver.next().await {
            if let Ok(json_msg) = serde_json::from_str::<serde_json::Value>(&text) {
                if let Some(action) = json_msg.get("action").and_then(|a| a.as_str()) {
                    match action {
                        "start_game" => {
                            let mut game = state_clone.lock().await;
                            game.start_game();
                            game.broadcast("game_started", json!({}));
                            broadcast_round_start(&game);
                        }
                        "play_card" => {
                            if let Ok(card) = serde_json::from_value::<Card>(json_msg["card"].clone()) {
                                let mut game = state_clone.lock().await;
                                match game.play_card(&pid, card.clone()) {
                                    Ok(()) => {
                                        game.broadcast("card_played", json!({ "player_id": pid.clone(), "card": card }));
                                        send_player_hand(&game, &pid);
                                        finish_turn_if_ready(&mut game);
                                    }
                                    Err(e) => {
                                        send_to_player(&game, &pid, "error", json!({ "message": e }));
                                    }
                                }
                            }
                        }
                        "make_prediction" => {
                            if let Some(value) = json_msg.get("prediction").and_then(|v| v.as_u64()) {
                                let mut game = state_clone.lock().await;
                                match game.make_prediction(&pid, value as u8) {
                                    Ok(()) => game.broadcast("prediction_made", json!({ "player_id": pid.clone(), "prediction": value })),
                                    Err(err) => send_to_player(&game, &pid, "error", json!({ "message": err })),
                                }
                            }
                        }
                        "end_turn" => {
                            let mut game = state_clone.lock().await;
                            if game.current_turn_cards.len() == game.players.len() {
                                finalize_turn(&mut game);
                            } else {
                                send_to_player(&game, &pid, "error", json!({ "message": "Turno non ancora completo" }));
                            }
                        }
                        "next_round" => {
                            let mut game = state_clone.lock().await;
                            if !game.is_round_over() {
                                send_to_player(&game, &pid, "error", json!({ "message": "Il round corrente non è terminato" }));
                            } else if game.round_number >= game.starting_cards {
                                send_to_player(&game, &pid, "error", json!({ "message": "La partita è già terminata" }));
                            } else {
                                game.next_round();
                                broadcast_round_start(&game);
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
    });

    // Attendi fine di uno dei due task
    tokio::select! {
        _ = (&mut send_task) => (),
        _ = (&mut recv_task) => (),
    }

    // Disconnessione
    {
        let mut game = state.lock().await;
        game.remove_player(&player_id);
        game.broadcast("player_left", json!({ "id": player_id }));
    }
}

//helper per inviare un messaggio a uno specifico giocatore
fn send_to_player(game: &GameState, player_id: &str, event: &str, data: serde_json::Value) {
    if let Some(player) = game.players.iter().find(|p| p.id == player_id) {
        let payload = json!({ "event": event, "data": data }).to_string();
        let _ = player.sender.send(Message::Text(payload));
    }
}

//helper per inviare mano aggiornata
fn send_player_hand(game: &GameState, player_id: &str) {
    if let Some(player) = game.players.iter().find(|p| p.id == player_id) {
        let hand = player.hand.clone();
        let payload = json!({
            "event": "hand_updated",
            "data": {
                "player_id": player.id.clone(),
                "hand": hand
            }
        })
        .to_string();
        let _ = player.sender.send(Message::Text(payload));
    }
}


fn finish_turn_if_ready(game: &mut GameState) {
    let player_count = game.players.len();
    if player_count == 0 {
        return;
    }
    if game.current_turn_cards.len() == player_count {
        finalize_turn(game);
    }
}

fn finalize_turn(game: &mut GameState) {
    if game.current_turn_cards.is_empty() {
        return;
    }
    if let Some(winner_id) = game.end_turn() {
        let standings: Vec<serde_json::Value> = game
            .players
            .iter()
            .map(|p| json!({ "player_id": p.id, "tricks_won": p.tricks_won }))
            .collect();
        game.broadcast(
            "turn_ended",
            json!({
                "winner_id": winner_id,
                "standings": standings
            }),
        );

        if game.is_round_over() {
            let results = game.end_round();
            game.broadcast("round_ended", json!({ "results": results.clone() }));

            if game.round_number >= game.starting_cards {
                game.broadcast("game_over", json!({ "results": results }));
            } else {
                game.next_round();
                broadcast_round_start(game);
            }
        }
    }
}

//notifica inizio di un round
fn broadcast_round_start(game: &GameState) {
    game.broadcast(
        "round_started",
        json!({
            "round_number": game.round_number,
            "starting_cards": game.starting_cards
        }),
    );

    for player in &game.players {
        let hand = player.hand.clone();
        let payload = json!({
            "event": "hand_updated",
            "data": {
                "player_id": player.id.clone(),
                "hand": hand
            }
        })
        .to_string();
        let _ = player.sender.send(Message::Text(payload));
    }
}
