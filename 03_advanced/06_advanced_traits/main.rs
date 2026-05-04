// ============================================================
// 📙 BELAJAR RUST #22 — Advanced Traits
// ============================================================
// Trait di Rust punya fitur lanjutan yang sangat powerful:
// - Associated Types
// - Operator Overloading
// - Supertraits
// - Newtype Pattern
// - Blanket Implementations
//
// 🎯 Tujuan: Memahami fitur-fitur advanced trait dan
//    kapan menggunakannya dalam desain API.
//
// 💡 Analogi Utama:
// Advanced traits seperti fitur premium pada membership —
//    associated types = "profil kustom", operator overloading =
//    "shortcut khusus", supertraits = "prasyarat", blanket impl =
//    "benefit otomatis untuk semua anggota".
// ============================================================

use std::fmt;
use std::ops::{Add, Mul, Neg};

// ══════════════════════════════════════════════════════════════
// ASSOCIATED TYPES — Tipe yang terkait dengan trait
// ══════════════════════════════════════════════════════════════

// Associated type lebih bersih dari generic pada trait.
//
// 💡 Perbedaan:
//   Generic:    trait Iterator<T> { fn next(&mut self) -> Option<T>; }
//   Associated: trait Iterator { type Item; fn next(&mut self) -> Option<Self::Item>; }
//
// Kenapa associated type lebih baik? Karena satu tipe hanya bisa
// punya SATU implementasi Iterator — tidak mungkin punya dua
// Item type berbeda. Generic memungkinkan <T> berbeda, yang
// kadang tidak masuk akal.

trait Koleksi {
    type Item;                              // associated type
    fn tambah(&mut self, item: Self::Item);
    fn ambil(&self, index: usize) -> Option<&Self::Item>;
    fn panjang(&self) -> usize;
}

struct Daftar<T> {
    items: Vec<T>,
}

impl<T> Koleksi for Daftar<T> {
    type Item = T; // specify associated type

    fn tambah(&mut self, item: T) {
        self.items.push(item);
    }

    fn ambil(&self, index: usize) -> Option<&T> {
        self.items.get(index)
    }

    fn panjang(&self) -> usize {
        self.items.len()
    }
}

// ══════════════════════════════════════════════════════════════
// OPERATOR OVERLOADING — Operator +, -, *, dll pada custom type
// ══════════════════════════════════════════════════════════════

#[derive(Debug, Clone, Copy, PartialEq)]
struct Vektor2D {
    x: f64,
    y: f64,
}

impl Vektor2D {
    fn new(x: f64, y: f64) -> Self {
        Vektor2D { x, y }
    }

    fn magnitude(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }
}

// Operator + (Add trait)
impl Add for Vektor2D {
    type Output = Vektor2D; // associated type: tipe hasil penjumlahan

    fn add(self, other: Vektor2D) -> Vektor2D {
        Vektor2D {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

// Operator * (Mul trait) — scalar multiplication
impl Mul<f64> for Vektor2D {
    type Output = Vektor2D;

    fn mul(self, scalar: f64) -> Vektor2D {
        Vektor2D {
            x: self.x * scalar,
            y: self.y * scalar,
        }
    }
}

// Operator - (Neg trait) — negasi
impl Neg for Vektor2D {
    type Output = Vektor2D;

    fn neg(self) -> Vektor2D {
        Vektor2D {
            x: -self.x,
            y: -self.y,
        }
    }
}

// Display
impl fmt::Display for Vektor2D {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({:.1}, {:.1})", self.x, self.y)
    }
}

// ══════════════════════════════════════════════════════════════
// MATRIX DENGAN OPERATOR OVERLOADING
// ══════════════════════════════════════════════════════════════

#[derive(Debug, Clone, PartialEq)]
struct Matrix2x2 {
    data: [[f64; 2]; 2],
}

impl Matrix2x2 {
    fn new(a: f64, b: f64, c: f64, d: f64) -> Self {
        Matrix2x2 {
            data: [[a, b], [c, d]],
        }
    }

    fn identity() -> Self {
        Matrix2x2::new(1.0, 0.0, 0.0, 1.0)
    }

    fn determinant(&self) -> f64 {
        self.data[0][0] * self.data[1][1] - self.data[0][1] * self.data[1][0]
    }
}

impl Add for Matrix2x2 {
    type Output = Matrix2x2;

    fn add(self, other: Matrix2x2) -> Matrix2x2 {
        Matrix2x2::new(
            self.data[0][0] + other.data[0][0],
            self.data[0][1] + other.data[0][1],
            self.data[1][0] + other.data[1][0],
            self.data[1][1] + other.data[1][1],
        )
    }
}

impl Mul for Matrix2x2 {
    type Output = Matrix2x2;

    fn mul(self, other: Matrix2x2) -> Matrix2x2 {
        let a = &self.data;
        let b = &other.data;
        Matrix2x2::new(
            a[0][0] * b[0][0] + a[0][1] * b[1][0],
            a[0][0] * b[0][1] + a[0][1] * b[1][1],
            a[1][0] * b[0][0] + a[1][1] * b[1][0],
            a[1][0] * b[0][1] + a[1][1] * b[1][1],
        )
    }
}

impl fmt::Display for Matrix2x2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "| {:.1}  {:.1} |\n| {:.1}  {:.1} |",
            self.data[0][0], self.data[0][1], self.data[1][0], self.data[1][1]
        )
    }
}

