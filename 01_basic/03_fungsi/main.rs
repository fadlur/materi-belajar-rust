// ============================================================
// 📗 BELAJAR RUST #03 — Fungsi
// ============================================================
// Fungsi adalah blok kode yang bisa dipanggil berulang kali.
// Konvensi penamaan: snake_case (huruf kecil, pemisah underscore)
//
// 🎯 Tujuan: Memahami cara mendeklarasikan fungsi, parameter,
//    return value, expression vs statement, dan berbagai
//    pola fungsi di Rust.
//
// 💡 Analogi: Fungsi seperti resep masakan — kamu tulis sekali,
//    bisa dipakai berkali-kali dengan bahan (parameter) berbeda.
// ============================================================

fn main() {
    // ── MEMANGGIL FUNGSI ────────────────────────────────────
    // Panggil fungsi dengan menulis nama diikuti tanda kurung
    // dan argumen yang diperlukan (jika ada).
    sapa();              // tanpa parameter
    sapa_nama("Fadlur"); // dengan satu parameter
    sapa_nama("Rust");   // dipanggil lagi dengan argumen berbeda

    // ── FUNGSI DENGAN RETURN VALUE ──────────────────────────
    // Return value bisa disimpan ke variabel untuk dipakai nanti
    let hasil = tambah(5, 3);
    println!("5 + 3 = {}", hasil);

    let luas = luas_persegi_panjang(10.0, 5.0);
    println!("Luas persegi panjang: {}", luas);

    // ── EXPRESSION vs STATEMENT ─────────────────────────────
    // Ini adalah konsep PENTING dan UNIK di Rust!
    //
    // STATEMENT → perintah yang TIDAK menghasilkan nilai
    //   Contoh: let x = 5;  (pernyataan penugasan)
    //           fn sapa() {} (deklarasi fungsi)
    //           x = 10;      (assignment — tidak menghasilkan nilai!)
    //
    // EXPRESSION → perintah yang MENGHASILKAN nilai
    //   Contoh: 5 + 3          (hasilnya 8)
    //           if x > 0 { 1 } else { -1 }  (hasilnya 1 atau -1)
    //           { let x = 5; x + 1 }  (hasilnya 6 — block expression!)
    //
    // 💡 Analogi:
    //   Statement = perintah ("Tolong cetak ini!")
    //   Expression = pertanyaan ("Berapa 5 + 3?") → ada jawaban
    //
    // 🔑 KUNCI: Di Rust, hampir semua hal adalah expression!
    //   Expression TANPA `;` di akhir akan di-return!

    // Contoh block expression:
    let angka = {
        let x = 5;   // statement di dalam block
        let y = 10;  // statement di dalam block
        x + y        // ⚠️ Tidak ada titik koma! Ini expression yang di-return
    };               // ← block ini menghasilkan nilai 15
    println!("Angka dari block expression: {}", angka);

    // Kalau ditambahkan `;` di akhir, menjadi statement → tidak menghasilkan nilai
    // let salah = { x + y; }; // ❌ ERROR! () tidak bisa di-assign ke i32

    // ── FUNGSI SEBAGAI EKSPRESI ─────────────────────────────
    // Fungsi `nilai_absolut` menggunakan if expression untuk return
    let abs_val = nilai_absolut(-42);
    println!("Nilai absolut -42 = {}", abs_val);

    // ── EARLY RETURN ────────────────────────────────────────
    // `return` keyword bisa digunakan untuk keluar lebih awal dari fungsi
    // Ini berguna untuk validasi atau kondisi khusus di awal fungsi.
    println!("Apakah 7 genap? {}", apakah_genap(7));
    println!("Apakah 10 genap? {}", apakah_genap(10));

    // ── MULTIPLE RETURN VALUES (TUPLE) ──────────────────────
    // Rust bisa return beberapa nilai sekaligus dengan tuple!
    // Ini sangat berguna saat fungsi perlu mengembalikan banyak informasi.
    let (jumlah, selisih) = jumlah_dan_selisih(20, 8);
    println!("Jumlah: {}, Selisih: {}", jumlah, selisih);

    // ── FUNGSI YANG TIDAK RETURN (NEVER TYPE) ───────────────
    // Beberapa fungsi tidak pernah return — misalnya panic!
    // Tipenya `!` (never type), tapi kita jarang deklarasikan sendiri

    // ── NESTED FUNCTION ─────────────────────────────────────
    // Fungsi bisa dideklarasikan di dalam fungsi lain
    // Scope-nya terbatas di dalam fungsi tempat dideklarasikan
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
    println!("Bilangan Fibonacci:");
    for i in 0..10 {
        print!("{} ", fibonacci(i));
    }
    println!(); // newline
}

// ── FUNGSI TANPA PARAMETER & TANPA RETURN VALUE ─────────────
// Kalau tidak ada return value, return type-nya `()` (unit)
// tapi tidak perlu ditulis eksplisit — compiler mengerti.
fn sapa() {
    println!("Halo dari fungsi sapa!");
}

