// ============================================================
// 📘 BELAJAR RUST #13 — Traits
// ============================================================
// Trait adalah kontrak/interface: "tipe ini bisa melakukan X".
// Mirip interface di Java/Go, atau typeclass di Haskell.
// Trait adalah cara Rust melakukan polymorphism.
//
// 🎯 Tujuan: Memahami definisi trait, implementasi, trait bounds,
//    dynamic dispatch, dan berbagai pola trait di Rust.
//
// 💡 Analogi Utama:
// Trait seperti SERTIFIKAT atau LISENSI. Kalau seseorang punya
// sertifikat "Bisa Mengemudi", kita tahu dia bisa menyetir
// meskipun tidak tahu detail mobil apa yang dia punya.
//
// Di Rust, trait mendefinisikan KEMAMPUAN — mirip superhero
// yang punya kekuatan khusus. Setiap tipe bisa "memiliki"
// berbagai trait (kekuatan) sesuai kebutuhan.
// ============================================================

use std::fmt;

// ── MENDEFINISIKAN TRAIT ────────────────────────────────────
// Trait mendeklarasikan method yang HARUS diimplementasikan
// oleh tipe yang mengadopsi trait tersebut.
//
// 💡 Analogi: Trait seperti daftar kemampuan yang harus dimiliki.
//    Kalau mau jadi "Dokter", harus bisa: diagnosa, resep obat,
//    operasi (opsional). Yang wajib = abstract method,
//    yang opsional = default implementation.
trait Ringkasan {
    // Method yang harus diimplementasikan (abstract)
    fn ringkasan(&self) -> String;

    // Method dengan default implementation — bisa di-override
    fn preview(&self) -> String {
        format!("Baca selengkapnya: {}", self.ringkasan())
    }
}

// ── STRUCT YANG IMPLEMENT TRAIT ─────────────────────────────
#[derive(Debug)]
struct Artikel {
    judul: String,
    penulis: String,
    konten: String,
}

#[derive(Debug)]
struct Tweet {
    username: String,
    isi: String,
    retweet: bool,
}

// Implement trait untuk Artikel
impl Ringkasan for Artikel {
    fn ringkasan(&self) -> String {
        format!("{} oleh {}", self.judul, self.penulis)
    }
    // preview() pakai default implementation
}

// Implement trait untuk Tweet
impl Ringkasan for Tweet {
    fn ringkasan(&self) -> String {
        format!("@{}: {}", self.username, self.isi)
    }

    // Override default implementation
    fn preview(&self) -> String {
        if self.retweet {
            format!("🔁 RT: {}", self.ringkasan())
        } else {
            format!("🐦 {}", self.ringkasan())
        }
    }
}

// ── TRAIT SEBAGAI PARAMETER ─────────────────────────────────
// Cara 1: `impl Trait` syntax (sugar syntax, paling umum)
//
// 💡 Analogi: Fungsi ini bilang "saya menerima sesuatu yang
//    bisa diringkas" — tidak peduli apakah Artikel, Tweet, atau
//    tipe lain yang implement Ringkasan.
fn cetak_ringkasan(item: &impl Ringkasan) {
    println!("Preview: {}", item.preview());
}

// Cara 2: Trait bound syntax (lebih eksplisit)
// <T: Ringkasan> artinya T harus implement trait Ringkasan
fn cetak_ringkasan2<T: Ringkasan>(item: &T) {
    println!("Ringkasan: {}", item.ringkasan());
}

// Multiple trait bounds
fn cetak_debug_ringkasan<T: Ringkasan + fmt::Debug>(item: &T) {
    println!("Debug: {:?}", item);
    println!("Ringkasan: {}", item.ringkasan());
}

// `where` clause — lebih rapi untuk bound yang banyak
fn proses_item<T>(item: &T) -> String
where
    T: Ringkasan + fmt::Debug,
{
    format!("[{:?}] → {}", item, item.ringkasan())
}

