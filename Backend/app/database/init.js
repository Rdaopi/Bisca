// MongoDB initialization script for Bisca Game
db = db.getSiblingDB('biscaDB');

print('Initializing Bisca database...');

// ===========================================
// USERS COLLECTION (Frequent read/write - Auth)
// ===========================================
db.createCollection("users", {
  validator: {
    $jsonSchema: {
      bsonType: "object",
      required: ["id", "username", "email", "password_hash"],
      properties: {
        id: { bsonType: "string" },
        username: { bsonType: "string" },
        email: { bsonType: "string" },
        password_hash: { bsonType: "string" },
        created_at: { bsonType: "date" },
        updated_at: { bsonType: "date" }
      }
    }
  }
});

// ===========================================
// USER_PROFILES COLLECTION (Read-heavy, Write-rare)
// ===========================================
db.createCollection("user_profiles", {
  validator: {
    $jsonSchema: {
      bsonType: "object",
      required: ["user_id", "name", "surname"],
      properties: {
        user_id: { bsonType: "string" },
        name: { bsonType: "string" },
        surname: { bsonType: "string" },
        bio: { bsonType: "string" },
        profile_picture: { bsonType: ["string", "null"] },
        updated_at: { bsonType: "date" }
      }
    }
  }
});

// ===========================================
// USER_STATISTICS COLLECTION (Write-heavy, Read-medium)
// ===========================================
db.createCollection("user_statistics", {
  validator: {
    $jsonSchema: {
      bsonType: "object",
      required: ["user_id", "last_updated"],
      properties: {
        user_id: { bsonType: "string" },
        victories: { bsonType: "int" },
        defeats: { bsonType: "int" },
        draws: { bsonType: "int" },
        total_games: { bsonType: "int" },
        win_rate: { bsonType: "double" },
        lose_rate: { bsonType: "double" },
        draw_rate: { bsonType: "double" },
        total_rate: { bsonType: "double" },
        total_rate_games: { bsonType: "int" },
        last_updated: { bsonType: "date" }
      }
    }
  }
});

// ===========================================
// USER_FRIENDS COLLECTION (Many-to-many)
// ===========================================
db.createCollection("user_friends", {
  validator: {
    $jsonSchema: {
      bsonType: "object",
      required: ["user_id", "friend_id", "status"],
      properties: {
        user_id: { bsonType: "string" },
        friend_id: { bsonType: "string" },
        status: { 
          bsonType: "string",
          enum: ["Pending", "Accepted", "Blocked"]
        },
        created_at: { bsonType: "date" },
        updated_at: { bsonType: "date" }
      }
    }
  }
});

// ===========================================
// USER_FRIEND_REQUESTS COLLECTION (Temporary)
// ===========================================
db.createCollection("user_friend_requests", {
  validator: {
    $jsonSchema: {
      bsonType: "object",
      required: ["request_id", "from_user_id", "to_user_id", "status"],
      properties: {
        request_id: { bsonType: "string" },
        from_user_id: { bsonType: "string" },
        to_user_id: { bsonType: "string" },
        status: { 
          bsonType: "string",
          enum: ["Pending", "Accepted", "Declined", "Expired"]
        },
        created_at: { bsonType: "date" },
        responded_at: { bsonType: ["date", "null"] }
      }
    }
  }
});

