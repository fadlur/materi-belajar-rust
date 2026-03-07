// ============================================================
// 📗 BELAJAR RUST #04 — Control Flow
// ============================================================
// Rust punya if/else, loop, while, dan for.
// Yang unik: `if` adalah EXPRESSION (menghasilkan nilai),
// dan `loop` bisa return nilai juga!
// ============================================================

fn main() {
    // ── IF / ELSE ───────────────────────────────────────────
    // Kondisi TIDAK perlu kurung `()` — berbeda dari C/Java
    let angka = 7;

    if angka > 0 {
        println!("{} positif", angka);
    } else if angka < 0 {
        println!("{} negatif", angka);
    } else {
        println!("{} adalah nol", angka);
    }

    // ── IF SEBAGAI EXPRESSION ───────────────────────────────
    // `if` bisa digunakan di sisi kanan `let` — hasilnya jadi nilai
    let kondisi = true;
    let nilai = if kondisi { 5 } else { 10 };
    println!("Nilai: {}", nilai); // 5

    // Kedua cabang HARUS return tipe yang sama!
    // let salah = if kondisi { 5 } else { "sepuluh" }; // ❌ ERROR!

    // ── MATCH ───────────────────────────────────────────────
    // `match` seperti switch-case tapi JAUH lebih powerful
    // HARUS exhaustive — semua kemungkinan harus ditangani!
    let hari = 3;
    let nama_hari = match hari {
        1 => "Senin",
        2 => "Selasa",
        3 => "Rabu",
        4 => "Kamis",
        5 => "Jumat",
        6 => "Sabtu",
        7 => "Minggu",
        _ => "Hari tidak valid", // `_` menangkap semua sisa (wildcard)
    };
    println!("Hari ke-{}: {}", hari, nama_hari);

    // Match dengan range
    let skor = 85;
    let grade = match skor {
        90..=100 => "A",
        80..=89 => "B",
        70..=79 => "C",
        60..=69 => "D",
        _ => "E",
    };
    println!("Skor {} = Grade {}", skor, grade);

    // Match dengan multiple patterns
    let karakter = 'a';
    let jenis = match karakter {
        'a' | 'i' | 'u' | 'e' | 'o' => "vokal",
        'A' | 'I' | 'U' | 'E' | 'O' => "vokal kapital",
        _ => "konsonan atau lainnya",
    };
    println!("'{}' adalah {}", karakter, jenis);

    // ── LOOP (INFINITE LOOP) ────────────────────────────────
    // `loop` membuat loop tanpa batas — harus di-break secara manual
    let mut counter = 0;
    loop {
        counter += 1;
        if counter == 5 {
            break; // keluar dari loop
        }
    }
    println!("Counter setelah loop: {}", counter);

    // ── LOOP DENGAN RETURN VALUE ────────────────────────────
    // Ini fitur unik Rust! `loop` bisa return nilai lewat `break`
    let mut angka_loop = 0;
    let hasil = loop {
        angka_loop += 1;
        if angka_loop == 10 {
            break angka_loop * 2; // return 20 dari loop
        }
    };
    println!("Hasil dari loop: {}", hasil);

    // ── LOOP LABELS ─────────────────────────────────────────
    // Saat ada nested loop, label membantu break/continue loop tertentu
    let mut count = 0;
    'luar: loop {
        let mut sisa = 10;
        loop {
            if sisa == 8 {
                break; // break loop dalam saja
            }
            if count == 2 {
                break 'luar; // break loop luar!
            }
            sisa -= 1;
        }
        count += 1;
    }
    println!("Count setelah nested loop: {}", count);

    // ── WHILE LOOP ──────────────────────────────────────────
    let mut n = 5;
    while n > 0 {
        println!("Countdown: {}", n);
        n -= 1;
    }
    println!("Liftoff! 🚀");

    // ── FOR LOOP ────────────────────────────────────────────
    // `for` di Rust selalu iterasi atas sebuah iterator
    // Ini cara PALING AMAN dan IDIOMATIK untuk looping

    // Range: 1..5 artinya 1, 2, 3, 4 (5 TIDAK termasuk)
    for i in 1..5 {
        print!("{} ", i);
    }
    println!(); // newline

    // Range inklusif: 1..=5 artinya 1, 2, 3, 4, 5
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
    for (index, b) in buah.iter().enumerate() {
        println!("Buah ke-{}: {}", index, b);
    }

    // Reverse range
    for i in (1..=5).rev() {
        print!("{} ", i);
    }
    println!("(reverse)");

    // ── CONTINUE ────────────────────────────────────────────
    // `continue` melompat ke iterasi berikutnya
    for i in 1..=10 {
        if i % 3 == 0 {
            continue; // skip kelipatan 3
        }
        print!("{} ", i);
    }
    println!("(skip kelipatan 3)");

    // ── WHILE LET ───────────────────────────────────────────
    // Pattern matching di dalam while — sangat berguna!
    let mut stack = vec![1, 2, 3, 4, 5];
    while let Some(top) = stack.pop() {
        print!("{} ", top);
    }
    println!("(pop dari stack)");

    // ── IF LET ──────────────────────────────────────────────
    // Shorthand untuk match saat hanya peduli satu pattern
    let angka_mungkin: Option<i32> = Some(42);
    if let Some(val) = angka_mungkin {
        println!("Dapat angka: {}", val);
    } else {
        println!("Tidak ada angka");
    }

    // ── CONTOH GABUNGAN: FizzBuzz ───────────────────────────
    println!("\n--- FizzBuzz ---");
    for i in 1..=30 {
        let output = match (i % 3, i % 5) {
            (0, 0) => String::from("FizzBuzz"),
            (0, _) => String::from("Fizz"),
            (_, 0) => String::from("Buzz"),
            _ => i.to_string(),
        };
        print!("{} ", output);
    }
    println!();
}

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
// ============================================================
