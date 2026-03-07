// ============================================================
// 📗 BELAJAR RUST #08 — String vs &str
// ============================================================
// Di Rust ada DUA tipe string utama:
// - String  → tipe owned, disimpan di heap, bisa diubah (growable)
// - &str    → string slice, reference ke data string, immutable
//
// String literal ("halo") bertipe &str
// String::from("halo") bertipe String
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
    s3.push_str("Rust "); // push string slice
    s3.push('🦀'); // push satu karakter
    println!("s3: {}", s3);

    // ── CONCATENATION (PENGGABUNGAN) ────────────────────────
    // Cara 1: operator `+` — perhatikan ownership!
    let hello = String::from("Hello, ");
    let world = String::from("World!");
    let gabung = hello + &world; // hello di-move! world dipinjam
    // println!("{}", hello); // ❌ hello sudah di-move
    println!("{}", gabung);
    println!("{}", world); // ✅ world masih valid (hanya dipinjam)

    // Cara 2: format! — lebih nyaman, tidak ada move
    let bagian1 = String::from("Belajar");
    let bagian2 = String::from("Rust");
    let gabung2 = format!("{} {} {}", bagian1, bagian2, "Seru!");
    println!("{}", gabung2);
    println!("Masih valid: {} {}", bagian1, bagian2); // ✅ keduanya valid

    // ── STRING INDEXING ─────────────────────────────────────
    // ⚠️ Rust TIDAK mendukung indexing langsung pada String!
    let teks = String::from("Halo");
    // let h = teks[0]; // ❌ ERROR! Rust string adalah UTF-8, bukan array char

    // Kenapa? Karena karakter UTF-8 punya ukuran variabel (1-4 bytes)
    // "Halo" = 4 bytes, tapi "Привет" = 12 bytes (2 bytes per karakter Cyrillic)

    // Cara akses karakter: gunakan .chars()
    for c in teks.chars() {
        print!("{} ", c);
    }
    println!();

    // Akses karakter ke-n
    let karakter_ke2 = teks.chars().nth(1); // Option<char>
    println!("Karakter ke-2: {:?}", karakter_ke2); // Some('a')

    // Akses byte
    for b in teks.bytes() {
        print!("{} ", b);
    }
    println!();

    // ── SLICE STRING ────────────────────────────────────────
    // Bisa pakai range — tapi harus tepat di batas karakter UTF-8!
    let salam = String::from("Halo Dunia");
    let slice = &salam[0..4]; // "Halo"
    println!("Slice: {}", slice);

    // ── STRING METHODS ──────────────────────────────────────
    let kalimat = String::from("  Rust itu Keren!  ");

    // Trim whitespace
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

    // Split whitespace
    let kata_kata = "  satu   dua   tiga  ";
    let words: Vec<&str> = kata_kata.split_whitespace().collect();
    println!("Words: {:?}", words);

    // Length
    let teks_id = String::from("Halo");
    let teks_jp = String::from("こんにちは");
    println!("'{}' → {} bytes, {} chars", teks_id, teks_id.len(), teks_id.chars().count());
    println!("'{}' → {} bytes, {} chars", teks_jp, teks_jp.len(), teks_jp.chars().count());

    // ── KONVERSI STRING ↔ ANGKA ─────────────────────────────
    // String ke angka: .parse()
    let angka_str = "42";
    let angka: i32 = angka_str.parse().unwrap(); // unwrap karena bisa gagal
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
    // Gunakan &str untuk:
    // - Parameter fungsi (lebih fleksibel)
    // - Saat tidak perlu mengubah atau memiliki string
    // - String literal
    //
    // Gunakan String untuk:
    // - Saat perlu memiliki (own) data string
    // - Saat perlu mengubah isi string
    // - Saat menyimpan di struct

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
// 🏋️ LATIHAN:
// 1. Buat fungsi yang menghitung jumlah kata dalam &str
// 2. Buat fungsi yang membalik string (reverse)
// 3. Buat fungsi yang mengecek apakah string adalah palindrome
// 4. Buat fungsi yang mengubah "hello world" → "Hello World" (title case)
// 5. Buat fungsi sederhana untuk Caesar cipher (geser huruf n posisi)
// ============================================================
