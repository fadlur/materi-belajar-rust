// ============================================================
// 📘 BELAJAR RUST #15 — Modules & Visibility
// ============================================================
// Module system Rust mengorganisir kode menjadi namespace.
// Bisa nested, bisa di file terpisah.
// Default: PRIVATE. Gunakan `pub` untuk membuat public.
//
// 🎯 Tujuan: Memahami sistem modul Rust, visibility rules,
//    path resolution, use statements, dan re-export.
//
// 💡 Analogi Utama:
// Module seperti LANTAI/LANTAI dalam gedung — setiap lantai
// punya ruangan (items) yang bisa publik (lobby) atau privat
// (ruang server). Tamu hanya bisa masuk ruang publik, tapi
// penghuni lantai bisa akses semua ruangan di lantainya.
//
// 🔑 Di Rust, SEMUA item default private — harus eksplisit
// diberi `pub` untuk bisa diakses dari luar module.
// ============================================================

// ── INLINE MODULE ───────────────────────────────────────────
// Module didefinisikan dengan keyword `mod`
mod restoran {
    // Semua item di dalam module default PRIVATE

    // `pub` membuat item bisa diakses dari luar module
    pub fn salam() {
        println!("Selamat datang di Restoran Rust! 🦀");
    }

    // Nested module — module di dalam module
    pub mod depan {
        pub fn terima_tamu() {
            println!("Silakan duduk!");
            // Bisa akses fungsi dari sibling module dengan super::
            super::internal::catat_tamu("Tamu baru");
        }

        pub fn ambil_pesanan(pesanan: &str) {
            println!("Pesanan: {}", pesanan);
            // Panggil fungsi dari module sebelah
            super::belakang::siapkan_makanan(pesanan);
        }
    }

    pub mod belakang {
        pub fn siapkan_makanan(menu: &str) {
            println!("🍳 Menyiapkan: {}", menu);
            cuci_bahan(); // private function — hanya bisa dipanggil di sini
        }

        // Fungsi private — tidak bisa diakses dari luar module `belakang`
        fn cuci_bahan() {
            println!("🧹 Mencuci bahan...");
        }
    }

    // Module private — tidak bisa diakses dari luar `restoran`
    mod internal {
        pub fn catat_tamu(info: &str) {
            println!("📝 [Internal] {}", info);
        }
    }

    // ── STRUCT DENGAN VISIBILITY ────────────────────────────
    // Struct bisa pub, tapi field-nya tetap private secara default!
    pub struct Pesanan {
        pub menu: String,       // public field
        pub jumlah: u32,        // public field
        harga_asli: f64,        // PRIVATE field — tidak bisa diakses langsung
    }

    impl Pesanan {
        // Constructor WAJIB karena ada private field
        pub fn baru(menu: &str, jumlah: u32, harga: f64) -> Pesanan {
            Pesanan {
                menu: menu.to_string(),
                jumlah,
                harga_asli: harga,
            }
        }

        pub fn total(&self) -> f64 {
            self.harga_asli * self.jumlah as f64
        }
    }

    // ── ENUM VISIBILITY ─────────────────────────────────────
    // Kalau enum pub, SEMUA variant-nya otomatis pub!
    // Berbeda dengan struct yang field-nya tetap private
    pub enum StatusMeja {
        Tersedia,
        Terisi,
        Dipesan,
    }
}

// ── USE STATEMENT ───────────────────────────────────────────
// `use` membawa item ke scope saat ini — seperti shortcut/alias.
use restoran::depan;
use restoran::belakang::siapkan_makanan;

// ── MODULE LAIN: MATEMATIKA ─────────────────────────────────
mod matematika {
    pub mod dasar {
        pub fn tambah(a: f64, b: f64) -> f64 {
            a + b
        }

        pub fn kurang(a: f64, b: f64) -> f64 {
            a - b
        }

        pub fn kali(a: f64, b: f64) -> f64 {
            a * b
        }

        pub fn bagi(a: f64, b: f64) -> Result<f64, String> {
            if b == 0.0 {
                Err(String::from("Tidak bisa bagi dengan 0"))
            } else {
                Ok(a / b)
            }
        }
    }

    pub mod lanjut {
        pub fn pangkat(base: f64, exp: u32) -> f64 {
            base.powi(exp as i32)
        }

        pub fn akar(n: f64) -> f64 {
            n.sqrt()
        }

        pub fn faktorial(n: u64) -> u64 {
            match n {
                0 | 1 => 1,
                _ => n * faktorial(n - 1),
            }
        }
    }

    // Re-export: buat item tersedia di level yang lebih tinggi
    // Sekarang bisa akses matematika::tambah (tanpa dasar::)
    pub use dasar::tambah;
    pub use dasar::kurang;
}

// ── USE DENGAN ALIAS ────────────────────────────────────────
use matematika::lanjut as mat_lanjut;
use matematika::tambah; // re-exported dari matematika
use matematika::kurang;

// ── MODULE UNTUK CONTOH: MINI LIBRARY ───────────────────────
mod koleksi {
    // Generic stack yang kita buat sebelumnya
    pub struct Stack<T> {
        data: Vec<T>,
    }

    impl<T> Stack<T> {
        pub fn new() -> Self {
            Stack { data: Vec::new() }
        }

        pub fn push(&mut self, item: T) {
            self.data.push(item);
        }

        pub fn pop(&mut self) -> Option<T> {
            self.data.pop()
        }

        pub fn peek(&self) -> Option<&T> {
            self.data.last()
        }

        pub fn len(&self) -> usize {
            self.data.len()
        }

