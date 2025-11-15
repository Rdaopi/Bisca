#!/bin/bash

# Generate self-signed certificates for development
echo "🔐 Generating development SSL certificates..."

# Create certs directory
mkdir -p certs

# Generate private key
openssl genrsa -out certs/server.key 2048

# Generate certificate signing request
openssl req -new -key certs/server.key -out certs/server.csr -subj "/C=US/ST=State/L=City/O=Organization/CN=localhost"

# Generate self-signed certificate (valid for 365 days)
openssl x509 -req -days 365 -in certs/server.csr -signkey certs/server.key -out certs/server.crt

# Clean up CSR file
rm certs/server.csr

echo "✅ Certificates generated in ./certs/"
echo "📁 server.crt - Certificate file"
echo "📁 server.key - Private key file"
echo ""
echo "⚠️  These are self-signed certificates for development only!"
echo "🌐 For production, use certificates from a trusted CA like Let's Encrypt"
