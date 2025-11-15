#!/bin/bash

# Setup HTTPS for production using Let's Encrypt
echo "🔐 Setting up production HTTPS with Let's Encrypt..."

# Install certbot (if not already installed)
# Ubuntu/Debian:
# sudo apt update && sudo apt install certbot

# macOS with Homebrew:
# brew install certbot

# Generate certificate for your domain
echo "📝 Replace 'yourdomain.com' with your actual domain"
echo "🔧 Run this command with your domain:"
echo ""
echo "sudo certbot certonly --standalone -d yourdomain.com"
echo ""
echo "📁 Certificates will be stored in:"
echo "   /etc/letsencrypt/live/yourdomain.com/fullchain.pem"
echo "   /etc/letsencrypt/live/yourdomain.com/privkey.pem"
echo ""
echo "🔄 Auto-renewal setup:"
echo "sudo crontab -e"
echo "Add: 0 12 * * * /usr/bin/certbot renew --quiet"
