// ============================================================
// 📕 BELAJAR RUST #26 — Design Patterns
// ============================================================
// Design patterns yang umum dipakai di Rust.
// Rust punya cara unik menerapkan pattern karena:
// - Tidak ada inheritance (composition > inheritance)
// - Ownership system mempengaruhi desain
// - Trait-based polymorphism
//
// 🎯 Tujuan: Memahami design patterns populer dan cara
//    mengimplementasikannya di Rust secara idiomatic.
//
// 💡 Analogi Utama:
// Design patterns seperti RESEP ARSITEKTUR — bukan blueprint
//    detail, tapi panduan umum untuk masalah umum. Rust punya
//    "bahan bangunan" berbeda (ownership, traits, enums), jadi
//    resepnya sedikit berbeda dari bahasa OOP tradisional.
// ============================================================

use std::collections::HashMap;
use std::fmt;

// ══════════════════════════════════════════════════════════════
// 1. BUILDER PATTERN
// ══════════════════════════════════════════════════════════════

// Berguna saat struct punya banyak field, beberapa opsional
#[derive(Debug)]
struct HttpRequest {
    method: String,
    url: String,
    headers: HashMap<String, String>,
    body: Option<String>,
    timeout_ms: u64,
}

// Builder struct terpisah
struct HttpRequestBuilder {
    method: String,
    url: String,
    headers: HashMap<String, String>,
    body: Option<String>,
    timeout_ms: u64,
}

impl HttpRequestBuilder {
    fn new(method: &str, url: &str) -> Self {
        HttpRequestBuilder {
            method: method.to_string(),
            url: url.to_string(),
            headers: HashMap::new(),
            body: None,
            timeout_ms: 30_000, // default 30 detik
        }
    }

    fn header(mut self, key: &str, value: &str) -> Self {
        self.headers.insert(key.to_string(), value.to_string());
        self // return self untuk chaining
    }

    fn body(mut self, body: &str) -> Self {
        self.body = Some(body.to_string());
        self
    }

    fn timeout(mut self, ms: u64) -> Self {
        self.timeout_ms = ms;
        self
    }

    fn build(self) -> HttpRequest {
        HttpRequest {
            method: self.method,
            url: self.url,
            headers: self.headers,
            body: self.body,
            timeout_ms: self.timeout_ms,
        }
    }
}

// ══════════════════════════════════════════════════════════════
// 2. NEWTYPE PATTERN
// ══════════════════════════════════════════════════════════════

// Bungkus tipe yang ada untuk type safety tambahan
// Mencegah tercampurnya tipe yang secara semantik berbeda

#[derive(Debug, Clone)]
struct Email(String);

#[derive(Debug, Clone)]
struct UserId(u64);

#[derive(Debug, Clone)]
struct Rupiah(f64);

impl Email {
    fn new(email: &str) -> Result<Self, String> {
        if email.contains('@') && email.contains('.') {
            Ok(Email(email.to_string()))
        } else {
            Err(format!("Email tidak valid: {}", email))
        }
    }

    fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for Email {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Rupiah {
    fn new(amount: f64) -> Self {
        Rupiah(amount)
    }

    fn amount(&self) -> f64 {
        self.0
    }
}

impl fmt::Display for Rupiah {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Rp {:.0}", self.0)
    }
}

// Sekarang compiler mencegah kesalahan:
// fn kirim_email(email: Email, user_id: UserId) { ... }
// kirim_email(user_id, email)  ← ❌ tipe berbeda!

// ══════════════════════════════════════════════════════════════
// 3. STATE MACHINE PATTERN (dengan Enum)
// ══════════════════════════════════════════════════════════════

#[derive(Debug)]
enum DoorState {
    Terkunci,
    Terbuka,
    Tertutup,
}

struct Pintu {
    state: DoorState,
    log: Vec<String>,
}

impl Pintu {
    fn new() -> Self {
        Pintu {
            state: DoorState::Terkunci,
            log: vec!["Pintu dibuat (terkunci)".to_string()],
        }
    }

    fn buka_kunci(&mut self) -> Result<(), &str> {
        match self.state {
            DoorState::Terkunci => {
                self.state = DoorState::Tertutup;
                self.log.push("Kunci dibuka".to_string());
                Ok(())
            }
            _ => Err("Pintu tidak terkunci"),
        }
    }

    fn buka(&mut self) -> Result<(), &str> {
        match self.state {
            DoorState::Tertutup => {
                self.state = DoorState::Terbuka;
                self.log.push("Pintu dibuka".to_string());
                Ok(())
            }
            DoorState::Terkunci => Err("Buka kunci dulu!"),
            DoorState::Terbuka => Err("Pintu sudah terbuka"),
        }
    }

    fn tutup(&mut self) -> Result<(), &str> {
        match self.state {
            DoorState::Terbuka => {
                self.state = DoorState::Tertutup;
                self.log.push("Pintu ditutup".to_string());
                Ok(())
            }
            _ => Err("Pintu tidak terbuka"),
        }
    }

