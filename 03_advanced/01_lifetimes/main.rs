// ============================================================
// 📙 BELAJAR RUST #17 — Lifetimes
// ============================================================
// Lifetime menjamin bahwa reference SELALU valid.
// Rust compiler (borrow checker) mengecek ini saat compile time.
// Lifetime annotation ('a) TIDAK mengubah berapa lama data hidup —
// hanya MENJELASKAN relasi antar reference.
//
// 🎯 Tujuan: Memahami konsep lifetime, lifetime annotation,
//    elision rules, dan penggunaan lifetime dalam struct dan fungsi.
//
// 💡 Analogi Utama:
// Lifetime seperti SERTIFIKAT MASA BERLAKU. Kalau kamu pinjam
// buku dari perpustakaan, kamu hanya boleh pakai buku itu selama
// masa pinjam berlaku. Lifetime annotation memberitahu compiler:
// "reference ini valid selama 'a" — compiler lalu memastikan
// data yang ditunjuk hidup SETIDAKNYA selama 'a.
//
// 🔑 Konsep Kunci: Lifetime BUKAN mengubah berapa lama data hidup.
//    Lifetime hanya MENJELASKAN relasi antar reference agar
//    compiler bisa memverifikasi keamanan.
// ============================================================

fn main() {
    // ── MASALAH: DANGLING REFERENCE ─────────────────────────
    // Tanpa lifetime, kode ini bisa menyebabkan dangling reference:
    //
    // let r;
    // {
    //     let x = 5;
    //     r = &x;  // ❌ ERROR! x akan di-drop di akhir scope
    // }
    // println!("{}", r);  // r menunjuk ke memori yang sudah dibebaskan!
    //
    // 💡 Compiler menolak kode di atas — Rust melindungi kita!

    // Yang benar: reference harus hidup SETIDAKNYA selama pemakainya
    let x = 5;
    let r = &x; // ✅ x hidup selama r masih dipakai
    println!("r = {}", r);

    // ── LIFETIME DALAM FUNGSI ───────────────────────────────
    let string1 = String::from("long string");

    {
        let string2 = String::from("xyz");
        let result = terpanjang(&string1, &string2);
        println!("Terpanjang: {}", result);
        // ✅ OK karena string2 masih hidup di sini
    }

    // Contoh yang lebih kompleks — result harus hidup di scope yang tepat
    let result2;
    {
        let string3 = String::from("hello");
        result2 = terpanjang(&string1, &string3);
        println!("Terpanjang: {}", result2);
        // result2 dipakai di sini, masih dalam scope string3
    }
    // println!("{}", result2); // ⚠️ Ini bisa error jika string3 sudah di-drop

    // ── LIFETIME ELISION RULES ──────────────────────────────
    // Rust punya 3 aturan yang memungkinkan kita TIDAK menulis lifetime:
    //
    // 💡 Analogi: Elision seperti "aturan baku" — kalau pola umum,
    //    tidak perlu tulis lifetime secara eksplisit.
    //
    // Rule 1: Setiap reference parameter dapat lifetime sendiri
    //   fn foo(x: &str, y: &str) → fn foo<'a, 'b>(x: &'a str, y: &'b str)
    //
    // Rule 2: Jika hanya ada SATU input lifetime, output mendapat lifetime itu
    //   fn foo(x: &str) -> &str → fn foo<'a>(x: &'a str) -> &'a str
    //
    // Rule 3: Jika ada &self atau &mut self, output mendapat lifetime self
    //   fn foo(&self, x: &str) -> &str → lifetime dari &self

    // Contoh: ini TIDAK perlu lifetime annotation (rule 2)
    let kata = "Halo Dunia";
    let pertama = kata_pertama(kata);
    println!("Kata pertama: {}", pertama);

    // ── LIFETIME DALAM STRUCT ───────────────────────────────
    // Struct yang menyimpan reference HARUS punya lifetime annotation
    let novel = String::from("Laskar Pelangi karya Andrea Hirata");
    let kalimat_pertama;
    {
        let i = novel.find(' ').unwrap_or(novel.len());
        kalimat_pertama = &novel[..i];
    }

    let kutipan = Kutipan {
        teks: kalimat_pertama,
    };
    println!("Kutipan: {}", kutipan.teks);
    println!("Level: {}", kutipan.level());

    // ── MULTIPLE LIFETIMES ──────────────────────────────────
    let s1 = String::from("panjang");
    let result;
    {
        let s2 = String::from("pendek");
        result = pertama_str(&s1, &s2);
        println!("Pertama: {}", result);
    }

    // ── STATIC LIFETIME ─────────────────────────────────────
    // 'static = reference yang hidup SELAMANYA (selama program berjalan)
    // Semua string literal punya lifetime 'static
    let s: &'static str = "Saya hidup selamanya!";
    println!("{}", s);

    // ⚠️ Jangan asal pakai 'static! Biasanya ada lifetime yang lebih tepat.
    // 'static untuk string literal dan leaked memory saja.

    // ── LIFETIME BOUND PADA GENERIC ─────────────────────────
    let ann_str = String::from("Pengumuman penting!");
    let s1_str = String::from("Teks pertama yang panjang sekali");
    let result3;
    {
        let s2_str = String::from("Pendek");
        result3 = terpanjang_dengan_pesan(&s1_str, &s2_str, &ann_str);
        println!("{}", result3);
    }

    // ── CONTOH PRAKTIS ──────────────────────────────────────
    let teks = String::from("Rust adalah bahasa pemrograman yang aman dan cepat");
    let kata_kata = bagi_kata(&teks);
    println!("Kata-kata: {:?}", kata_kata);

    // Parser sederhana
    let csv_line = "Budi,25,Jakarta";
    let parsed = parse_csv(csv_line);
    println!("Parsed CSV: {:?}", parsed);
}

