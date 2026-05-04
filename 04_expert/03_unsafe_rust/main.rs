// ============================================================
// 📕 BELAJAR RUST #25 — Unsafe Rust
// ============================================================
// "Safe" Rust menjamin memory safety saat compile time.
// "Unsafe" Rust menonaktifkan BEBERAPA jaminan tersebut.
//
// ⚠️ Unsafe BUKAN berarti "bahaya" — artinya programmer yang
// bertanggung jawab atas keamanan, bukan compiler.
//
// 5 hal yang HANYA bisa dilakukan di unsafe block:
// 1. Dereference raw pointer
// 2. Panggil unsafe function/method
// 3. Akses/modify mutable static variable
// 4. Implement unsafe trait
// 5. Akses field dari union
//
// 🎯 Tujuan: Memahami kapan dan bagaimana menggunakan unsafe
//    dengan benar, serta cara membuat safe abstraction di atasnya.
//
// 💡 Analogi Utama:
// Safe Rust seperti mengemudi dengan autopilot — sistem
//    mengontrol hampir semuanya. Unsafe Rust seperti mengemudi
//    manual — kamu punya kontrol penuh, tapi kesalahan bisa
//    fatal. Tujuannya: gunakan manual hanya saat perlu,
//    dan selalu bungkus dengan safety features.
// ============================================================

use std::slice;

// ══════════════════════════════════════════════════════════════
// 1. RAW POINTERS — *const T dan *mut T
// ══════════════════════════════════════════════════════════════

fn raw_pointer_demo() {
    println!("=== Raw Pointers ===");

    let mut angka = 42;

    // Membuat raw pointer — SAFE! (hanya membuat, tidak dereference)
    let ptr_r = &angka as *const i32;  // immutable raw pointer
    let ptr_w = &mut angka as *mut i32; // mutable raw pointer

    println!("Alamat ptr_r: {:?}", ptr_r);
    println!("Alamat ptr_w: {:?}", ptr_w);

    // Dereference raw pointer — UNSAFE!
    unsafe {
        println!("Nilai ptr_r: {}", *ptr_r);
        println!("Nilai ptr_w: {}", *ptr_w);

        // Modify melalui mutable raw pointer
        *ptr_w = 100;
        println!("Setelah modify: {}", *ptr_w);
    }

    // Raw pointer bisa menunjuk ke alamat sembarang (bahaya!)
    // let arbitrary = 0x012345usize as *const i32;
    // unsafe { println!("{}", *arbitrary); } // 💥 Kemungkinan SEGFAULT!
}

// ══════════════════════════════════════════════════════════════
// 2. UNSAFE FUNCTIONS
// ══════════════════════════════════════════════════════════════

// Fungsi unsafe — pemanggil HARUS dalam unsafe block
unsafe fn operasi_berbahaya() -> i32 {
    // Di sini kita bisa melakukan hal-hal unsafe
    42
}

// Contoh: membuat safe abstraction di atas unsafe code
fn split_at_mut_custom(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = values.len();
    let ptr = values.as_mut_ptr(); // mendapat raw pointer

    assert!(mid <= len, "Index out of bounds!");

    // Borrow checker tidak mengizinkan dua mutable reference ke slice yang sama
    // Tapi kita TAHU kedua bagian tidak overlap — jadi aman!
    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}

fn unsafe_function_demo() {
    println!("\n=== Unsafe Functions ===");

    // Panggil unsafe function
    unsafe {
        let val = operasi_berbahaya();
        println!("Hasil unsafe fn: {}", val);
    }

    // Safe wrapper
    let mut data = vec![1, 2, 3, 4, 5, 6];
    let (kiri, kanan) = split_at_mut_custom(&mut data, 3);
    println!("Kiri: {:?}", kiri);   // [1, 2, 3]
    println!("Kanan: {:?}", kanan); // [4, 5, 6]

    // Modify bagian kiri
    kiri[0] = 100;
    println!("Kiri setelah modify: {:?}", kiri);
}

// ══════════════════════════════════════════════════════════════
// 3. EXTERN — FFI (Foreign Function Interface)
// ══════════════════════════════════════════════════════════════

// Memanggil fungsi C dari Rust
extern "C" {
    fn abs(input: i32) -> i32;
    fn sqrt(input: f64) -> f64;
}

// Membuat fungsi Rust yang bisa dipanggil dari C
#[no_mangle] // jangan ubah nama fungsi saat compile
pub extern "C" fn panggil_dari_c(x: i32) -> i32 {
    x * 2
}

fn ffi_demo() {
    println!("\n=== FFI (Foreign Function Interface) ===");

    unsafe {
        println!("abs(-10) = {}", abs(-10));
        println!("sqrt(144) = {}", sqrt(144.0));
    }
}

// ══════════════════════════════════════════════════════════════
// 4. MUTABLE STATIC VARIABLES
// ══════════════════════════════════════════════════════════════

// Static variable global — immutable (safe)
static SALAM: &str = "Halo Dunia!";

// Mutable static — UNSAFE karena bisa race condition di multi-thread!
static mut COUNTER: u32 = 0;

fn static_var_demo() {
    println!("\n=== Static Variables ===");

    println!("Salam: {}", SALAM); // safe — immutable

    unsafe {
        COUNTER += 1;
        COUNTER += 1;
        COUNTER += 1;
        println!("Counter: {}", COUNTER);
    }

    // ⚠️ Mutable static sangat TIDAK DISARANKAN!
    // Gunakan Mutex atau Atomic untuk shared mutable state
    use std::sync::atomic::{AtomicU32, Ordering};
    static SAFE_COUNTER: AtomicU32 = AtomicU32::new(0);

    SAFE_COUNTER.fetch_add(1, Ordering::SeqCst);
    SAFE_COUNTER.fetch_add(1, Ordering::SeqCst);
    println!("Atomic counter: {}", SAFE_COUNTER.load(Ordering::SeqCst));
}