    fn kunci(&mut self) -> Result<(), &str> {
        match self.state {
            DoorState::Tertutup => {
                self.state = DoorState::Terkunci;
                self.log.push("Pintu dikunci".to_string());
                Ok(())
            }
            _ => Err("Tutup pintu dulu!"),
        }
    }

    fn status(&self) -> &DoorState {
        &self.state
    }

    fn riwayat(&self) -> &[String] {
        &self.log
    }
}

// ══════════════════════════════════════════════════════════════
// 4. STRATEGY PATTERN (dengan Closure/Trait)
// ══════════════════════════════════════════════════════════════

trait SortStrategy {
    fn sort(&self, data: &mut Vec<i32>);
    fn nama(&self) -> &str;
}

struct BubbleSort;
struct QuickSort;

impl SortStrategy for BubbleSort {
    fn sort(&self, data: &mut Vec<i32>) {
        let len = data.len();
        for i in 0..len {
            for j in 0..len - 1 - i {
                if data[j] > data[j + 1] {
                    data.swap(j, j + 1);
                }
            }
        }
    }
    fn nama(&self) -> &str {
        "Bubble Sort"
    }
}

impl SortStrategy for QuickSort {
    fn sort(&self, data: &mut Vec<i32>) {
        data.sort(); // pakai built-in sort (quicksort-based)
    }
    fn nama(&self) -> &str {
        "Quick Sort"
    }
}

struct Sorter {
    strategy: Box<dyn SortStrategy>,
}

impl Sorter {
    fn new(strategy: Box<dyn SortStrategy>) -> Self {
        Sorter { strategy }
    }

    fn sort(&self, data: &mut Vec<i32>) {
        println!("Sorting dengan {}...", self.strategy.nama());
        self.strategy.sort(data);
    }

    fn ganti_strategy(&mut self, strategy: Box<dyn SortStrategy>) {
        self.strategy = strategy;
    }
}

// ══════════════════════════════════════════════════════════════
// 5. OBSERVER PATTERN (Event System)
// ══════════════════════════════════════════════════════════════

type EventCallback = Box<dyn Fn(&str)>;

struct EventEmitter {
    listeners: HashMap<String, Vec<EventCallback>>,
}

impl EventEmitter {
    fn new() -> Self {
        EventEmitter {
            listeners: HashMap::new(),
        }
    }

    fn on(&mut self, event: &str, callback: impl Fn(&str) + 'static) {
        self.listeners
            .entry(event.to_string())
            .or_default()
            .push(Box::new(callback));
    }

    fn emit(&self, event: &str, data: &str) {
        if let Some(callbacks) = self.listeners.get(event) {
            for cb in callbacks {
                cb(data);
            }
        }
    }
}

// ══════════════════════════════════════════════════════════════
// 6. REPOSITORY PATTERN
// ══════════════════════════════════════════════════════════════

#[derive(Debug, Clone)]
struct User {
    id: u64,
    nama: String,
    email: Email,
}

trait UserRepository {
    fn find_by_id(&self, id: u64) -> Option<&User>;
    fn find_all(&self) -> Vec<&User>;
    fn save(&mut self, user: User);
    fn delete(&mut self, id: u64) -> bool;
}

struct InMemoryUserRepo {
    users: HashMap<u64, User>,
    next_id: u64,
}

impl InMemoryUserRepo {
    fn new() -> Self {
        InMemoryUserRepo {
            users: HashMap::new(),
            next_id: 1,
        }
    }
}

impl UserRepository for InMemoryUserRepo {
    fn find_by_id(&self, id: u64) -> Option<&User> {
        self.users.get(&id)
    }

    fn find_all(&self) -> Vec<&User> {
        self.users.values().collect()
    }

    fn save(&mut self, mut user: User) {
        if user.id == 0 {
            user.id = self.next_id;
            self.next_id += 1;
        }
        self.users.insert(user.id, user);
    }

