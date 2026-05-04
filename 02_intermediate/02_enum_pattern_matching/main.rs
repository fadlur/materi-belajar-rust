// ============================================================
// 📘 BELAJAR RUST #10 — Enum & Pattern Matching
// ============================================================
// Enum di Rust JAUH lebih powerful dari enum di bahasa lain.
// Setiap variant bisa menyimpan data dengan tipe berbeda!
// Digabung dengan `match`, ini jadi salah satu fitur terkuat Rust.
//
// 🎯 Tujuan: Memahami enum dengan data, pattern matching,
//    Option<T>, Result<T,E>, dan berbagai teknik pattern matching.
//
// 💡 Analogi Utama:
// Enum seperti KOTAK SURAT dengan beberapa slot — setiap slot
// bisa berisi barang berbeda. Slot "Surat" berisi kertas,
// slot "Paket" berisi kotak, slot "Kosong" tidak berisi apa-apa.
//
// Di Rust, enum tidak hanya label — setiap variant bisa
// menyimpan data lengkap dengan tipe yang berbeda-beda!
// ============================================================

// ── ENUM SEDERHANA ──────────────────────────────────────────
// Enum dasar — mirip enum di bahasa lain (C, Java)
// Setiap variant adalah label tanpa data.
#[derive(Debug)]
enum Arah {
    Utara,
    Selatan,
    Timur,
    Barat,
}

// ── ENUM DENGAN DATA ────────────────────────────────────────
// Setiap variant bisa punya tipe data berbeda!
//
// 💡 Analogi: Bayangkan pesan dalam game — ada banyak jenis pesan,
//    masing-masing butuh data berbeda:
//    - "Keluar" → tidak perlu data
//    - "Pindah" → butuh koordinat x, y
//    - "Tulis" → butuh teks
//    - "GantiWarna" → butuh RGB
#[derive(Debug)]
enum Pesan {
    Keluar,                          // tanpa data (unit variant)
    Pindah { x: i32, y: i32 },      // named fields (struct-like)
    Tulis(String),                   // satu field (tuple-like)
    GantiWarna(u8, u8, u8),         // multiple fields (tuple-like)
}

// Enum juga bisa punya method!
impl Pesan {
    fn proses(&self) {
        match self {
            Pesan::Keluar => println!("Perintah keluar diterima"),
            Pesan::Pindah { x, y } => println!("Pindah ke ({}, {})", x, y),
            Pesan::Tulis(teks) => println!("Pesan: {}", teks),
            Pesan::GantiWarna(r, g, b) => {
                println!("Warna baru: RGB({}, {}, {})", r, g, b)
            }
        }
    }
}

// ── ENUM UNTUK STATE MACHINE ────────────────────────────────
// Enum dengan data sangat cocok untuk state machine —
// menggambarkan status sistem dengan data terkait.
//
// 💡 Analogi: Status pesanan di toko online — setiap status
//    berbeda dan mungkin butuh informasi tambahan.
#[derive(Debug)]
enum StatusPesanan {
    Baru,
    Diproses,
    Dikirim { nomor_resi: String },
    Selesai,
    Dibatalkan { alasan: String },
}

impl StatusPesanan {
    fn deskripsi(&self) -> &str {
        match self {
            StatusPesanan::Baru => "Pesanan baru dibuat",
            StatusPesanan::Diproses => "Sedang diproses",
            StatusPesanan::Dikirim { .. } => "Sedang dalam pengiriman",
            StatusPesanan::Selesai => "Pesanan selesai",
            StatusPesanan::Dibatalkan { .. } => "Pesanan dibatalkan",
        }
    }
}

// ── ENUM SEPERTI C-STYLE (DENGAN NILAI) ────────────────────
// Enum bisa punya nilai integer eksplisit — berguna untuk
// interoperabilitas dengan C atau definisi konstanta.
#[derive(Debug)]
enum HttpStatus {
    Ok = 200,
    NotFound = 404,
    InternalServerError = 500,
}

