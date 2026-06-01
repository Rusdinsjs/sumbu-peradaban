import os
import sys
import shutil
import argparse
from datetime import datetime
from graphql_agent import SumbuPeradabanClient

# Konfigurasi Environment & Fallback Credentials
# Secara default skrip ini akan mencoba mencari kredensial di Environment Variables.
# Jika tidak ada, ia akan menggunakan fallback username dan password di bawah ini.
ADMIN_USERNAME = os.getenv("SUMBU_ADMIN_USERNAME", "admin")
ADMIN_PASSWORD = os.getenv("SUMBU_ADMIN_PASSWORD", "password_admin")

GRAPHQL_URL = os.getenv("SUMBU_GRAPHQL_URL", "https://super.sjsgroup.site/graphql")

# Path Konfigurasi
BASE_DIR = os.path.dirname(os.path.abspath(__file__))
PENDING_DIR = os.path.join(BASE_DIR, "data_input", "pending")
SUCCESS_DIR = os.path.join(BASE_DIR, "data_input", "success")
ERROR_DIR = os.path.join(BASE_DIR, "data_input", "error")

def setup_directories():
    """Memastikan direktori yang dibutuhkan tersedia."""
    os.makedirs(PENDING_DIR, exist_ok=True)
    os.makedirs(SUCCESS_DIR, exist_ok=True)
    os.makedirs(ERROR_DIR, exist_ok=True)

def move_file(filepath, status="success"):
    """Memindahkan file ke direktori yang sesuai (success atau error)."""
    filename = os.path.basename(filepath)
    # Tambahkan timestamp ke nama file agar tidak terjadi bentrok nama
    timestamp = datetime.now().strftime("%Y%m%d_%H%M%S")
    name, ext = os.path.splitext(filename)
    new_filename = f"{name}_{timestamp}{ext}"
    
    dest_dir = SUCCESS_DIR if status == "success" else ERROR_DIR
    dest_path = os.path.join(dest_dir, new_filename)
    
    shutil.move(filepath, dest_path)
    print(f"📦 File dipindahkan ke: {dest_path}")

def main():
    parser = argparse.ArgumentParser(description="Auto Injector Data Sumbu Peradaban untuk Agen Den")
    parser.add_argument("files", nargs="+", help="Path file JSON yang akan diinput (bisa lebih dari satu)")
    args = parser.parse_args()

    # Inisialisasi Klien dan Login (Cukup sekali)
    client = SumbuPeradabanClient(url=GRAPHQL_URL)
    try:
        print("🔐 Sedang melakukan login otomatis...")
        client.login(ADMIN_USERNAME, ADMIN_PASSWORD)
    except Exception as e:
        print(f"❌ Gagal login: {e}")
        print("💡 Tips: Setel Environment Variables SUMBU_ADMIN_USERNAME dan SUMBU_ADMIN_PASSWORD")
        sys.exit(1)

    # Proses Setiap File
    has_error = False
    for filepath in args.files:
        if not os.path.exists(filepath):
            print(f"❌ Error: File '{filepath}' tidak ditemukan! Melewati...")
            has_error = True
            continue

        print("\n==================================================")
        print(f"🚀 Memulai Auto Injection: {os.path.basename(filepath)}")
        print("==================================================")

        try:
            client.process_json_batch(filepath)
            print("\n✅ Eksekusi Selesai Tanpa Exception.")
            move_file(filepath, status="success")
        except Exception as e:
            print(f"\n❌ Eksekusi Gagal karena Exception: {e}")
            move_file(filepath, status="error")
            has_error = True
            
    if has_error:
        sys.exit(1)

if __name__ == "__main__":
    setup_directories()
    main()
