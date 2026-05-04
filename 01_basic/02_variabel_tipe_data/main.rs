// ============================================================
// 📗 BELAJAR RUST #02 — Variabel & Tipe Data
// ============================================================
// Di Rust, variabel IMMUTABLE (tidak bisa diubah) secara default.
// Ini adalah fitur keamanan Rust — mencegah perubahan data
// yang tidak disengaja.
//
// 🎯 Tujuan: Memahami cara mendeklarasikan variabel, konsep
//    mutability, shadowing, dan semua tipe data dasar di Rust.
//
// 💡 Filosofi Rust: "Secara default, semuanya immutable."
//    Kalau memang perlu diubah, kamu harus MINTA IZIN dulu
//    dengan keyword `mut`. Ini membuat kode lebih aman dan
//    mudah dipahami.
// ============================================================

fn main() {
    // ── VARIABEL IMMUTABLE ──────────────────────────────────
    // `let` mendeklarasikan variabel. Secara default TIDAK BISA diubah.
    //
    // 💡 Analogi: Immutable variable seperti kontrak yang sudah
    //    ditandatangani — isinya tidak bisa diubah lagi.
    let x = 5;
    println!("x = {}", x);

    // Ini akan ERROR kalau di-uncomment:
    // x = 10; // ❌ cannot assign twice to immutable variable
    //
    // ⚠️ PESAN ERROR: "cannot assign twice to immutable variable `x`"
    //    Compiler Rust melindungi kita dari bug perubahan data tak terduga!

    // ── VARIABEL MUTABLE ────────────────────────────────────
    // Tambahkan `mut` agar variabel bisa diubah nilainya.
    // `mut` = "mutable" → bisa berubah
    //
    // 💡 Analogi: Mutable variable seperti papan tulis —
    //    isinya bisa dihapus dan tulis ulang.
    let mut y = 10;
    println!("y awal = {}", y);
    y = 20; // ✅ OK karena pakai `mut`
    println!("y setelah diubah = {}", y);

    // ── SHADOWING ───────────────────────────────────────────
    // Kita bisa mendeklarasikan ulang variabel dengan nama yang sama.
    // Ini disebut "shadowing" — variabel lama "tertimpa" oleh yang baru.
    //
    // 💡 Analogi: Shadowing seperti menempel sticker baru di atas
    //    sticker lama. Sticker lama masih ada di bawah, tapi kita
    //    hanya melihat yang baru.
    //
    // 🔑 Perbedaan SHADOWING vs MUT:
    // - Shadowing: membuat variabel BARU dengan nama sama
    // - Mut: mengubah nilai variabel yang SUDAH ADA
    // - Shadowing bisa GANTI TIPE DATA, mut tidak bisa!
    let z = 5;
    let z = z + 1; // shadow: z sekarang 6
    let z = z * 2; // shadow lagi: z sekarang 12
    println!("z = {}", z);

    // Shadowing bisa ganti tipe — ini tidak bisa dengan `mut`
    let spasi = "   ";     // tipe: &str (string slice)
    let spasi = spasi.len(); // tipe: usize (angka!)
    //     ↑ variabel baru dengan nama sama, tipe berbeda
    println!("Jumlah spasi = {}", spasi);

    // ── KONSTANTA ───────────────────────────────────────────
    // `const` harus punya tipe eksplisit dan nilainya harus diketahui
    // saat compile time. Konvensi penamaan: HURUF_BESAR_SNAKE_CASE
    //
    // 💡 Perbedaan const vs let:
    // - const: nilai HARUS diketahui saat compile, immutable selamanya
    // - let: nilai bisa dihitung saat runtime, bisa di-shadow
    // - const tidak punya alamat memori tetap (inline ke kode)
    const MAX_SKOR: u32 = 100_000;
    println!("Skor maksimum: {}", MAX_SKOR);

    // ── TIPE DATA: INTEGER ──────────────────────────────────
    // Rust punya BANYAK tipe integer — pilih yang sesuai kebutuhan!
    //
    // Signed (bisa negatif):   i8, i16, i32, i64, i128, isize
    // Unsigned (positif saja): u8, u16, u32, u64, u128, usize
    //
    // 💡 Analogi: Integer types seperti ukuran kantong:
    //    i8  = kantong kecil  (-128 sampai 127)
    //    i32 = kantong sedang (~±2 miliar)
    //    i64 = kantong besar  (~±9 kuadriliun)
    //    isize = ukuran kantong mengikuti sistem (32/64 bit)
    //
    // Default integer: i32 — paling cepat di kebanyakan CPU
    let umur: u8 = 25; // 0 sampai 255 — cukup untuk umur manusia
    let suhu: i8 = -10; // -128 sampai 127 — cukup untuk suhu Celsius
    let besar: i64 = 1_000_000_000; // underscore sebagai pemisah ribuan
    println!("Umur: {}, Suhu: {}, Besar: {}", umur, suhu, besar);

    // Literal integer dalam berbagai basis (number systems)
    let desimal = 98_222;      // basis 10 (default)
    let heksa = 0xff;          // basis 16 — diawali 0x
    let oktal = 0o77;          // basis 8 — diawali 0o
    let biner = 0b1111_0000;   // basis 2 — diawali 0b
    let byte_val = b'A';       // khusus u8 — nilai ASCII dari 'A' = 65
    println!(
        "Desimal: {}, Hex: {}, Oktal: {}, Biner: {}, Byte: {}",
        desimal, heksa, oktal, biner, byte_val
    );

    // ── TIPE DATA: FLOAT ────────────────────────────────────
    // f32 (single precision, 32-bit) dan f64 (double precision, 64-bit)
    // Default: f64 — di Rust, kecepatan f64 sama dengan f32 di CPU modern
    let pi: f64 = 3.14159265358979;
    let gravitasi: f32 = 9.81;
    println!("Pi: {}, Gravitasi: {}", pi, gravitasi);

    // ── TIPE DATA: BOOLEAN ──────────────────────────────────
    // bool: true atau false — digunakan untuk kondisi dan logika
    let aktif: bool = true;
    let selesai = false; // tipe di-infer (ditebak) otomatis oleh compiler
    println!("Aktif: {}, Selesai: {}", aktif, selesai);

    // ── TIPE DATA: CHAR ─────────────────────────────────────
    // `char` di Rust adalah Unicode Scalar Value — 4 bytes!
    // Bisa menyimpan emoji, huruf CJK (Cina/Jepang/Korea), dll.
    //
    // 💡 Perbedaan dengan bahasa lain:
    // - C/C++: char = 1 byte (ASCII saja)
    // - Rust: char = 4 byte (Unicode penuh!)
    // - Python 3: string = Unicode, tapi tidak ada tipe char terpisah
    let huruf: char = 'A';
    let emoji: char = '🦀';
    let aksara: char = 'ñ';
    println!("Huruf: {}, Emoji: {}, Aksara: {}", huruf, emoji, aksara);

    // ── TIPE DATA: TUPLE ────────────────────────────────────
    // Tuple: kumpulan nilai dengan tipe berbeda-beda, ukuran TETAP
    // Setelah dibuat, tuple tidak bisa ditambah/dikurangi elemennya.
    //
    // 💡 Analogi: Tuple seperti baris dalam spreadsheet —
    //    setiap kolom bisa berisi tipe data berbeda.
    let orang: (&str, i32, f64) = ("Budi", 25, 170.5);
    // Akses elemen tuple dengan index: .0, .1, .2, ...
    println!("Nama: {}, Umur: {}, Tinggi: {}", orang.0, orang.1, orang.2);

    // Destructuring tuple — bongkar isinya ke variabel terpisah
    // Ini lebih rapi daripada akses satu per satu dengan .0, .1, dll.
    let (nama, umur_orang, tinggi) = orang;
    println!("Nama: {}, Umur: {}, Tinggi: {}", nama, umur_orang, tinggi);

    // Unit tuple `()` — tuple kosong, dipakai saat tidak ada return value
    // Mirip `void` di C/Java, tapi di Rust ini adalah tipe nyata!
    let _unit: () = ();

    // ── TIPE DATA: ARRAY ────────────────────────────────────
    // Array: kumpulan nilai dengan tipe SAMA, ukuran TETAP (fixed-size)
    // Disimpan di stack — sangat cepat! — tapi tidak fleksibel.
    //
    // 💡 Analogi: Array seperti deretan loker yang identik —
    //    semua loker sama besar, jumlahnya tetap, tidak bisa ditambah.
    //    Kalau butuh fleksibel (bisa tambah/kurang), gunakan Vec nanti.
    let bulan: [&str; 3] = ["Januari", "Februari", "Maret"];
    //          ↑     ↑
    //        tipe  jumlah elemen
    println!("Bulan pertama: {}", bulan[0]);
    println!("Bulan kedua: {}", bulan[1]);

    // Array dengan nilai yang sama semua — shorthand
    let nol = [0; 5]; // [0, 0, 0, 0, 0]
    //          ↑  ↑
    //       nilai  jumlah
    println!("Array nol: {:?}", nol);

    // Panjang array — diketahui saat compile time
    println!("Jumlah bulan: {}", bulan.len());

    // ── TYPE INFERENCE ──────────────────────────────────────
    // Rust bisa menebak tipe data dari konteks — tidak perlu selalu ditulis
    // Compiler Rust sangat pintar dalam menebak tipe!
    let tebakan = 42;         // Rust tahu ini i32 (default integer)
    let tebakan_float = 3.14; // Rust tahu ini f64 (default float)
    let tebakan_bool = true;  // Rust tahu ini bool
    println!("{} {} {}", tebakan, tebakan_float, tebakan_bool);

    // ── TYPE CASTING ────────────────────────────────────────
    // Gunakan `as` untuk konversi tipe secara eksplisit
    // Rust TIDAK melakukan implicit type casting (tidak seperti C!)
    //
    // ⚠️ Hati-hati: casting bisa menyebabkan data loss/truncation
    let angka_i32: i32 = 42;
    let angka_f64 = angka_i32 as f64; // i32 → f64 (aman, tidak ada data loss)
    let angka_u8 = angka_i32 as u8;   // i32 → u8 (bisa truncate! 42 tetap 42)
    println!(
        "i32: {}, f64: {}, u8: {}",
        angka_i32, angka_f64, angka_u8
    );

    // Contoh truncation:
    let besar_i32: i32 = 300;
    let kecil_u8 = besar_i32 as u8; // 300 mod 256 = 44
    println!("300 sebagai u8 = {} (truncate!)", kecil_u8);
}