fn main() {
    // ── MENGGUNAKAN ENUM ────────────────────────────────────
    let arah = Arah::Utara;
    println!("Arah: {:?}", arah);

    // ── MATCH — HARUS EXHAUSTIVE! ───────────────────────────
    // Semua variant HARUS ditangani — compiler akan error kalau ada yang miss!
    //
    // 💡 Analogi: Match seperti mesin sortir otomatis — setiap
    //    paket HARUS masuk ke slot yang sesuai. Kalau ada paket
    //    tanpa slot, mesin BERHENTI (compile error).
    //
    // 🔑 Ini PREVENTS BUGS! Kalau nanti enum ditambah variant baru,
    //    compiler akan kasih tahu semua match yang perlu diperbarui.
    let instruksi = match arah {
        Arah::Utara => "Maju ke utara",
        Arah::Selatan => "Mundur ke selatan",
        Arah::Timur => "Belok ke timur",
        Arah::Barat => "Belok ke barat",
    };
    println!("Instruksi: {}", instruksi);

    // ── ENUM DENGAN DATA ────────────────────────────────────
    let pesan1 = Pesan::Tulis(String::from("Halo Rust!"));
    let pesan2 = Pesan::Pindah { x: 10, y: 20 };
    let pesan3 = Pesan::GantiWarna(255, 128, 0);
    let pesan4 = Pesan::Keluar;

    pesan1.proses();
    pesan2.proses();
    pesan3.proses();
    pesan4.proses();

    // ── OPTION<T> — ENUM BAWAAN RUST ────────────────────────
    // Rust TIDAK punya null! Sebagai gantinya, ada Option<T>:
    //
    // enum Option<T> {
    //     Some(T),   // ada nilai
    //     None,      // tidak ada nilai
    // }
    //
    // 💡 Analogi: Option seperti kotak yang bisa berisi barang
    //    atau kosong. Kalau mau ambil barang, kamu HARUS cek
    //    dulu apakah kotaknya kosong — tidak bisa asal ambil!
    //
    // 🔑 Keunggulan: Compiler memaksa kita menangani kasus None.
    //    Tidak ada "null pointer exception" di Rust!

    let angka: Option<i32> = Some(42);
    let kosong: Option<i32> = None;

    println!("angka: {:?}", angka);
    println!("kosong: {:?}", kosong);

    // Mengambil nilai dari Option dengan match
    match angka {
        Some(n) => println!("Nilainya: {}", n),
        None => println!("Tidak ada nilai"),
    }

    // Method-method berguna pada Option
    println!("unwrap_or: {}", kosong.unwrap_or(0)); // default jika None
    println!("is_some: {}", angka.is_some());
    println!("is_none: {}", kosong.is_none());

    // map — transformasi nilai di dalam Some
    let double = angka.map(|n| n * 2);
    println!("Double: {:?}", double); // Some(84)

    // and_then — chain Option (flatmap)
    let result = angka.and_then(|n| {
        if n > 0 { Some(n.to_string()) } else { None }
    });
    println!("and_then: {:?}", result);

    // unwrap_or_else — dengan closure untuk default
    let val = kosong.unwrap_or_else(|| {
        println!("Menghitung default...");
        99
    });
    println!("unwrap_or_else: {}", val);

    // ── RESULT<T, E> — ENUM UNTUK ERROR HANDLING ────────────
    // enum Result<T, E> {
    //     Ok(T),    // berhasil, berisi nilai
    //     Err(E),   // gagal, berisi error
    // }
    //
    // 💡 Analogi: Result seperti amplop — ada amplop "Sukses" (hijau)
    //    dan amplop "Gagal" (merah). Kamu harus buka amplop untuk
    //    tahu isinya — dan compiler memastikan kamu menangani kedua
    //    jenis amplop!

    let berhasil: Result<i32, String> = Ok(42);
    let gagal: Result<i32, String> = Err(String::from("ada error"));

    match &berhasil {
        Ok(val) => println!("Berhasil: {}", val),
        Err(e) => println!("Error: {}", e),
    }

    match &gagal {
        Ok(val) => println!("Berhasil: {}", val),
        Err(e) => println!("Error: {}", e),
    }

    // ── PATTERN MATCHING LANJUTAN ───────────────────────────

    // Match dengan guard (kondisi tambahan)
    let angka2 = 15;
    let kategori = match angka2 {
        n if n < 0 => "negatif",
        0 => "nol",
        n if n <= 10 => "kecil",
        n if n <= 100 => "sedang",
        _ => "besar",
    };
    println!("{} adalah {}", angka2, kategori);

    // Match dengan binding (@)
    let umur = 25;
    match umur {
        n @ 0..=12 => println!("Anak-anak (umur {})", n),
        n @ 13..=17 => println!("Remaja (umur {})", n),
        n @ 18..=64 => println!("Dewasa (umur {})", n),
        n => println!("Lansia (umur {})", n),
    }

    // Destructuring dalam match
    let titik = (3, -5);
    match titik {
        (0, 0) => println!("Di origin"),
        (x, 0) => println!("Di sumbu X, x={}", x),
        (0, y) => println!("Di sumbu Y, y={}", y),
        (x, y) if x > 0 && y > 0 => println!("Kuadran I ({}, {})", x, y),
        (x, y) if x < 0 && y > 0 => println!("Kuadran II ({}, {})", x, y),
        (x, y) if x < 0 && y < 0 => println!("Kuadran III ({}, {})", x, y),
        (x, y) => println!("Kuadran IV ({}, {})", x, y),
    }

    // ── IF LET & WHILE LET ─────────────────────────────────
    // Shortcut saat hanya peduli satu pattern
    //
    // 💡 Analogi: Kalau kamu hanya peduli amplop hijau dan
    //    tidak peduli amplop merah, gunakan if let — lebih singkat!

    let mungkin_nama: Option<String> = Some(String::from("Budi"));
    if let Some(nama) = mungkin_nama {
        println!("Nama: {}", nama);
    }
    // Lebih singkat dari match saat hanya butuh satu case

    // ── LET ELSE (Rust 1.65+) ──────────────────────────────
    // Kebalikan if let — handle kasus "gagal" lalu lanjut
    let config_value: Option<&str> = Some("production");
    let Some(env) = config_value else {
        println!("Tidak ada config!");
        return; // HARUS diverge (return, break, panic!, dll)
    };
    println!("Environment: {}", env);

    // ── STATUS PESANAN ──────────────────────────────────────
    let pesanan = StatusPesanan::Dikirim {
        nomor_resi: String::from("JNE-12345"),
    };
    println!("Status: {}", pesanan.deskripsi());

    if let StatusPesanan::Dikirim { nomor_resi } = &pesanan {
        println!("Resi: {}", nomor_resi);
    }

    // ── C-STYLE ENUM ────────────────────────────────────────
    println!("HTTP 200 = {}", HttpStatus::Ok as i32);
    println!("HTTP 404 = {}", HttpStatus::NotFound as i32);

    // ── CONTOH PRAKTIS: KALKULATOR ──────────────────────────
    let operasi = vec![
        Operasi::Tambah(10.0),
        Operasi::Kali(3.0),
        Operasi::Kurang(5.0),
        Operasi::Bagi(5.0),
    ];

    let mut hasil = 0.0_f64;
    for op in &operasi {
        hasil = op.terapkan(hasil);
        println!("{:?} → hasil = {}", op, hasil);
    }
}