// ══════════════════════════════════════════════════════════════
// NEWTYPE PATTERN — Wrapper untuk implement trait pada foreign type
// ══════════════════════════════════════════════════════════════

// Rust tidak mengizinkan implement trait asing pada tipe asing
// (orphan rule). Solusi: bungkus dengan newtype!
//
// 💡 Analogi: Newtype seperti bungkus kado — barangnya sama,
//    tapi sekarang punya label (tipe) baru yang bisa kita
//    kasih kemampuan (trait) sendiri.

struct Rupiah(f64);

impl fmt::Display for Rupiah {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Format angka dengan pemisah ribuan
        let s = format!("{:.0}", self.0);
        let bytes: Vec<u8> = s.bytes().collect();
        let mut result = String::new();
        for (i, &b) in bytes.iter().enumerate() {
            if i > 0 && (bytes.len() - i) % 3 == 0 {
                result.push('.');
            }
            result.push(b as char);
        }
        write!(f, "Rp {}", result)
    }
}

impl Add for Rupiah {
    type Output = Rupiah;
    fn add(self, other: Rupiah) -> Rupiah {
        Rupiah(self.0 + other.0)
    }
}

// Newtype untuk Vec<String> agar bisa implement Display
struct DaftarBelanja(Vec<String>);

impl fmt::Display for DaftarBelanja {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "🛒 Daftar Belanja:")?;
        for (i, item) in self.0.iter().enumerate() {
            writeln!(f, "  {}. {}", i + 1, item)?;
        }
        Ok(())
    }
}

// ══════════════════════════════════════════════════════════════
// BLANKET IMPLEMENTATION — Implement trait untuk semua T: Trait
// ══════════════════════════════════════════════════════════════

// Blanket impl: semua tipe yang implement Display otomatis dapat Ringkasan!
trait Ringkasan {
    fn ringkasan(&self) -> String;
}

impl<T: fmt::Display> Ringkasan for T {
    fn ringkasan(&self) -> String {
        let s = format!("{}", self);
        if s.len() > 20 {
            format!("{}...", &s[..20])
        } else {
            s
        }
    }
}

// ══════════════════════════════════════════════════════════════
// FULLY QUALIFIED SYNTAX — Disambiguasi method name
// ══════════════════════════════════════════════════════════════

trait Pilot {
    fn terbang(&self);
}

trait Wizard {
    fn terbang(&self);
}

struct Manusia {
    nama: String,
}

impl Manusia {
    fn terbang(&self) {
        println!("{} melambai-lambaikan tangan", self.nama);
    }
}

impl Pilot for Manusia {
    fn terbang(&self) {
        println!("{} menerbangkan pesawat ✈️", self.nama);
    }
}

impl Wizard for Manusia {
    fn terbang(&self) {
        println!("{} terbang dengan sihir 🧙", self.nama);
    }
}