// ── RETURN `impl Trait` ─────────────────────────────────────
// Fungsi bisa return tipe yang implement trait tertentu
// tanpa perlu menyebutkan tipe konkretnya.
fn buat_tweet_default() -> impl Ringkasan {
    Tweet {
        username: String::from("bot"),
        isi: String::from("Halo dari bot!"),
        retweet: false,
    }
}

// ── TRAIT DENGAN MULTIPLE METHODS ───────────────────────────
trait Bangun2D {
    fn luas(&self) -> f64;
    fn keliling(&self) -> f64;
    fn nama(&self) -> &str;

    // Default method yang menggunakan method lain
    fn info(&self) -> String {
        format!(
            "{}: luas={:.2}, keliling={:.2}",
            self.nama(),
            self.luas(),
            self.keliling()
        )
    }
}

struct Lingkaran {
    radius: f64,
}

struct PersegiPanjang {
    panjang: f64,
    lebar: f64,
}

struct Segitiga {
    alas: f64,
    tinggi: f64,
    sisi_a: f64,
    sisi_b: f64,
    sisi_c: f64,
}

impl Bangun2D for Lingkaran {
    fn luas(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }

    fn keliling(&self) -> f64 {
        2.0 * std::f64::consts::PI * self.radius
    }

    fn nama(&self) -> &str {
        "Lingkaran"
    }
}

impl Bangun2D for PersegiPanjang {
    fn luas(&self) -> f64 {
        self.panjang * self.lebar
    }

    fn keliling(&self) -> f64 {
        2.0 * (self.panjang + self.lebar)
    }

    fn nama(&self) -> &str {
        "Persegi Panjang"
    }
}

impl Bangun2D for Segitiga {
    fn luas(&self) -> f64 {
        0.5 * self.alas * self.tinggi
    }

    fn keliling(&self) -> f64 {
        self.sisi_a + self.sisi_b + self.sisi_c
    }

    fn nama(&self) -> &str {
        "Segitiga"
    }
}

// ── IMPLEMENT STANDARD LIBRARY TRAITS ───────────────────────
// Rust punya banyak trait bawaan yang bisa kita implement
// untuk tipe custom kita.

struct Rupiah(f64);

// Display — untuk format {}
impl fmt::Display for Rupiah {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Rp {:,.0}", self.0)
    }
}

// Debug — untuk format {:?}
impl fmt::Debug for Rupiah {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Rupiah({})", self.0)
    }
}