// ===========================================
// GAME_INFOS COLLECTION (Join Phase - Fast Query)
// ===========================================
db.createCollection("game_infos", {
  validator: {
    $jsonSchema: {
      bsonType: "object",
      required: ["game_id", "game_name", "host_id", "host_username", "game_status", "rules"],
      properties: {
        game_id: { bsonType: "string" },
        game_name: { bsonType: "string" },
        host_id: { bsonType: "string" },
        host_username: { bsonType: "string" },
        game_status: { 
          bsonType: "string",
          enum: ["Waiting", "Predicting", "Playing", "Paused", "Finished"]
        },
        max_players: { bsonType: "int" },
        current_players_count: { bsonType: "int" },
        password_protected: { bsonType: "bool" },
        password_hash: { bsonType: ["string", "null"] },
        rules: {
          bsonType: "object",
          required: ["max_players", "min_players", "victory_conditions", "scoring_system"],
          properties: {
            max_players: { bsonType: "int" },
            min_players: { bsonType: "int" },
            cards_per_player: { bsonType: ["int", "null"] },
            time_per_turn: { bsonType: ["int", "null"] },
            time_per_prediction: { bsonType: ["int", "null"] },
            allow_spectators: { bsonType: "bool" },
            auto_start: { bsonType: "bool" },
            friendly_mode: { bsonType: "bool" },
            show_trump_card: { bsonType: "bool" },
            prediction_required: { bsonType: "bool" },
            max_predictions: { bsonType: ["int", "null"] },
            victory_conditions: { bsonType: "object" },
            scoring_system: { bsonType: "object" }
          }
        },
        created_at: { bsonType: "date" },
        last_updated: { bsonType: "date" }
      }
    }
  }
});

// ===========================================
// GAME_STATES COLLECTION (Game Phase - Complete Data)
// ===========================================
db.createCollection("game_states", {
  validator: {
    $jsonSchema: {
      bsonType: "object",
      required: ["game_id", "game_name", "host_id", "players", "game_status", "rules"],
      properties: {
        game_id: { bsonType: "string" },
        game_name: { bsonType: "string" },
        host_id: { bsonType: "string" },
        players: { 
          bsonType: "array",
          items: {
            bsonType: "object",
            required: ["player_id", "username", "is_guest"],
            properties: {
              player_id: { bsonType: "string" },
              username: { bsonType: "string" },
              user_id: { bsonType: ["string", "null"] },
              is_guest: { bsonType: "bool" },
              prediction: { bsonType: ["int", "null"] },
              actual_wins: { bsonType: "int" },
              is_ready: { bsonType: "bool" },
              profile_picture: { bsonType: ["string", "null"] },
              user_stats: { bsonType: ["object", "null"] },
              joined_at: { bsonType: "date" }
            }
          }
        },
        current_turn: { bsonType: "string" },
        cards_played: { bsonType: "array" },
        player_hands: { bsonType: "object" },
        trump_card: { bsonType: ["object", "null"] },
        game_status: { 
          bsonType: "string",
          enum: ["Waiting", "Predicting", "Playing", "Paused", "Finished"]
        },
        current_round: { bsonType: "int" },
        total_rounds: { bsonType: "int" },
        max_players: { bsonType: "int" },
        password_protected: { bsonType: "bool" },
        rules: {
          bsonType: "object",
          required: ["max_players", "min_players", "victory_conditions", "scoring_system"],
          properties: {
            max_players: { bsonType: "int" },
            min_players: { bsonType: "int" },
            cards_per_player: { bsonType: ["int", "null"] },
            time_per_turn: { bsonType: ["int", "null"] },
            time_per_prediction: { bsonType: ["int", "null"] },
            allow_spectators: { bsonType: "bool" },
            auto_start: { bsonType: "bool" },
            friendly_mode: { bsonType: "bool" },
            show_trump_card: { bsonType: "bool" },
            prediction_required: { bsonType: "bool" },
            max_predictions: { bsonType: ["int", "null"] },
            victory_conditions: { bsonType: "object" },
            scoring_system: { bsonType: "object" }
          }
        },
        created_at: { bsonType: "date" },
        last_updated: { bsonType: "date" }
      }
    }
  }
});

// ===========================================
// GAME_LOBBIES COLLECTION
// ===========================================
db.createCollection("game_lobbies", {
  validator: {
    $jsonSchema: {
      bsonType: "object",
      required: ["lobby_id", "host_id", "max_players"],
      properties: {
        lobby_id: { bsonType: "string" },
        host_id: { bsonType: "string" },
        max_players: { bsonType: "int" },
        current_players: { bsonType: "array" },
        is_public: { bsonType: "bool" },
        created_at: { bsonType: "date" }
      }
    }
  }
});

