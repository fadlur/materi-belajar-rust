// ============================================================
// 📗 BELAJAR RUST #04 — Control Flow
// ============================================================
// Rust punya if/else, loop, while, dan for.
// Yang unik: `if` adalah EXPRESSION (menghasilkan nilai),
// dan `loop` bisa return nilai juga!
//
// 🎯 Tujuan: Memahami semua cara mengontrol alur eksekusi program,
//    termasuk fitur unik Rust seperti if expression dan labeled loop.
//
// 💡 Analogi: Control flow seperti rambu lalu lintas —
//    menentukan ke mana program harus pergi berdasarkan kondisi.
// ============================================================

fn main() {
    // ── IF / ELSE ───────────────────────────────────────────
    // Kondisi TIDAK perlu kurung `()` — berbeda dari C/Java!
    //
    // 💡 Kenapa tanpa kurung? Rust mendesain syntax yang lebih bersih.
    //    Kurung dianggap noise/berlebihan karena kondisi sudah jelas.
    let angka = 7;

    if angka > 0 {
        println!("{} positif", angka);
    } else if angka < 0 {
        println!("{} negatif", angka);
    } else {
        println!("{} adalah nol", angka);
    }

    // ⚠️ Kondisi HARUS bertipe bool — tidak ada implicit conversion!
    // if angka { ... } // ❌ ERROR! expected bool, found integer
    // Di C: if (5) { ... } dianggap true — di Rust TIDAK!

    // ── IF SEBAGAI EXPRESSION ───────────────────────────────
    // `if` bisa digunakan di sisi kanan `let` — hasilnya jadi nilai!
    //
    // 💡 Analogi: if expression seperti vending machine —
    //    masukkan kondisi, keluar hasil sesuai pilihan.
    let kondisi = true;
    let nilai = if kondisi { 5 } else { 10 };
    println!("Nilai: {}", nilai); // 5

    // Kedua cabang HARUS return tipe yang sama!
    // let salah = if kondisi { 5 } else { "sepuluh" }; // ❌ ERROR!
    // Compiler Rust mengecek tipe di kedua cabang —
    // kalau berbeda, program tidak bisa compile!

    // ── MATCH ───────────────────────────────────────────────
    // `match` seperti switch-case tapi JAUH lebih powerful.
    // HARUS exhaustive — semua kemungkinan harus ditangani!
    //
    // 💡 Analogi: match seperti mesin sortir —
    //    setiap item masuk ke slot yang sesuai. Kalau ada item
    //    yang tidak punya slot, mesin berhenti (compile error).
    let hari = 3;
    let nama_hari = match hari {
        1 => "Senin",
        2 => "Selasa",
        3 => "Rabu",
        4 => "Kamis",
        5 => "Jumat",
        6 => "Sabtu",
        7 => "Minggu",
        _ => "Hari tidak valid", // `_` = wildcard, menangkap semua sisa
    };
    println!("Hari ke-{}: {}", hari, nama_hari);

    // Match dengan range
    let skor = 85;
    let grade = match skor {
        90..=100 => "A",   // ..= artinya inclusive range (termasuk 90 dan 100)
        80..=89 => "B",
        70..=79 => "C",
        60..=69 => "D",
        _ => "E",
    };
    println!("Skor {} = Grade {}", skor, grade);

    // Match dengan multiple patterns (| = OR)
    let karakter = 'a';
    let jenis = match karakter {
        'a' | 'i' | 'u' | 'e' | 'o' => "vokal",
        'A' | 'I' | 'U' | 'E' | 'O' => "vokal kapital",
        _ => "konsonan atau lainnya",
    };
    println!("'{}' adalah {}", karakter, jenis);

    // ── LOOP (INFINITE LOOP) ────────────────────────────────
    // `loop` membuat loop tanpa batas — harus di-break secara manual
    //
    // 💡 Kapan pakai loop? Saat kondisi berhenti kompleks dan
    //    tidak bisa dijelaskan dengan while/for sederhana.
    let mut counter = 0;
    loop {
        counter += 1;
        if counter == 5 {
            break; // keluar dari loop
        }
    }
    println!("Counter setelah loop: {}", counter);

    // ── LOOP DENGAN RETURN VALUE ────────────────────────────
    // Ini fitur UNIK Rust! `loop` bisa return nilai lewat `break`
    //
    // 💡 Analogi: loop seperti mesin pencari — terus mencari
    //    sampai menemukan hasil, lalu bawa hasil itu keluar.
    let mut angka_loop = 0;
    let hasil = loop {
        angka_loop += 1;
        if angka_loop == 10 {
            break angka_loop * 2; // return 20 dari loop
        }
    };
    println!("Hasil dari loop: {}", hasil);

    // ── LOOP LABELS ─────────────────────────────────────────
    // Saat ada nested loop (loop di dalam loop), label membantu
    // break/continue loop tertentu.
    //
    // 💡 Analogi: Label seperti memberi nama pada pintu —
    //    kalau mau keluar dari ruangan tertentu, sebutkan namanya.
    let mut count = 0;
    'luar: loop {              // ← label loop luar
        let mut sisa = 10;
        loop {
            if sisa == 8 {
                break;         // break loop DALAM saja
            }
            if count == 2 {
                break 'luar;   // break loop LUAR!
            }
            sisa -= 1;
        }
        count += 1;
    }
    println!("Count setelah nested loop: {}", count);

    // ── WHILE LOOP ──────────────────────────────────────────
    // `while` = loop selama kondisi bernilai true
    // Cocok untuk loop yang jumlah iterasinya tidak diketahui.
    let mut n = 5;
    while n > 0 {
        println!("Countdown: {}", n);
        n -= 1;
    }
    println!("Liftoff! 🚀");

    // ── FOR LOOP ────────────────────────────────────────────
    // `for` di Rust selalu iterasi atas sebuah iterator.
    // Ini cara PALING AMAN dan IDIOMATIK untuk looping di Rust!
    //
    // 💡 Analogi: Iterator seperti antrian di bank —
    //    setiap orang (elemen) dilayani satu per satu sampai habis.
    //    Tidak perlu khawatir index out of bounds!

    // Range: 1..5 artinya 1, 2, 3, 4 (5 TIDAK termasuk)
    //        ↑     ↑
    //     mulai  berakhir (eksklusif)
    for i in 1..5 {
        print!("{} ", i);
    }
    println!(); // newline

    // Range inklusif: 1..=5 artinya 1, 2, 3, 4, 5 (5 termasuk!)
    for i in 1..=5 {
        print!("{} ", i);
    }
    println!();

    // Iterasi atas array
    let buah = ["Apel", "Jeruk", "Mangga", "Durian"];
    for b in buah.iter() {
        println!("Saya suka {}", b);
    }

    // Iterasi dengan index menggunakan enumerate
    // enumerate menghasilkan pasangan (index, value)
    for (index, b) in buah.iter().enumerate() {
        println!("Buah ke-{}: {}", index, b);
    }

    // Reverse range
    for i in (1..=5).rev() {
        print!("{} ", i);
    }
    println!("(reverse)");

    // ── CONTINUE ────────────────────────────────────────────
    // `continue` melompat ke iterasi berikutnya (skip sisa kode di loop)
    for i in 1..=10 {
        if i % 3 == 0 {
            continue; // skip kelipatan 3
        }
        print!("{} ", i);
    }
    println!("(skip kelipatan 3)");

    // ── WHILE LET ───────────────────────────────────────────
    // Pattern matching di dalam while — sangat berguna untuk
    // mengkonsumsi iterator yang mungkin kosong.
    let mut stack = vec![1, 2, 3, 4, 5];
    while let Some(top) = stack.pop() {
        print!("{} ", top);
    }
    println!("(pop dari stack)");

    // ── IF LET ──────────────────────────────────────────────
    // Shorthand untuk match saat hanya peduli satu pattern.
    // Lebih ringkas daripada match dengan banyak cabang yang tidak dipakai.
    let angka_mungkin: Option<i32> = Some(42);
    if let Some(val) = angka_mungkin {
        println!("Dapat angka: {}", val);
    } else {
        println!("Tidak ada angka");
    }
    // Lebih singkat dari:
    // match angka_mungkin {
    //     Some(val) => println!("Dapat angka: {}", val),
    //     _ => println!("Tidak ada angka"),
    // }

    // ── CONTOH GABUNGAN: FizzBuzz ───────────────────────────
    // Klasik programming interview — menggunakan match dengan tuple!
    println!("\n--- FizzBuzz ---");
    for i in 1..=30 {
        // Match pada tuple (i % 3, i % 5)
        let output = match (i % 3, i % 5) {
            (0, 0) => String::from("FizzBuzz"), // kelipatan 3 DAN 5
            (0, _) => String::from("Fizz"),     // kelipatan 3 saja
            (_, 0) => String::from("Buzz"),     // kelipatan 5 saja
            _ => i.to_string(),                  // bukan kelipatan keduanya
        };
        print!("{} ", output);
    }
    println!();
}