// PartialEq — untuk operator ==
impl PartialEq for Rupiah {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

// ── DERIVE MACRO ────────────────────────────────────────────
// #[derive(...)] otomatis generate implementasi trait standar
//
// 💡 Analogi: derive seperti "paket hemat" — otomatis dapat
//    beberapa kemampuan tanpa harus tulis manual satu per satu.
//    Hanya untuk trait standar yang compiler bisa generate.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
struct Nilai {
    skor: f64,
    mata_pelajaran: String,
}

// ── TRAIT INHERITANCE (SUPERTRAIT) ──────────────────────────
// Trait bisa "mewarisi" trait lain — untuk menggabungkan kemampuan.
trait Printable: fmt::Display + fmt::Debug {
    fn cetak(&self) {
        println!("Display: {}", self);
        println!("Debug: {:?}", self);
    }
}

// Rupiah sudah implement Display dan Debug, jadi bisa implement Printable
impl Printable for Rupiah {}

fn main() {
    // ── Menggunakan Trait ───────────────────────────────────
    let artikel = Artikel {
        judul: String::from("Belajar Rust"),
        penulis: String::from("Fadlur"),
        konten: String::from("Rust itu keren..."),
    };

    let tweet = Tweet {
        username: String::from("rustlang"),
        isi: String::from("Rust 2026 sudah rilis!"),
        retweet: false,
    };

    let retweet = Tweet {
        username: String::from("dev_id"),
        isi: String::from("Keren!"),
        retweet: true,
    };

    // Panggil method trait
    println!("{}", artikel.ringkasan());
    println!("{}", artikel.preview()); // default impl
    println!("{}", tweet.preview()); // overridden impl
    println!("{}", retweet.preview());

    // ── Trait sebagai parameter ──────────────────────────────
    cetak_ringkasan(&artikel);
    cetak_ringkasan(&tweet);
    cetak_ringkasan2(&artikel);
    cetak_debug_ringkasan(&artikel);
    println!("{}", proses_item(&tweet));

    // ── Return impl Trait ───────────────────────────────────
    let default_tweet = buat_tweet_default();
    println!("Default: {}", default_tweet.ringkasan());

    // ── Bangun 2D ───────────────────────────────────────────
    let lingkaran = Lingkaran { radius: 5.0 };
    let persegi = PersegiPanjang {
        panjang: 10.0,
        lebar: 5.0,
    };
    let segitiga = Segitiga {
        alas: 6.0,
        tinggi: 4.0,
        sisi_a: 5.0,
        sisi_b: 5.0,
        sisi_c: 6.0,
    };

    println!("{}", lingkaran.info());
    println!("{}", persegi.info());
    println!("{}", segitiga.info());

    // ── Display & Custom Format ─────────────────────────────
    let harga = Rupiah(1_500_000.0);
    println!("Harga: {}", harga); // Display
    println!("Debug: {:?}", harga); // Debug
    harga.cetak(); // Printable trait

    // ── Derive ──────────────────────────────────────────────
    let n1 = Nilai {
        skor: 85.0,
        mata_pelajaran: String::from("Matematika"),
    };
    let n2 = n1.clone(); // Clone trait
    println!("n1 == n2? {}", n1 == n2); // PartialEq trait
    println!("{:?}", n1); // Debug trait
}

// ============================================================
// 🧠 RINGKUMAN TRAIT:
//
// ┌─────────────────────────────────────────────────────────────┐
// │                    KONSEP TRAIT                             │
// ├──────────────────┬──────────────────────────────────────────┤
// │ Definisi         │ trait Nama { fn method(&self); }         │
// │ Implementasi     │ impl Trait for Tipe { ... }              │
// │ Parameter        │ fn f(item: &impl Trait)                  │
// │ Generic Bound    │ fn f<T: Trait>(item: &T)                 │
// │ Multiple Bounds  │ T: TraitA + TraitB                       │
// │ Where Clause     │ fn f<T>(x: T) where T: Trait             │
// │ Return Trait     │ fn f() -> impl Trait                     │
// │ Default Method   │ fn method(&self) { ... } dalam trait     │
// │ Supertrait       │ trait Sub: Super { ... }                 │
// └──────────────────┴──────────────────────────────────────────┘
//
// ⚠️ COMMON MISTAKES:
// - Implement trait tanpa semua method wajib → compile error
// - Lupa `&` saat pass by reference ke trait parameter
// - Trait bound tidak terpenuhi → compile error
// - Orphan rule: tidak bisa impl trait asing untuk tipe asing
//
// 🔗 PERBANDINGAN:
// | Rust              | Java             | Go                |
// |-------------------|------------------|-------------------|
// | trait             | interface        | interface         |
// | impl Trait for T  | implements       | (implicit)        |
// | dyn Trait         | interface object │ interface value   |
// | derive            | (Lombok)         │ (struct tags)     |
// ============================================================

// ============================================================
// 🏋️ LATIHAN:
// 1. Buat trait `Hewan` dengan method `suara()` dan `jenis()`.
//    Implement untuk Kucing, Anjing, Burung
// 2. Buat trait `Konversi` dengan method `ke_string()`,
//    `ke_json()`, `ke_csv()`. Implement untuk struct Mahasiswa
// 3. Implement Display untuk struct Matriks 2x2
// 4. Buat trait `Sortable` dan implement sorting kustom
// 5. Buat trait hierarchy: Drawable -> Resizable -> Widget
// ============================================================
