/**
 * ──────────────────────────────────────────────────────────────
 * SUMBU PERADABAN — Sumber Data Terpusat (Single Source of Truth)
 * ──────────────────────────────────────────────────────────────
 * 
 * File ini adalah SATU-SATUNYA sumber data demo untuk keempat
 * dimensi Entitas Sejarah:
 *   1. Timeline   → melihat data dari sudut pandang WAKTU
 *   2. Pelaku     → melihat data dari sudut pandang AKTOR
 *   3. Peta       → melihat data dari sudut pandang LOKASI
 *   4. Rujukan    → melihat data dari sudut pandang SUMBER
 * 
 * Menambahkan 1 event di sini akan otomatis muncul di keempat
 * halaman dimensi secara konsisten.
 * ──────────────────────────────────────────────────────────────
 */

import type { TimelineEvent } from '$lib/types/timeline';

// ─── ERA DEFINITIONS ─────────────────────────────────────
export const eras = [
  { id: 'all',        label: 'Semua Era',            icon: '🌐', min: -99999, max: 99999 },
  { id: 'adam-nuh',   label: 'Adam — Nuh AS',        icon: '🌍', min: -99999, max: -3000 },
  { id: 'ibrahim',    label: 'Ibrahim — Yusuf AS',   icon: '🕋', min: -3000,  max: -1500 },
  { id: 'musa-daud',  label: 'Musa — Sulaiman AS',   icon: '📜', min: -1500,  max: -500  },
  { id: 'isa',        label: 'Isa AS & Romawi',      icon: '✝️', min: -500,   max: 570   },
  { id: 'nubuwwah',   label: 'Masa Nubuwwah',        icon: '☪️', min: 570,    max: 632   },
  { id: 'rasyidin',   label: 'Khulafaur Rasyidin',   icon: '⚔️', min: 632,    max: 661   },
  { id: 'umayyah',    label: 'Daulah Umayyah',       icon: '🏛️', min: 661,    max: 750   },
  { id: 'abbasiyah',  label: 'Daulah Abbasiyah',     icon: '📚', min: 750,    max: 1258  },
  { id: 'modern',     label: 'Pasca-Abbasiyah',      icon: '🌏', min: 1258,   max: 99999 },
];

// ─── MASTER SOURCE REGISTRY ───────────────────────────────
export const masterSources = [
  { id: '1', title: 'Shahih al-Bukhari',               author: 'Imam Al-Bukhari',            type: 'Hadits Shahih (Kategori 1)',        period: 'Abad ke-3 H / 9 M',    reliability: '98%', tier: 'canonical' as const, avatar: '📄' },
  { id: '2', title: 'Sirah Nabawiyyah Ibnu Hisyam',     author: 'Ibnu Hisyam / Ibnu Ishaq',   type: 'Historiografi Sirah Klasik',        period: 'Abad ke-2 H / 8 M',    reliability: '95%', tier: 'reviewed' as const,  avatar: '📄' },
  { id: '3', title: 'Prasasti Hassan (Makkah)',          author: 'Situs Arkeologi',             type: 'Epigrafi / Prasasti Batu Klasik',   period: 'Abad ke-1 H / 7 M',    reliability: '97%', tier: 'canonical' as const, avatar: '🪨' },
  { id: '4', title: 'Al-Quran Al-Karim',                author: 'Wahyu Ilahi',                 type: 'Kitab Suci / Sumber Primer',        period: 'Abad ke-1 H / 7 M',    reliability: '100%', tier: 'canonical' as const, avatar: '📖' },
  { id: '5', title: 'Tarikh at-Thabari',                author: 'Imam At-Thabari',             type: 'Historiografi Universal',           period: 'Abad ke-4 H / 10 M',   reliability: '90%', tier: 'reviewed' as const,  avatar: '📄' },
];

