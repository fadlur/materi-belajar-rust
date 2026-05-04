// ============================================================
// 📕 BELAJAR RUST #23 — Async/Await
// ============================================================
// Async programming memungkinkan kode berjalan secara non-blocking.
// Cocok untuk I/O-bound tasks: HTTP request, database query, file I/O.
// Rust async = zero-cost, no garbage collector, no runtime overhead.
//
// ⚠️ File ini membutuhkan dependency `tokio`. Tambahkan ke Cargo.toml:
//     [dependencies]
//     tokio = { version = "1", features = ["full"] }
//
// 🎯 Tujuan: Memahami konsep Future, async/await syntax,
//    concurrent execution, dan patterns async di Rust.
//
// 💡 Analogi Utama:
// Async seperti PESANAN DI RESTORAN. Kalau kamu pesan makanan,
//    tidak perlu menunggu di kasir sampai makanan selesai.
//    Kamu dapat nomor antrian (Future), duduk, dan pelayan
//    akan memanggilmu (.await) saat makanan siap.
//
// 🔑 Kalau sync = kamu menunggu, tidak bisa lakukan apa-apa.
//    Kalau async = kamu bisa lakukan hal lain sambil menunggu.
// ============================================================

use std::time::Duration;

// ══════════════════════════════════════════════════════════════
// KONSEP DASAR: Future
// ══════════════════════════════════════════════════════════════

// `async fn` mengembalikan `Future` — sebuah nilai yang BELUM dihitung
// Future bersifat LAZY — tidak dieksekusi sampai di-`.await` atau di-poll
//
// 💡 Analogi: Future seperti reservasi restoran — kamu sudah booking
//    (memanggil async fn), tapi belum makan. Makan dimulai saat
//    kamu datang (.await), bukan saat booking.

// Ini adalah async function:
async fn sapa(nama: &str) -> String {
    // await menunggu operasi async selesai
    // Di sini kita simulasi dengan sleep
    format!("Halo, {}! (dari async)", nama)
}

async fn hitung_lambat(n: u64) -> u64 {
    // Simulasi operasi yang memakan waktu
    // Dalam real code, ini bisa jadi HTTP request atau database query
    tokio::time::sleep(Duration::from_millis(100)).await;
    n * n
}

// ── SEQUENTIAL vs CONCURRENT ────────────────────────────────

async fn sequential_demo() {
    println!("\n=== Sequential (satu per satu) ===");
    let start = std::time::Instant::now();

    // Ini berjalan secara BERURUTAN — lambat!
    let a = hitung_lambat(1).await;
    let b = hitung_lambat(2).await;
    let c = hitung_lambat(3).await;

    println!("Hasil: {}, {}, {}", a, b, c);
    println!("Waktu: {:?}", start.elapsed());
}

async fn concurrent_demo() {
    println!("\n=== Concurrent (bersamaan) ===");
    let start = std::time::Instant::now();

    // tokio::join! menjalankan futures secara BERSAMAAN!
    let (a, b, c) = tokio::join!(
        hitung_lambat(1),
        hitung_lambat(2),
        hitung_lambat(3),
    );

    println!("Hasil: {}, {}, {}", a, b, c);
    println!("Waktu: {:?}", start.elapsed());
    // ~100ms, bukan ~300ms!
}

// ── ASYNC DENGAN ERROR HANDLING ─────────────────────────────
async fn fetch_data(url: &str) -> Result<String, String> {
    // Simulasi HTTP request
    tokio::time::sleep(Duration::from_millis(50)).await;

    if url.starts_with("https") {
        Ok(format!("Data dari {}", url))
    } else {
        Err(format!("URL tidak aman: {}", url))
    }
}

async fn proses_data() {
    println!("\n=== Async Error Handling ===");

    // Menggunakan ? di async function
    match fetch_data("https://api.example.com").await {
        Ok(data) => println!("Berhasil: {}", data),
        Err(e) => println!("Error: {}", e),
    }

    match fetch_data("http://insecure.com").await {
        Ok(data) => println!("Berhasil: {}", data),
        Err(e) => println!("Error: {}", e),
    }
}

