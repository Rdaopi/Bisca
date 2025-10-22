#!/bin/bash

# Script per fermare Bisca e liberare memoria
echo "🛑 Fermata Bisca Server e pulizia memoria"
echo "========================================="

# Ferma i container
echo "📦 Fermata container..."
docker-compose -f docker-compose.simple.yml down

# Pulisce le risorse Docker
echo "🧹 Pulizia risorse Docker..."
docker system prune -f

# Ferma WSL se possibile
echo "🔧 Ottimizzazione WSL..."
wsl --shutdown 2>/dev/null || true

echo ""
echo "✅ Pulizia completata!"
echo "💾 Memoria liberata"
echo "🔄 Sistema ottimizzato"
echo ""
echo "💡 Per verificare la memoria liberata, controlla il Task Manager"
