// ============================================================
// 📘 BELAJAR RUST #09 — Struct
// ============================================================
// Struct adalah cara untuk mengelompokkan data terkait menjadi
// satu tipe custom. Mirip class di OOP, tapi tanpa inheritance.
// Rust punya 3 jenis struct: named, tuple, dan unit struct.
// ============================================================

// ── NAMED STRUCT ────────────────────────────────────────────
// Struct paling umum — setiap field punya nama dan tipe
#[derive(Debug)] // derive Debug agar bisa di-print dengan {:?}
struct Pengguna {
    nama: String,
    email: String,
    umur: u32,
    aktif: bool,
}

// ── TUPLE STRUCT ────────────────────────────────────────────
// Struct tanpa nama field — akses pakai index (.0, .1, .2)
// Berguna untuk membuat "newtype" — tipe baru dari tipe yang ada
#[derive(Debug)]
struct Warna(u8, u8, u8); // RGB

#[derive(Debug)]
struct Titik(f64, f64); // koordinat X, Y

// ── UNIT STRUCT ─────────────────────────────────────────────
// Struct tanpa field — berguna untuk trait implementation
struct Penanda;

// ── IMPL BLOCK: METHODS & ASSOCIATED FUNCTIONS ──────────────
// `impl` mendefinisikan method (fungsi yang terkait dengan struct)
impl Pengguna {
    // Associated function (seperti static method di bahasa lain)
    // Dipanggil dengan `Pengguna::baru(...)` — tanpa `self`
    fn baru(nama: String, email: String, umur: u32) -> Pengguna {
        Pengguna {
            nama,   // shorthand: kalau nama field = nama variabel
            email,  // tidak perlu tulis `email: email`
            umur,
            aktif: true, // default value
        }
    }

    // Method — parameter pertama SELALU `self` (dalam berbagai bentuk)
    // `&self` = immutable reference ke instance
    fn salam(&self) -> String {
        format!("Halo, nama saya {} ({})", self.nama, self.email)
    }

    // `&mut self` = mutable reference — bisa mengubah data
    fn nonaktifkan(&mut self) {
        self.aktif = false;
        println!("{} telah dinonaktifkan", self.nama);
    }

    // `self` (tanpa &) = mengambil ownership — jarang dipakai
    // Biasa dipakai saat transformasi: konsumsi diri, return yang baru
    fn dengan_email(mut self, email_baru: String) -> Pengguna {
        self.email = email_baru;
        self // return self yang sudah dimodifikasi
    }

    // Method dengan parameter tambahan
    fn sudah_dewasa(&self) -> bool {
        self.umur >= 18
    }

    fn info(&self) {
        println!("┌───────────────────────────────");
        println!("│ Nama  : {}", self.nama);
        println!("│ Email : {}", self.email);
        println!("│ Umur  : {}", self.umur);
        println!("│ Aktif : {}", self.aktif);
        println!("│ Dewasa: {}", self.sudah_dewasa());
        println!("└───────────────────────────────");
    }
}

// Bisa punya MULTIPLE impl block untuk satu struct
impl Pengguna {
    fn ganti_nama(&mut self, nama_baru: &str) {
        self.nama = nama_baru.to_string();
    }
}

// Method untuk Titik
impl Titik {
    fn baru(x: f64, y: f64) -> Self {
        // `Self` = alias untuk tipe struct ini (Titik)
        Self(x, y)
    }

    fn jarak_ke(&self, lain: &Titik) -> f64 {
        let dx = self.0 - lain.0;
        let dy = self.1 - lain.1;
        (dx * dx + dy * dy).sqrt()
    }

    fn origin() -> Self {
        Self(0.0, 0.0)
    }
}

// ── STRUCT DENGAN LIFETIME (preview — detail di advanced) ───
// Kalau struct menyimpan reference, butuh lifetime annotation
#[derive(Debug)]
struct Kutipan<'a> {
    teks: &'a str,
    penulis: &'a str,
}