// ── FUNGSI DENGAN LIFETIME ANNOTATION ───────────────────────
// 'a dibaca "lifetime a" — artinya: returned reference akan hidup
// setidaknya selama KEDUA parameter hidup.
//
// 💡 Analogi: Kalau dua orang meminjamkan barang, hasilnya
//    hanya valid selama barang KEDUA orang masih ada.
fn terpanjang<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// Tanpa lifetime annotation — ini TIDAK compile:
// fn terpanjang(x: &str, y: &str) -> &str {  // ❌ missing lifetime
//     if x.len() > y.len() { x } else { y }
// }

// ── FUNGSI TANPA LIFETIME (ELISION) ────────────────────────
// Compiler bisa menebak lifetime — kita tidak perlu menulis
fn kata_pertama(s: &str) -> &str {
    // Rule 2: satu input reference → output dapat lifetime yang sama
    let bytes = s.as_bytes();
    for (i, &b) in bytes.iter().enumerate() {
        if b == b' ' {
            return &s[..i];
        }
    }
    s
}

// ── MULTIPLE LIFETIMES ──────────────────────────────────────
// Kadang parameter punya lifetime BERBEDA
fn pertama_str<'a, 'b>(x: &'a str, _y: &'b str) -> &'a str {
    // Return selalu dari x, jadi hanya perlu lifetime 'a
    x
}

// ── STRUCT DENGAN LIFETIME ──────────────────────────────────
// Struct yang menyimpan reference HARUS punya lifetime annotation
#[derive(Debug)]
struct Kutipan<'a> {
    teks: &'a str,
}

impl<'a> Kutipan<'a> {
    // Rule 3: &self → output mendapat lifetime self
    fn level(&self) -> &str {
        if self.teks.len() > 30 {
            "panjang"
        } else {
            "pendek"
        }
    }

    // Explicit lifetime di method
    fn umumkan(&self, pengumuman: &str) -> &str {
        println!("Pengumuman: {}", pengumuman);
        self.teks
    }
}

// ── LIFETIME + GENERIC + TRAIT BOUNDS ───────────────────────
// Semua bisa dicampur!
fn terpanjang_dengan_pesan<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: &T,
) -> &'a str
where
    T: std::fmt::Display,
{
    println!("Pesan: {}", ann);
    if x.len() > y.len() { x } else { y }
}

// ── CONTOH PRAKTIS ──────────────────────────────────────────
fn bagi_kata<'a>(teks: &'a str) -> Vec<&'a str> {
    // Return Vec berisi reference ke potongan string asli
    // Semua reference valid selama `teks` valid
    teks.split_whitespace().collect()
}

fn parse_csv<'a>(line: &'a str) -> Vec<&'a str> {
    line.split(',').collect()
}

// ============================================================
// 🧠 RINGKUMAN LIFETIME:
//
// ┌─────────────────────────────────────────────────────────────┐
// │                    KONSEP LIFETIME                          │
// ├──────────────────┬──────────────────────────────────────────┤
// │ 'a, 'b, dll.     │ Nama lifetime — konvensi huruf kecil     │
// │ &'a str          │ Reference ke str yang valid selama 'a    │
// │ Struct<'a>       │ Struct yang menyimpan reference 'a       │
// │ 'static          │ Hidup selamanya (string literal)         │
// └──────────────────┴──────────────────────────────────────────┘
//
// ┌─────────────────────────────────────────────────────────────┐
// │                    ELISION RULES                            │
// ├──────────────────┬──────────────────────────────────────────┤
// │ Rule 1           │ Tiap param &T dapat lifetime sendiri     │
// │ Rule 2           │ 1 input → output dapat lifetime itu      │
// │ Rule 3           │ &self → output dapat lifetime self       │
// └──────────────────┴──────────────────────────────────────────┘
//
// ⚠️ COMMON MISTAKES:
// - Return reference ke data lokal → lifetime error
// - Lupa lifetime annotation pada struct dengan reference
// - Asumsi 'static = solusi segala masalah → tidak!
// - Multiple references dengan lifetime sama padahal berbeda
//
// 💡 TIPS:
// - Kalau compiler minta lifetime, tambahkan eksplisit
// - Mulai dari yang paling sederhana, tambah kalau perlu
// - Prefer owned data (String, Vec) daripada reference
//   kalau lifetime menjadi terlalu kompleks
// ============================================================

// ============================================================
// 🏋️ LATIHAN:
// 1. Buat fungsi yang return reference ke string terpendek
// 2. Buat struct `Config<'a>` yang menyimpan &str references
//    untuk host, port, database
// 3. Buat fungsi yang menerima &str dan return Vec<&str>
//    berisi semua kata yang dimulai huruf kapital
// 4. Mengapa kode ini tidak compile? Fix-lah:
//    fn buat() -> &str { let s = String::from("halo"); &s }
// 5. Buat iterator custom yang mengembalikan reference
// ============================================================
