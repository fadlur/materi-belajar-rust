// ============================================================
// 📘 BELAJAR RUST #16 — Closures
// ============================================================
// Closure = anonymous function yang bisa "menangkap" (capture)
// variabel dari lingkungan sekitarnya.
// Mirip lambda di Python/Java, arrow function di JavaScript.
//
// 🎯 Tujuan: Memahami sintaks closure, capturing environment,
//    tiga trait closure (Fn, FnMut, FnOnce), dan penggunaan
//    closure dengan iterator methods.
//
// 💡 Analogi Utama:
// Closure seperti RUMAH dengan MEMORI — setiap closure "mengingat"
// variabel dari tempat ia dibuat. Kalau kamu membuat closure di
// dalam fungsi, closure itu bisa mengakses variabel fungsi tersebut
// bahkan setelah fungsi selesai (kalau di-move ke luar).
//
// 🔑 Closures sangat powerful di Rust karena mereka:
//   1. Bisa capture environment
//   2. Bisa dipassing sebagai parameter
//   3. Bisa di-return dari fungsi
//   4. Bekerja seamlessly dengan iterators
// ============================================================

fn main() {
    // ── SINTAKS CLOSURE ─────────────────────────────────────
    // Closure didefinisikan dengan `|parameter| body`
    //
    // 💡 Analogi: Closure seperti memo kecil yang bisa menyimpan
    //    catatan dari meja kerja — dan catatan itu bisa dibawa
    //    ke mana pun closure pergi.

    // Closure sederhana
    let sapa = |nama: &str| println!("Halo, {}!", nama);
    sapa("Fadlur");
    sapa("Rust");

    // Closure dengan return value (implisit)
    let tambah = |a: i32, b: i32| a + b;
    println!("3 + 5 = {}", tambah(3, 5));

    // Closure multi-line dengan block `{}`
    let hitung_pajak = |harga: f64, persen: f64| {
        let pajak = harga * persen / 100.0;
        let total = harga + pajak;
        total
    };
    println!("Total: Rp {:.0}", hitung_pajak(100_000.0, 11.0));

    // Closure tanpa parameter
    let salam = || println!("Selamat pagi!");
    salam();

    // ── TYPE INFERENCE ──────────────────────────────────────
    // Rust bisa menebak tipe closure dari konteks
    // Tapi sekali ditebak, tipenya FIXED — tidak bisa berubah!
    let contoh = |x| x; // tipe ditentukan saat pertama dipanggil
    let _s = contoh(String::from("halo")); // x adalah String
    // let n = contoh(5); // ❌ ERROR! x sudah ditetapkan String

    // ── CAPTURING ENVIRONMENT ───────────────────────────────
    // Closure bisa "menangkap" variabel dari scope luar!
    //
    // 💡 Analogi: Bayangkan closure seperti kapsul waktu —
    //    saat dibuat, ia menyimpan "snapshot" variabel sekitarnya.

    // Capture by reference (immutable borrow) — default
    let pesan = String::from("Halo dari luar");
    let cetak = || println!("{}", pesan);
    cetak();
    println!("pesan masih ada: {}", pesan); // ✅ masih valid

    // Capture by mutable reference
    let mut counter = 0;
    let mut tambah_counter = || {
        counter += 1; // mutable borrow
        println!("Counter: {}", counter);
    };
    tambah_counter();
    tambah_counter();
    tambah_counter();
    // counter tidak bisa diakses selama closure masih aktif
    // tapi setelah closure terakhir dipanggil:
    println!("Final counter: {}", counter); // ✅ OK

    // Capture by move (ambil ownership)
    let data = vec![1, 2, 3];
    let cetak_data = move || {
        // `move` memindahkan ownership `data` ke dalam closure
        println!("Data: {:?}", data);
    };
    cetak_data();
    // println!("{:?}", data); // ❌ ERROR! data sudah di-move

    // ── CLOSURE SEBAGAI PARAMETER FUNGSI ────────────────────
    // Ada 3 trait closure: Fn, FnMut, FnOnce
    //
    // 💡 Analogi: Tiga tingkat akses ke rumah:
    //   Fn     = boleh masuk dan lihat (immutable borrow)
    //   FnMut  = boleh masuk dan ubah interior (mutable borrow)
    //   FnOnce = ambil rumahnya (take ownership)

    // Fn — closure yang hanya baca (immutable borrow)
    let angka = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let genap: Vec<&i32> = angka.iter().filter(|&&x| x % 2 == 0).collect();
    println!("Genap: {:?}", genap);

    // FnMut — closure yang bisa mengubah state
    let mut hasil: Vec<i32> = Vec::new();
    angka.iter().for_each(|&x| {
        if x > 5 {
            hasil.push(x * 10);
        }
    });
    println!("Hasil filter & transform: {:?}", hasil);

    // ── CLOSURE DENGAN ITERATOR METHODS ─────────────────────
    let angka2 = vec![1, 2, 3, 4, 5];

    // map: transformasi setiap elemen
    let kuadrat: Vec<i32> = angka2.iter().map(|&x| x * x).collect();
    println!("Kuadrat: {:?}", kuadrat);

    // filter: ambil elemen yang memenuhi kondisi
    let besar: Vec<&i32> = angka2.iter().filter(|&&x| x > 3).collect();
    println!("Besar dari 3: {:?}", besar);

    // fold: akumulasi (reduce)
    let jumlah = angka2.iter().fold(0, |acc, &x| acc + x);
    println!("Jumlah: {}", jumlah);

    // find: cari elemen pertama yang memenuhi kondisi
    let pertama_genap = angka2.iter().find(|&&x| x % 2 == 0);
    println!("Pertama genap: {:?}", pertama_genap);

    // any / all: cek apakah ada/semua yang memenuhi kondisi
    let ada_genap = angka2.iter().any(|&x| x % 2 == 0);
    let semua_positif = angka2.iter().all(|&x| x > 0);
    println!("Ada genap: {}, Semua positif: {}", ada_genap, semua_positif);

    // ── CLOSURE SEBAGAI PARAMETER ───────────────────────────
    terapkan_dan_cetak(5, |x| x * x);
    terapkan_dan_cetak(5, |x| x + 100);
    terapkan_dan_cetak(5, |x| x * 2 + 1);

    // ── CLOSURE SEBAGAI RETURN VALUE ────────────────────────
    let pengali = buat_pengali(3);
    println!("5 × 3 = {}", pengali(5));
    println!("10 × 3 = {}", pengali(10));

    let penambah = buat_penambah(100);
    println!("5 + 100 = {}", penambah(5));

    // ── CONTOH PRAKTIS: SORTING DENGAN CLOSURE ──────────────
    let mut siswa = vec![
        ("Budi", 85),
        ("Ani", 92),
        ("Cici", 78),
        ("Dedi", 95),
    ];

    // Sort by skor (ascending)
    siswa.sort_by(|a, b| a.1.cmp(&b.1));
    println!("Sort by skor (asc): {:?}", siswa);

    // Sort by skor (descending)
    siswa.sort_by(|a, b| b.1.cmp(&a.1));
    println!("Sort by skor (desc): {:?}", siswa);

    // Sort by nama (alphabetical)
    siswa.sort_by(|a, b| a.0.cmp(&b.0));
    println!("Sort by nama: {:?}", siswa);

    // ── CONTOH: STRATEGY PATTERN DENGAN CLOSURE ─────────────
    let diskon_10 = |harga: f64| harga * 0.9;
    let diskon_beli3 = |harga: f64| if harga > 100_000.0 { harga * 0.8 } else { harga };
    let tanpa_diskon = |harga: f64| harga;

    println!("Harga 50000 (10%): {}", terapkan_diskon(50_000.0, diskon_10));
    println!("Harga 150000 (beli3): {}", terapkan_diskon(150_000.0, diskon_beli3));
    println!("Harga 80000 (normal): {}", terapkan_diskon(80_000.0, tanpa_diskon));

    // ── MOVE CLOSURE UNTUK THREAD (PREVIEW) ─────────────────
    // `move` closure penting saat data dikirim ke thread lain
    let pesan_thread = String::from("Halo dari main!");
    let handle = std::thread::spawn(move || {
        // `move` diperlukan karena thread mungkin hidup lebih lama dari scope
        println!("Thread berkata: {}", pesan_thread);
    });
    handle.join().unwrap();
}

