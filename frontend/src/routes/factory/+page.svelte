<script lang="ts">
  let textInput = $state('');
  let isProcessing = $state(false);
  let processStep = $state(0); // 0: input, 1: processing, 2: consensus, 3: completed
  let consensusScore = $state(0);

  function startProcessing() {
    if (!textInput.trim()) return;
    isProcessing = true;
    processStep = 1;

    // Simulated multi-agent consensus workflow steps
    setTimeout(() => {
      processStep = 2;
      consensusScore = 92; // Mock score above 85% threshold
      isProcessing = false;
    }, 2500);
  }

  function commitToGraph() {
    processStep = 3;
  }

  function reset() {
    textInput = '';
    processStep = 0;
    consensusScore = 0;
  }
</script>

<div class="w-full flex flex-col gap-6 animate-fade-in pb-12">
  <!-- Page Header -->
  <div class="glass p-6 rounded-2xl border border-border/10">
    <h1 class="text-xl font-extrabold text-gold-400">Data Factory</h1>
    <p class="text-xs text-text-secondary leading-relaxed mt-1">
      Multi-Agent AI Consensus Engine untuk ekstraksi naskah sejarah mentah menjadi entitas terstruktur pada graf pengetahuan.
    </p>
  </div>

  {#if processStep === 0}
    <!-- Step 1: Input Text -->
    <div class="glass p-6 rounded-2xl border border-border/10 flex flex-col gap-4">
      <div class="flex justify-between items-center">
        <label for="history-input" class="text-xs font-bold text-text-secondary uppercase tracking-wider">Naskah Sejarah Mentah</label>
        <span class="text-[10px] text-text-muted">Masukkan teks biografi, hadits, atau catatan sejarah klasik</span>
      </div>

      <textarea
        id="history-input"
        bind:value={textInput}
        rows="8"
        class="w-full bg-navy-950/60 border border-border/10 rounded-xl p-4 text-xs text-text-primary focus:border-gold-500/30 focus:outline-none leading-relaxed font-light"
        placeholder="Contoh: Pada tahun ke-1 Hijriah, tepatnya bulan Rabiul Awwal, Nabi Muhammad ﷺ beserta Abu Bakar As-Siddiq berhijrah dari kota Makkah menuju Yatsrib yang kemudian hari dikenal sebagai Madinah Al-Munawwarah..."
      ></textarea>

      <button
        onclick={startProcessing}
        disabled={!textInput.trim()}
        class="w-full md:w-auto self-end px-8 py-3 rounded-xl gradient-gold text-surface font-bold text-xs hover:shadow-[0_0_15px_rgba(212,168,83,0.4)] disabled:opacity-50 disabled:pointer-events-none transition-all"
      >
        Proses & Jalankan Konsensus Agen AI
      </button>
    </div>
  {:else if processStep === 1}
    <!-- Step 2: Processing Pipeline -->
    <div class="grid grid-cols-1 md:grid-cols-3 gap-6 animate-fade-in">
      <!-- Historian Agent -->
      <div class="glass p-6 rounded-2xl border border-border/10 flex flex-col justify-between min-h-[220px]">
        <div>
          <div class="flex justify-between items-center mb-4">
            <span class="text-sm font-bold text-gold-400">📜 Historian Agent</span>
            <span class="w-2.5 h-2.5 rounded-full bg-gold-500 animate-ping"></span>
          </div>
          <p class="text-xs text-text-secondary leading-relaxed mb-4">
            Menganalisis teks sejarah klasik untuk mendeteksi entitas kronologis (Tahun Hijriah/Masehi), aktor utama, dan penanda tempat.
          </p>
        </div>
        <div class="border-t border-border/10 pt-4 text-[10px] text-text-muted">
          Status: Mengekstraksi Entitas...
        </div>
      </div>

      <!-- Logician Agent -->
      <div class="glass p-6 rounded-2xl border border-border/10 flex flex-col justify-between min-h-[220px]">
        <div>
          <div class="flex justify-between items-center mb-4">
            <span class="text-sm font-bold text-emerald-400">🔍 Logician Agent</span>
            <span class="w-2.5 h-2.5 rounded-full bg-emerald-500 animate-ping"></span>
          </div>
          <p class="text-xs text-text-secondary leading-relaxed mb-4">
            Melakukan cek konsistensi silang data logis (penyesuaian konversi tanggal Hijriah ke Masehi, kelogisan umur tokoh).
          </p>
        </div>
        <div class="border-t border-border/10 pt-4 text-[10px] text-text-muted">
          Status: Memverifikasi Kalender Ganda...
        </div>
      </div>

      <!-- Architect Agent -->
      <div class="glass p-6 rounded-2xl border border-border/10 flex flex-col justify-between min-h-[220px]">
        <div>
          <div class="flex justify-between items-center mb-4">
            <span class="text-sm font-bold text-amber-400">🏗️ Architect Agent</span>
            <span class="w-2.5 h-2.5 rounded-full bg-amber-500 animate-ping"></span>
          </div>
          <p class="text-xs text-text-secondary leading-relaxed mb-4">
            Mengonstruksi relasi spasial, temporal, dan sosial menjadi bentuk properti Node dan Edge graf Neo4j.
          </p>
        </div>
        <div class="border-t border-border/10 pt-4 text-[10px] text-text-muted">
          Status: Menyiapkan Struktur Neo4j...
        </div>
      </div>
    </div>
  {:else if processStep === 2}
    <!-- Step 3: Consensus & Commit -->
    <div class="glass p-6 rounded-2xl border border-border/10 flex flex-col items-center text-center max-w-2xl mx-auto gap-5 animate-fade-in">
      <div class="w-24 h-24 rounded-full border-4 border-emerald-500 flex items-center justify-center bg-emerald-500/10">
        <span class="text-2xl font-extrabold text-emerald-400">{consensusScore}%</span>
      </div>

      <div>
        <h2 class="text-lg font-bold text-text-primary">Hasil Konsensus Multi-Agent AI Tinggi</h2>
        <p class="text-xs text-text-secondary mt-1 max-w-md mx-auto">
          Tiga agen AI setuju atas validasi struktur entitas dengan tingkat kecocokan 92% (melebihi ambang batas aman 85%).
        </p>
      </div>

      <!-- Extracted preview -->
      <div class="w-full p-4 rounded-xl bg-navy-950/60 border border-border/10 text-left flex flex-col gap-3">
        <h3 class="text-xs font-bold text-gold-500 border-b border-border/10 pb-2">Pratinjau Hasil Ekstraksi</h3>
        
        <div class="grid grid-cols-2 gap-4 text-xs">
          <div>
            <span class="text-[10px] text-text-secondary block">Peristiwa Baru:</span>
            <span class="font-bold text-text-primary">Hijrah ke Madinah</span>
          </div>
          <div>
            <span class="text-[10px] text-text-secondary block">Penanggalan Ganda:</span>
            <span class="font-bold text-text-primary">1 H / 622 M</span>
          </div>
          <div>
            <span class="text-[10px] text-text-secondary block">Aktor:</span>
            <span class="font-bold text-text-primary">Nabi Muhammad ﷺ, Abu Bakar</span>
          </div>
          <div>
            <span class="text-[10px] text-text-secondary block">Lokasi:</span>
            <span class="font-bold text-text-primary">Makkah, Madinah</span>
          </div>
        </div>
      </div>

      <div class="flex gap-3 w-full">
        <button
          onclick={reset}
          class="flex-1 py-3 rounded-xl glass border border-border/10 text-text-primary font-bold text-xs hover:bg-navy-900 transition-all"
        >
          Mulai Baru
        </button>
        <button
          onclick={commitToGraph}
          class="flex-1 py-3 rounded-xl gradient-gold text-surface font-bold text-xs hover:shadow-[0_0_15px_rgba(212,168,83,0.4)] transition-all"
        >
          Commit Langsung ke Graph Database
        </button>
      </div>
    </div>
  {:else}
    <!-- Step 4: Finished -->
    <div class="glass p-8 rounded-2xl border border-border/10 flex flex-col items-center text-center max-w-md mx-auto gap-4 animate-fade-in">
      <span class="text-4xl text-emerald-400">✅</span>
      <h2 class="text-lg font-bold text-text-primary">Graf Berhasil Diperbarui</h2>
      <p class="text-xs text-text-secondary">
        Entitas sejarah baru berhasil divalidasi oleh Curator (BozzQ) secara otomatis, dicatat ke PostgreSQL audit logs, dan dikomit permanen ke Neo4j Graph.
      </p>
      
      <div class="flex gap-3 w-full mt-4">
        <a
          href="/graph"
          class="flex-1 py-3 rounded-xl glass border border-border/10 text-text-primary font-bold text-xs hover:bg-navy-900 text-center transition-all"
        >
          Lihat di Graf
        </a>
        <button
          onclick={reset}
          class="flex-1 py-3 rounded-xl gradient-gold text-surface font-bold text-xs hover:shadow-[0_0_15px_rgba(212,168,83,0.4)] transition-all"
        >
          Input Baru
        </button>
      </div>
    </div>
  {/if}
</div>
