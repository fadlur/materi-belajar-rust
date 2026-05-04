// ============================================================
// 📙 BELAJAR RUST #21 — Trait Objects (Dynamic Dispatch)
// ============================================================
// Trait objects memungkinkan polymorphism di RUNTIME.
// Berbeda dari generics yang di-resolve saat COMPILE time.
// Notasi: `dyn TraitName`
//
// 🎯 Tujuan: Memahami perbedaan static vs dynamic dispatch,
//    object safety, dan kapan menggunakan trait objects.
//
// 💡 Analogi Utama:
// Static dispatch (generics) seperti restoran dengan menu tetap —
//    pesananmu sudah diketahui saat masuk (compile time).
// Dynamic dispatch (trait objects) seperti restoran all-you-can-eat —
//    kamu ambil makanan saat di sana (runtime), tidak tahu
juga tidak tahu sebelumnya apa yang akan diambil.
//
// 🔑 Trait objects memungkinkan kita menyimpan tipe BERBEDA
//    dalam satu koleksi — sesuatu yang tidak bisa dilakukan
//    dengan generics!
// ============================================================

use std::fmt;

// ── TRAIT UNTUK CONTOH ──────────────────────────────────────
trait Drawable {
    fn draw(&self);
    fn nama(&self) -> &str;
}

#[derive(Debug)]
struct Tombol {
    label: String,
    width: u32,
    height: u32,
}

#[derive(Debug)]
struct TextField {
    placeholder: String,
    value: String,
}

#[derive(Debug)]
struct Gambar {
    url: String,
    alt: String,
}

impl Drawable for Tombol {
    fn draw(&self) {
        println!("[Button: {} ({}x{})]", self.label, self.width, self.height);
    }
    fn nama(&self) -> &str {
        "Tombol"
    }
}

impl Drawable for TextField {
    fn draw(&self) {
        println!("[Input: '{}' value='{}']", self.placeholder, self.value);
    }
    fn nama(&self) -> &str {
        "TextField"
    }
}

impl Drawable for Gambar {
    fn draw(&self) {
        println!("[Image: {} alt='{}']", self.url, self.alt);
    }
    fn nama(&self) -> &str {
        "Gambar"
    }
}

// ── SCREEN: MENYIMPAN TRAIT OBJECTS ─────────────────────────
// Vec<Box<dyn Drawable>> = vektor dari trait objects
// Bisa menyimpan TIPE BERBEDA selama implement Drawable!
//
// 💡 Analogi: Screen seperti layar ponsel — bisa menampilkan
//    tombol, input, gambar, dll. Semua berbeda tipe, tapi
//    bisa ditampilkan di layar yang sama.
struct Screen {
    komponen: Vec<Box<dyn Drawable>>,
}

impl Screen {
    fn new() -> Self {
        Screen {
            komponen: Vec::new(),
        }
    }

    fn tambah(&mut self, komponen: Box<dyn Drawable>) {
        self.komponen.push(komponen);
    }

    fn render(&self) {
        println!("=== Rendering Screen ===");
        for (i, k) in self.komponen.iter().enumerate() {
            print!("  {}. ", i + 1);
            k.draw(); // dynamic dispatch!
        }
        println!("========================");
    }
}

// ── STATIC vs DYNAMIC DISPATCH ──────────────────────────────
// Static dispatch (generics) → di-resolve saat compile, lebih cepat
fn cetak_static(item: &impl Drawable) {
    item.draw();
}

// Dynamic dispatch (trait object) → di-resolve saat runtime, lebih fleksibel
fn cetak_dynamic(item: &dyn Drawable) {
    item.draw();
}

// Return trait object — berguna saat return tipe berbeda berdasarkan kondisi
fn buat_komponen(jenis: &str) -> Box<dyn Drawable> {
    match jenis {
        "tombol" => Box::new(Tombol {
            label: "OK".to_string(),
            width: 100,
            height: 40,
        }),
        "input" => Box::new(TextField {
            placeholder: "Ketik di sini...".to_string(),
            value: String::new(),
        }),
        _ => Box::new(Gambar {
            url: "default.png".to_string(),
            alt: "Default".to_string(),
        }),
    }
}

// ── TRAIT DENGAN MULTIPLE METHODS ───────────────────────────
trait Hewan: fmt::Display {
    fn suara(&self) -> &str;
    fn kaki(&self) -> u32;
    fn deskripsi(&self) -> String {
        format!("{} berkaki {} berbunyi '{}'", self, self.kaki(), self.suara())
    }
}

struct Kucing {
    nama: String,
}

struct Anjing {
    nama: String,
}

struct Ular {
    nama: String,
}

impl fmt::Display for Kucing {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Kucing '{}'", self.nama)
    }
}

impl fmt::Display for Anjing {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Anjing '{}'", self.nama)
    }
}

impl fmt::Display for Ular {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Ular '{}'", self.nama)
    }
}

impl Hewan for Kucing {
    fn suara(&self) -> &str { "Meong" }
    fn kaki(&self) -> u32 { 4 }
}

impl Hewan for Anjing {
    fn suara(&self) -> &str { "Guk guk" }
    fn kaki(&self) -> u32 { 4 }
}

impl Hewan for Ular {
    fn suara(&self) -> &str { "Ssss" }
    fn kaki(&self) -> u32 { 0 }
}

