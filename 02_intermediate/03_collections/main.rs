// ============================================================
// 📘 BELAJAR RUST #11 — Collections (Vec, HashMap, HashSet)
// ============================================================
// Collections menyimpan data di HEAP — ukurannya dinamis.
// Yang paling sering dipakai:
// - Vec<T>     → array dinamis (seperti ArrayList)
// - HashMap    → key-value store (seperti dictionary/map)
// - HashSet    → kumpulan nilai unik
// ============================================================

use std::collections::HashMap;
use std::collections::HashSet;

fn main() {
    // ════════════════════════════════════════════════════════
    // VEC<T> — Dynamic Array
    // ════════════════════════════════════════════════════════

    // ── Membuat Vec ─────────────────────────────────────────
    let mut angka: Vec<i32> = Vec::new(); // Vec kosong dengan type annotation
    let buah = vec!["Apel", "Jeruk", "Mangga"]; // macro vec! dengan isi awal
    let nol = vec![0; 5]; // [0, 0, 0, 0, 0]

    println!("buah: {:?}", buah);
    println!("nol: {:?}", nol);

    // ── Push & Pop ──────────────────────────────────────────
    angka.push(10);
    angka.push(20);
    angka.push(30);
    println!("Setelah push: {:?}", angka);

    let terakhir = angka.pop(); // return Option<i32>
    println!("Pop: {:?}, Sisa: {:?}", terakhir, angka);

    // ── Akses Elemen ────────────────────────────────────────
    // Cara 1: indexing langsung — PANIC jika out of bounds!
    println!("Elemen ke-0: {}", angka[0]);

    // Cara 2: .get() — return Option, AMAN!
    match angka.get(10) {
        Some(val) => println!("Elemen ke-10: {}", val),
        None => println!("Elemen ke-10 tidak ada!"),
    }

    // ── Iterasi ─────────────────────────────────────────────
    let skor = vec![85, 92, 78, 95, 88];

    // Immutable iteration
    for s in &skor {
        print!("{} ", s);
    }
    println!();

    // Mutable iteration — ubah elemen
    let mut nilai = vec![1, 2, 3, 4, 5];
    for n in &mut nilai {
        *n *= 10; // dereference dan kalikan 10
    }
    println!("Setelah x10: {:?}", nilai);

    // Iteration dengan enumerate
    for (i, val) in skor.iter().enumerate() {
        println!("Index {}: {}", i, val);
    }

    // ── Vec Methods ─────────────────────────────────────────
    let mut data = vec![3, 1, 4, 1, 5, 9, 2, 6];

    println!("Panjang: {}", data.len());
    println!("Kosong? {}", data.is_empty());
    println!("Contains 5? {}", data.contains(&5));

    data.sort(); // sort in-place
    println!("Sorted: {:?}", data);

    data.dedup(); // hapus duplikat BERTURUT-TURUT (harus sorted dulu!)
    println!("Dedup: {:?}", data);

    data.reverse();
    println!("Reverse: {:?}", data);

    // Insert & Remove
    data.insert(0, 100); // insert 100 di index 0
    println!("Insert: {:?}", data);

    data.remove(0); // hapus elemen di index 0
    println!("Remove: {:?}", data);

    // Retain — keep elemen yang memenuhi kondisi
    let mut genap = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    genap.retain(|&x| x % 2 == 0); // hanya simpan yang genap
    println!("Genap saja: {:?}", genap);

    // Extend — gabungkan dengan iterator lain
    let mut gabung = vec![1, 2, 3];
    gabung.extend(vec![4, 5, 6]);
    println!("Gabung: {:?}", gabung);

    // ── Vec dengan Enum (untuk menyimpan tipe berbeda) ──────
    #[derive(Debug)]
    enum Sel {
        Int(i32),
        Float(f64),
        Teks(String),
    }

    let baris: Vec<Sel> = vec![
        Sel::Int(1),
        Sel::Teks(String::from("Halo")),
        Sel::Float(3.14),
    ];
    println!("Baris multi-tipe: {:?}", baris);

    // ════════════════════════════════════════════════════════
    // HASHMAP<K, V> — Key-Value Store
    // ════════════════════════════════════════════════════════

    // ── Membuat HashMap ─────────────────────────────────────
    let mut skor_siswa: HashMap<String, i32> = HashMap::new();

    // ── Insert ──────────────────────────────────────────────
    skor_siswa.insert(String::from("Budi"), 85);
    skor_siswa.insert(String::from("Ani"), 92);
    skor_siswa.insert(String::from("Cici"), 78);
    println!("Skor: {:?}", skor_siswa);

    // ── Akses Nilai ─────────────────────────────────────────
    // .get() return Option<&V>
    let nama_cari = String::from("Ani");
    match skor_siswa.get(&nama_cari) {
        Some(skor) => println!("Skor {}: {}", nama_cari, skor),
        None => println!("{} tidak ditemukan", nama_cari),
    }

    // ── Update ──────────────────────────────────────────────
    // Insert dengan key yang sama = overwrite
    skor_siswa.insert(String::from("Budi"), 90);
    println!("Budi updated: {:?}", skor_siswa.get("Budi"));

    // entry() — insert HANYA jika key belum ada
    skor_siswa.entry(String::from("Dedi")).or_insert(75);
    skor_siswa.entry(String::from("Budi")).or_insert(0); // tidak overwrite!
    println!("Setelah entry: {:?}", skor_siswa);

    // entry() dengan modifikasi
    let teks = "halo dunia halo rust halo semua";
    let mut frekuensi: HashMap<&str, i32> = HashMap::new();
    for kata in teks.split_whitespace() {
        let count = frekuensi.entry(kata).or_insert(0);
        *count += 1; // dereference dan tambah 1
    }
    println!("Frekuensi kata: {:?}", frekuensi);

    // ── Iterasi HashMap ─────────────────────────────────────
    for (nama, skor) in &skor_siswa {
        println!("{}: {}", nama, skor);
    }

    // ── HashMap Methods ─────────────────────────────────────
    println!("Jumlah siswa: {}", skor_siswa.len());
    println!("Contains Ani? {}", skor_siswa.contains_key("Ani"));

    // Remove
    skor_siswa.remove("Cici");
    println!("Setelah remove Cici: {:?}", skor_siswa);

    // Keys dan Values
    let semua_nama: Vec<&String> = skor_siswa.keys().collect();
    let semua_skor: Vec<&i32> = skor_siswa.values().collect();
    println!("Nama: {:?}", semua_nama);
    println!("Skor: {:?}", semua_skor);

    // ── Membuat HashMap dari Vec tuple ──────────────────────
    let data_vec = vec![
        (String::from("X"), 10),
        (String::from("Y"), 20),
        (String::from("Z"), 30),
    ];
    let map_dari_vec: HashMap<String, i32> = data_vec.into_iter().collect();
    println!("Dari Vec: {:?}", map_dari_vec);

    // ════════════════════════════════════════════════════════
    // HASHSET<T> — Kumpulan Nilai Unik
    // ════════════════════════════════════════════════════════

    // ── Membuat HashSet ─────────────────────────────────────
    let mut bahasa: HashSet<String> = HashSet::new();

    // ── Insert ──────────────────────────────────────────────
    bahasa.insert(String::from("Rust"));
    bahasa.insert(String::from("Go"));
    bahasa.insert(String::from("Python"));
    bahasa.insert(String::from("Rust")); // duplikat diabaikan!
    println!("Bahasa: {:?}", bahasa);
    println!("Jumlah: {}", bahasa.len()); // 3, bukan 4!

    // ── Set Operations ──────────────────────────────────────
    let set_a: HashSet<i32> = vec![1, 2, 3, 4, 5].into_iter().collect();
    let set_b: HashSet<i32> = vec![3, 4, 5, 6, 7].into_iter().collect();

    // Union: semua elemen dari kedua set
    let union: HashSet<&i32> = set_a.union(&set_b).collect();
    println!("Union: {:?}", union);

    // Intersection: elemen yang ada di kedua set
    let intersect: HashSet<&i32> = set_a.intersection(&set_b).collect();
    println!("Intersection: {:?}", intersect);

    // Difference: elemen di A tapi tidak di B
    let diff: HashSet<&i32> = set_a.difference(&set_b).collect();
    println!("Difference (A-B): {:?}", diff);

    // Symmetric difference: elemen yang hanya ada di salah satu
    let sym_diff: HashSet<&i32> = set_a.symmetric_difference(&set_b).collect();
    println!("Symmetric diff: {:?}", sym_diff);

    // Subset & Superset
    let set_c: HashSet<i32> = vec![1, 2, 3].into_iter().collect();
    println!("C subset of A? {}", set_c.is_subset(&set_a));
    println!("A superset of C? {}", set_a.is_superset(&set_c));

    // ── Contains & Remove ───────────────────────────────────
    println!("Contains Rust? {}", bahasa.contains("Rust"));
    bahasa.remove("Go");
    println!("Setelah remove Go: {:?}", bahasa);
}

// ============================================================
// 🏋️ LATIHAN:
// 1. Buat program yang menghitung frekuensi huruf dalam string
// 2. Buat "phonebook" dengan HashMap — bisa add, search, delete
// 3. Buat fungsi yang menerima Vec<i32> dan return Vec<i32>
//    tanpa duplikat (gunakan HashSet)
// 4. Buat program inventory sederhana: item name → quantity
// 5. Implementasikan "two sum" problem menggunakan HashMap
// ============================================================
