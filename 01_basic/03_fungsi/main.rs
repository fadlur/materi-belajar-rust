// ============================================================
// 📗 BELAJAR RUST #03 — Fungsi
// ============================================================
// Fungsi adalah blok kode yang bisa dipanggil berulang kali.
// Konvensi penamaan: snake_case (huruf kecil, pemisah underscore)
// ============================================================

fn main() {
    // ── MEMANGGIL FUNGSI ────────────────────────────────────
    sapa();
    sapa_nama("Fadlur");
    sapa_nama("Rust");

    // ── FUNGSI DENGAN RETURN VALUE ──────────────────────────
    let hasil = tambah(5, 3);
    println!("5 + 3 = {}", hasil);

    let luas = luas_persegi_panjang(10.0, 5.0);
    println!("Luas persegi panjang: {}", luas);

    // ── EXPRESSION vs STATEMENT ─────────────────────────────
    // Di Rust, hampir semua hal adalah EXPRESSION (menghasilkan nilai).
    // Statement diakhiri `;` dan TIDAK menghasilkan nilai.
    // Expression TANPA `;` menghasilkan nilai — ini cara Rust me-return.

    let angka = {
        let x = 5;
        let y = 10;
        x + y // ⚠️ Tidak ada titik koma! Ini expression yang di-return
    };
    println!("Angka dari block expression: {}", angka);

    // ── FUNGSI SEBAGAI EKSPRESI ─────────────────────────────
    let abs_val = nilai_absolut(-42);
    println!("Nilai absolut -42 = {}", abs_val);

    // ── EARLY RETURN ────────────────────────────────────────
    println!("Apakah 7 genap? {}", apakah_genap(7));
    println!("Apakah 10 genap? {}", apakah_genap(10));

    // ── MULTIPLE RETURN VALUES (TUPLE) ──────────────────────
    let (jumlah, selisih) = jumlah_dan_selisih(20, 8);
    println!("Jumlah: {}, Selisih: {}", jumlah, selisih);

    // ── FUNGSI YANG TIDAK RETURN (NEVER TYPE) ───────────────
    // Beberapa fungsi tidak pernah return — misalnya panic!
    // Tipenya `!` (never type), tapi kita jarang deklarasikan sendiri

    // ── NESTED FUNCTION ─────────────────────────────────────
    // Fungsi bisa dideklarasikan di dalam fungsi lain
    fn kuadrat(n: i32) -> i32 {
        n * n
    }
    println!("Kuadrat 7 = {}", kuadrat(7));

    // ── CONTOH LEBIH KOMPLEKS ───────────────────────────────
    let suhu_c = 100.0;
    let suhu_f = celsius_ke_fahrenheit(suhu_c);
    println!("{}°C = {}°F", suhu_c, suhu_f);

    let suhu_c_balik = fahrenheit_ke_celsius(suhu_f);
    println!("{}°F = {}°C", suhu_f, suhu_c_balik);

    // Fibonacci
    for i in 0..10 {
        print!("{} ", fibonacci(i));
    }
    println!(); // newline
}

// ── FUNGSI TANPA PARAMETER & TANPA RETURN VALUE ─────────────
// Kalau tidak ada return value, return type-nya `()` (unit)
// tapi tidak perlu ditulis eksplisit
fn sapa() {
    println!("Halo dari fungsi sapa!");
}

// ── FUNGSI DENGAN PARAMETER ─────────────────────────────────
// Parameter HARUS punya type annotation — Rust tidak menebak tipe parameter
fn sapa_nama(nama: &str) {
    println!("Halo, {}!", nama);
}

// ── FUNGSI DENGAN RETURN VALUE ──────────────────────────────
// `-> tipe` menandakan tipe return value
// Baris terakhir TANPA `;` otomatis menjadi return value
fn tambah(a: i32, b: i32) -> i32 {
    a + b // tidak ada `;` — ini return value!
}

fn luas_persegi_panjang(panjang: f64, lebar: f64) -> f64 {
    panjang * lebar
}

// ── FUNGSI DENGAN IF EXPRESSION ─────────────────────────────
fn nilai_absolut(x: i32) -> i32 {
    // `if` di Rust adalah expression — bisa langsung menghasilkan nilai
    if x < 0 { -x } else { x }
}

// ── EARLY RETURN DENGAN `return` KEYWORD ────────────────────
// Kita bisa pakai `return` untuk keluar lebih awal dari fungsi
fn apakah_genap(n: i32) -> bool {
    if n % 2 == 0 {
        return true; // early return
    }
    false // implicit return
}

// ── RETURN MULTIPLE VALUES DENGAN TUPLE ─────────────────────
fn jumlah_dan_selisih(a: i32, b: i32) -> (i32, i32) {
    (a + b, a - b)
}

// ── KONVERSI SUHU ───────────────────────────────────────────
fn celsius_ke_fahrenheit(c: f64) -> f64 {
    (c * 9.0 / 5.0) + 32.0
}

fn fahrenheit_ke_celsius(f: f64) -> f64 {
    (f - 32.0) * 5.0 / 9.0
}

// ── REKURSI ─────────────────────────────────────────────────
// Fungsi bisa memanggil dirinya sendiri
fn fibonacci(n: u32) -> u64 {
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

// ============================================================
// 🏋️ LATIHAN:
// 1. Buat fungsi `kali(a, b)` yang mengalikan dua angka
// 2. Buat fungsi `pangkat(base, exp)` secara rekursif
// 3. Buat fungsi `keliling_lingkaran(radius)` → 2 * PI * r
// 4. Buat fungsi yang menerima suhu dalam Kelvin dan konversi ke Celsius
// 5. Buat fungsi `is_palindrome(s: &str) -> bool`
//    Hint: s.chars().rev().collect::<String>()
// ============================================================
