#!/bin/bash

# Start Bisca server with HTTPS support

echo "🎮 Bisca Server Startup"
echo "=================================="

# Check if certificates exist
if [ ! -f "certs/server.crt" ] || [ ! -f "certs/server.key" ]; then
    echo "⚠️  SSL certificates not found!"
    echo "🔧 Generating development certificates..."
    ./scripts/generate_dev_certs.sh
    echo ""
fi

# Check command line arguments
if [ "$1" = "--https" ]; then
    echo "🔐 Starting HTTPS server on port 443..."
    cargo run -- --https
elif [ "$1" = "--dev" ]; then
    echo "🚀 Starting development server on port 3000..."
    cargo run -- --dev
else
    echo "📖 Usage:"
    echo "  ./scripts/start_server.sh --dev     # HTTP development"
    echo "  ./scripts/start_server.sh --https   # HTTPS production"
    echo ""
    echo "🔐 For production HTTPS, use Let's Encrypt certificates:"
    echo "  ./scripts/setup_production_https.sh"
fi