fn main() {
    // ── MEMBUAT INSTANCE STRUCT ─────────────────────────────
    let user1 = Pengguna {
        nama: String::from("Budi"),
        email: String::from("budi@email.com"),
        umur: 25,
        aktif: true,
    };
    println!("{:?}", user1);

    // Menggunakan associated function (constructor)
    let user2 = Pengguna::baru(
        String::from("Ani"),
        String::from("ani@email.com"),
        30,
    );
    user2.info();

    // ── AKSES FIELD ─────────────────────────────────────────
    println!("Nama: {}", user2.nama);
    println!("Dewasa: {}", user2.sudah_dewasa());

    // ── MENGUBAH FIELD (harus mut) ──────────────────────────
    let mut user3 = Pengguna::baru(
        String::from("Cici"),
        String::from("cici@email.com"),
        16,
    );
    user3.umur = 17;
    user3.nonaktifkan();
    user3.info();

    // ── STRUCT UPDATE SYNTAX ────────────────────────────────
    // Buat struct baru berdasarkan yang lama — sisanya di-copy/move
    let user4 = Pengguna {
        nama: String::from("Dedi"),
        email: String::from("dedi@email.com"),
        ..user1 // sisa field diambil dari user1
        // ⚠️ user1 sebagian di-move! (field String di-move)
        // user1.umur dan user1.aktif masih bisa diakses (Copy types)
    };
    user4.info();

    // ── METHOD CHAINING (Builder Pattern) ───────────────────
    let user5 = Pengguna::baru(
        String::from("Eka"),
        String::from("eka@old.com"),
        28,
    )
    .dengan_email(String::from("eka@new.com")); // consume & return
    user5.info();

    // ── TUPLE STRUCT ────────────────────────────────────────
    let merah = Warna(255, 0, 0);
    let biru = Warna(0, 0, 255);
    println!("Merah: RGB({}, {}, {})", merah.0, merah.1, merah.2);
    println!("Biru: {:?}", biru);

    let p1 = Titik::baru(3.0, 4.0);
    let p2 = Titik::origin();
    println!("Jarak p1 ke origin: {:.2}", p1.jarak_ke(&p2));

    // ── STRUCT DENGAN REFERENCE ─────────────────────────────
    let kutipan = Kutipan {
        teks: "Saya berpikir, maka saya ada",
        penulis: "Descartes",
    };
    println!("{:?}", kutipan);
    println!("\"{}\" — {}", kutipan.teks, kutipan.penulis);

    // ── DESTRUCTURING STRUCT ────────────────────────────────
    let Titik(x, y) = p1;
    println!("x = {}, y = {}", x, y);

    // Named struct destructuring
    let user6 = Pengguna::baru(
        String::from("Fani"),
        String::from("fani@email.com"),
        22,
    );
    let Pengguna { nama, umur, .. } = user6; // `..` = abaikan sisanya
    println!("Destructured: {} umur {}", nama, umur);

    // ── PRINT DEBUG ─────────────────────────────────────────
    let user7 = Pengguna::baru(
        String::from("Gani"),
        String::from("gani@email.com"),
        35,
    );
    println!("Debug: {:?}", user7);
    println!("Pretty: {:#?}", user7);

    // ── UNIT STRUCT ─────────────────────────────────────────
    let _p = Penanda; // tidak ada data, berguna untuk marker trait
}

// ============================================================
// 🏋️ LATIHAN:
// 1. Buat struct `Persegi` dengan field `sisi: f64` dan method
//    `luas()` dan `keliling()`
// 2. Buat struct `Buku` dengan field judul, penulis, halaman, harga.
//    Tambahkan method `diskon(persen)` yang return harga setelah diskon
// 3. Buat struct `Kalkulator` yang menyimpan riwayat operasi (Vec<String>)
//    dan punya method tambah, kurang, kali, bagi
// 4. Implementasikan builder pattern untuk struct yang kompleks
// 5. Buat struct `Matrix2x2` dan implementasikan perkalian matrix
// ============================================================