fn main() {
    // ── Associated Types ────────────────────────────────────
    let mut daftar: Daftar<String> = Daftar { items: Vec::new() };
    daftar.tambah(String::from("Rust"));
    daftar.tambah(String::from("Go"));
    daftar.tambah(String::from("Python"));
    println!("Item ke-1: {:?}", daftar.ambil(1));
    println!("Panjang: {}", daftar.panjang());

    // ── Operator Overloading ────────────────────────────────
    println!("\n=== Vektor 2D ===");
    let v1 = Vektor2D::new(3.0, 4.0);
    let v2 = Vektor2D::new(1.0, 2.0);

    println!("v1 = {}", v1);
    println!("v2 = {}", v2);
    println!("v1 + v2 = {}", v1 + v2);
    println!("v1 * 2.5 = {}", v1 * 2.5);
    println!("-v1 = {}", -v1);
    println!("|v1| = {:.2}", v1.magnitude());

    // ── Matrix ──────────────────────────────────────────────
    println!("\n=== Matrix 2x2 ===");
    let m1 = Matrix2x2::new(1.0, 2.0, 3.0, 4.0);
    let m2 = Matrix2x2::new(5.0, 6.0, 7.0, 8.0);
    let identity = Matrix2x2::identity();

    println!("M1:\n{}", m1);
    println!("M2:\n{}", m2);
    println!("M1 + M2:\n{}", m1.clone() + m2.clone());
    println!("M1 * M2:\n{}", m1.clone() * m2);
    println!("M1 * I:\n{}", m1.clone() * identity);
    println!("det(M1) = {}", m1.determinant());

    // ── Newtype Pattern ─────────────────────────────────────
    println!("\n=== Newtype ===");
    let harga1 = Rupiah(1_500_000.0);
    let harga2 = Rupiah(750_000.0);
    let total = Rupiah(1_500_000.0) + Rupiah(750_000.0);
    println!("Harga 1: {}", harga1);
    println!("Harga 2: {}", harga2);
    println!("Total: {}", total);

    let belanja = DaftarBelanja(vec![
        "Beras 5kg".to_string(),
        "Minyak goreng".to_string(),
        "Telur 1 kg".to_string(),
        "Gula pasir".to_string(),
    ]);
    println!("{}", belanja);

    // ── Blanket Implementation ──────────────────────────────
    println!("=== Blanket Impl ===");
    println!("{}", 42_i32.ringkasan());
    println!("{}", "Halo dunia dari Rust!".ringkasan());
    println!(
        "{}",
        "Ini adalah kalimat yang sangat panjang untuk menguji ringkasan".ringkasan()
    );

    // ── Fully Qualified Syntax ──────────────────────────────
    println!("\n=== Disambiguation ===");
    let orang = Manusia {
        nama: "Budi".to_string(),
    };

    orang.terbang(); // panggil method Manusia
    Pilot::terbang(&orang); // panggil method Pilot
    Wizard::terbang(&orang); // panggil method Wizard

    // Fully qualified syntax (paling eksplisit)
    <Manusia as Pilot>::terbang(&orang);
    <Manusia as Wizard>::terbang(&orang);
}

// ============================================================
// 🧠 RINGKUMAN ADVANCED TRAITS:
//
// ┌─────────────────────────────────────────────────────────────┐
// │                    FITUR LANJUTAN TRAIT                     │
// ├──────────────────┬──────────────────────────────────────────┤
// │ Associated Types │ type Item; dalam trait                  │
// │                  │ Lebih bersih dari generic                │
// ├──────────────────┼──────────────────────────────────────────┤
// │ Operator Overload│ Add, Mul, Neg, Sub, Div, dll            │
// │                  │ type Output = ...                        │
// ├──────────────────┼──────────────────────────────────────────┤
// │ Newtype Pattern  │ struct Wrapper(T)                       │
// │                  │ Bypass orphan rule                       │
// ├──────────────────┼──────────────────────────────────────────┤
// │ Blanket Impl     │ impl<T: TraitA> TraitB for T {}         │
// │                  │ Auto-implement untuk semua T             │
// ├──────────────────┼──────────────────────────────────────────┤
// │ Supertrait       │ trait Sub: Super { ... }                │
// │                  │ Trait memerlukan trait lain              │
// ├──────────────────┼──────────────────────────────────────────┤
// │ Fully Qualified  │ <T as Trait>::method(&self)             │
// │ Syntax           │ Disambiguasi nama method                 │
// └──────────────────┴──────────────────────────────────────────┘
//
// ⚠️ COMMON MISTAKES:
// - Orphan rule violation → newtype needed
// - Conflicting blanket impl → design issue
// - Operator overloading yang tidak intuitif
// - Lupa Clone/Copy saat operator consuming
//
// 🔗 PERBANDINGAN:
// | Rust              | C++              | Python            |
// |-------------------|------------------|-------------------|
// | operator+         | operator+        │ __add__           │
// | Add trait         │ operator overloading│ dunder methods │
// | newtype           │ wrapper class    │ subclass/wrapper  │
// | blanket impl      │ (no equivalent)  │ mixin/ABC         │
// ============================================================

// ============================================================
// 🏋️ LATIHAN:
// 1. Buat struct `Pecahan` (numerator, denominator) dengan operator
//    +, -, *, / dan Display
// 2. Implementasikan Vektor3D dengan cross product dan dot product
// 3. Buat newtype `Email(String)` dengan validasi
// 4. Buat trait `Serializable` dengan associated type `Output`
// 5. Implementasikan Index trait untuk custom collection
// 6. Buat blanket impl: semua tipe yang Debug otomatis Printable
// ============================================================
