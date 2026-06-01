# Panduan Injeksi Data Sumbu Peradaban (SOP untuk AI Agent "Den")

Panduan ini adalah Standard Operating Procedure (SOP) permanen bagi Den untuk memvalidasi dan memasukkan data sejarah ke dalam *Knowledge Graph* Sumbu Peradaban.

Sistem menggunakan prinsip **Topological Sorting** (Upsert Base Nodes -> Upsert Pivot Nodes -> Link Edges). Skrip `inject.py` telah diperbarui untuk mendukung injeksi dua tahap ini.

## Aturan Utama (Golden Rules)
1. **Idempotensi**: Semua operasi bersifat `UPSERT`. Jika entitas (Tokoh/Lokasi/Peristiwa) sudah ada berdasarkan nama/judul, skrip hanya akan memperbarui data parsial. Jangan takut mengeksekusi script berulang kali.
2. **Ketergantungan**: Sebuah `Event` **TIDAK BOLEH** diinjeksi jika `Source` (Kitab Rujukan) belum dibuat dan belum memiliki UUID.
3. **Format**: Gunakan ekstensi `.json` dan panggil menggunakan `python scripts/inject.py nama_file.json`.

---

## 🗂️ TAHAP 1: Injeksi Entitas Dasar (Base Entities)
Pada tahap ini, Den harus membuat JSON berisi daftar Tokoh, Lokasi, atau Sumber (Rujukan) baru yang belum ada di database.

### A. Format JSON untuk Injeksi Tokoh (`actors.json`)
```json
{
  "batch_type": "ACTORS",
  "data": [
    {
      "name": "Khalid bin Walid",
      "type": "INDIVIDUAL",
      "description": "Panglima militer legendaris.",
      "cultural_sphere": "Arabia",
      "birth_year": 592,
      "death_year": 642,
      "works": [],
      "roles": ["Panglima", "Sahabat Nabi"]
    }
  ]
}
```

### B. Format JSON untuk Injeksi Lokasi (`locations.json`)
```json
{
  "batch_type": "LOCATIONS",
  "data": [
    {
      "name": "Lembah Hunain",
      "precision": "AREA",
      "lat": 21.3666,
      "lng": 40.0,
      "geography_climate": "Lembah kering berbukit",
      "historical_role": "Medan pertempuran strategis paska Fathu Makkah"
    }
  ]
}
```

### C. Format JSON untuk Injeksi Sumber/Rujukan (`sources.json`)
*(Catatan: Rujukan kini dapat diinjeksi secara otomatis. Sistem akan melakukan Upsert berdasarkan `title` atau `reference_text`).*
```json
{
  "batch_type": "SOURCES",
  "data": [
    {
      "title": "Sirah Nabawiyah",
      "reference_text": "Kutipan atau sinopsis singkat rujukan (Wajib)",
      "domain": "Sejarah Klasik",
      "author": "Ibnu Hisham",
      "publication_era": "Abad ke-8 Masehi",
      "interpretation_method": "Harfiah",
      "reliability_score": 0.95,
      "reliability_assessment": "Sangat diandalkan karena..."
    }
  ]
}
```

---

## 🌪️ TAHAP 2: Injeksi Peristiwa & Relasi Multidimensi (Events)
Setelah entitas di atas eksis, buat file JSON untuk Peristiwa. Skrip akan otomatis membuat Peristiwa, lalu **merajut relasinya** dengan Tokoh, Lokasi, dan Sumber menggunakan data yang ada.

### Format JSON untuk Injeksi Event (`events.json`)
```json
{
  "batch_type": "EVENTS",
  "data": [
    {
      "event": {
        "title": "Perang Hunain",
        "description": "Pertempuran penting setelah Fathu Makkah.",
        "year": 8,
        "gregorian_year": 630
      },
      "actors": [
        { "name": "Khalid bin Walid", "type": "INDIVIDUAL", "role": "Komandan Sayap Kanan" }
      ],
      "locations": [
        { "name": "Lembah Hunain", "precision": "AREA" }
      ],
      "sources": [
        { 
          "id": "MASUKKAN-UUID-SOURCE-DARI-TAHAP-1", 
          "sub_refs": [
            { "section": "Jilid 2", "point": "Halaman 430", "note": "Keterangan detail taktik pengepungan." }
          ]
        }
      ]
    }
  ]
}
```

## Eksekusi
Jalankan injeksi menggunakan perintah berikut dari dalam root folder:
```bash
python scripts/inject.py scripts/data_input/pending/nama_file.json
```
Jika sukses, file akan otomatis dipindahkan ke direktori `success`. Jika gagal, akan dipindahkan ke direktori `error`.
