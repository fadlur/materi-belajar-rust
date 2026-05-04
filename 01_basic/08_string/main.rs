// ============================================================
// 📗 BELAJAR RUST #08 — String vs &str
// ============================================================
// Di Rust ada DUA tipe string utama:
// - String  → tipe owned, disimpan di heap, bisa diubah (growable)
// - &str    → string slice, reference ke data string, immutable
//
// 🎯 Tujuan: Memahami perbedaan String dan &str, cara membuat,
//    memanipulasi, dan mengkonversi antar keduanya.
//
// 💡 Analogi Utama:
// String seperti KERTAS GULUNG — kamu punya gulungannya, bisa
// tulis, hapus, tambah, potong. Ukuran fleksibel tapi kamu
// yang bertanggung jawab (punya ownership).
//
// &str seperti FOTO KOPI dari kertas — kamu hanya melihat isinya,
// tidak bisa ubah, tidak perlu urus kepemilikan. Bisa jadi foto
// kopi dari gulung kertas (String) atau dari buku (string literal).
//
// 🔑 Ingat: String literal ("halo") bertipe &str
//            String::from("halo") bertipe String
// ============================================================

fn main() {
    // ── MEMBUAT STRING ──────────────────────────────────────
    // Cara 1: String::from()
    let s1 = String::from("Halo Rust");

    // Cara 2: .to_string() dari &str
    let s2 = "Halo Dunia".to_string();

    // Cara 3: String::new() — string kosong
    let mut s3 = String::new();

    // Cara 4: format! macro — seperti println! tapi return String
    let nama = "Fadlur";
    let s4 = format!("Halo, {}!", nama);

    println!("{}, {}, {:?}, {}", s1, s2, s3, s4);

    // ── MENAMBAHKAN TEKS (PUSH) ─────────────────────────────
    // String bisa bertambah karena disimpan di heap
    //
    // 💡 Analogi: String seperti tas ransel — bisa dimasukkan
    //    barang baru selama masih ada ruang (atau ruang diperbesar).
    s3.push_str("Rust "); // push string slice (banyak karakter)
    s3.push('🦀');        // push satu karakter
    println!("s3: {}", s3);

    // ── CONCATENATION (PENGGABUNGAN) ────────────────────────
    // Cara 1: operator `+` — perhatikan ownership!
    //
    // 💡 Perhatian: `+` mengambil ownership operand kiri!
    //    `hello + &world` → hello di-move, world dipinjam.
    let hello = String::from("Hello, ");
    let world = String::from("World!");
    let gabung = hello + &world; // hello di-move! world dipinjam
    // println!("{}", hello); // ❌ hello sudah di-move
    println!("{}", gabung);
    println!("{}", world); // ✅ world masih valid (hanya dipinjam)

    // Cara 2: format! — lebih nyaman, tidak ada move
    //
    // 💡 format! lebih aman untuk concatenation karena tidak
    //    mengambil ownership siapa pun — semua dipinjam.
    let bagian1 = String::from("Belajar");
    let bagian2 = String::from("Rust");
    let gabung2 = format!("{} {} {}", bagian1, bagian2, "Seru!");
    println!("{}", gabung2);
    println!("Masih valid: {} {}", bagian1, bagian2); // ✅ keduanya valid

    // ── STRING INDEXING ─────────────────────────────────────
    // ⚠️ Rust TIDAK mendukung indexing langsung pada String!
    // let h = teks[0]; // ❌ ERROR!
    //
    // 💡 Kenapa? Karena karakter UTF-8 punya ukuran variabel (1-4 bytes).
    //    Indexing byte bisa memotong di tengah karakter!
    //
    //    Contoh: "Halo" = 4 bytes (semua ASCII = 1 byte/karakter)
    //            "Привет" (Russia) = 12 bytes (2 bytes/karakter)
    //            "こんにちは" (Jepang) = 15 bytes (3 bytes/karakter)
    //            "🦀" (emoji) = 4 bytes
    //
    //    Kalau kita akses teks[0], apakah itu byte ke-0 atau karakter ke-0?
    //    Rust memilih untuk melarang indexing agar aman.

    let teks = String::from("Halo");

    // Cara akses karakter: gunakan .chars()
    for c in teks.chars() {
        print!("{} ", c);
    }
    println!();

    // Akses karakter ke-n (mengembalikan Option<char>)
    let karakter_ke2 = teks.chars().nth(1); // Option<char>
    println!("Karakter ke-2: {:?}", karakter_ke2); // Some('a')
    // nth() bisa return None kalau index melebihi panjang!

    // Akses byte (untuk yang memang butuh level byte)
    for b in teks.bytes() {
        print!("{} ", b);
    }
    println!();

    // ── SLICE STRING ────────────────────────────────────────
    // Bisa pakai range — tapi harus tepat di batas karakter UTF-8!
    let salam = String::from("Halo Dunia");
    let slice = &salam[0..4]; // "Halo" — setiap karakter 1 byte
    println!("Slice: {}", slice);

    // ── STRING METHODS ──────────────────────────────────────
    let kalimat = String::from("  Rust itu Keren!  ");

    // Trim whitespace (spasi, tab, newline di awal/akhir)
    println!("Trim: '{}'", kalimat.trim());

    // Uppercase / Lowercase
    println!("Upper: {}", kalimat.trim().to_uppercase());
    println!("Lower: {}", kalimat.trim().to_lowercase());

    // Contains, starts_with, ends_with
    println!("Contains 'Keren': {}", kalimat.contains("Keren"));
    println!("Starts with 'Rust': {}", kalimat.trim().starts_with("Rust"));

    // Replace
    let baru = kalimat.trim().replace("Keren", "Awesome");
    println!("Replace: {}", baru);

    // Split
    let csv = "apel,jeruk,mangga,durian";
    let buah: Vec<&str> = csv.split(',').collect();
    println!("Buah: {:?}", buah);

    // Split whitespace (berguna untuk parsing input)
    let kata_kata = "  satu   dua   tiga  ";
    let words: Vec<&str> = kata_kata.split_whitespace().collect();
    println!("Words: {:?}", words);

    // Length: .len() = jumlah BYTE, bukan jumlah karakter!
    let teks_id = String::from("Halo");
    let teks_jp = String::from("こんにちは");
    println!("'{}' → {} bytes, {} chars", teks_id, teks_id.len(), teks_id.chars().count());
    println!("'{}' → {} bytes, {} chars", teks_jp, teks_jp.len(), teks_jp.chars().count());

    // ── KONVERSI STRING ↔ ANGKA ─────────────────────────────
    // String ke angka: .parse()
    //
    // 💡 parse() return Result — bisa Ok atau Err!
    //    Kita harus handle kedua kemungkinan.
    let angka_str = "42";
    let angka: i32 = angka_str.parse().unwrap(); // unwrap = ambil Ok, panic kalau Err
    println!("Angka: {}", angka);

    // Lebih aman dengan match
    let mungkin_angka = "bukan angka";
    match mungkin_angka.parse::<i32>() {
        Ok(n) => println!("Berhasil parse: {}", n),
        Err(e) => println!("Gagal parse: {}", e),
    }

    // Angka ke String: .to_string() atau format!
    let num = 123;
    let num_str = num.to_string();
    let num_str2 = format!("Angka: {}", num);
    println!("{}, {}", num_str, num_str2);

    // ── STRING BUILDER PATTERN ──────────────────────────────
    // Untuk membangun string secara efisien
    // String::with_capacity() mengalokasikan memori sekali,
    // menghindari realokasi berkali-kali.
    let mut builder = String::with_capacity(100); // pre-alokasi kapasitas
    for i in 0..5 {
        builder.push_str(&format!("item-{} ", i));
    }
    println!("Builder: {}", builder.trim());
    println!("Len: {}, Capacity: {}", builder.len(), builder.capacity());

    // ── ESCAPE CHARACTERS ───────────────────────────────────
    let escape = "Tab:\tNewline:\nBackslash: \\Quote: \"";
    println!("{}", escape);

    // Raw string — tidak memproses escape
    let raw = r"Ini raw string: \n tidak jadi newline";
    println!("{}", raw);

    // Raw string dengan hashtag (jika perlu tanda kutip di dalamnya)
    let raw2 = r#"Bisa pakai "tanda kutip" di dalam"#;
    println!("{}", raw2);

    // ── &str vs String: KAPAN PAKAI YANG MANA? ──────────────
    //
    // Gunakan &str untuk:
    // - Parameter fungsi (lebih fleksibel)
    // - Saat tidak perlu mengubah atau memiliki string
    // - String literal
    // - Return type saat hanya perlu baca
    //
    // Gunakan String untuk:
    // - Saat perlu memiliki (own) data string
    // - Saat perlu mengubah isi string
    // - Saat menyimpan di struct
    // - Saat data dibuat di runtime

    cetak_pesan("Halo dari &str literal"); // &str langsung
    cetak_pesan(&s1); // &String auto-coerce ke &str

    let owned = buat_salam("Fadlur");
    println!("{}", owned);
}

