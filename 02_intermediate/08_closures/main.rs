// ============================================================
// 📘 BELAJAR RUST #16 — Closures
// ============================================================
// Closure = anonymous function yang bisa "menangkap" (capture)
// variabel dari lingkungan sekitarnya.
// Mirip lambda di Python/Java, arrow function di JavaScript.
// ============================================================

fn main() {
    // ── SINTAKS CLOSURE ─────────────────────────────────────
    // Closure didefinisikan dengan `|parameter| body`

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
    // Tapi sekali ditebak, tipenya FIXED — tidak bisa berubah
    let contoh = |x| x; // tipe ditentukan saat pertama dipanggil
    let _s = contoh(String::from("halo")); // x adalah String
    // let n = contoh(5); // ❌ ERROR! x sudah ditetapkan String

    // ── CAPTURING ENVIRONMENT ───────────────────────────────
    // Closure bisa "menangkap" variabel dari scope luar!
    let pesan = String::from("Halo dari luar");

    // Capture by reference (immutable borrow) — default
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
// 📚 TIGA TRAIT CLOSURE:
//
// 1. FnOnce — closure yang mengambil ownership dari captured values
//    Hanya bisa dipanggil SEKALI. Semua closure implement ini.
//
// 2. FnMut — closure yang bisa mengubah captured values (mutable borrow)
//    Bisa dipanggil berulang kali.
//
// 3. Fn — closure yang hanya membaca captured values (immutable borrow)
//    Paling restrictive, tapi paling fleksibel untuk dipanggil.
//
// Hierarki: Fn ⊂ FnMut ⊂ FnOnce
// Jika fungsi menerima FnOnce, bisa terima semua closure.
// Jika fungsi menerima Fn, hanya bisa terima Fn closures.
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
// ============================================================
