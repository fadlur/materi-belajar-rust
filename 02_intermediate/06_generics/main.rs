// ============================================================
// 📘 BELAJAR RUST #14 — Generics
// ============================================================
// Generics memungkinkan kita menulis kode yang bekerja
// untuk BANYAK tipe data — tanpa duplikasi kode.
// Mirip template di C++ atau generics di Java/TypeScript.
// ============================================================

use std::fmt;

fn main() {
    // ── GENERIC FUNCTION ────────────────────────────────────
    // Tanpa generics, kita harus tulis fungsi berbeda untuk tiap tipe:
    // fn terbesar_i32(list: &[i32]) -> &i32 { ... }
    // fn terbesar_f64(list: &[f64]) -> &f64 { ... }
    // fn terbesar_char(list: &[char]) -> &char { ... }

    // Dengan generics: satu fungsi untuk semua!
    let angka = vec![34, 50, 25, 100, 65];
    println!("Terbesar: {}", terbesar(&angka));

    let huruf = vec!['y', 'm', 'a', 'q'];
    println!("Terbesar: {}", terbesar(&huruf));

    let desimal = vec![3.14, 2.71, 1.41, 1.73];
    println!("Terbesar: {}", terbesar(&desimal));

    // ── GENERIC STRUCT ──────────────────────────────────────
    let titik_int = Titik { x: 5, y: 10 };
    let titik_float = Titik { x: 1.5, y: 4.2 };
    let titik_campur = TitikCampur { x: 5, y: 3.14 };

    println!("Titik int: {:?}", titik_int);
    println!("Titik float: {:?}", titik_float);
    println!("Titik campur: {:?}", titik_campur);

    // Method pada generic struct
    println!("X: {}, Y: {}", titik_int.x(), titik_int.y());
    println!("Jarak: {:.2}", titik_float.jarak_dari_origin());

    // Method yang mencampur generic types
    let p1 = TitikCampur { x: 1, y: 2.0 };
    let p2 = TitikCampur { x: "hello", y: 'c' };
    let p3 = p1.mixup(p2);
    println!("Mixup: {:?}", p3); // TitikCampur { x: 1, y: 'c' }

    // ── GENERIC ENUM ────────────────────────────────────────
    // Option<T> dan Result<T, E> adalah contoh generic enum!
    // enum Option<T> { Some(T), None }
    // enum Result<T, E> { Ok(T), Err(E) }

    // Custom generic enum
    let data: Hasil<i32> = Hasil::Sukses(42);
    let error: Hasil<i32> = Hasil::Error(String::from("gagal"));

    match data {
        Hasil::Sukses(val) => println!("Sukses: {}", val),
        Hasil::Error(msg) => println!("Error: {}", msg),
    }
    match error {
        Hasil::Sukses(val) => println!("Sukses: {}", val),
        Hasil::Error(msg) => println!("Error: {}", msg),
    }

    // ── TRAIT BOUNDS PADA GENERICS ──────────────────────────
    // Kita bisa membatasi generic hanya untuk tipe yang implement trait tertentu

    cetak_item(42);
    cetak_item("halo");
    cetak_item(3.14);

    // Multiple trait bounds
    cetak_detail(&42);
    cetak_detail(&"halo");

    // ── GENERIC DENGAN WHERE CLAUSE ─────────────────────────
    let a = Wrapper::new(10);
    let b = Wrapper::new(20);
    println!("a lebih besar? {}", a.lebih_besar(&b));

    // ── CONTOH PRAKTIS: Stack Generic ───────────────────────
    let mut stack = Stack::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);
    println!("Stack: {:?}", stack);
    println!("Peek: {:?}", stack.peek());
    println!("Pop: {:?}", stack.pop());
    println!("Stack setelah pop: {:?}", stack);
    println!("Size: {}", stack.size());

    // Stack dengan String
    let mut str_stack = Stack::new();
    str_stack.push(String::from("a"));
    str_stack.push(String::from("b"));
    println!("String stack: {:?}", str_stack);

    // ── CONTOH: Pair yang bisa dibandingkan ─────────────────
    let pair = Pair::new(10, 20);
    pair.cetak_terbesar();

    let pair2 = Pair::new("apel", "jeruk");
    pair2.cetak_terbesar();

    // ── MONOMORPHIZATION ────────────────────────────────────
    // Rust menggunakan "monomorphization" saat compile:
    // Generic di-expand menjadi kode spesifik untuk setiap tipe.
    // Jadi TIDAK ADA runtime overhead! Sama cepatnya dengan kode non-generic.
    //
    // Misal: terbesar(&angka) → Rust generate terbesar_i32()
    //        terbesar(&huruf) → Rust generate terbesar_char()
    // Ini disebut "zero-cost abstraction"
}