// Parameter &str — bisa menerima &str maupun &String
fn cetak_pesan(pesan: &str) {
    println!("Pesan: {}", pesan);
}

// Return String — karena kita membuat data baru
fn buat_salam(nama: &str) -> String {
    format!("Selamat datang, {}!", nama)
}

// ============================================================
// 🧠 RINGKUMAN STRING DI RUST:
//
// ┌─────────────────────────────────────────────────────────────┐
// │                    STRING vs &str                           │
// ├──────────────────┬──────────────────┬───────────────────────┤
// │                  │ String           │ &str                  │
// ├──────────────────┼──────────────────┼───────────────────────┤
// │ Owned?           │ ✅ Ya            │ ❌ Tidak (borrow)     │
// │ Mutable?         │ ✅ Bisa (mut)    │ ❌ Tidak              │
// │ Stored in        │ Heap             │ Stack (reference)     │
// │ Growable?        │ ✅ Ya            │ ❌ Tidak              │
// │ Indexing?        │ ❌ Tidak langsung│ ❌ Tidak langsung     │
// │ Parameter fn     │ Kalau perlu own  │ ✅ Prefer &str        │
// │ String literal   │ String::from()   │ "halo"                │
// └──────────────────┴──────────────────┴───────────────────────┘
//
// 💡 RULE OF THUMB:
//   - Butuh own data? → String
//   - Hanya baca? → &str
//   - Parameter fungsi? → &str (paling fleksibel)
//   - Return value baru? → String
//
// ⚠️ COMMON MISTAKES:
// - Mengira teks[0] mengembalikan karakter → sebenarnya forbidden!
// - Lupa parse() return Result → unwrap bisa panic
// - Concatenation dengan + mengambil ownership kiri
// - Menghitung .len() sebagai jumlah karakter → itu jumlah byte!
// - Slice string di tengah karakter multi-byte → PANIC
//
// 🔗 PERBANDINGAN:
// | Rust              | Python           | JavaScript        |
// |-------------------|------------------|-------------------|
// | String            | str              | String            |
// | &str              | (string literal) | (string primitive)|
// | "halo".to_string()| str("halo")      | new String("halo")|
// | s.push_str()      | s += "..."       | s += "..."        |
// | s.len()           | len(s)           | s.length          |
// | s.chars().nth(0)  | s[0]             | s[0]              |
// | s.parse::<i32>()  | int(s)           | parseInt(s)       |
// ============================================================

// ============================================================
// 🏋️ LATIHAN:
// 1. Buat fungsi yang menghitung jumlah kata dalam &str
// 2. Buat fungsi yang membalik string (reverse)
// 3. Buat fungsi yang mengecek apakah string adalah palindrome
// 4. Buat fungsi yang mengubah "hello world" → "Hello World" (title case)
// 5. Buat fungsi sederhana untuk Caesar cipher (geser huruf n posisi)
// 6. Hitung perbedaan byte vs karakter untuk string Jepang
// 7. Buat fungsi yang parse CSV sederhana: "a,b,c" → Vec<&str>
// ============================================================
