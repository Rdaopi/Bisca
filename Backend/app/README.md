# Bisca - Backend API

A Rust-based REST API for the Italian card game "Bisca" with real-time multiplayer functionality.

## Features

- Real-time multiplayer game sessions
- Card game logic implementation
- User management and authentication
- WebSocket support for live updates
- Game state management
- RESTful API endpoints
- HTTPS support with SSL certificates

## Tech Stack

- **Language**: Rust
- **Web Framework**: Axum
- **Database**: MongoDB
- **Real-time**: Server-Sent Events (SSE)
- **Authentication**: JWT tokens
- **Serialization**: Serde
- **Security**: HTTPS with SSL certificates

## Quick Start

### Prerequisites
- Rust 1.70+
- MongoDB running locally or remotely
- Git

### Running the Application

1. **Navigate to the backend directory**:
   ```bash
   cd Backend/app
   ```

2. **Install dependencies**:
   ```bash
   cargo build
   ```

3. **Start the server**:
   ```bash
   cargo run
   ```

4. **Test the API**:
   ```bash
   # Health check (HTTPS required)
   curl -k https://localhost:3000/health
   ```

## API Endpoints

### Core Endpoints
- `GET /` - Welcome message
- `GET /health` - Health check

### Game Management
- `POST /api/games` - Create new game
- `GET /api/games/:id` - Get game state
- `POST /api/games/:id/join` - Join a game
- `POST /api/games/:id/play` - Play a card
- `GET /api/games/:id/events` - Server-Sent Events stream

### Users
- `GET /api/users` - Get all users
- `POST /api/users` - Create new user
- `GET /api/users/:id` - Get specific user

## Development

```bash
# Run in development mode
cargo run

# Run with hot reload
cargo install cargo-watch
cargo watch -x run

# Run tests
cargo test

# Check code formatting
cargo fmt

# Run clippy for linting
cargo clippy
```

## Project Structure

```
src/
├── main.rs           # Main application entry point
├── https_server.rs   # HTTPS server configuration
├── sse.rs            # Server-Sent Events implementation
└── models/
    ├── game.rs       # Game logic and models
    ├── cards.rs      # Card definitions
    └── users.rs      # User management
```

## Configuration

Copy `config.example.toml` to `config.toml` and adjust the settings:

```toml
[server]
host = "0.0.0.0"
port = 3000
https = true

[database]
url = "mongodb://localhost:27017"
name = "bisca"

[ssl]
cert_path = "certs/cert.pem"
key_path = "certs/key.pem"
```

## Docker Support

```bash
# Build and run with Docker
docker-compose up --build

# Run in detached mode
docker-compose up --build -d

# View logs
docker-compose logs -f app
```