fn main() {
    // ── Trait Objects dalam Vec ──────────────────────────────
    let mut screen = Screen::new();
    screen.tambah(Box::new(Tombol {
        label: "Submit".to_string(),
        width: 120,
        height: 40,
    }));
    screen.tambah(Box::new(TextField {
        placeholder: "Username".to_string(),
        value: "budi123".to_string(),
    }));
    screen.tambah(Box::new(Gambar {
        url: "logo.png".to_string(),
        alt: "Logo".to_string(),
    }));
    screen.tambah(Box::new(Tombol {
        label: "Cancel".to_string(),
        width: 100,
        height: 40,
    }));

    screen.render();

    // ── Static vs Dynamic Dispatch ──────────────────────────
    let btn = Tombol {
        label: "Test".to_string(),
        width: 80,
        height: 30,
    };
    cetak_static(&btn); // static dispatch — compiler tahu tipe saat compile
    cetak_dynamic(&btn); // dynamic dispatch — via vtable saat runtime

    // ── Factory Pattern dengan Trait Object ─────────────────
    let komponen_list = vec!["tombol", "input", "gambar", "tombol"];
    for jenis in komponen_list {
        let k = buat_komponen(jenis);
        print!("  Created {}: ", k.nama());
        k.draw();
    }

    // ── Koleksi Hewan ───────────────────────────────────────
    let kebun_binatang: Vec<Box<dyn Hewan>> = vec![
        Box::new(Kucing { nama: "Kitty".to_string() }),
        Box::new(Anjing { nama: "Buddy".to_string() }),
        Box::new(Ular { nama: "Nagini".to_string() }),
        Box::new(Kucing { nama: "Mimi".to_string() }),
    ];

    println!("\n=== Kebun Binatang ===");
    for hewan in &kebun_binatang {
        println!("  {}", hewan.deskripsi());
    }

    // Hitung total kaki
    let total_kaki: u32 = kebun_binatang.iter().map(|h| h.kaki()).sum();
    println!("Total kaki: {}", total_kaki);

    // ── OBJECT SAFETY ───────────────────────────────────────
    // Tidak semua trait bisa dijadikan trait object!
    // Trait HARUS "object safe":
    // 1. Return type bukan Self
    // 2. Tidak ada generic type parameters pada methods
    //
    // Contoh trait yang TIDAK object safe:
    // trait Clone { fn clone(&self) -> Self; }  // return Self!
    // trait Foo { fn bar<T>(&self, x: T); }     // generic method!

    // Trait yang object safe:
    // ✅ fn method(&self) -> String
    // ✅ fn method(&self, x: i32) -> i32
    // ❌ fn method(&self) -> Self
    // ❌ fn method<T>(&self, x: T)
}

// ============================================================
// 🧠 RINGKUMAN STATIC vs DYNAMIC DISPATCH:
//
// ┌─────────────────────────────────────────────────────────────┐
// │                    STATIC vs DYNAMIC                        │
// ├──────────────────┬──────────────────┬───────────────────────┤
// │                  │ Static (generics)│ Dynamic (dyn Trait)   │
// ├──────────────────┼──────────────────┼───────────────────────┤
// │ Dispatch         │ Compile time     │ Runtime               │
// │ Performance      │ Lebih cepat      │ Sedikit lebih lambat  │
// │ Inline           │ Bisa di-inline   │ Tidak bisa            │
// │ Code size        │ Lebih besar      │ Lebih kecil           │
// │ Heterogeneous    │ ❌ Tidak bisa    │ ✅ Bisa               │
// │ Collection       │                  │                       │
// │ Use case         │ Default          │ Plugin, UI, factory   │
// └──────────────────┴──────────────────┴───────────────────────┘
//
// 💡 Rule of thumb: Gunakan generics secara default.
//    Gunakan trait objects saat PERLU heterogeneous collection
//    atau saat return tipe berbeda berdasarkan runtime condition.
//
// ⚠️ COMMON MISTAKES:
// - Menggunakan dyn Trait tanpa Box/& → compile error (unsized!)
// - Trait tidak object safe → compile error
// - Lupa Box::new() saat membuat trait object
// - Expect trait object bisa di-clone → tidak bisa (Self!)
//
// 🔗 PERBANDINGAN:
// | Rust              | C++              | Java              |
// |-------------------|------------------|-------------------|
// | dyn Trait         | virtual class    │ interface         |
// | vtable            │ vtable           │ vtable            │
// | Box<dyn Trait>    │ unique_ptr<Base> │ new Impl()        │
// | &dyn Trait        │ Base*            │ Base ref          │
// ============================================================

// ============================================================
// 🏋️ LATIHAN:
// 1. Buat plugin system: trait Plugin dengan method execute()
//    Load berbagai plugin ke Vec<Box<dyn Plugin>>
// 2. Buat event system: trait EventHandler, register multiple
//    handlers yang bereaksi terhadap event berbeda
// 3. Buat shape renderer yang bisa menggambar berbagai bentuk
// 4. Implementasi strategy pattern: sorting algorithm yang bisa
//    diganti saat runtime
// 5. Buat serializer: trait Serialize dengan method to_json()
//    dan to_xml() — implement untuk beberapa struct
// 6. Buat file parser yang return Box<dyn Parser> berdasarkan ekstensi
// ============================================================
