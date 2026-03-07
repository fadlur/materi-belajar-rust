// ============================================================
// 📗 BELAJAR RUST #06 — References & Borrowing
// ============================================================
// Masalah: memindahkan ownership bolak-balik itu merepotkan.
// Solusi: REFERENCES — "meminjam" data tanpa mengambil ownership.
//
// ATURAN BORROWING:
// 1. Boleh punya BANYAK immutable reference (&T) ATAU
// 2. SATU mutable reference (&mut T) — TIDAK BOLEH keduanya bersamaan
// 3. Reference harus selalu valid (no dangling references)
// ============================================================

fn main() {
    // ── IMMUTABLE REFERENCE (&) ─────────────────────────────
    // `&` membuat reference — meminjam tanpa mengambil ownership
    let s1 = String::from("halo");
    let panjang = hitung_panjang(&s1); // &s1 = reference ke s1
    // s1 masih valid! Kita hanya "meminjamkan"-nya
    println!("Panjang '{}' adalah {}", s1, panjang);

    // Bisa punya BANYAK immutable reference sekaligus
    let r1 = &s1;
    let r2 = &s1;
    let r3 = &s1;
    println!("r1={}, r2={}, r3={}", r1, r2, r3); // ✅ Semua OK!

    // ── MUTABLE REFERENCE (&mut) ────────────────────────────
    // Untuk mengubah data yang dipinjam, butuh mutable reference
    let mut s2 = String::from("halo");
    ubah_string(&mut s2); // &mut s2 = mutable reference
    println!("Setelah diubah: {}", s2);

    // ⚠️ HANYA BOLEH SATU mutable reference pada satu waktu!
    let mut s3 = String::from("data");
    let r4 = &mut s3;
    // let r5 = &mut s3; // ❌ ERROR! cannot borrow `s3` as mutable more than once
    println!("r4 = {}", r4);
    // Setelah r4 terakhir dipakai, kita bisa buat mutable ref baru
    let r5 = &mut s3; // ✅ OK! r4 sudah tidak dipakai lagi
    println!("r5 = {}", r5);

    // ⚠️ TIDAK BOLEH campur immutable & mutable reference!
    let mut s4 = String::from("campur");
    let r6 = &s4; // immutable borrow
    let r7 = &s4; // immutable borrow kedua — OK
    println!("r6={}, r7={}", r6, r7);
    // Setelah r6 dan r7 terakhir dipakai...
    let r8 = &mut s4; // ✅ OK karena r6, r7 sudah "selesai"
    r8.push_str(" aduk");
    println!("r8 = {}", r8);

    // Ini disebut "Non-Lexical Lifetimes" (NLL) — Rust cukup pintar
    // untuk tahu kapan reference terakhir dipakai

    // ── REFERENCE DALAM FUNGSI ──────────────────────────────
    let mut kata = String::from("Rust");
    tambah_seru(&mut kata);
    println!("{}", kata); // "Rust!!!"

    let panjang2 = panjang_string(&kata);
    println!("Panjang: {}", panjang2);

    // ── REFERENCING & DEREFERENCING ─────────────────────────
    // `&` untuk membuat reference, `*` untuk dereference
    let x = 5;
    let y = &x; // y adalah reference ke x

    assert_eq!(5, x);
    assert_eq!(5, *y); // *y = dereference, ambil nilai yang ditunjuk
    // assert_eq!(5, y); // ❌ ERROR! tidak bisa bandingkan i32 dengan &i32

    println!("x = {}, *y = {}", x, *y);

    // Untuk tipe primitif, Rust sering auto-dereference
    // tapi penting untuk paham konsepnya

    // ── CONTOH PRAKTIS ──────────────────────────────────────
    let mut skor = vec![85, 92, 78, 95, 88];
    let rata_rata = hitung_rata_rata(&skor); // pinjam immutable
    println!("Rata-rata: {:.1}", rata_rata);

    tambah_skor(&mut skor, 100); // pinjam mutable
    println!("Skor setelah ditambah: {:?}", skor);

    let tertinggi = cari_tertinggi(&skor);
    println!("Skor tertinggi: {}", tertinggi);

    // ── DANGLING REFERENCE (DICEGAH RUST) ───────────────────
    // Rust TIDAK MENGIZINKAN dangling reference!
    // fn dangling() -> &String {
    //     let s = String::from("halo");
    //     &s  // ❌ ERROR! `s` akan di-drop, reference jadi invalid
    // }
    // Solusinya: return String langsung (transfer ownership)
    let valid = tidak_dangling();
    println!("Valid: {}", valid);
}

// ── FUNGSI DENGAN IMMUTABLE REFERENCE ───────────────────────
// Parameter `&String` berarti kita MEMINJAM String, bukan mengambil ownership
fn hitung_panjang(s: &String) -> usize {
    s.len()
    // `s` keluar scope, tapi karena tidak punya ownership,
    // data yang ditunjuk TIDAK di-drop
}

// Versi lebih idiomatik: gunakan &str (string slice) sebagai parameter
// Ini lebih fleksibel — bisa menerima &String maupun &str
fn panjang_string(s: &str) -> usize {
    s.len()
}

// ── FUNGSI DENGAN MUTABLE REFERENCE ────────────────────────
fn ubah_string(s: &mut String) {
    s.push_str(", dunia!"); // bisa mengubah data yang dipinjam
}

fn tambah_seru(s: &mut String) {
    s.push_str("!!!");
}

// ── CONTOH FUNGSI YANG IDIOMATIK ────────────────────────────
fn hitung_rata_rata(skor: &[i32]) -> f64 {
    // `&[i32]` adalah slice — lebih idiomatik dari &Vec<i32>
    let total: i32 = skor.iter().sum();
    total as f64 / skor.len() as f64
}

fn tambah_skor(skor: &mut Vec<i32>, nilai: i32) {
    skor.push(nilai);
}

fn cari_tertinggi(skor: &[i32]) -> i32 {
    *skor.iter().max().unwrap() // unwrap karena kita tahu tidak kosong
}

fn tidak_dangling() -> String {
    let s = String::from("tidak dangling");
    s // return ownership, bukan reference
}

// ============================================================
// 🏋️ LATIHAN:
// 1. Buat fungsi yang menerima &Vec<String> dan mencetak semua elemen
// 2. Buat fungsi yang menerima &mut Vec<i32> dan menghapus angka genap
// 3. Coba buat dua mutable reference bersamaan — lihat error-nya
// 4. Buat fungsi yang menerima &str dan return jumlah vokal
// 5. Mengapa `fn buat(s: &str) -> &str { &s[0..3] }` valid?
//    Kapan reference yang di-return valid/invalid?
// ============================================================
