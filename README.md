# 🦀 Belajar Rust - Hands-On Tutorial

Tutorial Rust dari level **Basic** sampai **Expert** dengan pendekatan hands-on.

## 🎯 Untuk Siapa Tutorial Ini?

- **Pemula pemrograman** yang ingin belajar Rust sebagai bahasa pertama
- **Developer berpengalaman** dari bahasa lain (Python, JavaScript, Java, Go, C/C++) yang ingin pindah ke Rust
- **Mahasiswa** yang ingin memahami konsep sistem programming modern
- **Siapa saja** yang penasaran kenapa Rust begitu dicintai developer!

## 🧠 Mengapa Rust?

| Fitur | Rust | Bahasa Lain |
|-------|------|-------------|
| Memory Safety | ✅ Compile-time guarantee | ❌ Runtime errors (C/C++) atau GC pause (Java/Go) |
| Performance | ✅ Secepat C/C++ | ❌ Interpreted atau GC overhead |
| Concurrency | ✅ Fearless concurrency | ❌ Race condition sulit di-debug |
| Type System | ✅ Expressive & aman | ❌ Null pointer exception |
| Tooling | ✅ Cargo, rustfmt, clippy | ❌ Fragmented |

## 📖 Cara Belajar (PENTING!)

### Metode yang Direkomendasikan

1. **Baca file `.rs` di setiap folder secara berurutan** — jangan skip!
2. **Baca KOMENTAR dengan cermat** — komentar ditulis dalam Bahasa Indonesia dengan penjelasan detail
3. **Ketik ulang kode tersebut (JANGAN copy-paste!)** di `src/main.rs`
4. **Jalankan dengan `cargo run`** — lihat output di terminal
5. **Eksperimen: ubah-ubah kode, lihat apa yang terjadi** — ini cara terbaik belajar!

### 💡 Tips Belajar Efektif

- **Jangan buru-buru!** Pahami setiap konsep sebelum lanjut
- **Ownership & Borrowing adalah fondasi Rust** — luangkan waktu ekstra di situ
- **Kalau error, baca pesan error dengan cermat** — compiler Rust adalah "guru" terbaikmu
- **Praktikkan dengan menulis kode**, bukan hanya membaca
- **Jangan menyerah saat stuck** — konsep Rust memang berbeda tapi sepadan!

### 🔄 Alur Belajar yang Direkomendasikan

```
01_hello_world (30 menit)
    ↓
02_variabel_tipe_data (1 jam)
    ↓
03_fungsi (1 jam)
    ↓
04_control_flow (1 jam)
    ↓
⭐ 05_ownership (2-3 jam) — KONSEP PALING PENTING!
    ↓
06_references_borrowing (2 jam)
    ↓
07_slice (1 jam)
    ↓
08_string (1 jam)
    ↓
01_struct (1.5 jam)
    ↓
02_enum_pattern_matching (2 jam)
    ↓
03_collections (1.5 jam)
    ↓
04_error_handling (2 jam)
    ↓
05_traits (2 jam)
    ↓
06_generics (1.5 jam)
    ↓
07_modules (1 jam)
    ↓
08_closures (1.5 jam)
    ↓
[Advanced & Expert — sesuai minat]
```

**Total waktu estimasi untuk Basic-Intermediate: ~20-25 jam**

## 🚀 Cara Menjalankan

### Prasyarat

- Install Rust: https://rustup.rs/
- Pastikan `cargo` tersedia di PATH

### Menjalankan File Latihan

```bash
# 1. Copy isi file latihan ke src/main.rs
# 2. Jalankan:
cargo run

# Atau jalankan langsung file tertentu:
rustc 01_basic/01_hello_world/main.rs -o output && ./output
```

### Membuat Project Baru untuk Eksperimen

```bash
# Buat project baru
cargo new latihan-saya
cd latihan-saya

# Edit src/main.rs, lalu:
cargo run
```

## 📚 Struktur Tutorial

### 📗 01_basic (Pemula)
Konsep fundamental yang HARUS dikuasai sebelum lanjut.

| # | Folder | Topik | Estimasi |
|---|--------|-------|----------|
| 1 | `01_hello_world` | Program pertama, macro println! | 30 menit |
| 2 | `02_variabel_tipe_data` | Variabel, mutability, tipe data scalar & compound | 1 jam |
| 3 | `03_fungsi` | Fungsi, parameter, return value, expression vs statement | 1 jam |
| 4 | `04_control_flow` | if/else, loop, while, for, match | 1 jam |
| 5 | `05_ownership` | ⭐ Konsep unik Rust: ownership, move, copy, clone | 2-3 jam |
| 6 | `06_references_borrowing` | References, mutable references, aturan borrowing | 2 jam |
| 7 | `07_slice` | String slice, array slice, &str vs String | 1 jam |
| 8 | `08_string` | String manipulation, UTF-8, methods | 1 jam |

### 📘 02_intermediate (Menengah)
Mulai membangun abstraksi dan menggunakan fitur powerful Rust.

