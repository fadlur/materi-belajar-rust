// ============================================================
// 📙 BELAJAR RUST #18 — Iterators
// ============================================================
// Iterator adalah pola untuk memproses sequence of items secara lazy.
// "Lazy" artinya tidak dihitung sampai benar-benar dibutuhkan.
// Iterator di Rust sangat powerful dan ZERO-COST (secepat manual loop).
// ============================================================

fn main() {
    // ── DASAR ITERATOR ──────────────────────────────────────
    // Trait Iterator hanya butuh satu method: `next()`
    //
    // trait Iterator {
    //     type Item;
    //     fn next(&mut self) -> Option<Self::Item>;
    // }

    let angka = vec![1, 2, 3, 4, 5];

    // .iter() → iterator atas &T (immutable reference)
    let mut iter = angka.iter();
    println!("next: {:?}", iter.next()); // Some(&1)
    println!("next: {:?}", iter.next()); // Some(&2)
    println!("next: {:?}", iter.next()); // Some(&3)
    println!("next: {:?}", iter.next()); // Some(&4)
    println!("next: {:?}", iter.next()); // Some(&5)
    println!("next: {:?}", iter.next()); // None — habis!

    // .into_iter() → iterator atas T (ownership, consume collection)
    // .iter_mut()  → iterator atas &mut T (mutable reference)

    // ── FOR LOOP = SYNTACTIC SUGAR UNTUK ITERATOR ───────────
    // `for item in collection` secara otomatis memanggil .into_iter()
    for n in &angka {
        print!("{} ", n);
    }
    println!();

    // ══════════════════════════════════════════════════════════
    // ITERATOR ADAPTORS — Transformasi iterator (lazy!)
    // ══════════════════════════════════════════════════════════

    let angka2 = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    // ── MAP — transformasi setiap elemen ────────────────────
    let kuadrat: Vec<i32> = angka2.iter().map(|&x| x * x).collect();
    println!("Kuadrat: {:?}", kuadrat);

    // ── FILTER — ambil elemen yang memenuhi kondisi ─────────
    let genap: Vec<&i32> = angka2.iter().filter(|&&x| x % 2 == 0).collect();
    println!("Genap: {:?}", genap);

    // ── CHAINING — gabungkan beberapa adaptor ───────────────
    // Ini idiom yang SANGAT umum di Rust!
    let hasil: Vec<i32> = angka2
        .iter()
        .filter(|&&x| x % 2 != 0) // ambil ganjil
        .map(|&x| x * x)          // kuadratkan
        .filter(|&x| x > 10)      // ambil yang > 10
        .collect();                // kumpulkan ke Vec
    println!("Ganjil, kuadrat, >10: {:?}", hasil);

    // ── TAKE & SKIP ─────────────────────────────────────────
    let lima_pertama: Vec<&i32> = angka2.iter().take(5).collect();
    let skip_3: Vec<&i32> = angka2.iter().skip(3).collect();
    println!("5 pertama: {:?}", lima_pertama);
    println!("Skip 3: {:?}", skip_3);

    // take_while & skip_while
    let sampai_5: Vec<&i32> = angka2.iter().take_while(|&&x| x <= 5).collect();
    println!("Sampai 5: {:?}", sampai_5);

    // ── ENUMERATE — tambahkan index ─────────────────────────
    let buah = vec!["Apel", "Jeruk", "Mangga"];
    for (i, b) in buah.iter().enumerate() {
        println!("  {}. {}", i + 1, b);
    }

    // ── ZIP — gabungkan dua iterator ────────────────────────
    let nama = vec!["Budi", "Ani", "Cici"];
    let skor = vec![85, 92, 78];
    let rapor: Vec<(&str, &i32)> = nama.iter().copied().zip(skor.iter()).collect();
    println!("Rapor: {:?}", rapor);

    // ── FLAT_MAP — map + flatten ────────────────────────────
    let kalimat = vec!["halo dunia", "rust keren"];
    let kata: Vec<&str> = kalimat.iter().flat_map(|s| s.split(' ')).collect();
    println!("Kata: {:?}", kata);

    // ── CHAIN — sambungkan dua iterator ─────────────────────
    let a = vec![1, 2, 3];
    let b = vec![4, 5, 6];
    let gabung: Vec<&i32> = a.iter().chain(b.iter()).collect();
    println!("Chain: {:?}", gabung);

    // ── PEEKABLE — intip elemen berikutnya tanpa consume ────
    let mut iter2 = angka2.iter().peekable();
    println!("Peek: {:?}", iter2.peek()); // Some(&&1) — tidak consume
    println!("Next: {:?}", iter2.next()); // Some(&1) — consume

    // ══════════════════════════════════════════════════════════
    // CONSUMING ADAPTORS — Menghasilkan nilai akhir
    // ══════════════════════════════════════════════════════════

    let angka3 = vec![10, 20, 30, 40, 50];

    // ── SUM ─────────────────────────────────────────────────
    let total: i32 = angka3.iter().sum();
    println!("Sum: {}", total);

    // ── PRODUCT ─────────────────────────────────────────────
    let faktorial: i64 = (1..=10).product();
    println!("10! = {}", faktorial);

    // ── COUNT ───────────────────────────────────────────────
    let jumlah_genap = angka3.iter().filter(|&&x| x % 20 == 0).count();
    println!("Kelipatan 20: {}", jumlah_genap);

    // ── MIN, MAX ────────────────────────────────────────────
    println!("Min: {:?}", angka3.iter().min());
    println!("Max: {:?}", angka3.iter().max());

    // min_by, max_by — dengan comparator custom
    let kata_vec = vec!["Rust", "Go", "Python", "C"];
    let terpanjang = kata_vec.iter().max_by_key(|s| s.len());
    println!("Kata terpanjang: {:?}", terpanjang);

    // ── FOLD — akumulasi (paling fleksibel) ─────────────────
    // fold(initial_value, |accumulator, item| ...)
    let jumlah_fold = angka3.iter().fold(0, |acc, &x| acc + x);
    println!("Fold sum: {}", jumlah_fold);

    // Fold untuk membangun string
    let teks = angka3
        .iter()
        .map(|x| x.to_string())
        .fold(String::new(), |acc, s| {
            if acc.is_empty() { s } else { format!("{}, {}", acc, s) }
        });
    println!("Fold string: {}", teks);

    // ── REDUCE — seperti fold tapi tanpa initial value ──────
    let max_val = angka3.iter().copied().reduce(|a, b| if a > b { a } else { b });
    println!("Reduce max: {:?}", max_val);

    // ── ANY, ALL ────────────────────────────────────────────
    println!("Ada > 25? {}", angka3.iter().any(|&x| x > 25));
    println!("Semua > 5? {}", angka3.iter().all(|&x| x > 5));

    // ── FIND, POSITION ──────────────────────────────────────
    println!("Find > 25: {:?}", angka3.iter().find(|&&x| x > 25));
    println!("Position > 25: {:?}", angka3.iter().position(|&x| x > 25));

    // ── COLLECT KE BERBAGAI TIPE ────────────────────────────
    // Vec
    let vec_result: Vec<i32> = (1..=5).collect();
    println!("Vec: {:?}", vec_result);

    // String
    let huruf: String = vec!['H', 'a', 'l', 'o'].into_iter().collect();
    println!("String: {}", huruf);

    // HashMap
    use std::collections::HashMap;
    let map: HashMap<&str, i32> = vec![("a", 1), ("b", 2)].into_iter().collect();
    println!("HashMap: {:?}", map);

    // ── CUSTOM ITERATOR ─────────────────────────────────────
    let counter = Penghitung::new(5);
    let result: Vec<u32> = counter.collect();
    println!("Counter: {:?}", result);

    // Menggunakan iterator adaptors pada custom iterator
    let jumlah_kuadrat: u32 = Penghitung::new(5)
        .map(|x| x * x)
        .sum();
    println!("Jumlah kuadrat 1-5: {}", jumlah_kuadrat);

    // ── FIBONACCI ITERATOR ──────────────────────────────────
    let fib: Vec<u64> = Fibonacci::new().take(15).collect();
    println!("Fibonacci: {:?}", fib);

    // Fibonacci genap sampai 4 juta
    let sum_genap: u64 = Fibonacci::new()
        .take_while(|&x| x < 4_000_000)
        .filter(|x| x % 2 == 0)
        .sum();
    println!("Sum fibonacci genap < 4M: {}", sum_genap);

    // ── RANGE ITERATOR ──────────────────────────────────────
    // Range sudah implement Iterator!
    let sum_100: i32 = (1..=100).sum();
    println!("Sum 1-100: {}", sum_100);

    let squares: Vec<i32> = (1..=5).map(|x| x * x).collect();
    println!("Squares 1-5: {:?}", squares);
}

