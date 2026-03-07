// ============================================================
// 📗 BELAJAR RUST #05 — Ownership ⭐
// ============================================================
// Ownership adalah konsep PALING PENTING di Rust.
// Ini yang membuat Rust bisa aman tanpa garbage collector.
//
// TIGA ATURAN OWNERSHIP:
// 1. Setiap nilai di Rust punya satu "owner" (pemilik)
// 2. Hanya boleh ada SATU owner pada satu waktu
// 3. Saat owner keluar dari scope, nilai tersebut di-drop (dihapus)
// ============================================================

fn main() {
    // ── SCOPE & OWNERSHIP ───────────────────────────────────
    {
        // `s` belum ada di sini
        let s = String::from("halo"); // `s` valid mulai dari sini
        println!("{}", s); // gunakan `s`
    } // scope berakhir — `s` di-drop (memorinya dibebaskan)

    // println!("{}", s); // ❌ ERROR! `s` sudah tidak ada

    // ── STACK vs HEAP ───────────────────────────────────────
    // Tipe sederhana (integer, bool, char, float) disimpan di STACK
    // → Ukuran diketahui saat compile time, sangat cepat
    // Tipe kompleks (String, Vec, dll) disimpan di HEAP
    // → Ukuran dinamis, perlu alokasi/dealokasi memori

    // ── COPY (untuk tipe Stack) ─────────────────────────────
    // Tipe stack di-COPY saat di-assign — kedua variabel valid
    let x = 5;
    let y = x; // x di-COPY ke y
    println!("x = {}, y = {}", x, y); // ✅ Keduanya valid!

    // Tipe yang implement trait `Copy`: integer, float, bool, char, tuple (jika isinya Copy)

    // ── MOVE (untuk tipe Heap) ──────────────────────────────
    // Tipe heap di-MOVE saat di-assign — owner berpindah!
    let s1 = String::from("halo");
    let s2 = s1; // s1 di-MOVE ke s2 — s1 TIDAK VALID lagi!

    // println!("{}", s1); // ❌ ERROR! value used after being moved
    println!("s2 = {}", s2); // ✅ s2 adalah owner baru

    // Kenapa? Karena Rust tidak mau ada dua variabel yang menunjuk
    // ke memori heap yang sama — ini bisa menyebabkan "double free" bug.

    // ── CLONE (Deep Copy untuk tipe Heap) ───────────────────
    // Kalau memang mau duplikat data heap, gunakan `.clone()`
    let s3 = String::from("dunia");
    let s4 = s3.clone(); // deep copy — data di-heap di-duplikat
    println!("s3 = {}, s4 = {}", s3, s4); // ✅ Keduanya valid

    // ── OWNERSHIP & FUNGSI ──────────────────────────────────
    // Memanggil fungsi = sama seperti assignment (move atau copy)

    let teks = String::from("Rust itu keren");
    ambil_ownership(teks); // `teks` di-move ke dalam fungsi
    // println!("{}", teks); // ❌ ERROR! `teks` sudah di-move

    let angka = 42;
    buat_copy(angka); // `angka` di-copy (karena i32 implement Copy)
    println!("angka masih valid: {}", angka); // ✅ OK!

    // ── RETURN VALUE & OWNERSHIP ────────────────────────────
    // Fungsi bisa "mengembalikan" ownership ke pemanggil
    let s5 = beri_ownership(); // ownership berpindah ke s5
    println!("s5 = {}", s5);

    let s6 = String::from("halo");
    let s7 = ambil_dan_kembalikan(s6); // s6 di-move masuk, return ke s7
    // println!("{}", s6); // ❌ s6 sudah tidak valid
    println!("s7 = {}", s7); // ✅ s7 valid

    // ── PATTERN: RETURN TUPLE UNTUK KEMBALIKAN OWNERSHIP ────
    // Ini agak merepotkan — nanti ada solusi lebih baik (references!)
    let s8 = String::from("hitung panjang saya");
    let (s9, panjang) = hitung_panjang(s8);
    println!("'{}' panjangnya {} karakter", s9, panjang);

    // ── DEMONSTRASI DROP ────────────────────────────────────
    {
        let _data = String::from("akan segera di-drop");
        println!("Data masih di scope");
    } // `_data` di-drop di sini — Rust memanggil `drop()` otomatis

    println!("Data sudah di-drop di scope sebelumnya");

    // ── MEMAHAMI DENGAN VISUALISASI ─────────────────────────
    // Stack:          Heap:
    // ┌─────┐        ┌───────────┐
    // │ s1  │──────→ │ "halo"    │  ← setelah move, pointer ini invalid
    // └─────┘        └───────────┘
    // ┌─────┐             ↑
    // │ s2  │─────────────┘      ← s2 sekarang yang menunjuk ke sini
    // └─────┘
    //
    // Dengan clone:
    // ┌─────┐        ┌───────────┐
    // │ s3  │──────→ │ "dunia"   │  ← data asli
    // └─────┘        └───────────┘
    // ┌─────┐        ┌───────────┐
    // │ s4  │──────→ │ "dunia"   │  ← salinan baru di heap
    // └─────┘        └───────────┘
}

fn ambil_ownership(teks: String) {
    println!("Di dalam fungsi: {}", teks);
} // `teks` di-drop di sini

fn buat_copy(angka: i32) {
    println!("Di dalam fungsi: {}", angka);
} // `angka` keluar scope, tapi karena Copy, tidak ada yang di-drop

fn beri_ownership() -> String {
    let s = String::from("dari fungsi");
    s // ownership berpindah ke pemanggil
}

fn ambil_dan_kembalikan(s: String) -> String {
    println!("Di dalam fungsi: {}", s);
    s // kembalikan ownership
}

fn hitung_panjang(s: String) -> (String, usize) {
    let panjang = s.len();
    (s, panjang) // kembalikan string DAN panjangnya
}

// ============================================================
// 🏋️ LATIHAN:
// 1. Buat dua variabel String, move satu ke yang lain, buktikan yang
//    pertama tidak bisa dipakai lagi
// 2. Buat fungsi yang menerima Vec<i32> dan return Vec<i32>
//    yang sudah di-sort (perhatikan ownership!)
// 3. Coba buat fungsi yang menerima String, lalu gunakan String
//    tersebut lagi setelah pemanggilan — apa yang terjadi?
// 4. Pahami: kenapa `let x = 5; let y = x;` tidak move tapi copy?
// 5. Coba `drop(s)` secara manual lalu gunakan `s` — lihat error-nya
// ============================================================