// ===========================================
// GAME_INVITES COLLECTION
// ===========================================
db.createCollection("game_invites", {
  validator: {
    $jsonSchema: {
      bsonType: "object",
      required: ["invite_id", "game_id", "from_player_id", "to_player_id", "status"],
      properties: {
        invite_id: { bsonType: "string" },
        game_id: { bsonType: "string" },
        from_player_id: { bsonType: "string" },
        to_player_id: { bsonType: "string" },
        status: { 
          bsonType: "string",
          enum: ["Pending", "Accepted", "Declined", "Expired"]
        },
        created_at: { bsonType: "date" },
        responded_at: { bsonType: ["date", "null"] }
      }
    }
  }
});

// ===========================================
// CREATE INDEXES FOR PERFORMANCE
// ===========================================

print('Creating indexes...');

// Users indexes
db.users.createIndex({ "id": 1 }, { unique: true });
db.users.createIndex({ "created_at": 1 });

// User profiles indexes
db.user_profiles.createIndex({ "user_id": 1 }, { unique: true });

// User statistics indexes
db.user_statistics.createIndex({ "user_id": 1 }, { unique: true });
db.user_statistics.createIndex({ "total_games": -1 });
db.user_statistics.createIndex({ "win_rate": -1 });
db.user_statistics.createIndex({ "victories": -1 });

// User friends indexes
db.user_friends.createIndex({ "user_id": 1 });
db.user_friends.createIndex({ "friend_id": 1 });
db.user_friends.createIndex({ "status": 1 });
db.user_friends.createIndex({ "user_id": 1, "friend_id": 1 }, { unique: true });

// User friend requests indexes
db.user_friend_requests.createIndex({ "request_id": 1 }, { unique: true });
db.user_friend_requests.createIndex({ "from_user_id": 1 });
db.user_friend_requests.createIndex({ "to_user_id": 1 });
db.user_friend_requests.createIndex({ "status": 1 });
db.user_friend_requests.createIndex({ "created_at": -1 });

// Game infos indexes (Join Phase)
db.game_infos.createIndex({ "game_id": 1 }, { unique: true });
db.game_infos.createIndex({ "game_status": 1 });
db.game_infos.createIndex({ "password_protected": 1 });
db.game_infos.createIndex({ "max_players": 1 });
db.game_infos.createIndex({ "current_players_count": 1 });
db.game_infos.createIndex({ "created_at": -1 });
db.game_infos.createIndex({ "host_id": 1 });

// Game states indexes (Game Phase)
db.game_states.createIndex({ "game_id": 1 }, { unique: true });
db.game_states.createIndex({ "game_status": 1 });
db.game_states.createIndex({ "current_turn": 1 });
db.game_states.createIndex({ "current_round": 1 });
db.game_states.createIndex({ "created_at": -1 });
db.game_states.createIndex({ "last_updated": -1 });
db.game_states.createIndex({ "players.player_id": 1 });
db.game_states.createIndex({ "host_id": 1 });

// Game lobbies indexes
db.game_lobbies.createIndex({ "lobby_id": 1 }, { unique: true });
db.game_lobbies.createIndex({ "host_id": 1 });
db.game_lobbies.createIndex({ "is_public": 1 });
db.game_lobbies.createIndex({ "created_at": -1 });

// Game invites indexes
db.game_invites.createIndex({ "invite_id": 1 }, { unique: true });
db.game_invites.createIndex({ "game_id": 1 });
db.game_invites.createIndex({ "from_player_id": 1 });
db.game_invites.createIndex({ "to_player_id": 1 });
db.game_invites.createIndex({ "status": 1 });
db.game_invites.createIndex({ "created_at": -1 });

print('Database initialization completed successfully!');
