# 🦀 Belajar Rust - Hands-On Tutorial

Tutorial Rust dari level **Basic** sampai **Expert** dengan pendekatan hands-on.

## Cara Belajar

1. Buka file `.rs` di setiap folder secara berurutan
2. Baca komentar di setiap baris kode — komentar ditulis dalam Bahasa Indonesia
3. Ketik ulang kode tersebut (JANGAN copy-paste!) di `src/main.rs`
4. Jalankan dengan `cargo run`
5. Eksperimen: ubah-ubah kode, lihat apa yang terjadi

## Cara Menjalankan

```bash
# Jalankan file utama
cargo run

# Atau jalankan file tertentu sebagai contoh:
# 1. Copy isi file latihan ke src/main.rs
# 2. cargo run

# Atau jalankan langsung file tertentu:
rustc 01_basic/01_hello_world/main.rs -o output && ./output
```

## Struktur Tutorial

### 📗 01_basic (Pemula)
- `01_hello_world` — Program pertama, macro println!
- `02_variabel_tipe_data` — Variabel, mutability, tipe data
- `03_fungsi` — Fungsi, parameter, return value
- `04_control_flow` — if/else, loop, while, for
- `05_ownership` — ⭐ Konsep unik Rust: ownership
- `06_references_borrowing` — References & borrowing
- `07_slice` — Slice: referensi ke sebagian data
- `08_string` — String vs &str, manipulasi string

### 📘 02_intermediate (Menengah)
- `01_struct` — Struct dan method
- `02_enum_pattern_matching` — Enum, Option, Result, match
- `03_collections` — Vec, HashMap, HashSet
- `04_error_handling` — Error handling yang proper
- `05_traits` — Traits (mirip interface)
- `06_generics` — Generic types & functions
- `07_modules` — Sistem module & visibility
- `08_closures` — Closures (anonymous functions)

### 📙 03_advanced (Lanjutan)
- `01_lifetimes` — Lifetime annotations
- `02_iterators` — Iterator & iterator adaptors
- `03_smart_pointers` — Box, Rc, RefCell
- `04_concurrency` — Thread, Mutex, Channel
- `05_trait_objects` — Dynamic dispatch
- `06_advanced_traits` — Associated types, operator overloading

### 📕 04_expert (Ahli)
- `01_async_await` — Asynchronous programming
- `02_macros` — Declarative & procedural macros
- `03_unsafe_rust` — Unsafe Rust
- `04_design_patterns` — Design patterns di Rust
- `05_mini_project` — Project akhir: CLI Todo App

---

> 💡 **Tips**: Jangan buru-buru! Pahami setiap konsep sebelum lanjut.
> Ownership & Borrowing adalah fondasi Rust — luangkan waktu ekstra di situ.
