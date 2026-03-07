// ============================================================
// 📙 BELAJAR RUST #19 — Smart Pointers
// ============================================================
// Smart pointers adalah struct yang berperilaku seperti pointer
// tapi punya kemampuan tambahan (metadata, automatic cleanup).
//
// Yang paling penting:
// - Box<T>    → alokasi di heap
// - Rc<T>     → reference counting (multiple ownership)
// - RefCell<T> → interior mutability (bypass borrow rules at runtime)
// ============================================================

use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    // ════════════════════════════════════════════════════════
    // BOX<T> — Alokasi di Heap
    // ════════════════════════════════════════════════════════

    // Box menyimpan data di HEAP, bukan stack
    // Berguna saat:
    // 1. Data berukuran besar (hindari copy di stack)
    // 2. Tipe dengan ukuran tidak diketahui saat compile (recursive types)
    // 3. Transfer ownership tanpa copy data

    // ── Box dasar ───────────────────────────────────────────
    let b = Box::new(5);
    println!("Box: {}", b); // auto-dereference
    println!("Box value: {}", *b); // explicit dereference

    // ── Box untuk data besar ────────────────────────────────
    let data_besar = Box::new([0u8; 1_000_000]); // 1MB di heap, bukan stack!
    println!("Data besar length: {}", data_besar.len());

    // ── Box untuk recursive types ───────────────────────────
    // Tanpa Box, compiler tidak tahu ukuran List (infinite size!)
    // enum List { Cons(i32, List), Nil } // ❌ ERROR! infinite size

    // Dengan Box: pointer punya ukuran tetap (usize)
    let list = List::Cons(
        1,
        Box::new(List::Cons(
            2,
            Box::new(List::Cons(3, Box::new(List::Nil))),
        )),
    );
    println!("List: {:?}", list);
    cetak_list(&list);

    // ── Box untuk trait objects ──────────────────────────────
    let bentuk: Vec<Box<dyn Bentuk>> = vec![
        Box::new(Lingkaran { radius: 5.0 }),
        Box::new(Persegi { sisi: 4.0 }),
    ];

    for b in &bentuk {
        println!("{}: luas = {:.2}", b.nama(), b.luas());
    }

    // ════════════════════════════════════════════════════════
    // RC<T> — Reference Counting (Multiple Ownership)
    // ════════════════════════════════════════════════════════

    // Rc<T> memungkinkan MULTIPLE OWNER untuk data yang sama
    // Data di-drop saat reference count = 0
    // ⚠️ Hanya untuk single-threaded! (untuk multi-thread, pakai Arc<T>)

    // ── Masalah tanpa Rc ────────────────────────────────────
    // let a = List::Cons(5, Box::new(List::Cons(10, Box::new(List::Nil))));
    // let b = List::Cons(3, Box::new(a)); // a di-move!
    // let c = List::Cons(4, Box::new(a)); // ❌ a sudah di-move

    // ── Solusi dengan Rc ────────────────────────────────────
    let shared = Rc::new(RcList::Cons(
        10,
        Rc::new(RcList::Cons(20, Rc::new(RcList::Nil))),
    ));
    println!("Reference count awal: {}", Rc::strong_count(&shared));

    let branch_a = Rc::new(RcList::Cons(3, Rc::clone(&shared)));
    println!("Count setelah branch_a: {}", Rc::strong_count(&shared));

    let branch_b = Rc::new(RcList::Cons(4, Rc::clone(&shared)));
    println!("Count setelah branch_b: {}", Rc::strong_count(&shared));

    {
        let _branch_c = Rc::new(RcList::Cons(5, Rc::clone(&shared)));
        println!("Count di dalam scope: {}", Rc::strong_count(&shared));
    }
    println!("Count setelah scope: {}", Rc::strong_count(&shared));

    // Rc::clone() TIDAK melakukan deep copy — hanya increment counter!
    // Ini sangat murah (O(1))

    // ════════════════════════════════════════════════════════
    // REFCELL<T> — Interior Mutability
    // ════════════════════════════════════════════════════════

    // RefCell memungkinkan MUTASI data meskipun reference immutable!
    // Borrow rules dicek saat RUNTIME (bukan compile time)
    // Jika dilanggar → PANIC!

    // ── RefCell dasar ───────────────────────────────────────
    let data = RefCell::new(5);
    println!("Data awal: {:?}", data);

    // .borrow() → immutable reference (Ref<T>)
    {
        let r = data.borrow();
        println!("Borrow: {}", *r);
    } // r di-drop di sini

    // .borrow_mut() → mutable reference (RefMut<T>)
    {
        let mut w = data.borrow_mut();
        *w += 10;
        println!("Setelah mutasi: {}", *w);
    } // w di-drop di sini

    println!("Data akhir: {:?}", data);

    // ⚠️ PANIC jika borrow rules dilanggar saat runtime!
    // let r1 = data.borrow();
    // let w1 = data.borrow_mut(); // 💥 PANIC! sudah ada immutable borrow

    // ── Rc + RefCell = Multiple Owners + Mutability ─────────
    // Pattern ini sangat umum di Rust!
    let shared_data = Rc::new(RefCell::new(vec![1, 2, 3]));

    // Clone Rc — sekarang ada 2 owner
    let owner_a = Rc::clone(&shared_data);
    let owner_b = Rc::clone(&shared_data);

    // Kedua owner bisa MUTASI data yang sama!
    owner_a.borrow_mut().push(4);
    owner_b.borrow_mut().push(5);

    println!("Shared data: {:?}", shared_data.borrow());
    // Output: [1, 2, 3, 4, 5]

    // ── Contoh Praktis: Observer Pattern ────────────────────
    let messenger = Rc::new(RefCell::new(Vec::<String>::new()));

    let logger1 = Rc::clone(&messenger);
    let logger2 = Rc::clone(&messenger);

    logger1.borrow_mut().push("Log dari logger1".to_string());
    logger2.borrow_mut().push("Log dari logger2".to_string());
    logger1.borrow_mut().push("Log kedua dari logger1".to_string());

    println!("\nSemua log:");
    for (i, log) in messenger.borrow().iter().enumerate() {
        println!("  {}. {}", i + 1, log);
    }

    // ══════════════════════════════════════════════════════════
    // DEREF TRAIT — Custom Dereference
    // ══════════════════════════════════════════════════════════

    let my_box = MyBox::new(String::from("Halo"));
    // Rust auto-dereference: &MyBox<String> → &String → &str
    cetak_str(&my_box);
    cetak_str(&(*my_box)); // explicit dereference

    // ══════════════════════════════════════════════════════════
    // DROP TRAIT — Custom Cleanup
    // ══════════════════════════════════════════════════════════

    {
        let _resource = Resource {
            nama: String::from("Database Connection"),
        };
        println!("Resource dibuat");
        // Resource akan otomatis di-drop di akhir scope
    }
    println!("Resource sudah di-drop");

    // Manual drop dengan std::mem::drop
    let r = Resource {
        nama: String::from("File Handle"),
    };
    println!("Sebelum manual drop");
    drop(r); // force drop sekarang
    println!("Setelah manual drop");
    // r tidak bisa dipakai lagi
}

