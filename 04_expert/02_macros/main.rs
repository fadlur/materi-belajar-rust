// ============================================================
// 📕 BELAJAR RUST #24 — Macros
// ============================================================
// Macro = kode yang menulis kode (metaprogramming).
// Dieksekusi saat COMPILE TIME — zero runtime cost.
//
// Dua jenis:
// 1. Declarative macros (macro_rules!) — pattern matching pada kode
// 2. Procedural macros — manipulasi token stream (lebih advanced)
//
// 🎯 Tujuan: Memahami declarative macros dan cara menggunakannya
//    untuk mengurangi boilerplate kode.
//
// 💡 Analogi Utama:
// Macro seperti PHOTOCOPY CANGGIH — kamu buat template (macro),
//    lalu mesin fotocopy otomatis mengisi template dengan data
//    yang berbeda-beda saat compile time. Hasilnya: banyak kode
//    spesifik tanpa harus menulis satu per satu.
//
// 🔑 Macro berbeda dari fungsi: macro di-expand saat compile,
//    bukan dipanggil saat runtime. Ini memungkinkan fleksibilitas
//    syntax yang tidak mungkin dengan fungsi biasa.
// ============================================================

// ══════════════════════════════════════════════════════════════
// DECLARATIVE MACROS (macro_rules!)
// ══════════════════════════════════════════════════════════════

// ── Macro paling sederhana ──────────────────────────────────
macro_rules! sapa {
    () => {
        println!("Halo dari macro!");
    };
}

// ── Macro dengan parameter ──────────────────────────────────
// `$nama:expr` artinya: tangkap expression, simpan di $nama
macro_rules! sapa_nama {
    ($nama:expr) => {
        println!("Halo, {}! (dari macro)", $nama);
    };
}

// ── Macro dengan multiple arms (pattern matching) ───────────
macro_rules! hitung {
    // Pattern 1: satu angka → return apa adanya
    ($x:expr) => {
        $x
    };
    // Pattern 2: dua angka dengan operator
    ($x:expr, tambah, $y:expr) => {
        $x + $y
    };
    ($x:expr, kurang, $y:expr) => {
        $x - $y
    };
    ($x:expr, kali, $y:expr) => {
        $x * $y
    };
}

// ── Macro variadic (jumlah argumen variabel) ────────────────
// `$($x:expr),*` = nol atau lebih expression dipisah koma
// `$($x:expr),+` = satu atau lebih expression dipisah koma
macro_rules! vektor {
    // Pattern: vec-like creation
    ( $( $elem:expr ),* ) => {
        {
            let mut v = Vec::new();
            $( v.push($elem); )* // repeat untuk setiap elemen
            v
        }
    };
}

// ── Macro untuk membuat HashMap ─────────────────────────────
macro_rules! hashmap {
    ( $( $key:expr => $value:expr ),* $(,)? ) => {
        {
            let mut map = std::collections::HashMap::new();
            $( map.insert($key, $value); )*
            map
        }
    };
}

// ── Macro untuk debug print ─────────────────────────────────
// Mirip dbg! tapi dengan format custom
macro_rules! debug {
    ($val:expr) => {
        println!(
            "[DEBUG] {} = {:?} ({}:{})",
            stringify!($val), // konversi expression ke string literal
            $val,
            file!(),    // nama file
            line!(),    // nomor baris
        );
    };
}

// ── Macro untuk error handling ──────────────────────────────
macro_rules! coba {
    ($expr:expr) => {
        match $expr {
            Ok(val) => val,
            Err(e) => {
                eprintln!("Error di {}:{}: {}", file!(), line!(), e);
                return;
            }
        }
    };
}

// ── Macro untuk membuat struct dengan builder ───────────────
macro_rules! buat_struct {
    (
        $nama:ident {
            $( $field:ident : $tipe:ty ),* $(,)?
        }
    ) => {
        #[derive(Debug, Clone)]
        struct $nama {
            $( $field: $tipe, )*
        }

        impl $nama {
            fn new( $( $field: $tipe, )* ) -> Self {
                $nama { $( $field, )* }
            }
        }
    };
}

