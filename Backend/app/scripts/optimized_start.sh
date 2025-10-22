#!/bin/bash

# Script per avviare Bisca in modo ottimizzato
echo "🚀 Avvio Bisca Server Ottimizzato"
echo "=================================="

# Verifica se Docker è in esecuzione
if ! docker info > /dev/null 2>&1; then
    echo "❌ Docker non è in esecuzione. Avvialo prima."
    exit 1
fi

# Pulisce le risorse Docker non utilizzate
echo "🧹 Pulizia risorse Docker..."
docker system prune -f

# Imposta limiti di memoria per Docker Desktop (Windows)
echo "⚙️  Configurazione ottimizzata..."

# Avvia i servizi con limiti di memoria
echo "🎮 Avvio Bisca Server..."
docker-compose -f docker-compose.simple.yml up --build -d

# Verifica lo stato dei container
echo "📊 Stato dei container:"
docker-compose -f docker-compose.simple.yml ps

echo ""
echo "✅ Bisca Server avviato in modalità ottimizzata!"
echo "🌐 API disponibile su: http://localhost:3000"
echo "📡 Health check: http://localhost:3000/health"
echo ""
echo "💡 Per monitorare l'uso di memoria:"
echo "   docker stats"
echo ""
echo "🛑 Per fermare: docker-compose -f docker-compose.simple.yml down"