// ── ASYNC DENGAN TOKIO::SPAWN ───────────────────────────────
async fn spawn_demo() {
    println!("\n=== Tokio Spawn (Task) ===");

    // tokio::spawn membuat task baru (seperti thread tapi lebih ringan)
    let handle1 = tokio::spawn(async {
        tokio::time::sleep(Duration::from_millis(100)).await;
        println!("  Task 1 selesai");
        1
    });

    let handle2 = tokio::spawn(async {
        tokio::time::sleep(Duration::from_millis(50)).await;
        println!("  Task 2 selesai");
        2
    });

    let handle3 = tokio::spawn(async {
        println!("  Task 3 selesai (cepat)");
        3
    });

    // Tunggu semua task selesai dan kumpulkan hasilnya
    let r1 = handle1.await.unwrap();
    let r2 = handle2.await.unwrap();
    let r3 = handle3.await.unwrap();
    println!("Hasil tasks: {}, {}, {}", r1, r2, r3);
}

// ── ASYNC STREAM (PRODUCER-CONSUMER) ────────────────────────
async fn producer(tx: tokio::sync::mpsc::Sender<i32>) {
    for i in 1..=5 {
        println!("  Producing: {}", i);
        tx.send(i).await.unwrap();
        tokio::time::sleep(Duration::from_millis(50)).await;
    }
}

async fn consumer(mut rx: tokio::sync::mpsc::Receiver<i32>) {
    while let Some(val) = rx.recv().await {
        println!("  Consumed: {} (kuadrat: {})", val, val * val);
    }
}

async fn channel_demo() {
    println!("\n=== Async Channel ===");
    let (tx, rx) = tokio::sync::mpsc::channel(32);

    let producer_task = tokio::spawn(producer(tx));
    let consumer_task = tokio::spawn(consumer(rx));

    producer_task.await.unwrap();
    consumer_task.await.unwrap();
}

// ── SELECT: MENUNGGU SALAH SATU FUTURE ──────────────────────
async fn select_demo() {
    println!("\n=== Tokio Select ===");

    // select! menunggu SALAH SATU future yang selesai duluan
    tokio::select! {
        _ = tokio::time::sleep(Duration::from_millis(100)) => {
            println!("  Timer 100ms selesai duluan");
        }
        _ = tokio::time::sleep(Duration::from_millis(200)) => {
            println!("  Timer 200ms selesai duluan");
        }
    }
}

// ── TIMEOUT ─────────────────────────────────────────────────
async fn timeout_demo() {
    println!("\n=== Timeout ===");

    // Operasi yang terlalu lama bisa di-timeout
    let result = tokio::time::timeout(
        Duration::from_millis(50),
        hitung_lambat(42), // ini butuh 100ms
    )
    .await;

    match result {
        Ok(val) => println!("  Berhasil: {}", val),
        Err(_) => println!("  Timeout! Operasi terlalu lama"),
    }

    // Operasi yang cukup cepat
    let result2 = tokio::time::timeout(
        Duration::from_millis(200),
        hitung_lambat(7),
    )
    .await;

    match result2 {
        Ok(val) => println!("  Berhasil: {}", val),
        Err(_) => println!("  Timeout!"),
    }
}

// ── ASYNC TRAIT (membutuhkan crate async-trait atau Rust 1.75+) ──
// Sejak Rust 1.75, async fn bisa langsung di trait!
trait DataStore {
    async fn get(&self, key: &str) -> Option<String>;
    async fn set(&mut self, key: &str, value: &str);
}

struct InMemoryStore {
    data: std::collections::HashMap<String, String>,
}

impl DataStore for InMemoryStore {
    async fn get(&self, key: &str) -> Option<String> {
        tokio::time::sleep(Duration::from_millis(10)).await;
        self.data.get(key).cloned()
    }

    async fn set(&mut self, key: &str, value: &str) {
        tokio::time::sleep(Duration::from_millis(10)).await;
        self.data.insert(key.to_string(), value.to_string());
    }
}