// Gunakan macro untuk membuat struct
buat_struct! {
    Mahasiswa {
        nama: String,
        npm: String,
        ipk: f64,
    }
}

buat_struct! {
    Buku {
        judul: String,
        penulis: String,
        halaman: u32,
    }
}

// ── Macro untuk implementasi trait secara massal ────────────
macro_rules! impl_display {
    ( $( $tipe:ty => $format:expr ),* $(,)? ) => {
        $(
            impl std::fmt::Display for $tipe {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    write!(f, $format, self)
                }
            }
        )*
    };
}

// ── Macro untuk test helper ─────────────────────────────────
macro_rules! assert_antara {
    ($val:expr, $min:expr, $max:expr) => {
        assert!(
            $val >= $min && $val <= $max,
            "{} = {} tidak berada di antara {} dan {}",
            stringify!($val),
            $val,
            $min,
            $max
        );
    };
}

// ── Macro repeat dengan separator ───────────────────────────
macro_rules! cetak_semua {
    // Terima banyak expression
    ( $( $item:expr ),* ) => {
        $(
            println!("→ {}", $item);
        )*
    };
}

// ── Macro untuk enum dengan method ──────────────────────────
macro_rules! buat_enum_warna {
    ( $( $variant:ident => ($r:expr, $g:expr, $b:expr) ),* $(,)? ) => {
        #[derive(Debug, Clone, Copy)]
        enum Warna {
            $( $variant, )*
        }

        impl Warna {
            fn rgb(&self) -> (u8, u8, u8) {
                match self {
                    $( Warna::$variant => ($r, $g, $b), )*
                }
            }

            fn hex(&self) -> String {
                let (r, g, b) = self.rgb();
                format!("#{:02X}{:02X}{:02X}", r, g, b)
            }
        }
    };
}

buat_enum_warna! {
    Merah => (255, 0, 0),
    Hijau => (0, 255, 0),
    Biru => (0, 0, 255),
    Kuning => (255, 255, 0),
    Putih => (255, 255, 255),
    Hitam => (0, 0, 0),
}

fn main() {
    // ── Macro sederhana ─────────────────────────────────────
    sapa!();
    sapa_nama!("Fadlur");
    sapa_nama!(42); // macro menerima expression apapun!

    // ── Macro hitung ────────────────────────────────────────
    println!("Hitung 5: {}", hitung!(5));
    println!("3 + 4: {}", hitung!(3, tambah, 4));
    println!("10 - 3: {}", hitung!(10, kurang, 3));
    println!("6 * 7: {}", hitung!(6, kali, 7));

    // ── Macro vektor ────────────────────────────────────────
    let v = vektor![1, 2, 3, 4, 5];
    println!("Vektor: {:?}", v);

    let buah = vektor!["Apel", "Jeruk", "Mangga"];
    println!("Buah: {:?}", buah);

    // ── Macro hashmap ───────────────────────────────────────
    let skor = hashmap! {
        "Budi" => 85,
        "Ani" => 92,
        "Cici" => 78,
    };
    println!("Skor: {:?}", skor);

    // ── Macro debug ─────────────────────────────────────────
    let x = 42;
    let nama = "Rust";
    debug!(x);
    debug!(nama);
    debug!(x * 2 + 1);

    // ── Struct dari macro ───────────────────────────────────
    let mhs = Mahasiswa::new(
        String::from("Budi"),
        String::from("123456"),
        3.75,
    );
    println!("Mahasiswa: {:?}", mhs);

    let buku = Buku::new(
        String::from("The Rust Programming Language"),
        String::from("Steve Klabnik"),
        560,
    );
    println!("Buku: {:?}", buku);

    // ── cetak_semua ─────────────────────────────────────────
    cetak_semua!("Halo", 42, 3.14, true);

    // ── Enum dari macro ─────────────────────────────────────
    let warna = Warna::Merah;
    println!("Warna: {:?}, RGB: {:?}, Hex: {}", warna, warna.rgb(), warna.hex());

    let biru = Warna::Biru;
    println!("Warna: {:?}, RGB: {:?}, Hex: {}", biru, biru.rgb(), biru.hex());

    // ── assert_antara ───────────────────────────────────────
    let nilai = 85;
    assert_antara!(nilai, 0, 100);
    println!("Nilai {} valid (0-100)", nilai);

    // ── BUILT-IN MACROS ─────────────────────────────────────
    println!("\n=== Built-in Macros ===");

    // file! dan line! — info posisi kode
    println!("File: {}", file!());
    println!("Line: {}", line!());

    // stringify! — konversi kode ke string
    println!("Kode: {}", stringify!(1 + 2 * 3));

    // concat! — gabung string literal saat compile time
    let s = concat!("Halo", " ", "Dunia", "!");
    println!("Concat: {}", s);

    // env! — baca environment variable saat compile time
    // println!("HOME: {}", env!("HOME"));

    // include_str! — include file sebagai string saat compile time
    // let readme = include_str!("../../README.md");

    // cfg! — conditional compilation
    if cfg!(target_os = "linux") {
        println!("Running on Linux! 🐧");
    }

    // todo!, unimplemented!, unreachable!
    // todo!("Belum diimplementasi"); // panic dengan pesan
    // unimplemented!(); // panic
    // unreachable!(); // panic — seharusnya tidak pernah tercapai
}