    fn delete(&mut self, id: u64) -> bool {
        self.users.remove(&id).is_some()
    }
}

fn main() {
    // ── 1. Builder Pattern ──────────────────────────────────
    println!("=== Builder Pattern ===");
    let request = HttpRequestBuilder::new("POST", "https://api.example.com/data")
        .header("Content-Type", "application/json")
        .header("Authorization", "Bearer token123")
        .body(r#"{"nama": "Budi"}"#)
        .timeout(5000)
        .build();

    println!("{:#?}\n", request);

    // ── 2. Newtype Pattern ──────────────────────────────────
    println!("=== Newtype Pattern ===");
    match Email::new("budi@email.com") {
        Ok(email) => println!("Email valid: {}", email),
        Err(e) => println!("Error: {}", e),
    }

    match Email::new("invalid-email") {
        Ok(email) => println!("Email valid: {}", email),
        Err(e) => println!("Error: {}", e),
    }

    let harga = Rupiah::new(1_500_000.0);
    println!("Harga: {}\n", harga);

    // ── 3. State Machine ────────────────────────────────────
    println!("=== State Machine ===");
    let mut pintu = Pintu::new();

    println!("Status: {:?}", pintu.status());
    pintu.buka_kunci().unwrap();
    pintu.buka().unwrap();
    println!("Status: {:?}", pintu.status());

    // Coba operasi invalid
    if let Err(e) = pintu.kunci() {
        println!("Error: {}", e);
    }

    pintu.tutup().unwrap();
    pintu.kunci().unwrap();
    println!("Status: {:?}", pintu.status());
    println!("Riwayat: {:?}\n", pintu.riwayat());

    // ── 4. Strategy Pattern ─────────────────────────────────
    println!("=== Strategy Pattern ===");
    let mut data1 = vec![5, 2, 8, 1, 9, 3];
    let mut sorter = Sorter::new(Box::new(BubbleSort));
    sorter.sort(&mut data1);
    println!("Hasil: {:?}", data1);

    let mut data2 = vec![5, 2, 8, 1, 9, 3];
    sorter.ganti_strategy(Box::new(QuickSort));
    sorter.sort(&mut data2);
    println!("Hasil: {:?}\n", data2);

    // ── 5. Observer Pattern ─────────────────────────────────
    println!("=== Observer Pattern ===");
    let mut emitter = EventEmitter::new();

    emitter.on("login", |data| {
        println!("  📧 Kirim email selamat datang ke {}", data);
    });

    emitter.on("login", |data| {
        println!("  📝 Log: {} telah login", data);
    });

    emitter.on("logout", |data| {
        println!("  👋 {} telah logout", data);
    });

    emitter.emit("login", "Budi");
    emitter.emit("login", "Ani");
    emitter.emit("logout", "Budi");

    // ── 6. Repository Pattern ───────────────────────────────
    println!("\n=== Repository Pattern ===");
    let mut repo = InMemoryUserRepo::new();

    repo.save(User {
        id: 0, // auto-increment
        nama: "Budi".to_string(),
        email: Email::new("budi@email.com").unwrap(),
    });

    repo.save(User {
        id: 0,
        nama: "Ani".to_string(),
        email: Email::new("ani@email.com").unwrap(),
    });

    println!("Semua user:");
    for user in repo.find_all() {
        println!("  {:?}", user);
    }

    if let Some(user) = repo.find_by_id(1) {
        println!("Found: {} ({})", user.nama, user.email);
    }

    repo.delete(1);
    println!("Setelah delete id=1: {} users", repo.find_all().len());
}

// ============================================================
// 🧠 RINGKUMAN DESIGN PATTERNS DI RUST:
//
// ┌─────────────────────────────────────────────────────────────┐
// │                    PATTERN YANG UMUM                        │
// ├──────────────────┬──────────────────────────────────────────┤
// │ Builder          │ Struct terpisah + method chaining        │
// │ Newtype          │ Wrapper struct untuk type safety         │
// │ State Machine    │ Enum untuk state + match untuk transisi  │
// │ Strategy         │ Trait + Box<dyn Trait>                   │
// │ Observer         │ HashMap<String, Vec<closure>>            │
// │ Repository       │ Trait + impl (bisa ganti storage)        │
// │ Singleton        │ lazy_static / OnceCell / OnceLock        │
// │ Factory          │ fn/impl yang return Box<dyn Trait>       │
// │ Command          │ Enum dengan data + method execute        │
// │ Decorator        │ Wrapper struct + Deref                   │
// └──────────────────┴──────────────────────────────────────────┘
//
// ⚠️ PERBEDAAN DENGAN OOP TRADISIONAL:
// - Composition over inheritance → trait + struct
// - No null → Option<T>
// - Error handling → Result<T, E>
// - Ownership affects design → borrow checker
// - Pattern matching → match + enum
//
// 🔗 PERBANDINGAN:
// | Rust              | Java             | Go                |
// |-------------------|------------------|-------------------|
// | trait + struct    | interface + class│ interface + struct│
// | Box<dyn Trait>    | new Class()      │ interface value   │
// | enum state        | class state      │ interface + type  │
// | closure           │ lambda           │ func value        │
// | match             │ switch           │ switch            │
// ============================================================

// ============================================================
// 🏋️ LATIHAN:
// 1. Implementasikan Command pattern (undo/redo) untuk text editor
// 2. Buat Factory pattern untuk membuat berbagai jenis report
// 3. Implementasikan Chain of Responsibility untuk middleware
// 4. Buat Singleton pattern yang thread-safe (gunakan OnceCell/OnceLock)
// 5. Implementasikan Decorator pattern untuk menambah logging
//    pada fungsi yang sudah ada
// 6. Buat Visitor pattern menggunakan enum dan match
// ============================================================