| # | Folder | Topik | Estimasi |
|---|--------|-------|----------|
| 9 | `01_struct` | Struct, method, associated functions | 1.5 jam |
| 10 | `02_enum_pattern_matching` | Enum, Option<T>, Result<T,E>, match | 2 jam |
| 11 | `03_collections` | Vec, HashMap, HashSet | 1.5 jam |
| 12 | `04_error_handling` | panic!, Result, ?, custom errors | 2 jam |
| 13 | `05_traits` | Interface Rust: definisi, implementasi, bounds | 2 jam |
| 14 | `06_generics` | Generic functions, structs, monomorphization | 1.5 jam |
| 15 | `07_modules` | Sistem module, visibility, use | 1 jam |
| 16 | `08_closures` | Lambda, capturing environment, Fn/FnMut/FnOnce | 1.5 jam |

### 📙 03_advanced (Lanjutan)
Konsep yang membuat Rust unik dan powerful.

| # | Folder | Topik | Estimasi |
|---|--------|-------|----------|
| 17 | `01_lifetimes` | Lifetime annotations, elision, 'static | 2 jam |
| 18 | `02_iterators` | Iterator trait, adaptors, custom iterators | 1.5 jam |
| 19 | `03_smart_pointers` | Box, Rc, RefCell, Deref, Drop | 2 jam |
| 20 | `04_concurrency` | Thread, Mutex, Channel, Arc | 2 jam |
| 21 | `05_trait_objects` | Dynamic dispatch, dyn Trait, object safety | 1.5 jam |
| 22 | `06_advanced_traits` | Associated types, operator overloading, blanket impl | 2 jam |

### 📕 04_expert (Ahli)
Topik untuk production-grade Rust development.

| # | Folder | Topik | Estimasi |
|---|--------|-------|----------|
| 23 | `01_async_await` | Asynchronous programming dengan Tokio | 3 jam |
| 24 | `02_macros` | Declarative macros (macro_rules!) | 2 jam |
| 25 | `03_unsafe_rust` | Raw pointers, FFI, unsafe traits | 1.5 jam |
| 26 | `04_design_patterns` | Builder, Strategy, Observer, Repository | 2 jam |
| 27 | `05_mini_project` | Project akhir: CLI Todo App | 3 jam |

## 🗺️ Peta Konsep Rust

```
                    ┌─────────────────┐
                    │   Ownership     │  ← Fondasi Segalanya
                    │   & Borrowing   │
                    └────────┬────────┘
                             │
        ┌────────────────────┼────────────────────┐
        │                    │                    │
        ▼                    ▼                    ▼
   ┌─────────┐        ┌──────────┐        ┌──────────┐
   │ Struct  │        │  Enums   │        │  Traits  │
   │ & Methods│        │ & Pattern│        │ & Generics│
   └─────────┘        │ Matching │        └──────────┘
                      └──────────┘
                             │
        ┌────────────────────┼────────────────────┐
        │                    │                    │
        ▼                    ▼                    ▼
   ┌─────────┐        ┌──────────┐        ┌──────────┐
   │Collections│       │ Error    │        │ Lifetimes │
   │ Vec, Map │        │ Handling │        │           │
   └─────────┘        └──────────┘        └──────────┘
                             │
                             ▼
                    ┌─────────────────┐
                    │  Concurrency    │
                    │  & Smart Ptrs   │
                    └─────────────────┘
```

## 🔑 Kata Kunci Penting

| Konsep | Penjelasan Singkat |
|--------|-------------------|
| **Ownership** | Setiap data punya satu pemilik; saat pemilik keluar scope, data dihapus |
| **Borrowing** | Meminjam data tanpa mengambil ownership; ada immutable & mutable borrow |
| **Lifetime** | Menjamin reference selalu valid; dicek saat compile time |
| **Trait** | Kontrak/kemampuan yang bisa dimiliki tipe; mirip interface |
| **Generics** | Kode yang bekerja untuk banyak tipe tanpa duplikasi |
| **Pattern Matching** | Cara powerful untuk memeriksa dan destructuring data |
| **Zero-Cost Abstraction** | Fitur high-level yang tidak ada runtime overhead |

## 📖 Referensi Tambahan

- [The Rust Programming Language Book](https://doc.rust-lang.org/book/) — Buku resmi, gratis
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/) — Belajar melalui contoh kode
- [Rustlings](https://github.com/rust-lang/rustlings) — Latihan interaktif di terminal
- [Exercism Rust Track](https://exercism.org/tracks/rust) — Latihan dengan mentoring
- [Crates.io](https://crates.io) — Repository library Rust
- [Rust Cheat Sheet](https://cheats.rs/) — Referensi cepat

## 🤝 Komunitas

- **Rust Indonesia** — Cari di Telegram/Discord
- **r/rust** — Subreddit Rust
- **Rust Discord** — discord.gg/rust-lang

---

> 💡 **Tips Terakhir**: Rust memiliki kurva belajar yang tajam di awal, tapi setelah kamu menguasai ownership dan borrowing, sisanya akan terasa sangat natural. Jangan menyerah — komunitas Rust sangat supportive!

> 🦀 **Selamat Belajar!** Semangat menjadi Rustacean!
