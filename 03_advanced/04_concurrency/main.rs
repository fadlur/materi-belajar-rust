// ============================================================
// 📙 BELAJAR RUST #20 — Concurrency (Thread, Mutex, Channel)
// ============================================================
// Rust menjamin "fearless concurrency" — banyak bug concurrency
// ditangkap saat COMPILE TIME, bukan runtime!
// Ini berkat ownership system dan type system Rust.
//
// 🎯 Tujuan: Memahami thread, message passing, shared state,
//    dan patterns concurrency di Rust.
//
// 💡 Analogi Utama:
// Thread seperti PEKERJA di pabrik — setiap pekerja (thread)
//    bisa mengerjakan tugas secara paralel. Tapi kalau mereka
//    perlu berbagi alat (data), harus ada aturan agar tidak
//    berebut atau merusak alat.
//
// 🔑 Rust mencegah data race saat COMPILE TIME melalui
//    ownership dan borrowing — tidak perlu khawatir race
//    condition yang sulit di-debug!
// ============================================================

use std::sync::{Arc, Mutex, mpsc};
use std::thread;
use std::time::Duration;

fn main() {
    // ════════════════════════════════════════════════════════
    // THREAD — Unit Eksekusi Paralel
    // ════════════════════════════════════════════════════════

    // ── Membuat Thread ──────────────────────────────────────
    // thread::spawn menerima closure dan menjalankannya di thread baru
    let handle = thread::spawn(|| {
        for i in 1..=5 {
            println!("Thread anak: {}", i);
            thread::sleep(Duration::from_millis(100));
        }
    });

    // Main thread juga berjalan bersamaan
    for i in 1..=3 {
        println!("Main thread: {}", i);
        thread::sleep(Duration::from_millis(150));
    }

    // .join() menunggu thread selesai — BLOCKING!
    handle.join().unwrap();
    println!("Semua thread selesai!\n");

    // ── MOVE Closure untuk Thread ───────────────────────────
    // Thread bisa hidup lebih lama dari scope pembuatnya,
    // jadi data harus di-MOVE ke dalam thread.
    let pesan = String::from("Halo dari main!");

    let handle2 = thread::spawn(move || {
        // `move` memindahkan ownership `pesan` ke thread ini
        println!("Thread berkata: {}", pesan);
    });
    // println!("{}", pesan); // ❌ ERROR! pesan sudah di-move
    handle2.join().unwrap();

    // ── Multiple Threads ────────────────────────────────────
    let mut handles = vec![];

    for i in 0..5 {
        let handle = thread::spawn(move || {
            println!("Thread {} mulai", i);
            thread::sleep(Duration::from_millis(100));
            println!("Thread {} selesai", i);
            i * i // return value dari thread
        });
        handles.push(handle);
    }

    // Kumpulkan hasil dari semua thread
    let mut hasil = vec![];
    for h in handles {
        let val = h.join().unwrap();
        hasil.push(val);
    }
    println!("Hasil threads: {:?}\n", hasil);

    // ════════════════════════════════════════════════════════
    // CHANNEL — Komunikasi Antar Thread (Message Passing)
    // ════════════════════════════════════════════════════════

    // mpsc = Multiple Producer, Single Consumer
    // Seperti walkie-talkie: banyak pengirim, satu penerima
    //
    // 💡 Analogi: Channel seperti pipa — thread pengirim (tx)
    //    memasukkan data di satu ujung, thread penerima (rx)
    //    mengambil data di ujung lain.

    // ── Channel Dasar ───────────────────────────────────────
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let pesan = String::from("Halo dari thread!");
        tx.send(pesan).unwrap();
        // println!("{}", pesan); // ❌ pesan sudah di-move oleh send()
    });

    // recv() blocking — tunggu sampai ada pesan
    let diterima = rx.recv().unwrap();
    println!("Diterima: {}\n", diterima);

    // ── Kirim Beberapa Pesan ────────────────────────────────
    let (tx2, rx2) = mpsc::channel();

    thread::spawn(move || {
        let pesan_list = vec![
            String::from("halo"),
            String::from("dari"),
            String::from("thread"),
            String::from("lain"),
        ];

        for p in pesan_list {
            tx2.send(p).unwrap();
            thread::sleep(Duration::from_millis(100));
        }
    });

    // rx sebagai iterator — otomatis berhenti saat channel ditutup
    for pesan in rx2 {
        println!("Diterima: {}", pesan);
    }
    println!();

    // ── Multiple Producers ──────────────────────────────────
    let (tx3, rx3) = mpsc::channel();

    for i in 0..3 {
        let tx_clone = tx3.clone(); // clone transmitter untuk setiap thread
        thread::spawn(move || {
            let pesan = format!("Pesan dari producer {}", i);
            tx_clone.send(pesan).unwrap();
        });
    }
    drop(tx3); // drop original tx agar channel bisa ditutup

    for pesan in rx3 {
        println!("{}", pesan);
    }
    println!();

    // ════════════════════════════════════════════════════════
    // MUTEX — Shared State (Mutual Exclusion)
    // ════════════════════════════════════════════════════════

    // Mutex memastikan hanya SATU thread yang bisa akses data pada satu waktu
    // .lock() → mendapatkan akses (blocking jika thread lain sedang pakai)
    // Lock otomatis dilepas saat MutexGuard keluar scope
    //
    // 💡 Analogi: Mutex seperti kunci toilet — hanya satu orang
    //    yang bisa masuk pada satu waktu. Orang lain harus menunggu
    //    di luar sampai kunci dilepas.

    // ── Mutex Dasar (single thread) ─────────────────────────
    let m = Mutex::new(5);

    {
        let mut num = m.lock().unwrap(); // mendapat MutexGuard
        *num = 10;
        println!("Mutex value: {}", *num);
    } // MutexGuard di-drop → lock dilepas

    println!("Mutex setelah: {:?}\n", m);

    // ── Arc + Mutex (multi-thread) ──────────────────────────
    // Arc = Atomic Reference Counting (thread-safe version of Rc)
    // Rc TIDAK thread-safe! Harus pakai Arc untuk multi-thread
    //
    // 💡 Pattern: Arc<Mutex<T>> adalah combo paling umum untuk
    //    shared mutable state di Rust.

    let counter = Arc::new(Mutex::new(0));
    let mut handles2 = vec![];

    for _ in 0..10 {
        let counter_clone = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter_clone.lock().unwrap();
            *num += 1;
        });
        handles2.push(handle);
    }

    for h in handles2 {
        h.join().unwrap();
    }

    println!("Counter akhir: {}\n", *counter.lock().unwrap());

    // ── Contoh Praktis: Parallel Sum ────────────────────────
    let data = Arc::new(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    let total = Arc::new(Mutex::new(0));
    let mut handles3 = vec![];

    // Bagi data ke beberapa thread
    let chunk_size = 3;
    let num_chunks = (data.len() + chunk_size - 1) / chunk_size;

    for i in 0..num_chunks {
        let data_clone = Arc::clone(&data);
        let total_clone = Arc::clone(&total);
        let start = i * chunk_size;

        let handle = thread::spawn(move || {
            let end = (start + chunk_size).min(data_clone.len());
            let partial_sum: i32 = data_clone[start..end].iter().sum();

            println!(
                "Thread {}: sum of {:?} = {}",
                i,
                &data_clone[start..end],
                partial_sum
            );

            let mut total = total_clone.lock().unwrap();
            *total += partial_sum;
        });
        handles3.push(handle);
    }

    for h in handles3 {
        h.join().unwrap();
    }

    println!("Total parallel sum: {}\n", *total.lock().unwrap());

    // ── Channel + Processing Pipeline ───────────────────────
    println!("=== Pipeline ===");
    let (tx_input, rx_input) = mpsc::channel();
    let (tx_output, rx_output) = mpsc::channel();

    // Stage 1: Generator
    thread::spawn(move || {
        for i in 1..=5 {
            tx_input.send(i).unwrap();
            thread::sleep(Duration::from_millis(50));
        }
    });

    // Stage 2: Processor (kuadratkan)
    thread::spawn(move || {
        for val in rx_input {
            let processed = val * val;
            println!("  Processing: {} → {}", val, processed);
            tx_output.send(processed).unwrap();
        }
    });

    // Stage 3: Collector
    let mut results = vec![];
    for val in rx_output {
        results.push(val);
    }
    println!("Pipeline results: {:?}", results);
}