// ============================================================
// 🧠 RINGKUMAN MACROS:
//
// ┌─────────────────────────────────────────────────────────────┐
// │                    DECLARATIVE MACRO                        │
// ├──────────────────┬──────────────────────────────────────────┤
// │ Definisi         │ macro_rules! nama { (pattern) => {…} }  │
// │ Parameter        │ $nama:fragmen_type                       │
// │ Repetition       │ $( … ),* atau $( … ),+                    │
// │ Optional         │ $( … )?                                  │
// │ Separator        │ , ; + => (bisa custom)                   │
// └──────────────────┴──────────────────────────────────────────┘
//
// ┌─────────────────────────────────────────────────────────────┐
// │                    FRAGMENT TYPES                           │
// ├──────────────────┬──────────────────────────────────────────┤
// │ $x:expr          │ Expression (42, "halo", a + b)           │
// │ $x:ident         │ Identifier (nama variabel/fungsi/struct) │
// │ $x:ty            │ Type (i32, String, Vec<T>)               │
// │ $x:pat           │ Pattern (Some(x), (a, b), _)             │
// │ $x:stmt          │ Statement (let x = 5;)                   │
// │ $x:block         │ Block ({ ... })                          │
// │ $x:item          │ Item (fn, struct, impl, use)             │
// │ $x:meta          │ Meta item (derive(Debug))                │
// │ $x:tt            │ Token tree (anything — paling fleksibel) │
// │ $x:literal       │ Literal (42, "halo", true)               │
// │ $x:path          │ Path (std::collections::HashMap)         │
// └──────────────────┴──────────────────────────────────────────┘
//
// ⚠️ COMMON MISTAKES:
// - Macro expansion yang tidak terduga → debug dengan cargo expand
// - Recursive macro tanpa base case → infinite recursion
// - Hygiene issue (variable names collide)
// - Pattern yang overlap → macro pilih yang pertama cocok
// - Lupa ; setelah macro call (kadang perlu, kadang tidak)
//
// 🔗 PERBANDINGAN:
// | Rust              | C                | Lisp              |
// |-------------------|------------------|-------------------|
// | macro_rules!      | #define          │ defmacro          │
// | compile-time      │ preprocessor     │ compile-time      │
// | hygienic          │ not hygienic     │ hygienic          │
// | pattern matching  │ text substitution│ code-as-data      │
// ============================================================

// ============================================================
// 🏋️ LATIHAN:
// 1. Buat macro `min!` yang menerima 2+ angka dan return terkecil
// 2. Buat macro `json!` yang membuat JSON-like structure
// 3. Buat macro `measure!` yang mengukur waktu eksekusi expression
// 4. Buat macro `derive_new!` yang generate constructor `new()`
//    untuk struct apapun
// 5. Buat macro `match_str!` yang membuat string matching
//    case-insensitive
// 6. Buat macro recursive untuk menghitung factorial saat compile
// ============================================================