// ── RECURSIVE TYPE DENGAN BOX ───────────────────────────────
#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

fn cetak_list(list: &List) {
    match list {
        List::Cons(val, next) => {
            print!("{} → ", val);
            cetak_list(next);
        }
        List::Nil => println!("Nil"),
    }
}

// ── TRAIT OBJECT DENGAN BOX ─────────────────────────────────
trait Bentuk {
    fn luas(&self) -> f64;
    fn nama(&self) -> &str;
}

struct Lingkaran {
    radius: f64,
}

struct Persegi {
    sisi: f64,
}

impl Bentuk for Lingkaran {
    fn luas(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
    fn nama(&self) -> &str {
        "Lingkaran"
    }
}

impl Bentuk for Persegi {
    fn luas(&self) -> f64 {
        self.sisi * self.sisi
    }
    fn nama(&self) -> &str {
        "Persegi"
    }
}

// ── RC LIST ─────────────────────────────────────────────────
#[derive(Debug)]
enum RcList {
    Cons(i32, Rc<RcList>),
    Nil,
}

// ── CUSTOM SMART POINTER (DEREF) ────────────────────────────
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

// Implement Deref agar bisa di-dereference dengan *
impl<T> std::ops::Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

fn cetak_str(s: &str) {
    println!("String: {}", s);
}

// ── DROP TRAIT ───────────────────────────────────────────────
struct Resource {
    nama: String,
}

impl Drop for Resource {
    fn drop(&mut self) {
        println!("🗑️  Membersihkan resource: {}", self.nama);
    }
}

// ============================================================
// 🏋️ LATIHAN:
// 1. Buat binary tree menggunakan Box: enum Tree { Leaf(i32), Node(Box, Box) }
// 2. Implementasikan graph sederhana menggunakan Rc dan RefCell
// 3. Buat cache sederhana menggunakan RefCell<HashMap<K, V>>
// 4. Implementasi doubly-linked list dengan Rc<RefCell<Node>>
// 5. Buat custom smart pointer yang logging setiap akses
// ============================================================
