// ============================================================
// 📘 BELAJAR RUST #12 — Error Handling
// ============================================================
// Rust TIDAK punya exceptions! Sebagai gantinya:
// - Recoverable errors → Result<T, E>
// - Unrecoverable errors → panic!
//
// 🎯 Tujuan: Memahami filosofi error handling Rust dan
//    menguasai Result<T,E>, Option<T>, operator ?, dan
//    custom error types.
//
// 💡 Filosofi Rust: Error harus ditangani secara EKSPLISIT.
//    Kalau fungsi bisa gagal, tipe return-nya HARUS menunjukkan
//    kemungkinan gagal — tidak bisa "diam-diam" gagal seperti
//    null pointer exception di bahasa lain.
//
// 🔑 Analogi: Result<T,E> seperti amplop berisi hasil —
//    amploknya ada stiker "Sukses" (Ok) atau "Gagal" (Err).
//    Penerima HARUS buka amplop dan cek stiker-nya —
//    tidak bisa asal ambil isi!
// ============================================================

use std::fs;
use std::io;
use std::num::ParseIntError;

fn main() {
    // ════════════════════════════════════════════════════════
    // PANIC! — Error yang Tidak Bisa Dipulihkan
    // ════════════════════════════════════════════════════════

    // panic! langsung menghentikan program dengan pesan error
    // panic!("Ini crash!"); // uncomment untuk lihat

    // Beberapa operasi bisa menyebabkan panic secara implisit:
    // let v = vec![1, 2, 3];
    // v[99]; // ❌ panic: index out of bounds
    //
    // 💡 Tips: Gunakan .get() untuk menghindari panic saat indexing!

    // ════════════════════════════════════════════════════════
    // RESULT<T, E> — Error yang Bisa Dipulihkan
    // ════════════════════════════════════════════════════════

    // ── Match pada Result ───────────────────────────────────
    let angka_str = "42";
    let hasil: Result<i32, ParseIntError> = angka_str.parse();

    match hasil {
        Ok(n) => println!("Berhasil parse: {}", n),
        Err(e) => println!("Gagal parse: {}", e),
    }

    // Parse yang gagal
    let gagal_str = "bukan_angka";
    match gagal_str.parse::<i32>() {
        Ok(n) => println!("Berhasil: {}", n),
        Err(e) => println!("Error: {} (tipe: {:?})", e, e),
    }

    // ── unwrap() dan expect() ───────────────────────────────
    // unwrap(): ambil nilai Ok, PANIC jika Err
    //
    // ⚠️ Gunakan hanya saat kamu YAKIN tidak akan Err!
    //    Kalau Err, program CRASH — jangan dipakai di production.
    let pasti_angka: i32 = "123".parse().unwrap();
    println!("unwrap: {}", pasti_angka);

    // expect(): seperti unwrap tapi dengan pesan error custom
    // Berguna untuk debugging — pesan custom membantu identifikasi sumber error.
    let pasti_angka2: i32 = "456".parse().expect("Harusnya bisa di-parse!");
    println!("expect: {}", pasti_angka2);

    // ⚠️ Jangan pakai unwrap/expect di production code (kecuali memang yakin)!
    // Gunakan proper error handling dengan match atau `?`

    // ── unwrap_or, unwrap_or_else, unwrap_or_default ────────
    // Cara aman mendapatkan nilai dengan default kalau Err
    let val1: Result<i32, &str> = Err("error");
    println!("unwrap_or: {}", val1.unwrap_or(0)); // default 0 jika Err

    let val2: Result<i32, &str> = Err("error");
    println!("unwrap_or_else: {}", val2.unwrap_or_else(|e| {
        println!("  Error terjadi: {}", e);
        -1
    }));

    let val3: Result<i32, &str> = Err("error");
    println!("unwrap_or_default: {}", val3.unwrap_or_default()); // 0 untuk i32

    // ── map, and_then, or_else ──────────────────────────────
    // Method untuk chaining/transformasi Result
    let parsed: Result<i32, _> = "10".parse::<i32>();

    // map: transformasi nilai Ok (Err tetap Err)
    let doubled = parsed.map(|n| n * 2);
    println!("Doubled: {:?}", doubled); // Ok(20)

    // and_then: chain Result (flatmap) — return Result baru
    let chained = "5".parse::<i32>().and_then(|n| {
        if n > 0 {
            Ok(n * 10)
        } else {
            Err("harus positif".parse().unwrap())
        }
    });
    println!("Chained: {:?}", chained);

    // ── Membaca File (Contoh Real-World) ────────────────────
    match fs::read_to_string("/etc/hostname") {
        Ok(isi) => println!("Hostname: {}", isi.trim()),
        Err(e) => println!("Gagal baca file: {}", e),
    }

    // ══════════════════════════════════════════════════════════
    // OPERATOR `?` — Error Propagation
    // ══════════════════════════════════════════════════════════

    // `?` otomatis return Err jika hasilnya Err, lanjut jika Ok
    // Ini cara PALING IDIOMATIK untuk handle error di Rust!
    //
    // 💡 Analogi: `?` seperti "kalau ada masalah, berhenti dan
    //    laporkan ke atas. Kalau tidak ada masalah, lanjutkan."
    //    Sangat berguna saat ada banyak operasi yang bisa gagal
    //    secara berurutan.

    match baca_dan_parse("42") {
        Ok(n) => println!("baca_dan_parse: {}", n),
        Err(e) => println!("Error: {}", e),
    }

    match baca_dan_parse("bukan angka") {
        Ok(n) => println!("baca_dan_parse: {}", n),
        Err(e) => println!("Error: {}", e),
    }

    // ── Contoh: validasi input ──────────────────────────────
    match validasi_umur("25") {
        Ok(umur) => println!("Umur valid: {}", umur),
        Err(e) => println!("Umur invalid: {}", e),
    }

    match validasi_umur("-5") {
        Ok(umur) => println!("Umur valid: {}", umur),
        Err(e) => println!("Umur invalid: {}", e),
    }

    match validasi_umur("abc") {
        Ok(umur) => println!("Umur valid: {}", umur),
        Err(e) => println!("Umur invalid: {}", e),
    }

    // ══════════════════════════════════════════════════════════
    // CUSTOM ERROR TYPE
    // ══════════════════════════════════════════════════════════
    // Untuk aplikasi/library yang kompleks, buat custom error type
    // agar pemanggil bisa match error spesifik.
    match proses_pesanan("ORD-001", 5) {
        Ok(msg) => println!("Sukses: {}", msg),
        Err(e) => println!("Error pesanan: {}", e),
    }

    match proses_pesanan("", 5) {
        Ok(msg) => println!("Sukses: {}", msg),
        Err(e) => println!("Error pesanan: {}", e),
    }

    match proses_pesanan("ORD-002", 0) {
        Ok(msg) => println!("Sukses: {}", msg),
        Err(e) => println!("Error pesanan: {}", e),
    }

    // ══════════════════════════════════════════════════════════
    // TIPS & BEST PRACTICES
    // ══════════════════════════════════════════════════════════
    // 1. Gunakan Result<T, E> untuk error yang bisa ditangani
    // 2. Gunakan panic! hanya untuk kondisi yang BENAR-BENAR fatal
    // 3. Gunakan ? untuk propagasi error yang bersih
    // 4. Buat custom error type untuk library/aplikasi besar
    // 5. Di main(), bisa return Result untuk handle error global
}