// ── CUSTOM ITERATOR ─────────────────────────────────────────
struct Penghitung {
    hitungan: u32,
    maks: u32,
}

impl Penghitung {
    fn new(maks: u32) -> Self {
        Penghitung { hitungan: 0, maks }
    }
}

// Implement trait Iterator
impl Iterator for Penghitung {
    type Item = u32; // tipe elemen yang dihasilkan

    fn next(&mut self) -> Option<Self::Item> {
        if self.hitungan < self.maks {
            self.hitungan += 1;
            Some(self.hitungan)
        } else {
            None // iterator habis
        }
    }
}

// ── FIBONACCI ITERATOR ──────────────────────────────────────
struct Fibonacci {
    a: u64,
    b: u64,
}

impl Fibonacci {
    fn new() -> Self {
        Fibonacci { a: 0, b: 1 }
    }
}

impl Iterator for Fibonacci {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let result = self.a;
        let next = self.a + self.b;
        self.a = self.b;
        self.b = next;
        Some(result) // infinite iterator!
    }
}

// ============================================================
// 🏋️ LATIHAN:
// 1. Gunakan iterator chain untuk: ambil angka 1-100, filter prima,
//    ambil 10 pertama, jumlahkan
// 2. Buat custom iterator `Range2D` yang iterasi atas (x, y) pairs
// 3. Implementasikan `moving_average` menggunakan windows()
// 4. Buat iterator yang menghasilkan kolasi Collatz sequence
// 5. Implementasikan `group_by` sederhana menggunakan fold()
// ============================================================
