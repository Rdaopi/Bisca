# Bisca - Online Card Game

A full-stack implementation of the Italian card game "Bisca" with real-time multiplayer functionality.

## Project Structure

```
Bisca/
├── Backend/          # Rust API server
│   └── app/         # Main backend application
├── Frontend/         # Web client (coming soon)
├── docs/            # Documentation
└── scripts/         # Development scripts
```

## Features

- **Real-time multiplayer** - Play with friends online
- **Card game logic** - Complete Bisca rules implementation
- **WebSocket support** - Live game updates
- **HTTPS security** - Secure connections
- **Responsive design** - Works on desktop and mobile

## Quick Start

### Backend (Rust API)

```bash
cd Backend/app
cargo run
```

The API will be available at `https://localhost:3000`

### Frontend (Coming Soon)

The web client will be implemented using modern web technologies.

## Development

This is a monorepo containing both backend and frontend components:

- **Backend**: Rust with Axum framework
- **Frontend**: Modern web technologies (TBD)
- **Database**: MongoDB for game state
- **Real-time**: Server-Sent Events for live updates

## Contributing

1. Fork the repository
2. Create a feature branch: `git checkout -b feature/amazing-feature`
3. Commit your changes: `git commit -m 'Add amazing feature'`
4. Push to the branch: `git push origin feature/amazing-feature`
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the LICENSE file for details.
