// ============================================================
// 📗 BELAJAR RUST #07 — Slice
// ============================================================
// Slice adalah REFERENCE ke sebagian (atau seluruh) data yang
// berurutan (contiguous) dalam memori.
// Slice TIDAK memiliki ownership — hanya meminjam.
// Notasi: &[T] untuk slice of T, &str untuk string slice.
// ============================================================

fn main() {
    // ── STRING SLICE (&str) ─────────────────────────────────
    let kalimat = String::from("Halo Dunia Rust");

    // &kalimat[start..end] — start inklusif, end eksklusif
    let kata1 = &kalimat[0..4]; // "Halo"
    let kata2 = &kalimat[5..10]; // "Dunia"
    let kata3 = &kalimat[11..15]; // "Rust"
    println!("{} {} {}", kata1, kata2, kata3);

    // Shorthand: jika mulai dari 0, bisa hilangkan angka pertama
    let slice_awal = &kalimat[..4]; // sama dengan [0..4]
    // Jika sampai akhir, bisa hilangkan angka terakhir
    let slice_akhir = &kalimat[11..]; // sama dengan [11..15]
    // Seluruh string
    let slice_semua = &kalimat[..]; // seluruh string
    println!("{} | {} | {}", slice_awal, slice_akhir, slice_semua);

    // ⚠️ Hati-hati! Slice string berdasarkan BYTE, bukan karakter!
    // Karakter Unicode bisa lebih dari 1 byte
    let emoji = String::from("🦀 Rust");
    // let bad_slice = &emoji[0..2]; // ❌ PANIC! 🦀 butuh 4 byte
    let good_slice = &emoji[0..4]; // ✅ "🦀" (4 bytes)
    println!("Emoji slice: {}", good_slice);

    // ── STRING LITERAL ADALAH SLICE ─────────────────────────
    // String literal (&str) sudah berupa slice yang menunjuk ke binary
    let literal: &str = "Aku adalah string literal"; // tipe: &str
    println!("{}", literal);

    // ── FUNGSI DENGAN STRING SLICE ──────────────────────────
    let teks = String::from("Selamat Pagi Dunia");
    let kata_pertama = cari_kata_pertama(&teks);
    println!("Kata pertama: {}", kata_pertama);

    // &str bisa menerima baik &String maupun &str
    let literal_test = "Halo Semua";
    let kata = cari_kata_pertama(literal_test); // ✅ &str langsung
    println!("Kata pertama literal: {}", kata);

    // ── ARRAY SLICE (&[T]) ──────────────────────────────────
    let angka = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    // Slice dari array
    let sebagian = &angka[2..5]; // [3, 4, 5]
    println!("Sebagian: {:?}", sebagian);

    let awal = &angka[..3]; // [1, 2, 3]
    let akhir = &angka[7..]; // [8, 9, 10]
    println!("Awal: {:?}, Akhir: {:?}", awal, akhir);

    // ── MUTABLE SLICE (&mut [T]) ────────────────────────────
    let mut data = [10, 20, 30, 40, 50];
    println!("Sebelum: {:?}", data);
    gandakan_slice(&mut data[1..4]); // ubah elemen index 1-3
    println!("Sesudah: {:?}", data); // [10, 40, 60, 80, 50]

    // ── SLICE DARI VEC ──────────────────────────────────────
    let vec_data = vec![100, 200, 300, 400, 500];
    let vec_slice = &vec_data[1..4]; // [200, 300, 400]
    println!("Vec slice: {:?}", vec_slice);

    // Fungsi yang menerima &[i32] bisa terima slice dari array MAUPUN Vec
    println!("Jumlah dari array: {}", jumlah(&angka[2..5]));
    println!("Jumlah dari vec: {}", jumlah(&vec_data[1..4]));

    // ── ITERASI ATAS SLICE ──────────────────────────────────
    let buah = vec!["Apel", "Jeruk", "Mangga", "Durian", "Rambutan"];
    let favorit = &buah[1..4]; // ["Jeruk", "Mangga", "Durian"]
    for (i, b) in favorit.iter().enumerate() {
        println!("Favorit ke-{}: {}", i, b);
    }

    // ── SPLIT & WINDOWS ─────────────────────────────────────
    let data2 = [1, 2, 3, 4, 5, 6, 7, 8];

    // chunks: bagi slice menjadi potongan berukuran n
    for chunk in data2.chunks(3) {
        println!("Chunk: {:?}", chunk);
    }

    // windows: sliding window berukuran n
    for window in data2.windows(3) {
        println!("Window: {:?}", window);
    }

    // split_at: bagi menjadi dua slice
    let (kiri, kanan) = data2.split_at(4);
    println!("Kiri: {:?}, Kanan: {:?}", kiri, kanan);

    // ── SLICE METHODS ───────────────────────────────────────
    let nums = [5, 2, 8, 1, 9, 3];
    println!("Contains 8? {}", nums.contains(&8));
    println!("Starts with [5,2]? {}", nums.starts_with(&[5, 2]));
    println!("Is empty? {}", nums.is_empty());
    println!("First: {:?}", nums.first());
    println!("Last: {:?}", nums.last());
}

// ── FUNGSI YANG MENERIMA STRING SLICE ───────────────────────
// &str lebih fleksibel dari &String — bisa terima keduanya
fn cari_kata_pertama(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &byte) in bytes.iter().enumerate() {
        if byte == b' ' {
            return &s[..i]; // return slice sampai spasi pertama
        }
    }

    s // kalau tidak ada spasi, return seluruh string
}

// ── FUNGSI YANG MENERIMA ARRAY SLICE ────────────────────────
fn jumlah(slice: &[i32]) -> i32 {
    slice.iter().sum()
}

// ── FUNGSI DENGAN MUTABLE SLICE ─────────────────────────────
fn gandakan_slice(slice: &mut [i32]) {
    for elem in slice.iter_mut() {
        *elem *= 2; // dereference dan kalikan 2
    }
}

// ============================================================
// 🏋️ LATIHAN:
// 1. Buat fungsi yang menerima &str dan return kata terakhir
// 2. Buat fungsi yang menerima &[i32] dan return elemen terbesar
// 3. Buat fungsi yang menerima &mut [i32] dan reverse isinya
// 4. Coba buat string slice dari karakter Unicode multi-byte
//    (misal bahasa Jepang/Arab) — lihat apa yang terjadi
// 5. Buat fungsi split sederhana yang menerima &str dan char delimiter,
//    return Vec<&str>
// ============================================================
