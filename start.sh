#!/bin/bash
# -----------------------------------------------------------------------------
# Sumbu Peradaban — One-Click Setup & Run Script for Arch Linux
# -----------------------------------------------------------------------------
set -e

echo -e "\e[1;33m=== Sumbu Peradaban: One-Click Setup (Arch Linux) ===\e[0m"

# 1. Update system & install Docker if not exists
if ! command -v docker &> /dev/null; then
    echo -e "\e[1;32m[+] Docker belum terdeteksi. Menginstal Docker & Docker Compose...\e[0m"
    sudo pacman -Syu --noconfirm docker docker-compose
else
    echo -e "\e[1;32m[✓] Docker sudah terpasang.\e[0m"
fi

# 2. Check and start Docker Daemon
echo -e "\e[1;32m[+] Memastikan Docker Daemon berjalan...\e[0m"
if systemctl is-active --quiet docker; then
    echo -e "\e[1;32m[✓] Docker Service aktif.\e[0m"
else
    echo -e "\e[1;33m[!] Docker Service mati. Mencoba menyalakan...\e[0m"
    # Try systemd first
    if command -v systemctl &> /dev/null; then
        sudo systemctl start docker || true
    fi
    
    # Fallback for WSL without systemd
    if ! sudo docker ps &> /dev/null; then
        echo -e "\e[1;33m[!] Menjalankan dockerd di latar belakang (WSL Fallback)...\e[0m"
        sudo dockerd > /tmp/dockerd.log 2>&1 &
        sleep 5
    fi
fi

# 3. Create .env file if it doesn't exist
if [ ! -f .env ]; then
    echo -e "\e[1;32m[+] Membuat file konfigurasi .env...\e[0m"
    cp .env.example .env
fi

# 4. Run Docker Compose
echo -e "\e[1;32m[+] Membangun & Menjalankan Kontainer Sumbu Peradaban...\e[0m"
sudo docker compose up --build -d

echo -e "\e[1;36m========================================================\e[0m"
echo -e "\e[1;32mSUKSES! Aplikasi Sumbu Peradaban telah aktif.\e[0m"
echo -e "\e[1;35m👉 Main App:      http://localhost:5173\e[0m"
echo -e "\e[1;35m👉 GraphQL API:   http://localhost:8080\e[0m"
echo -e "\e[1;35m👉 Neo4j Browser: http://localhost:7474\e[0m"
echo -e "\e[1;36m========================================================\e[0m"
