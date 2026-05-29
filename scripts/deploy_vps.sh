#!/bin/bash
# Script Hot-Deploy Manual ke VPS Sumbu Peradaban

echo "🚀 Memulai Deployment Manual ke VPS (43.134.17.13)..."

ssh ubuntu@43.134.17.13 << 'EOF'
  cd ~/sumbu-peradaban
  
  echo "📥 Menarik kode terbaru dari GitHub..."
  git pull origin main
  
  echo "🔄 Mengunduh Docker Image terbaru (Backend & Caddy)..."
  # Catatan: Jika GHCR dikonfigurasi sebagai private, pastikan VPS sudah login: 
  # docker login ghcr.io -u <username> -p <token>
  sudo docker compose -f docker-compose.prod.yml pull backend
  
  echo "⚡ Merestart Kontainer secara halus (Zero Downtime)..."
  sudo docker compose -f docker-compose.prod.yml up -d
  
  echo "🧹 Membersihkan sisa image lama..."
  sudo docker image prune -f
EOF

echo "✅ Hot-Deploy Selesai! Aplikasi sudah menggunakan versi terbaru."
