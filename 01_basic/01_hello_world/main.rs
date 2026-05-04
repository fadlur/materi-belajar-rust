// ============================================================
// 📗 BELAJAR RUST #01 — Hello World
// ============================================================
// Setiap program Rust dimulai dari fungsi `main`.
// `fn` adalah keyword untuk mendeklarasikan fungsi.
// `println!` adalah MACRO (ditandai tanda seru `!`), bukan fungsi biasa.
// Macro bisa menerima jumlah argumen yang bervariasi.
//
// 🎯 Tujuan: Memahami struktur program Rust paling dasar dan
//    berbagai cara mencetak teks ke terminal.
//
// 💡 Analogi: `fn main()` seperti "tombol start" pada mesin —
//    ini adalah titik masuk (entry point) setiap program Rust.
//    Tanpa fungsi main, program tidak tahu harus mulai dari mana!
// ============================================================

fn main() {
    // println! mencetak teks ke terminal dengan newline (baris baru) di akhir
    // "println" = "print line" → cetak satu baris penuh
    println!("Halo, dunia! Selamat datang di Rust! 🦀");

    // ── FORMAT STRING: Menyisipkan Nilai ────────────────────
    // Kita bisa menyisipkan nilai ke dalam string menggunakan `{}`
    // Ini disebut "format string" — mirip printf di C atau f-string di Python
    //
    // 💡 Analogi: `{}` seperti "blank" atau "kotak kosong" dalam formulir
    //    yang akan diisi dengan nilai variabel.
    let nama = "Fadlur";
    println!("Halo, {}! Semangat belajar Rust!", nama);

    // Kita juga bisa mencetak beberapa nilai sekaligus
    // Rust akan mengisi `{}` secara berurutan dari kiri ke kanan
    let bahasa = "Rust";
    let tahun = 2026;
    println!("{} mulai belajar {} di tahun {}", nama, bahasa, tahun);
    //     ↑              ↑              ↑
    //   nama= Fadlur  bahasa= Rust   tahun= 2026

    // ── FORMAT DENGAN POSISI (INDEX) ─────────────────────────
    // `{0}` merujuk ke argumen ke-0 (pertama), `{1}` ke argumen ke-1, dst.
    // Ini berguna kalau mau menggunakan nilai yang sama berkali-kali
    println!("{0} suka {1}. {1} itu keren!", nama, bahasa);
    //            ↑         ↑
    //         nama      bahasa → dipakai 2 kali!

    // ── FORMAT DENGAN NAMED PARAMETER ────────────────────────
    // Bisa memberi nama pada placeholder — lebih jelas saat banyak argumen
    println!(
        "{nama} sedang coding {bahasa}",
        nama = "Aku",      // ← named parameter 'nama'
        bahasa = "Rust"    // ← named parameter 'bahasa'
    );

    // ── PERBEDAAN print! DAN println! ────────────────────────
    // `print!` → cetak tanpa newline (teks berikutnya menyambung)
    // `println!` → cetak DENGAN newline di akhir
    print!("Ini tanpa newline... ");
    println!("dan ini lanjutannya!");
    // Output: "Ini tanpa newline... dan ini lanjutannya!" (satu baris)

    // ── eprintln! — CETAK KE STDERR ──────────────────────────
    // Program punya DUA output stream:
    //   1. stdout (standard output) → untuk output normal
    //   2. stderr (standard error)  → untuk error/debug
    //
    // 💡 Kenapa dipisah? Saat redirect output ke file, error tetap
    //    muncul di terminal sehingga user bisa melihat masalah.
    eprintln!("Ini pesan error ke stderr");

    // ── DEBUG PRINT: {:?} DAN {:#?} ──────────────────────────
    // `{:?}` = format debug — menampilkan nilai dalam format yang bisa di-inspect
    // Berguna untuk melihat isi variabel kompleks (array, struct, enum)
    let angka = [1, 2, 3, 4, 5];
    println!("Debug array: {:?}", angka);
    // Output: Debug array: [1, 2, 3, 4, 5]

    // `{:#?}` = "pretty debug print" — format lebih rapi dengan indentasi
    println!("Pretty debug: {:#?}", angka);
    // Output:
    // Pretty debug: [
    //     1,
    //     2,
    //     3,
    //     4,
    //     5,
    // ]
}

// ============================================================
// 🧠 KONSEP PENTING YANG HARUS DIINGAT:
//
// 1. Setiap program Rust HARUS punya fungsi `main()`
// 2. Macro ditandai dengan tanda seru `!` — beda dari fungsi biasa
// 3. `{}` adalah placeholder untuk nilai dalam format string
// 4. `println!` otomatis tambahkan newline, `print!` tidak
// 5. `eprintln!` untuk error/debug — terpisah dari output normal
// 6. `{:?}` untuk debug print — semua tipe yang derive Debug bisa dipakai
//
// ⚠️ COMMON MISTAKES:
// - Lupa tanda seru `!` di println → compile error
// - Jumlah `{}` tidak sama dengan jumlah argumen → compile error
// - Mencoba print tipe yang tidak implement Debug tanpa formatter
//
// 🔗 REFERENSI BAHASA LAIN:
// | Rust              | Python              | JavaScript          |
// |-------------------|---------------------|---------------------|
// | println!("{}", x) | print(f"{x}")       | console.log(x)      |
// | print!("{}", x)   | print(x, end="")    | process.stdout.write|
// | eprintln!("{}",x) | print(x, file=sys.stderr) | console.error(x) |
// | {:?}              | repr(x)             | JSON.stringify(x)   |
// ============================================================

// ============================================================
// 🏋️ LATIHAN:
// 1. Ubah nama menjadi nama kamu sendiri
// 2. Tambahkan variabel `hobi` dan cetak bersama nama
// 3. Coba cetak angka 1 sampai 10 menggunakan array dan {:?}
// 4. Apa yang terjadi kalau kamu tulis println! tanpa tanda seru?
//    (Coba dan lihat error-nya!)
// 5. Gunakan {:#?} untuk mencetak tuple: ("Budi", 25, true)
// ============================================================