// ============================================================
// 🧠 RINGKUMAN CONCURRENCY:
//
// ┌─────────────────────────────────────────────────────────────┐
// │                    MESSAGE PASSING vs SHARED STATE          │
// ├──────────────────┬──────────────────┬───────────────────────┤
// │                  │ Channel          │ Mutex                 │
// ├──────────────────┼──────────────────┼───────────────────────┤
// │ Konsep           │ Kirim data       │ Kunci akses           │
// │ Copy data?       │ Ya (move)        │ Tidak (shared)        │
// │ Race condition?  │ Tidak mungkin    │ Dicek runtime         │
// │ Deadlock?        │ Jarang           │ Mungkin               │
// │ Use case         │ Pipeline, worker │ Counter, cache        │
// └──────────────────┴──────────────────┴───────────────────────┘
//
// 💡 Rule of thumb: "Do not communicate by sharing memory;
//    share memory by communicating." — tapi Rust support keduanya!
//
// ┌─────────────────────────────────────────────────────────────┐
// │                    THREAD-SAFE TYPES                        │
// ├──────────────────┬──────────────────────────────────────────┤
// │ Arc<T>           │ Thread-safe Rc (reference counting)      │
// │ Mutex<T>         │ Mutual exclusion (single lock)           │
// │ RwLock<T>        │ Read-write lock (banyak reader)          │
// │ mpsc::channel    │ Message passing channel                  │
// └──────────────────┴──────────────────────────────────────────┘
//
// ⚠️ COMMON MISTAKES:
// - Rc di multi-thread → compile error, gunakan Arc
// - RefCell di multi-thread → compile error, gunakan Mutex
// - Deadlock: lock A → lock B vs lock B → lock A
// - Lupa .join() → thread mungkin belum selesai
// - Move data yang masih dipakai setelah spawn
//
// 🔗 PERBANDINGAN:
// | Rust              | Go               | Java              |
// |-------------------|------------------|-------------------|
// | thread::spawn     | go func()        | new Thread()      |
// | mpsc::channel     | channel          | BlockingQueue     |
// | Mutex<T>          | sync.Mutex       | synchronized      |
// | Arc<T>            | (GC-managed)     | (GC-managed)      |
// | .join()           | (no explicit)    | .join()           |
// ============================================================

// ============================================================
// 🏋️ LATIHAN:
// 1. Buat program multi-thread yang mendownload "halaman web"
//    (simulasi dengan sleep) secara paralel
// 2. Implementasikan thread pool sederhana
// 3. Buat producer-consumer pattern: 3 producer, 1 consumer
// 4. Buat shared HashMap yang bisa diakses dari multiple threads
// 5. Implementasikan parallel map: vec.par_map(|x| x * 2)
// 6. Buat program yang menghitung jumlah kata di beberapa file
//    secara paralel menggunakan thread
// ============================================================
