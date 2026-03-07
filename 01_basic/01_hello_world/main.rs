// ============================================================
// 📗 BELAJAR RUST #01 — Hello World
// ============================================================
// Setiap program Rust dimulai dari fungsi `main`.
// `fn` adalah keyword untuk mendeklarasikan fungsi.
// `println!` adalah MACRO (ditandai tanda seru `!`), bukan fungsi biasa.
// Macro bisa menerima jumlah argumen yang bervariasi.
// ============================================================

fn main() {
    // println! mencetak teks ke terminal dengan newline di akhir
    println!("Halo, dunia! Selamat datang di Rust! 🦀");

    // Kita bisa menyisipkan nilai ke dalam string menggunakan `{}`
    // Ini disebut "format string" — mirip printf di C atau format di Python
    let nama = "Fadlur";
    println!("Halo, {}! Semangat belajar Rust!", nama);

    // Kita juga bisa mencetak beberapa nilai sekaligus
    let bahasa = "Rust";
    let tahun = 2026;
    println!("{} mulai belajar {} di tahun {}", nama, bahasa, tahun);

    // Format dengan posisi (index)
    println!("{0} suka {1}. {1} itu keren!", nama, bahasa);

    // Format dengan named parameter
    println!(
        "{nama} sedang coding {bahasa}",
        nama = "Aku",
        bahasa = "Rust"
    );

    // print! tanpa newline — teks berikutnya menyambung di baris yang sama
    print!("Ini tanpa newline... ");
    println!("dan ini lanjutannya!");

    // eprintln! untuk mencetak ke stderr (biasa untuk error/debug)
    eprintln!("Ini pesan error ke stderr");

    // Debug print menggunakan {:?} — berguna untuk melihat isi variabel
    let angka = [1, 2, 3, 4, 5];
    println!("Debug array: {:?}", angka);

    // Pretty debug print menggunakan {:#?} — format lebih rapi
    println!("Pretty debug: {:#?}", angka);
}

// ============================================================
// 🏋️ LATIHAN:
// 1. Ubah nama menjadi nama kamu sendiri
// 2. Tambahkan variabel `hobi` dan cetak bersama nama
// 3. Coba cetak angka 1 sampai 10 menggunakan array dan {:?}
// 4. Apa yang terjadi kalau kamu tulis println! tanpa tanda seru?
//    (Coba dan lihat error-nya!)
// ============================================================