// ─── WORLD EVENTS: PROPHETIC AXIS + SYNCHRONIC CIVILIZATIONS ───
export const worldEvents: TimelineEvent[] = [
  // === ZAMAN ADAM — NUH ===
  {
    title: 'Turunnya Adam AS ke Bumi',
    year: '~Awal Penciptaan',
    yearSort: -50000,
    description: 'Awal mula peradaban manusia. Adam AS sebagai khalifah pertama di muka bumi, menerima wahyu dan mengajarkan nama-nama segala sesuatu.',
    tier: 'canonical',
    actors: [{ name: 'Nabi Adam AS', role: 'Khalifah Pertama' }, { name: 'Hawa', role: 'Pasangan Pertama' }],
    locations: [{ name: 'Jazirah Arab', type: 'Titik Turun' }],
    sources: [{ id: '4', title: 'Al-Quran Al-Karim' }],
  },
  {
    title: 'Banjir Besar Nabi Nuh AS',
    year: '~3000 SM',
    yearSort: -3000,
    description: 'Peristiwa banjir global yang menandai reset peradaban. Kisah ini hadir di hampir seluruh kebudayaan kuno dunia.',
    tier: 'canonical',
    actors: [{ name: 'Nabi Nuh AS', role: 'Rasul' }],
    locations: [{ name: 'Mesopotamia', type: 'Titik Peradaban' }],
    sources: [{ id: '4', title: 'Al-Quran Al-Karim' }, { id: '5', title: 'Tarikh at-Thabari' }],
  },
  {
    title: 'Peradaban Sumeria Awal',
    year: '~3000 SM',
    yearSort: -3000,
    description: 'Bangkitnya peradaban Sumeria di lembah Tigris-Eufrat. Penemuan aksara paku (cuneiform) dan sistem irigasi pertama di dunia.',
    tier: 'reviewed',
    civilization: 'Peradaban Mesopotamia',
    isSynchrnic: true,
    locations: [{ name: 'Mesopotamia', type: 'Lembah Sungai' }],
  },

  // === ZAMAN IBRAHIM ===
  {
    title: 'Pembangunan Ka\'bah',
    year: '~2000 SM',
    yearSort: -2000,
    description: 'Nabi Ibrahim AS bersama Ismail AS membangun kembali Ka\'bah di Makkah sebagai rumah ibadah pertama bagi seluruh umat manusia.',
    tier: 'canonical',
    actors: [{ name: 'Nabi Ibrahim AS', role: 'Bapak Para Nabi' }, { name: 'Nabi Ismail AS', role: 'Putra / Pendiri Bangsa Arab' }],
    locations: [{ name: 'Makkah', type: 'Tanah Haram' }],
    sources: [{ id: '4', title: 'Al-Quran Al-Karim' }],
  },
  {
    title: 'Kerajaan Hammurabi & Hukum Tertua',
    year: '~1792 SM',
    yearSort: -1792,
    description: 'Raja Hammurabi menyusun kode hukum tertulis tertua yang diketahui. Babylonia menjadi pusat kekuasaan di Mesopotamia.',
    tier: 'reviewed',
    civilization: 'Kekaisaran Babylonia',
    isSynchrnic: true,
    locations: [{ name: 'Baghdad', type: 'Babylonia Kuno' }],
  },

  // === ZAMAN MUSA — SULAIMAN ===
  {
    title: 'Exodus — Pembebasan Bani Israel',
    year: '~1250 SM',
    yearSort: -1250,
    description: 'Nabi Musa AS memimpin Bani Israel keluar dari perbudakan Fir\'aun. Peristiwa terbelahnya Laut Merah dan turunnya Kitab Taurat.',
    tier: 'canonical',
    actors: [{ name: 'Nabi Musa AS', role: 'Pembebas / Rasul' }],
    locations: [{ name: 'Alexandria', type: 'Mesir Kuno' }],
    sources: [{ id: '4', title: 'Al-Quran Al-Karim' }],
  },
  {
    title: 'Kerajaan Mesir Baru (New Kingdom)',
    year: '~1250 SM',
    yearSort: -1250,
    description: 'Puncak kekuasaan Fir\'aun. Pembangunan kuil-kuil raksasa di Luxor dan Karnak. Ramses II memperluas kekuasaan ke Syam.',
    tier: 'reviewed',
    civilization: 'Kerajaan Mesir Kuno',
    isSynchrnic: true,
    locations: [{ name: 'Alexandria', type: 'Thebes / Mesir Hulu' }],
  },
  {
    title: 'Kerajaan Nabi Sulaiman AS',
    year: '~950 SM',
    yearSort: -950,
    description: 'Puncak kerajaan Israel di bawah Nabi Sulaiman AS. Pembangunan Baitul Maqdis (Masjid Al-Aqsha pertama) dan kekuasaan atas jin serta angin.',
    tier: 'canonical',
    actors: [{ name: 'Nabi Sulaiman AS', role: 'Raja & Nabi' }],
    locations: [{ name: 'Baitul Maqdis', type: 'Yerusalem' }],
    sources: [{ id: '4', title: 'Al-Quran Al-Karim' }],
  },
  {
    title: 'Kebangkitan Peradaban Yunani Kuno',
    year: '~800 SM',
    yearSort: -800,
    description: 'Lahirnya polis (negara-kota) Athena dan Sparta. Awal tradisi filosofi, demokrasi, dan Olimpiade.',
    tier: 'reviewed',
    civilization: 'Peradaban Yunani',
    isSynchrnic: true,
    locations: [{ name: 'Konstantinopel', type: 'Athena, Yunani' }],
  },

  // === ZAMAN ISA AS ===
  {
    title: 'Kelahiran Nabi Isa Al-Masih AS',
    year: '~4 SM',
    yearSort: -4,
    description: 'Kelahiran ajaib Nabi Isa AS dari Maryam. Titik awal perhitungan kalender Masehi dan penyebaran risalah Injil.',
    tier: 'canonical',
    actors: [{ name: 'Nabi Isa AS', role: 'Rasul / Al-Masih' }, { name: 'Maryam', role: 'Ibunda' }],
    locations: [{ name: 'Baitul Maqdis', type: 'Baitul Lahm (Bethlehem)' }],
    sources: [{ id: '4', title: 'Al-Quran Al-Karim' }],
  },
  {
    title: 'Pax Romana — Puncak Kekaisaran Romawi',
    year: '~27 SM',
    yearSort: -27,
    description: 'Era kedamaian Romawi di bawah Augustus Caesar. Kekaisaran terbentang dari Britania hingga Mesopotamia.',
    tier: 'reviewed',
    civilization: 'Kekaisaran Romawi',
    isSynchrnic: true,
    locations: [{ name: 'Konstantinopel', type: 'Roma' }],
  },
  {
    title: 'Dinasti Han & Jalur Sutra',
    year: '~100 M',
    yearSort: 100,
    description: 'Puncak peradaban Tiongkok klasik. Pembukaan Jalur Sutra menghubungkan Timur dan Barat melalui Asia Tengah.',
    tier: 'reviewed',
    civilization: 'Dinasti Han (Tiongkok)',
    isSynchrnic: true,
    locations: [{ name: 'Chang\'an', type: 'Ibu Kota Tiongkok' }],
  },

  // === MASA NUBUWWAH ===
  {
    title: 'Kelahiran Nabi Muhammad ﷺ',
    year: '570 M',
    yearSort: 570,
    description: 'Tahun Gajah (\'Aam al-Fiil). Kelahiran penutup para nabi di Makkah, dari suku Quraisy, klan Bani Hasyim.',
    tier: 'canonical',
    actors: [{ name: 'Nabi Muhammad ﷺ', role: 'Rasulullah' }],
    locations: [{ name: 'Makkah', type: 'Tanah Haram' }],
    sources: [{ id: '1', title: 'Shahih al-Bukhari' }, { id: '2', title: 'Sirah Nabawiyyah Ibnu Hisyam' }],
  },
  {
    title: 'Perang Romawi — Persia (Sassanid)',
    year: '~570 M',
    yearSort: 571,
    description: 'Konflik berkepanjangan antara dua adidaya dunia: Kekaisaran Romawi Timur (Bizantium) dan Kekaisaran Sassanid Persia.',
    tier: 'reviewed',
    civilization: 'Romawi Timur vs Sassanid',
    isSynchrnic: true,
    locations: [{ name: 'Damaskus', type: 'Perbatasan Syam' }],
  },
  {
    title: 'Peristiwa Hijrah ke Madinah',
    year: '1 H / 622 M',
    yearSort: 622,
    description: 'Titik awal kalender Hijriah. Kepindahan umat Islam ke Madinah dan berdirinya Daulah Madinah — negara konstitusional pertama.',
    tier: 'canonical',
    uuid: 'hijrah-001',
    actors: [{ name: 'Nabi Muhammad ﷺ', role: 'Pemimpin' }, { name: 'Abu Bakar As-Siddiq', role: 'Pendamping' }],
    locations: [{ name: 'Makkah', type: 'Keberangkatan' }, { name: 'Madinah', type: 'Tujuan' }],
    sources: [{ id: '1', title: 'Shahih al-Bukhari' }],
  },
  {
    title: 'Perang Badar Al-Kubra',
    year: '2 H / 624 M',
    yearSort: 624,
    description: 'Kemenangan besar pertama umat Islam. 313 pejuang Muslim mengalahkan 1000 pasukan Quraisy di lembah Badr.',
    tier: 'canonical',
    uuid: 'badar-001',
    actors: [{ name: 'Nabi Muhammad ﷺ', role: 'Panglima' }],
    locations: [{ name: 'Badr', type: 'Medan Pertempuran' }],
    sources: [{ id: '1', title: 'Shahih al-Bukhari' }, { id: '2', title: 'Sirah Nabawiyyah Ibnu Hisyam' }],
  },
  {
    title: 'Fathu Makkah (Pembebasan Makkah)',
    year: '8 H / 630 M',
    yearSort: 630,
    description: 'Rasulullah ﷺ kembali ke Makkah bersama 10.000 pasukan. Ka\'bah dibersihkan dari 360 berhala tanpa pertumpahan darah.',
    tier: 'canonical',
    uuid: 'fathu-makkah-001',
    actors: [{ name: 'Nabi Muhammad ﷺ', role: 'Pemimpin' }],
    locations: [{ name: 'Makkah', type: 'Tanah Haram' }],
    sources: [{ id: '1', title: 'Shahih al-Bukhari' }],
  },

  // === KHULAFAUR RASYIDIN ===
  {
    title: 'Wafatnya Nabi Muhammad ﷺ',
    year: '11 H / 632 M',
    yearSort: 632,
    description: 'Wafatnya Rasulullah dan dimulainya era Khulafaur Rasyidin di bawah kepemimpinan Abu Bakar As-Siddiq.',
    tier: 'canonical',
    actors: [{ name: 'Nabi Muhammad ﷺ', role: 'Rasulullah' }, { name: 'Abu Bakar As-Siddiq', role: 'Khalifah I' }],
    locations: [{ name: 'Madinah', type: 'Kota Nabi' }],
    sources: [{ id: '1', title: 'Shahih al-Bukhari' }],
  },
  {
    title: 'Pembebasan Baitul Maqdis',
    year: '16 H / 637 M',
    yearSort: 637,
    description: 'Khalifah Umar bin Khattab menerima kunci Yerusalem secara damai dari Patriark Sophronius. Jaminan keamanan bagi seluruh pemeluk agama.',
    tier: 'canonical',
    actors: [{ name: 'Umar bin Khattab', role: 'Khalifah II' }],
    locations: [{ name: 'Baitul Maqdis', type: 'Yerusalem' }],
    sources: [{ id: '5', title: 'Tarikh at-Thabari' }],
  },
  {
    title: 'Dinasti Tang Berdiri di Tiongkok',
    year: '618 M',
    yearSort: 638,
    description: 'Salah satu dinasti terbesar Tiongkok. Era keemasan seni, sastra, dan perdagangan internasional melalui Jalur Sutra.',
    tier: 'reviewed',
    civilization: 'Dinasti Tang (Tiongkok)',
    isSynchrnic: true,
    locations: [{ name: 'Chang\'an', type: 'Ibu Kota' }],
  },

  // === DAULAH UMAYYAH ===
  {
    title: 'Daulah Umayyah Berdiri',
    year: '41 H / 661 M',
    yearSort: 661,
    description: 'Mu\'awiyah bin Abi Sufyan mendirikan dinasti Islam pertama berpusat di Damaskus. Ekspansi massif ke Afrika Utara dan Asia Tengah.',
    tier: 'reviewed',
    actors: [{ name: 'Mu\'awiyah bin Abi Sufyan', role: 'Pendiri Daulah' }],
    locations: [{ name: 'Damaskus', type: 'Ibu Kota' }],
    sources: [{ id: '5', title: 'Tarikh at-Thabari' }],
  },
  {
    title: 'Pembukaan Andalusia (Spanyol)',
    year: '92 H / 711 M',
    yearSort: 711,
    description: 'Thariq bin Ziyad menyeberangi Selat Gibraltar dan membuka babak baru peradaban Islam di Eropa selama 800 tahun.',
    tier: 'reviewed',
    actors: [{ name: 'Thariq bin Ziyad', role: 'Panglima' }],
    locations: [{ name: 'Cordoba', type: 'Andalusia' }],
  },

  // === DAULAH ABBASIYAH ===
  {
    title: 'Pendirian Baitul Hikmah',
    year: '~215 H / 830 M',
    yearSort: 830,
    description: 'Khalifah Al-Ma\'mun mendirikan pusat penerjemahan dan riset terbesar di dunia. Karya Yunani, Persia, dan India diterjemahkan ke bahasa Arab.',
    tier: 'canonical',
    actors: [{ name: 'Khalifah Al-Ma\'mun', role: 'Khalifah Abbasiyah' }],
    locations: [{ name: 'Baghdad', type: 'Ibu Kota' }],
    sources: [{ id: '5', title: 'Tarikh at-Thabari' }],
  },
  {
    title: 'Kerajaan Majapahit Berdiri',
    year: '~1293 M',
    yearSort: 1293,
    description: 'Kerajaan maritim terbesar di Nusantara. Menguasai hampir seluruh wilayah Asia Tenggara maritim.',
    tier: 'reviewed',
    civilization: 'Kerajaan Nusantara',
    isSynchrnic: true,
    locations: [{ name: 'Demak', type: 'Jawa Timur' }],
  },

  // === PASCA-ABBASIYAH ===
  {
    title: 'Jatuhnya Baghdad ke Mongol',
    year: '656 H / 1258 M',
    yearSort: 1258,
    description: 'Hulagu Khan menghancurkan Baghdad dan mengakhiri Kekhalifahan Abbasiyah. Perpustakaan dibakar, ratusan ribu tewas.',
    tier: 'canonical',
    locations: [{ name: 'Baghdad', type: 'Ibu Kota Abbasiyah' }],
    sources: [{ id: '5', title: 'Tarikh at-Thabari' }],
  },
  {
    title: 'Fathu Konstantinopel',
    year: '857 H / 1453 M',
    yearSort: 1453,
    description: 'Sultan Muhammad Al-Fatih menaklukkan Konstantinopel, mengakhiri Kekaisaran Romawi Timur yang bertahan 1000 tahun.',
    tier: 'canonical',
    actors: [{ name: 'Sultan Muhammad Al-Fatih', role: 'Sultan Utsmani' }],
    locations: [{ name: 'Konstantinopel', type: 'Istanbul' }],
    sources: [{ id: '1', title: 'Shahih al-Bukhari' }],
  },
  {
    title: 'Kesultanan Samudera Pasai',
    year: '~1267 M',
    yearSort: 1267,
    description: 'Kerajaan Islam pertama di Nusantara. Titik awal penyebaran Islam di Asia Tenggara melalui jalur perdagangan maritim.',
    tier: 'reviewed',
    locations: [{ name: 'Samudera Pasai', type: 'Aceh, Nusantara' }],
  },
];

