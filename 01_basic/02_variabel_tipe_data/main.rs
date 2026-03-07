// ============================================================
// 📗 BELAJAR RUST #02 — Variabel & Tipe Data
// ============================================================
// Di Rust, variabel IMMUTABLE (tidak bisa diubah) secara default.
// Ini adalah fitur keamanan Rust — mencegah perubahan data
// yang tidak disengaja.
// ============================================================

fn main() {
    // ── VARIABEL IMMUTABLE ──────────────────────────────────
    // `let` mendeklarasikan variabel. Secara default TIDAK BISA diubah.
    let x = 5;
    println!("x = {}", x);

    // Ini akan ERROR kalau di-uncomment:
    // x = 10; // ❌ cannot assign twice to immutable variable

    // ── VARIABEL MUTABLE ────────────────────────────────────
    // Tambahkan `mut` agar variabel bisa diubah nilainya
    let mut y = 10;
    println!("y awal = {}", y);
    y = 20; // ✅ OK karena pakai `mut`
    println!("y setelah diubah = {}", y);

    // ── SHADOWING ───────────────────────────────────────────
    // Kita bisa mendeklarasikan ulang variabel dengan nama yang sama.
    // Ini disebut "shadowing" — variabel lama "tertimpa".
    // Berbeda dengan mut: shadowing bisa GANTI TIPE DATA!
    let z = 5;
    let z = z + 1; // shadow: z sekarang 6
    let z = z * 2; // shadow lagi: z sekarang 12
    println!("z = {}", z);

    // Shadowing bisa ganti tipe — ini tidak bisa dengan `mut`
    let spasi = "   "; // tipe: &str
    let spasi = spasi.len(); // tipe: usize (angka!)
    println!("Jumlah spasi = {}", spasi);

    // ── KONSTANTA ───────────────────────────────────────────
    // `const` harus punya tipe eksplisit dan nilainya harus diketahui
    // saat compile time. Konvensi: HURUF_BESAR_SNAKE_CASE
    const MAX_SKOR: u32 = 100_000;
    println!("Skor maksimum: {}", MAX_SKOR);

    // ── TIPE DATA: INTEGER ──────────────────────────────────
    // Signed (bisa negatif): i8, i16, i32, i64, i128, isize
    // Unsigned (positif saja): u8, u16, u32, u64, u128, usize
    // Default integer: i32
    let umur: u8 = 25; // 0 sampai 255
    let suhu: i8 = -10; // -128 sampai 127
    let besar: i64 = 1_000_000_000; // underscore sebagai pemisah ribuan
    println!("Umur: {}, Suhu: {}, Besar: {}", umur, suhu, besar);

    // Literal integer dalam berbagai basis
    let desimal = 98_222;
    let heksa = 0xff;
    let oktal = 0o77;
    let biner = 0b1111_0000;
    let byte_val = b'A'; // khusus u8 — nilai ASCII dari 'A'
    println!(
        "Desimal: {}, Hex: {}, Oktal: {}, Biner: {}, Byte: {}",
        desimal, heksa, oktal, biner, byte_val
    );

    // ── TIPE DATA: FLOAT ────────────────────────────────────
    // f32 (single precision) dan f64 (double precision, default)
    let pi: f64 = 3.14159265358979;
    let gravitasi: f32 = 9.81;
    println!("Pi: {}, Gravitasi: {}", pi, gravitasi);

    // ── TIPE DATA: BOOLEAN ──────────────────────────────────
    let aktif: bool = true;
    let selesai = false; // tipe di-infer otomatis
    println!("Aktif: {}, Selesai: {}", aktif, selesai);

    // ── TIPE DATA: CHAR ─────────────────────────────────────
    // `char` di Rust adalah Unicode Scalar Value — 4 bytes!
    // Bisa menyimpan emoji, huruf CJK, dll.
    let huruf: char = 'A';
    let emoji: char = '🦀';
    let aksara: char = 'ñ';
    println!("Huruf: {}, Emoji: {}, Aksara: {}", huruf, emoji, aksara);

    // ── TIPE DATA: TUPLE ────────────────────────────────────
    // Tuple: kumpulan nilai dengan tipe berbeda-beda, ukuran tetap
    let orang: (&str, i32, f64) = ("Budi", 25, 170.5);
    println!("Nama: {}, Umur: {}, Tinggi: {}", orang.0, orang.1, orang.2);

    // Destructuring tuple — bongkar isinya ke variabel terpisah
    let (nama, umur_orang, tinggi) = orang;
    println!("Nama: {}, Umur: {}, Tinggi: {}", nama, umur_orang, tinggi);

    // Unit tuple `()` — tuple kosong, dipakai saat tidak ada return value
    let _unit: () = ();

    // ── TIPE DATA: ARRAY ────────────────────────────────────
    // Array: kumpulan nilai dengan tipe SAMA, ukuran TETAP (fixed-size)
    // Disimpan di stack — sangat cepat!
    let bulan: [&str; 3] = ["Januari", "Februari", "Maret"];
    println!("Bulan pertama: {}", bulan[0]);
    println!("Bulan kedua: {}", bulan[1]);

    // Array dengan nilai yang sama semua
    let nol = [0; 5]; // [0, 0, 0, 0, 0]
    println!("Array nol: {:?}", nol);

    // Panjang array
    println!("Jumlah bulan: {}", bulan.len());

    // ── TYPE INFERENCE ──────────────────────────────────────
    // Rust bisa menebak tipe data dari konteks — tidak perlu selalu ditulis
    let tebakan = 42; // Rust tahu ini i32
    let tebakan_float = 3.14; // Rust tahu ini f64
    let tebakan_bool = true; // Rust tahu ini bool
    println!("{} {} {}", tebakan, tebakan_float, tebakan_bool);

    // ── TYPE CASTING ────────────────────────────────────────
    // Gunakan `as` untuk konversi tipe
    let angka_i32: i32 = 42;
    let angka_f64 = angka_i32 as f64;
    let angka_u8 = angka_i32 as u8;
    println!(
        "i32: {}, f64: {}, u8: {}",
        angka_i32, angka_f64, angka_u8
    );
}

// ============================================================
// 🏋️ LATIHAN:
// 1. Buat variabel mutable berisi umur kamu, lalu ubah nilainya
// 2. Buat tuple berisi (nama, umur, kota) dan destructure-nya
// 3. Buat array berisi 5 angka favorit, cetak semuanya dengan {:?}
// 4. Coba akses array di luar batas (misal index 10) — lihat error!
// 5. Buat konstanta untuk kecepatan cahaya (299_792_458 m/s)
// ============================================================