#[derive(Debug)]
enum Operasi {
    Tambah(f64),
    Kurang(f64),
    Kali(f64),
    Bagi(f64),
}

impl Operasi {
    fn terapkan(&self, nilai: f64) -> f64 {
        match self {
            Operasi::Tambah(n) => nilai + n,
            Operasi::Kurang(n) => nilai - n,
            Operasi::Kali(n) => nilai * n,
            Operasi::Bagi(n) => {
                if *n == 0.0 {
                    eprintln!("Tidak bisa bagi dengan 0!");
                    nilai
                } else {
                    nilai / n
                }
            }
        }
    }
}

// ============================================================
// 🧠 RINGKUMAN ENUM & PATTERN MATCHING:
//
// ┌─────────────────────────────────────────────────────────────┐
// │                    JENIS VARIANT ENUM                       │
// ├──────────────────┬──────────────────────────────────────────┤
// │ Unit variant     │ Enum::Variant (tanpa data)               │
// │ Tuple variant    │ Enum::Variant(T1, T2)                    │
// │ Struct variant   │ Enum::Variant { field: T }               │
// └──────────────────┴──────────────────────────────────────────┘
//
// ┌─────────────────────────────────────────────────────────────┐
// │                    OPTION<T> & RESULT<T,E>                  │
// ├──────────────────┬──────────────────┬───────────────────────┤
// │                  │ Option<T>        │ Result<T, E>          │
// ├──────────────────┼──────────────────┼───────────────────────┤
// │ Sukses           │ Some(T)          │ Ok(T)                 │
// │ Gagal            │ None             │ Err(E)                │
// │unwrap_or(default)│ unwrap_or(T)     │ unwrap_or(T)          │
// │unwrap_or_else    │ unwrap_or_else   │ unwrap_or_else        │
// │map                │ map(|t| ...)     │ map(|t| ...)          │
// └──────────────────┴──────────────────┴───────────────────────┘
//
// ⚠️ COMMON MISTAKES:
// - unwrap() tanpa handle Err/None → panic!
// - Match tidak exhaustive → compile error (bagus!)
// - Lupa `ref` atau `&` saat pattern match reference
// - Asumsi Option selalu Some → gunakan match atau if let!
//
// 🔗 PERBANDINGAN NULL SAFETY:
// | Rust (Option)     | Java (Optional)   | TypeScript        |
// |-------------------|-------------------|-------------------|
// | Some/None         | Optional.of/null  │ T | null/undefined|
// | Compile-time check│ Runtime check     │ Compile (opt)     |
// | match mandatory   │ ifPresent()       │ ?. operator       |
// ============================================================

// ============================================================
// 🏋️ LATIHAN:
// 1. Buat enum `Bentuk` dengan Circle(radius), Rectangle(w,h),
//    Triangle(base, height) dan method `luas()`
// 2. Buat enum `Json` yang bisa menyimpan Null, Bool, Number,
//    Str, Array, Object — mirip JSON value
// 3. Buat state machine untuk mesin ATM dengan enum
// 4. Implementasikan linked list sederhana menggunakan enum
// 5. Buat fungsi yang menerima Vec<Option<i32>> dan return
//    jumlah semua Some values (skip None)
// 6. Gunakan let else untuk unwrap Option<&str>
// 7. Buat enum Message dengan method broadcast()
// ============================================================