// ============================================================
// 🧠 RINGKUMAN CONTROL FLOW:
//
// ┌──────────────┬────────────────────────────────────────────┐
// │ Konstruksi   │ Gunakan saat...                            │
// ├──────────────┼────────────────────────────────────────────┤
// │ if/else      │ Memilih 2-3 cabang berdasarkan kondisi     │
// │ match        │ Memilih banyak cabang (seperti switch)     │
// │ loop         │ Loop tanpa batas, perlu break manual       │
// │ loop + break │ Loop yang menghasilkan nilai               │
// │ while        │ Loop selama kondisi true                   │
// │ for          │ Iterasi koleksi/range (PALING IDIOMATIK)   │
// │ if let       │ Match tapi hanya peduli 1 pattern          │
// │ while let    │ Loop sambil pattern masih cocok            │
// └──────────────┴────────────────────────────────────────────┘
//
// ⚠️ COMMON MISTAKES:
// - Kondisi if tanpa bool → compile error!
// - Lupa break di loop → infinite loop
// - Match tidak exhaustive → compile error (tapi ini BAGUS!)
// - Range 1..5 vs 1..=5 — hati-hati inclusive/exclusive
// - Modifikasi iterator saat iterasi → borrow error
//
// 🔗 PERBANDINGAN:
// | Rust              | Python           | JavaScript         |
// |-------------------|------------------|--------------------|
// | if x > 0 { }      | if x > 0:        | if (x > 0) { }     |
// | match x { }       | match x:         | switch (x) { }     |
// | loop { }          | while True:      | while (true) { }   |
// | while x > 0 { }   | while x > 0:     | while (x > 0) { }  |
// | for i in 0..5 { } | for i in range(5)| for (let i=0;...)  |
// ============================================================

// ============================================================
// 🏋️ LATIHAN:
// 1. Buat program yang mencetak bilangan prima dari 1 sampai 50
// 2. Buat program tebak angka menggunakan loop dan break
// 3. Buat program yang menghitung faktorial menggunakan for loop
// 4. Buat match yang mengkategorikan BMI (underweight/normal/overweight)
// 5. Buat nested loop yang mencetak pola segitiga bintang:
//    *
//    **
//    ***
//    ****
// 6. Gunakan if let untuk meng-handle Option<&str>
// 7. Buat loop yang menghasilkan jumlah digit dari sebuah angka
// ============================================================