// ── FUNGSI DENGAN PARAMETER ─────────────────────────────────
// Parameter HARUS punya type annotation — Rust tidak menebak tipe parameter!
//
// 💡 Kenapa? Rust memerlukan type annotation untuk memastikan
//    keamanan tipe (type safety) saat compile time.
//    Dengan menulis tipe parameter, compiler bisa mengecek
//    apakah pemanggilan fungsi benar SEBELUM program dijalankan.
fn sapa_nama(nama: &str) {
    println!("Halo, {}!", nama);
}

// ── FUNGSI DENGAN RETURN VALUE ──────────────────────────────
// `-> tipe` menandakan tipe return value
// Baris terakhir TANPA `;` otomatis menjadi return value
//
// 💡 Analogi: `->` seperti panah yang menunjuk "fungsi ini
//    menghasilkan tipe apa". Panahnya mengarah ke tipe output.
fn tambah(a: i32, b: i32) -> i32 {
    a + b // tidak ada `;` — ini expression yang di-return
}

// Bisa juga pakai return keyword (tapi tidak idiomatic untuk baris terakhir)
fn tambah_explicit(a: i32, b: i32) -> i32 {
    return a + b; // works, tapi `a + b` tanpa ; lebih idiomatic
}

fn luas_persegi_panjang(panjang: f64, lebar: f64) -> f64 {
    panjang * lebar
}

// ── FUNGSI DENGAN IF EXPRESSION ─────────────────────────────
// `if` di Rust adalah expression — bisa langsung menghasilkan nilai!
//
// 💡 Perbedaan dengan C/Java:
//   C:      return (x < 0) ? -x : x;   // ternary operator
//   Rust:   if x < 0 { -x } else { x } // if sebagai expression
fn nilai_absolut(x: i32) -> i32 {
    if x < 0 { -x } else { x }
}

// ── EARLY RETURN DENGAN `return` KEYWORD ────────────────────
// Kita bisa pakai `return` untuk keluar lebih awal dari fungsi
// Pattern ini sering dipakai untuk validasi input.
fn apakah_genap(n: i32) -> bool {
    if n % 2 == 0 {
        return true; // early return — keluar segera
    }
    false // implicit return — baris terakhir tanpa ;
}

// ── RETURN MULTIPLE VALUES DENGAN TUPLE ─────────────────────
// Tuple memungkinkan return beberapa nilai sekaligus
fn jumlah_dan_selisih(a: i32, b: i32) -> (i32, i32) {
    (a + b, a - b) // return tuple
}

// ── KONVERSI SUHU ───────────────────────────────────────────
fn celsius_ke_fahrenheit(c: f64) -> f64 {
    (c * 9.0 / 5.0) + 32.0
}

fn fahrenheit_ke_celsius(f: f64) -> f64 {
    (f - 32.0) * 5.0 / 9.0
}

// ── REKURSI ─────────────────────────────────────────────────
// Fungsi bisa memanggil dirinya sendiri — ini disebut rekursi
//
// 💡 Analogi: Rekursi seperti cermin di depan cermin —
//    fungsi memanggil dirinya sendiri dengan input yang lebih kecil
//    sampai mencapai "base case" (kondisi berhenti).
//
// ⚠️ Pastikan selalu ada base case! Kalau tidak, infinite recursion
//    → stack overflow!
fn fibonacci(n: u32) -> u64 {
    match n {
        0 => 0,                    // base case 1
        1 => 1,                    // base case 2
        _ => fibonacci(n - 1) + fibonacci(n - 2), // recursive case
    }
}

// ============================================================
// 🧠 KONSEP PENTING:
//
// 1. Parameter WAJIB punya type annotation — tidak ada pengecualian!
// 2. Return value = expression terakhir tanpa `;`
// 3. `return` keyword untuk early return (validasi, error handling)
// 4. `if` adalah expression — bisa return nilai
// 5. Block `{ ... }` juga expression — bisa return nilai terakhir
// 6. Tuple = cara return multiple values
// 7. Rekursi perlu base case untuk menghindari infinite loop
//
// ⚠️ COMMON MISTAKES:
// - Menambahkan `;` di baris terakhir fungsi → return () bukan nilai!
// - Lupa type annotation pada parameter → compile error
// - Return tipe berbeda di cabang if → compile error
// - Infinite recursion tanpa base case → stack overflow
//
// 🔗 PERBANDINGAN RETURN VALUE:
// | Rust              | Python           | JavaScript        |
// |-------------------|------------------|-------------------|
// | fn f() -> i32 {x} | def f(): return x| function f() {return x}|
// | fn f() { println!() } | def f(): print() | function f() { console.log() } |
// | (a, b)            | return a, b      | return [a, b]     |
// | if expr {a} else {b} | a if cond else b | cond ? a : b    |
// ============================================================

// ============================================================
// 🏋️ LATIHAN:
// 1. Buat fungsi `kali(a, b)` yang mengalikan dua angka
// 2. Buat fungsi `pangkat(base, exp)` secara rekursif
// 3. Buat fungsi `keliling_lingkaran(radius)` → 2 * PI * r
// 4. Buat fungsi yang menerima suhu dalam Kelvin dan konversi ke Celsius
// 5. Buat fungsi `is_palindrome(s: &str) -> bool`
//    Hint: s.chars().rev().collect::<String>()
// 6. Buat fungsi yang return tuple: (min, max, avg) dari tiga i32
// 7. Buat fungsi rekursif untuk faktorial
// ============================================================
