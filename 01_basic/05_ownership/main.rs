// ============================================================
// 📗 BELAJAR RUST #05 — Ownership ⭐
// ============================================================
// Ownership adalah konsep PALING PENTING di Rust.
// Ini yang membuat Rust bisa aman tanpa garbage collector.
//
// 🎯 Tujuan: Memahami 3 aturan ownership, perbedaan stack vs heap,
//    konsep move, copy, dan clone — fondasi dari semua fitur Rust.
//
// 💡 Mengapa Ownership Penting?
// Bahasa lain menggunakan garbage collector (Java, Python, Go) atau
// manual memory management (C, C++). Garbage collector lambat dan
// tidak prediktabil. Manual management rentan bug (double free,
// use-after-free, memory leak).
//
// Rust menemukan jalan ketiga: OWNERSHIP — aturan compile-time
// yang menjamin memory safety TANPA runtime overhead!
//
// 🏠 Analogi Utama:
// Bayangkan setiap nilai data seperti RUMAH. Di Rust, setiap rumah
// hanya boleh punya SATU pemilik (owner). Kalau pemilik pindah,
// rumah dijual (move). Kalau pemilik meninggal (scope berakhir),
// rumah dihancurkan (drop/memory dibebaskan).
// ============================================================

fn main() {
    // ── SCOPE & OWNERSHIP ───────────────────────────────────
    // Scope = "area berlaku" sebuah variabel — dibatasi oleh `{ }`
    //
    // 💡 Analogi: Scope seperti masa kontrak sewa. Selama kontrak
    //    berlaku, kamu boleh pakai apartemen. Setelah kontrak habis,
    //    apartemen dikembalikan/dibersihkan.
    {
        // `s` belum ada di sini — belum dideklarasikan
        let s = String::from("halo"); // `s` valid mulai dari sini
        println!("{}", s); // gunakan `s`
    } // scope berakhir — `s` di-drop (memorinya dibebaskan secara otomatis)

    // println!("{}", s); // ❌ ERROR! `s` sudah tidak ada
    // Pesan error: "cannot find value `s` in this scope"
    //
    // 💡 Ini BAGUS! Rust mencegah "use-after-free" bug yang umum di C/C++.

    // ── STACK vs HEAP ───────────────────────────────────────
    // Memori program dibagi dua: STACK dan HEAP.
    //
    // 📚 STACK — seperti tumpukan piring:
    //   - Sangat cepat (hanya memindahkan pointer)
    //   - Data harus berukuran tetap (fixed size) saat compile
    //   - Otomatis di-manage (masuk scope = push, keluar scope = pop)
    //   - Contoh: i32, f64, bool, char, array, tuple
    //
    // 📚 HEAP — seperti gudang penyimpanan:
    //   - Lebih lambat (perlu alokasi dan dealokasi)
    //   - Data bisa berukuran dinamis (grow/shrink)
    //   - Perlu manual tracking — tapi Rust lakukan via ownership!
    //   - Contoh: String, Vec, Box, Rc
    //
    // 💡 Analogi: Stack seperti meja kerja (cepat, terbatas, teratur).
    //    Heap seperti gudang (besar, fleksibel, perlu administrasi).

    // ── COPY (untuk tipe Stack) ─────────────────────────────
    // Tipe stack di-COPY saat di-assign — kedua variabel valid.
    // Ini karena tipe stack kecil dan ukurannya tetap.
    let x = 5;
    let y = x; // x di-COPY ke y (duplikat nilai)
    println!("x = {}, y = {}", x, y); // ✅ Keduanya valid!

    // Tipe yang implement trait `Copy`:
    // - Semua integer (i8, i16, i32, i64, i128, isize)
    // - Semua unsigned integer (u8, u16, u32, u64, u128, usize)
    // - Float (f32, f64)
    // - Boolean (bool)
    // - Character (char)
    // - Tuple yang semua isinya Copy
    // - Array yang semua isinya Copy
    // - Reference (&T)

    // ── MOVE (untuk tipe Heap) ──────────────────────────────
    // Tipe heap di-MOVE saat di-assign — owner berpindah!
    //
    // 💡 Analogi: Move seperti jual beli rumah. Kalau A menjual
    //    rumah ke B, A tidak lagi punya rumah itu. B sekarang
    //    pemilik baru. Tidak ada dua pemilik untuk satu rumah!
    let s1 = String::from("halo");
    let s2 = s1; // s1 di-MOVE ke s2 — s1 TIDAK VALID lagi!

    // println!("{}", s1); // ❌ ERROR! value used after being moved
    println!("s2 = {}", s2); // ✅ s2 adalah owner baru

    // Kenapa? Karena Rust tidak mau ada dua variabel yang menunjuk
    // ke memori heap yang sama — ini bisa menyebabkan "double free" bug.
    //
    // 🔍 Ilustrasi Move:
    // Sebelum:          Setelah move:
    // Stack:            Stack:
    // ┌─────┐           ┌─────┐
    // │ s1  │──┐        │ s1  │──┐ (invalid!)
    // └─────┘  │        └─────┘  │   ↑
    //          ▼                  │   │ (pointer dihapus)
    // Heap:    "halo"             ▼   │
    //                             Heap: "halo"
    //                                    ▲
    //                              ┌─────┘
    //                              │ s2  │ (owner baru)
    //                              └─────┘

    // ── CLONE (Deep Copy untuk tipe Heap) ───────────────────
    // Kalau memang mau duplikat data heap, gunakan `.clone()`
    //
    // 💡 Analogi: Clone seperti membangun rumah identik di lokasi
    //    berbeda. Sekarang ada dua rumah yang identik, masing-masing
    //    punya pemilik sendiri.
    let s3 = String::from("dunia");
    let s4 = s3.clone(); // deep copy — data di-heap di-duplikat
    println!("s3 = {}, s4 = {}", s3, s4); // ✅ Keduanya valid

    // ⚠️ Clone bisa MAHAL (lambat) kalau data besar! Gunakan dengan bijak.
    // Kalau bisa pakai reference (&), jangan clone.

    // ── OWNERSHIP & FUNGSI ──────────────────────────────────
    // Memanggil fungsi = sama seperti assignment (move atau copy)
    //
    // 💡 Analogi: Memanggil fungsi seperti meminjamkan/menyerahkan
    //    barang ke teman. Kalau barangnya Copy (seperti foto), kamu
    //    dan teman punya salinan. Kalau barangnya Move (seperti rumah),
    //    teman yang punya sekarang.

    let teks = String::from("Rust itu keren");
    ambil_ownership(teks); // `teks` di-move ke dalam fungsi
    // println!("{}", teks); // ❌ ERROR! `teks` sudah di-move

    let angka = 42;
    buat_copy(angka); // `angka` di-copy (karena i32 implement Copy)
    println!("angka masih valid: {}", angka); // ✅ OK!

    // ── RETURN VALUE & OWNERSHIP ────────────────────────────
    // Fungsi bisa "mengembalikan" ownership ke pemanggil
    //
    // 💡 Analogi: Fungsi seperti tempat penitipan barang. Kamu
    //    titipkan barang (move ke fungsi), fungsi memprosesnya,
    //    lalu mengembalikannya (return ownership).
    let s5 = beri_ownership(); // ownership berpindah ke s5
    println!("s5 = {}", s5);

    let s6 = String::from("halo");
    let s7 = ambil_dan_kembalikan(s6); // s6 di-move masuk, return ke s7
    // println!("{}", s6); // ❌ s6 sudah tidak valid
    println!("s7 = {}", s7); // ✅ s7 valid

    // ── PATTERN: RETURN TUPLE UNTUK KEMBALIKAN OWNERSHIP ────
    // Kalau fungsi perlu return value DAN kembalikan ownership,
    // gunakan tuple. Tapi ini agak merepotkan — nanti ada solusi
    // lebih baik: REFERENCES!
    let s8 = String::from("hitung panjang saya");
    let (s9, panjang) = hitung_panjang(s8);
    println!("'{}' panjangnya {} karakter", s9, panjang);

    // ── DEMONSTRASI DROP ────────────────────────────────────
    // Rust otomatis memanggil `drop()` saat variabel keluar scope.
    // Ini memastikan memori selalu dibersihkan — tidak pernah leak!
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
} // `teks` di-drop di sini — memory dibebaskan