// ── FUNGSI DENGAN `?` OPERATOR ──────────────────────────────
// `?` hanya bisa dipakai di fungsi yang return Result atau Option
//
// 💡 Cara kerja `?`:
//    let x = operasi()?;
//    ↓ diterjemahkan menjadi:
//    let x = match operasi() {
//        Ok(val) => val,
//        Err(e) => return Err(e.into()),
//    };
fn baca_dan_parse(input: &str) -> Result<i32, String> {
    // parse() return Result<i32, ParseIntError>
    // .map_err() konversi tipe error ke String
    let angka = input.parse::<i32>().map_err(|e| format!("Parse error: {}", e))?;
    // Jika parse gagal, langsung return Err(...)
    // Jika berhasil, lanjut ke baris berikutnya

    Ok(angka * 2) // kalikan 2 dan bungkus Ok
}

// ── VALIDASI DENGAN RESULT ──────────────────────────────────
fn validasi_umur(input: &str) -> Result<u32, String> {
    let umur: i32 = input
        .parse()
        .map_err(|_| format!("'{}' bukan angka valid", input))?;

    if umur < 0 {
        return Err(format!("Umur tidak boleh negatif: {}", umur));
    }

    if umur > 150 {
        return Err(format!("Umur tidak realistis: {}", umur));
    }

    Ok(umur as u32)
}