// ── FUNGSI YANG MENERIMA CLOSURE ────────────────────────────
// `Fn` trait: closure yang hanya membaca environment
fn terapkan_dan_cetak<F: Fn(i32) -> i32>(angka: i32, f: F) {
    let hasil = f(angka);
    println!("f({}) = {}", angka, hasil);
}

fn terapkan_diskon<F: Fn(f64) -> f64>(harga: f64, strategi: F) -> f64 {
    strategi(harga)
}

// ── FUNGSI YANG RETURN CLOSURE ──────────────────────────────
// Harus pakai `impl Fn` atau `Box<dyn Fn>` karena closure punya tipe unik
fn buat_pengali(faktor: i32) -> impl Fn(i32) -> i32 {
    move |x| x * faktor // `move` agar `faktor` di-capture by value
}

fn buat_penambah(n: i32) -> impl Fn(i32) -> i32 {
    move |x| x + n
}

// ============================================================
// 🧠 RINGKUMAN CLOSURE TRAITS:
//
// ┌─────────────────────────────────────────────────────────────┐
// │                    TIGA TRAIT CLOSURE                       │
// ├──────────────────┬──────────────────────────────────────────┤
// │ Fn               │ Immutable borrow dari captured vars      │
// │                  │ Bisa dipanggil berkali-kali              │
// ├──────────────────┼──────────────────────────────────────────┤
// │ FnMut            │ Mutable borrow dari captured vars        │
// │                  │ Bisa dipanggil berkali-kali              │
// ├──────────────────┼──────────────────────────────────────────┤
// │ FnOnce           │ Take ownership dari captured vars        │
// │                  │ Hanya bisa dipanggil SEKALI              │
// └──────────────────┴──────────────────────────────────────────┘
//
// 💡 Hierarki: Fn ⊂ FnMut ⊂ FnOnce
//    - Setiap closure implement FnOnce
//    - Kalau hanya baca, juga implement FnMut dan Fn
//    - Kalau mutasi, implement FnMut dan FnOnce (tapi bukan Fn)
//    - Kalau move, implement FnOnce saja (mungkin)
//
// ┌─────────────────────────────────────────────────────────────┐
// │                    CAPTURE MODES                            │
// ├──────────────────┬──────────────────────────────────────────┤
// │ (default)        │ Compiler pilih: &T, &mut T, atau T       │
// │ move             │ Paksa capture by value (ownership)       │
// └──────────────────┴──────────────────────────────────────────┘
//
// ⚠️ COMMON MISTAKES:
// - Closure borrow mutable sementara variabel dipakai → borrow error
// - Return closure tanpa `move` → lifetime error
// - Panggil FnOnce lebih dari sekali → compile error
// - Lupa `mut` pada variabel closure yang FnMut
//
// 🔗 PERBANDINGAN:
// | Rust              | Python           | JavaScript        |
// |-------------------|------------------|-------------------|
// | \|x\| x + 1       | lambda x: x+1    | (x) => x + 1      |
// | move \|x\| ...    | (capture by ref) | (closure capture) |
// | .iter().map()     | map()            | .map()            |
// | .filter()         | filter()         | .filter()         |
// | .fold()           | reduce()         | .reduce()         |
// ============================================================

// ============================================================
// 🏋️ LATIHAN:
// 1. Buat fungsi `apply_twice` yang menerima closure dan angka,
//    terapkan closure dua kali: f(f(x))
// 2. Buat fungsi `compose` yang menerima dua closure dan return
//    closure baru: compose(f, g) = |x| f(g(x))
// 3. Buat "calculator" yang menerima Vec<Box<dyn Fn(f64) -> f64>>
//    dan terapkan secara berurutan
// 4. Implementasi memoization closure untuk fibonacci
// 5. Buat event handler sederhana yang menyimpan Vec closures
// 6. Gunakan iterator chain untuk: filter > map > collect
// ============================================================