// ── GENERIC FUNCTION ────────────────────────────────────────
// <T: PartialOrd> artinya T harus bisa dibandingkan (implement PartialOrd)
fn terbesar<T: PartialOrd>(list: &[T]) -> &T {
    let mut terbesar = &list[0];
    for item in &list[1..] {
        if item > terbesar {
            terbesar = item;
        }
    }
    terbesar
}

// ── GENERIC STRUCT ──────────────────────────────────────────
#[derive(Debug)]
struct Titik<T> {
    x: T,
    y: T, // x dan y HARUS tipe yang sama
}

// Struct dengan generic types berbeda
#[derive(Debug)]
struct TitikCampur<T, U> {
    x: T,
    y: U,
}

// ── IMPL UNTUK GENERIC STRUCT ───────────────────────────────
// Method untuk SEMUA tipe T
impl<T> Titik<T> {
    fn x(&self) -> &T {
        &self.x
    }

    fn y(&self) -> &T {
        &self.y
    }
}

// Method HANYA untuk Titik<f64>
impl Titik<f64> {
    fn jarak_dari_origin(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

// Method yang mencampur generic types
impl<T, U> TitikCampur<T, U> {
    fn mixup<V, W>(self, other: TitikCampur<V, W>) -> TitikCampur<T, W> {
        TitikCampur {
            x: self.x,   // ambil x dari self
            y: other.y,  // ambil y dari other
        }
    }
}

// ── GENERIC ENUM ────────────────────────────────────────────
#[derive(Debug)]
enum Hasil<T> {
    Sukses(T),
    Error(String),
}

// ── FUNGSI DENGAN TRAIT BOUND ───────────────────────────────
fn cetak_item<T: fmt::Display>(item: T) {
    println!("Item: {}", item);
}

fn cetak_detail<T: fmt::Display + fmt::Debug>(item: &T) {
    println!("Display: {} | Debug: {:?}", item, item);
}

// ── GENERIC DENGAN WHERE CLAUSE ─────────────────────────────
#[derive(Debug)]
struct Wrapper<T> {
    value: T,
}

impl<T> Wrapper<T> {
    fn new(value: T) -> Self {
        Wrapper { value }
    }
}

impl<T> Wrapper<T>
where
    T: PartialOrd + fmt::Display,
{
    fn lebih_besar(&self, other: &Wrapper<T>) -> bool {
        self.value > other.value
    }
}

// ── GENERIC STACK ───────────────────────────────────────────
#[derive(Debug)]
struct Stack<T> {
    elements: Vec<T>,
}

impl<T> Stack<T> {
    fn new() -> Self {
        Stack {
            elements: Vec::new(),
        }
    }

    fn push(&mut self, item: T) {
        self.elements.push(item);
    }

    fn pop(&mut self) -> Option<T> {
        self.elements.pop()
    }

    fn peek(&self) -> Option<&T> {
        self.elements.last()
    }

    fn size(&self) -> usize {
        self.elements.len()
    }

    fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }
}

// ── PAIR DENGAN CONDITIONAL METHOD ──────────────────────────
struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Pair { x, y }
    }
}

// Method ini HANYA tersedia jika T implement Display + PartialOrd
impl<T: fmt::Display + PartialOrd> Pair<T> {
    fn cetak_terbesar(&self) {
        if self.x >= self.y {
            println!("Terbesar: {}", self.x);
        } else {
            println!("Terbesar: {}", self.y);
        }
    }
}

// ============================================================
// 🏋️ LATIHAN:
// 1. Buat generic function `min` yang return nilai terkecil dari slice
// 2. Buat generic struct `Queue<T>` dengan enqueue dan dequeue
// 3. Buat generic function `filter` yang menerima slice dan predicate,
//    return Vec elemen yang memenuhi kondisi
// 4. Buat generic struct `Cache<K, V>` dengan get dan set
// 5. Buat generic binary search function
// ============================================================