// ─── DERIVED DATA HELPERS ───────────────────────────────────

/** Extract unique actors from all events with their related events */
export function deriveActors(events: TimelineEvent[]) {
  const actorMap = new Map<string, { name: string; role: string; events: { title: string; year: string; uuid?: string }[] }>();

  for (const ev of events) {
    if (!ev.actors) continue;
    for (const actor of ev.actors) {
      if (!actorMap.has(actor.name)) {
        actorMap.set(actor.name, { name: actor.name, role: actor.role || '', events: [] });
      }
      actorMap.get(actor.name)!.events.push({ title: ev.title, year: ev.year, uuid: ev.uuid });
    }
  }

  return Array.from(actorMap.values()).sort((a, b) => b.events.length - a.events.length);
}

/** Extract unique locations from all events with their related events */
export function deriveLocations(events: TimelineEvent[]) {
  const locMap = new Map<string, { name: string; type: string; events: { title: string; year: string; uuid?: string }[] }>();

  for (const ev of events) {
    if (!ev.locations) continue;
    for (const loc of ev.locations) {
      if (!locMap.has(loc.name)) {
        locMap.set(loc.name, { name: loc.name, type: loc.type || '', events: [] });
      }
      locMap.get(loc.name)!.events.push({ title: ev.title, year: ev.year, uuid: ev.uuid });
    }
  }

  return Array.from(locMap.values()).sort((a, b) => b.events.length - a.events.length);
}

/** Extract unique sources from all events with their related events */
export function deriveSources(events: TimelineEvent[]) {
  const srcMap = new Map<string, { id: string; title: string; events: { title: string; year: string; uuid?: string }[] }>();

  for (const ev of events) {
    if (!ev.sources) continue;
    for (const src of ev.sources) {
      if (!srcMap.has(src.id)) {
        srcMap.set(src.id, { id: src.id, title: src.title, events: [] });
      }
      srcMap.get(src.id)!.events.push({ title: ev.title, year: ev.year, uuid: ev.uuid });
    }
  }

  return Array.from(srcMap.values()).sort((a, b) => b.events.length - a.events.length);
}