// ══════════════════════════════════════════════════════════════
// MAIN — Tokio Runtime
// ══════════════════════════════════════════════════════════════

#[tokio::main] // macro ini membuat tokio runtime
async fn main() {
    println!("=== Async/Await di Rust ===");

    // Dasar
    let pesan = sapa("Fadlur").await;
    println!("{}", pesan);

    // Sequential vs Concurrent
    sequential_demo().await;
    concurrent_demo().await;

    // Error handling
    proses_data().await;

    // Spawn tasks
    spawn_demo().await;

    // Channel
    channel_demo().await;

    // Select
    select_demo().await;

    // Timeout
    timeout_demo().await;

    // Async trait
    println!("\n=== Async Trait ===");
    let mut store = InMemoryStore {
        data: std::collections::HashMap::new(),
    };
    store.set("nama", "Fadlur").await;
    store.set("bahasa", "Rust").await;
    println!("nama = {:?}", store.get("nama").await);
    println!("bahasa = {:?}", store.get("bahasa").await);
    println!("kota = {:?}", store.get("kota").await);
}

// ============================================================
// 🧠 RINGKUMAN ASYNC:
//
// ┌─────────────────────────────────────────────────────────────┐
// │                    KONSEP ASYNC                             │
// ├──────────────────┬──────────────────────────────────────────┤
// │ async fn         │ Mengembalikan Future (lazy)              │
// │ .await           │ Menunggu Future selesai (non-blocking)   │
// │ tokio::join!     │ Jalankan futures bersamaan               │
// │ tokio::spawn     │ Jalankan task di background              │
// │ tokio::select!   │ Tunggu salah satu future                 │
// │ tokio::time::timeout │ Batas waktu operasi                  │
// │ Channel          │ Komunikasi antar tasks                   │
// └──────────────────┴──────────────────────────────────────────┘
//
// ┌─────────────────────────────────────────────────────────────┐
// │                    ASYNC vs SYNC                            │
// ├──────────────────┬──────────────────┬───────────────────────┤
// │                  │ Sync             │ Async                 │
// ├──────────────────┼──────────────────┼───────────────────────┤
// │ Blocking         │ Ya               │ Tidak                 │
// │ Thread usage     │ 1 per operasi    │ Banyak per thread     │
// │ Cocok untuk      │ CPU-bound        │ I/O-bound             │
// │ Kompleksitas     │ Sederhana        │ Lebih kompleks        │
// │ Memory overhead  │ Stack besar      │ Stack kecil (task)    │
// └──────────────────┴──────────────────┴───────────────────────┘
//
// ⚠️ COMMON MISTAKES:
// - Menjalankan async fn tanpa .await → Future tidak jalan!
// - Blocking call di async (std::thread::sleep) → blok semua task
// - Lupa tokio::main atau runtime lain
// - Mix sync dan async tanpa spawn_blocking
// - Deadlock di async (rare tapi mungkin)
//
// 🔗 PERBANDINGAN:
// | Rust (async/await) | Go (goroutine)   | JavaScript        |
// |--------------------|------------------|-------------------|
// | async fn           | func (implicit)  | async function    │
// | .await             | (implicit)       | await             │
// | tokio::spawn       | go func()        | (event loop)      │
// | Future             | (no explicit)    | Promise           │
// | tokio::join!       | (no explicit)    | Promise.all       │
// | select!            | select           | Promise.race      │
// ============================================================

// ============================================================
// 🏋️ LATIHAN:
// 1. Buat async function yang "download" 5 halaman secara concurrent
//    (simulasi dengan sleep random 100-500ms)
// 2. Implementasikan rate limiter: max 3 request per detik
// 3. Buat async web scraper sederhana (simulasi)
// 4. Implementasi retry logic: coba ulang operasi yang gagal
//    (max 3 kali, dengan exponential backoff)
// 5. Buat chat system sederhana menggunakan async channels
// 6. Bandingkan waktu sequential vs concurrent untuk 10 operasi
// ============================================================