// ── CUSTOM ERROR TYPE ───────────────────────────────────────
// Custom enum untuk error spesifik aplikasi
#[derive(Debug)]
enum PesananError {
    IdKosong,
    JumlahNol,
    StokHabis { tersedia: u32, diminta: u32 },
    IoError(io::Error),
}

// Implement Display agar bisa di-print dengan {}
impl std::fmt::Display for PesananError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PesananError::IdKosong => write!(f, "ID pesanan tidak boleh kosong"),
            PesananError::JumlahNol => write!(f, "Jumlah pesanan harus > 0"),
            PesananError::StokHabis { tersedia, diminta } => {
                write!(f, "Stok tidak cukup: tersedia {}, diminta {}", tersedia, diminta)
            }
            PesananError::IoError(e) => write!(f, "IO Error: {}", e),
        }
    }
}

// Implement From untuk konversi otomatis dengan `?`
// Kalau fungsi return Result<_, PesananError>, kita bisa pakai ?
// pada operasi IO dan error-nya otomatis dikonversi!
impl From<io::Error> for PesananError {
    fn from(e: io::Error) -> Self {
        PesananError::IoError(e)
    }
}

fn proses_pesanan(id: &str, jumlah: u32) -> Result<String, PesananError> {
    if id.is_empty() {
        return Err(PesananError::IdKosong);
    }

    if jumlah == 0 {
        return Err(PesananError::JumlahNol);
    }

    let stok = 10; // simulasi stok
    if jumlah > stok {
        return Err(PesananError::StokHabis {
            tersedia: stok,
            diminta: jumlah,
        });
    }

    Ok(format!("Pesanan {} untuk {} item berhasil diproses", id, jumlah))
}

// ============================================================
// 🧠 RINGKUMAN ERROR HANDLING:
//
// ┌─────────────────────────────────────────────────────────────┐
// │                    ERROR HANDLING HIERARCHY                 │
// ├──────────────────┬──────────────────────────────────────────┤
// │ panic!           │ Fatal, tidak bisa dipulihkan             │
// │                  │ Program dihentikan                       │
// ├──────────────────┼──────────────────────────────────────────┤
// │ unwrap/expect    │ Ambil nilai, panic kalau Err             │
// │                  │ Hanya untuk prototyping/testing          │
// ├──────────────────┼──────────────────────────────────────────┤
// │ unwrap_or/else   │ Ambil nilai dengan default               │
// │                  │ Aman untuk production                    │
// ├──────────────────┼──────────────────────────────────────────┤
// │ match            │ Handle Ok dan Err eksplisit              │
// │                  │ Paling verbose, paling kontrol           │
// ├──────────────────┼──────────────────────────────────────────┤
// │ ? operator       │ Propagasi error secara idiomatik         │
// │                  │ Terbaik untuk chaining operasi           │
// └──────────────────┴──────────────────────────────────────────┘
//
// ⚠️ COMMON MISTAKES:
// - unwrap() di production → program bisa crash!
// - Lupa handle Err di match → compile warning (bagus!)
// - ? di fungsi yang tidak return Result → compile error
// - Custom error tanpa implement Display → tidak bisa print
//
// 🔗 PERBANDINGAN:
// | Rust              | Python           | JavaScript        |
// |-------------------|------------------|-------------------|
// | Result<T,E>       | try/except       | try/catch         |
// | ? operator        | raise (implicit) | throw (implicit)  |
// | unwrap()          | (no equivalent)  | (no equivalent)   |
// | panic!            | unhandled except │ uncaught error    |
// | map_err           | except Specific  | catch(e) { if }   |
// ============================================================

// ============================================================
// 🏋️ LATIHAN:
// 1. Buat fungsi `bagi(a: f64, b: f64) -> Result<f64, String>`
//    yang return error jika pembagi = 0
// 2. Buat program yang baca file dan hitung jumlah baris
//    Handle error jika file tidak ada
// 3. Buat fungsi yang parse CSV line "nama,umur,kota" ke struct
//    Return error jika format salah
// 4. Buat custom error type untuk aplikasi kalkulator
// 5. Implementasikan chain of ? untuk operasi multi-step:
//    baca file → parse angka → hitung → return hasil
// 6. Gunakan unwrap_or_else untuk memberi default custom
// ============================================================