        pub fn is_empty(&self) -> bool {
            self.data.is_empty()
        }
    }

    // Implement Debug secara manual
    impl<T: std::fmt::Debug> std::fmt::Debug for Stack<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "Stack{:?}", self.data)
        }
    }
}

fn main() {
    println!("=== RESTORAN ===");

    // Panggil fungsi module
    restoran::salam();

    // Menggunakan `use`
    depan::terima_tamu();
    depan::ambil_pesanan("Nasi Goreng");
    siapkan_makanan("Es Teh"); // langsung, karena sudah di-use

    // Struct dengan private field
    let pesanan = restoran::Pesanan::baru("Bakso", 2, 15000.0);
    println!("Menu: {}, Jumlah: {}, Total: Rp {}", pesanan.menu, pesanan.jumlah, pesanan.total());
    // println!("{}", pesanan.harga_asli); // ❌ ERROR! private field

    // Enum — semua variant public
    let status = restoran::StatusMeja::Tersedia;
    match status {
        restoran::StatusMeja::Tersedia => println!("Meja tersedia!"),
        restoran::StatusMeja::Terisi => println!("Meja terisi"),
        restoran::StatusMeja::Dipesan => println!("Meja sudah dipesan"),
    }

    println!("\n=== MATEMATIKA ===");

    // Menggunakan re-export
    println!("2 + 3 = {}", tambah(2.0, 3.0));
    println!("10 - 4 = {}", kurang(10.0, 4.0));

    // Menggunakan path lengkap
    println!("5 × 6 = {}", matematika::dasar::kali(5.0, 6.0));
    println!("10 ÷ 3 = {:.2}", matematika::dasar::bagi(10.0, 3.0).unwrap());

    // Menggunakan alias
    println!("2^10 = {}", mat_lanjut::pangkat(2.0, 10));
    println!("√144 = {}", mat_lanjut::akar(144.0));
    println!("10! = {}", mat_lanjut::faktorial(10));

    println!("\n=== KOLEKSI ===");

    let mut stack = koleksi::Stack::new();
    stack.push("satu");
    stack.push("dua");
    stack.push("tiga");
    println!("{:?}", stack);
    println!("Pop: {:?}", stack.pop());
    println!("{:?}", stack);

    // ── PATH TYPES ──────────────────────────────────────────
    // Ada beberapa cara merujuk ke item di module:
    //
    // 1. Absolute path dari crate root:
    //    crate::restoran::salam();
    //
    // 2. Relative path dari module saat ini:
    //    restoran::salam();
    //
    // 3. `self::` — relative dari module ini:
    //    self::restoran::salam();
    //
    // 4. `super::` — parent module:
    //    super::some_function(); (dari dalam nested module)

    // ── USE BEST PRACTICES ──────────────────────────────────
    // Untuk fungsi: use sampai parent module
    //   use restoran::depan;
    //   depan::terima_tamu(); // jelas dari mana asalnya
    //
    // Untuk struct/enum: use langsung item-nya
    //   use std::collections::HashMap;
    //   let map = HashMap::new();
    //
    // Untuk nama yang konflik: use dengan alias
    //   use std::fmt::Result as FmtResult;
    //   use std::io::Result as IoResult;
}

// ============================================================
// 🧠 RINGKUMAN MODULE SYSTEM:
//
// ┌─────────────────────────────────────────────────────────────┐
// │                    VISIBILITY RULES                         │
// ├──────────────────┬──────────────────────────────────────────┤
// │ pub              │ Public — bisa diakses dari mana saja     │
// │ (default)        │ Private — hanya module dan children     │
// │ pub(crate)       │ Hanya di crate ini                      │
// │ pub(super)       │ Hanya parent module                     │
// │ pub(in path)     │ Hanya di path tertentu                  │
// └──────────────────┴──────────────────────────────────────────┘
//
// ┌─────────────────────────────────────────────────────────────┐
// │                    PATH RESOLUTION                          │
// ├──────────────────┬──────────────────────────────────────────┤
// │ crate::          │ Absolute path dari crate root           │
// │ self::           │ Relative dari module saat ini           │
// │ super::          │ Parent module                           │
// │ ::               │ Crate root (external crate)             │
// └──────────────────┴──────────────────────────────────────────┘
//
// ⚠️ COMMON MISTAKES:
// - Lupa `pub` pada item yang perlu diakses luar → compile error
// - Lupa `pub` pada field struct → field private
// - Circular module dependency → compile error
// - Path salah → "not found in this scope"
//
// 🔗 FILE STRUCTURE UNTUK PROJECT BESAR:
// src/
// ├── main.rs          (crate root)
// ├── restoran.rs      (atau restoran/mod.rs)
// ├── restoran/
// │   ├── depan.rs
// │   └── belakang.rs
// └── matematika.rs
//
// Di main.rs: `mod restoran;` (tanpa body — Rust cari file-nya)
// Di restoran.rs: `pub mod depan;` (tanpa body — cari file di restoran/)
// ============================================================

// ============================================================
// 🏋️ LATIHAN:
// 1. Buat module `bank` dengan sub-module `atm` dan `teller`
//    Setiap sub-module punya fungsi untuk tarik/setor uang
// 2. Buat struct `Rekening` dengan field private `saldo`
//    Expose hanya method `setor()`, `tarik()`, `cek_saldo()`
// 3. Re-export item dari nested module ke level yang lebih tinggi
// 4. Buat module terpisah di file yang berbeda (opsional)
// 5. Buat module `utils` dengan berbagai helper function
// 6. Eksperimen dengan pub(crate) dan pub(super)
// ============================================================