// ============================================================
// 🧠 RINGKUMAN TIPE DATA RUST:
//
// ┌─────────────┬─────────────────────────────────────────────┐
// │ Tipe        │ Penjelasan                                  │
// ├─────────────┼─────────────────────────────────────────────┤
// │ i8 - i128   │ Integer signed (bisa negatif)               │
// │ u8 - u128   │ Integer unsigned (positif saja)             │
// │ isize/usize │ Integer mengikuti arsitektur (32/64 bit)    │
// │ f32, f64    │ Floating point (desimal)                    │
// │ bool        │ true / false                                │
// │ char        │ Unicode character (4 bytes!)                │
// │ tuple       │ Kumpulan tipe berbeda, ukuran tetap         │
// │ array       │ Kumpulan tipe sama, ukuran tetap            │
// │ ()          │ Unit tuple — "tidak ada nilai"              │
// └─────────────┴─────────────────────────────────────────────┘
//
// ⚠️ COMMON MISTAKES:
// - Integer overflow: `let x: u8 = 256;` → compile error!
// - Index out of bounds: `bulan[10]` → PANIC saat runtime!
// - Lupa `mut` saat perlu mengubah variabel
// - Type mismatch: `let x: i32 = 3.14;` → compile error!
//
// 🔗 PERBANDINGAN DENGAN BAHASA LAIN:
// | Rust        | Python       | JavaScript    | C/C++       |
// |-------------|--------------|---------------|-------------|
// | let x = 5   | x = 5        | let x = 5     | int x = 5   |
// | let mut x   | x = 5        | let x = 5     | int x = 5   |
// | i32, u64    | int (arbitrary)| Number      | int, long   |
// | f64         | float        | Number        | double      |
// | bool        | bool         | boolean       | bool        |
// | char        | (string[0])  | (string[0])   | char        |
// | tuple       | tuple        | -             | -           |
// | array       | list         | Array         | array       |
// ============================================================

// ============================================================
// 🏋️ LATIHAN:
// 1. Buat variabel mutable berisi umur kamu, lalu ubah nilainya
// 2. Buat tuple berisi (nama, umur, kota) dan destructure-nya
// 3. Buat array berisi 5 angka favorit, cetak semuanya dengan {:?}
// 4. Coba akses array di luar batas (misal index 10) — lihat error!
// 5. Buat konstanta untuk kecepatan cahaya (299_792_458 m/s)
// 6. Coba shadowing dengan mengubah &str menjadi i32
// 7. Casting f64 ke i32 — apa yang terjadi dengan bagian desimal?
// ============================================================