// ══════════════════════════════════════════════════════════════
// 5. UNSAFE TRAIT
// ══════════════════════════════════════════════════════════════

// Unsafe trait — implementor HARUS menjamin invariant tertentu
unsafe trait Trustable {
    fn is_valid(&self) -> bool;
}

struct SafeData {
    value: i32,
}

// Unsafe impl — programmer menjamin implementasi benar
unsafe impl Trustable for SafeData {
    fn is_valid(&self) -> bool {
        self.value >= 0
    }
}

fn unsafe_trait_demo() {
    println!("\n=== Unsafe Trait ===");

    let data = SafeData { value: 42 };
    println!("Data valid? {}", data.is_valid());
}

// ══════════════════════════════════════════════════════════════
// CONTOH PRAKTIS: Simple Allocator
// ══════════════════════════════════════════════════════════════

struct SimplePool {
    buffer: Vec<u8>,
    offset: usize,
}

impl SimplePool {
    fn new(size: usize) -> Self {
        SimplePool {
            buffer: vec![0; size],
            offset: 0,
        }
    }

    fn alloc(&mut self, size: usize) -> Option<&mut [u8]> {
        if self.offset + size > self.buffer.len() {
            return None;
        }

        let start = self.offset;
        self.offset += size;

        // Safe karena kita tahu range valid
        Some(&mut self.buffer[start..start + size])
    }

    fn reset(&mut self) {
        self.offset = 0;
    }

    fn used(&self) -> usize {
        self.offset
    }
}

fn allocator_demo() {
    println!("\n=== Simple Pool Allocator ===");

    let mut pool = SimplePool::new(1024);

    // Alokasi beberapa block
    if let Some(block1) = pool.alloc(256) {
        block1[0] = 42;
        println!("Block 1: {} bytes, first byte = {}", block1.len(), block1[0]);
    }

    if let Some(block2) = pool.alloc(512) {
        println!("Block 2: {} bytes", block2.len());
    }

    println!("Used: {}/1024 bytes", pool.used());

    // Coba alokasi lebih dari yang tersedia
    match pool.alloc(512) {
        Some(_) => println!("Berhasil alokasi"),
        None => println!("Gagal: tidak cukup ruang!"),
    }

    pool.reset();
    println!("Setelah reset, used: {} bytes", pool.used());
}

fn main() {
    raw_pointer_demo();
    unsafe_function_demo();
    ffi_demo();
    static_var_demo();
    unsafe_trait_demo();
    allocator_demo();

    println!("\n=== Panduan Unsafe ===");
    println!("1. Minimize unsafe block — buat sekecil mungkin");
    println!("2. Bungkus dengan safe abstraction");
    println!("3. Dokumentasikan MENGAPA unsafe diperlukan");
    println!("4. Gunakan #[deny(unsafe_code)] untuk cegah di codebase");
    println!("5. Pertimbangkan crate yang sudah teruji (libc, memmap2)");
}

// ============================================================
// 🧠 RINGKUMAN UNSAFE:
//
// ┌─────────────────────────────────────────────────────────────┐
// │                    5 HAL YANG HANYA BISA DI UNSAFE          │
// ├──────────────────┬──────────────────────────────────────────┤
// │ Raw pointers     │ *const T, *mut T — tidak ada borrow check│
// │ Unsafe functions │ Fungsi yang mungkin tidak aman           │
// │ Mutable static   │ static mut — rentan race condition       │
// │ Unsafe trait     │ Invariant harus dijamin manual           │
// │ Union fields     │ Akses field union (like C union)         │
// └──────────────────┴──────────────────────────────────────────┘
//
// ┌─────────────────────────────────────────────────────────────┐
// │                    KAPAN PAKAI UNSAFE?                      │
// ├──────────────────┬──────────────────────────────────────────┤
// │ ✅ PERLU         │ FFI, optimisasi kritis, low-level DS     │
// │                  │ hardware access, allocator custom        │
// ├──────────────────┼──────────────────────────────────────────┤
// │ ❌ HINDARI       │ "Malas handle borrow checker"            │
// │                  │ Premature optimization                   │
// │                  │ Meniru pattern C tanpa alasan            │
// └──────────────────┴──────────────────────────────────────────┘
//
// ⚠️ COMMON MISTAKES:
// - unsafe block terlalu besar → sulit debug
// - Tidak validasi input sebelum unsafe operation
// - Data race dengan mutable static
// - Dereference null/invalid raw pointer → segfault
// - Lupa invariant pada unsafe trait
//
// 🔗 PERBANDINGAN:
// | Rust (unsafe)     | C/C++          | Rust (safe)       |
// |-------------------|----------------|-------------------|
// | *const T          | const T*       | &T                │
// | *mut T            | T*             | &mut T            │
// | unsafe fn         | (semua fn)     | fn (safe)         │
// | static mut        | global var     │ Mutex/Atomic      │
// | FFI               │ (native)       │ (wrapper)         │
// ============================================================

// ============================================================
// 🏋️ LATIHAN:
// 1. Buat safe wrapper untuk C string (CStr/CString)
// 2. Implementasikan ring buffer menggunakan unsafe
// 3. Buat wrapper aman untuk raw pointer ke array
// 4. Implementasikan `transmute` manual untuk konversi tipe
//    (hanya untuk tipe yang ukurannya sama)
// 5. Buat benchmark: bandingkan bounds-checked vs unchecked
//    array access (get() vs get_unchecked())
// 6. Tulis unsafe trait dengan invariant yang jelas
// ============================================================