fn buat_copy(angka: i32) {
    println!("Di dalam fungsi: {}", angka);
} // `angka` keluar scope, tapi karena Copy, tidak ada yang di-drop

fn beri_ownership() -> String {
    let s = String::from("dari fungsi");
    s // ownership berpindah ke pemanggil (return tanpa `;`)
}

fn ambil_dan_kembalikan(s: String) -> String {
    println!("Di dalam fungsi: {}", s);
    s // kembalikan ownership ke pemanggil
}

fn hitung_panjang(s: String) -> (String, usize) {
    let panjang = s.len();
    (s, panjang) // kembalikan string DAN panjangnya sebagai tuple
}

// ============================================================
// 🧠 RINGKUMAN OWNERSHIP — TIGA ATURAN EMAS:
//
// 1️⃣ Setiap nilai di Rust punya satu "owner" (pemilik)
// 2️⃣ Hanya boleh ada SATU owner pada satu waktu
// 3️⃣ Saat owner keluar dari scope, nilai tersebut di-drop (dihapus)
//
// ┌─────────────────────────────────────────────────────────────┐
// │                    STACK vs HEAP                            │
// ├──────────────────┬──────────────────┬───────────────────────┤
// │                  │ STACK            │ HEAP                  │
// ├──────────────────┼──────────────────┼───────────────────────┤
// │ Kecepatan        │ Sangat cepat     │ Lebih lambat          │
// │ Ukuran           │ Tetap (compile)  │ Dinamis (runtime)     │
// │ Assign           │ Copy (duplikat)  │ Move (pindah owner)   │
// │ Cleanup          │ Otomatis (pop)   │ Via drop() saat scope │
// │ Contoh           │ i32, bool, [T;N] │ String, Vec, Box      │
// └──────────────────┴──────────────────┴───────────────────────┘
//
// ⚠️ COMMON MISTAKES:
// - Menggunakan variabel setelah move → compile error!
// - Clone berlebihan → performa buruk
// - Lupa return ownership dari fungsi → data "hilang"
// - Mengira semua tipe di-Copy → String dan Vec di-MOVE!
//
// 🔗 PERBANDINGAN MEMORI:
// | Rust (Ownership)   | C/C++ (Manual)      | Java/Go (GC)        |
// |--------------------|---------------------|---------------------|
// | Compile-time check | Runtime error       | Runtime pause       |
// | Zero overhead      | Prone to bugs       | GC overhead         |
// | Predictable        | Manual free needed  | Unpredictable pause |
// | Move/Copy/Borrow   | malloc/free         | new/delete (gc)     |
// ============================================================

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
// 6. Buat fungsi yang menerima String, return tuple (String, String)
//    berisi string asli dan versi uppercase-nya
// ============================================================
