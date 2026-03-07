// ============================================================
// 📕 BELAJAR RUST #27 — Mini Project: CLI Todo App
// ============================================================
// Project akhir: aplikasi Todo List berbasis command line.
// Menggabungkan SEMUA konsep yang sudah dipelajari:
// - Struct, Enum, Trait, Generics
// - Error Handling, Collections
// - File I/O, Serialization
// - Module organization
// - Pattern matching
// ============================================================

use std::fmt;
use std::fs;
use std::io::{self, Write};

// ══════════════════════════════════════════════════════════════
// DATA MODELS
// ══════════════════════════════════════════════════════════════

#[derive(Debug, Clone, PartialEq)]
enum Prioritas {
    Rendah,
    Sedang,
    Tinggi,
}

impl fmt::Display for Prioritas {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Prioritas::Rendah => write!(f, "🟢 Rendah"),
            Prioritas::Sedang => write!(f, "🟡 Sedang"),
            Prioritas::Tinggi => write!(f, "🔴 Tinggi"),
        }
    }
}

impl Prioritas {
    fn from_str(s: &str) -> Result<Self, String> {
        match s.to_lowercase().trim() {
            "rendah" | "r" | "1" => Ok(Prioritas::Rendah),
            "sedang" | "s" | "2" => Ok(Prioritas::Sedang),
            "tinggi" | "t" | "3" => Ok(Prioritas::Tinggi),
            _ => Err(format!("Prioritas tidak valid: '{}'. Gunakan: rendah/sedang/tinggi", s)),
        }
    }

    fn to_save_str(&self) -> &str {
        match self {
            Prioritas::Rendah => "rendah",
            Prioritas::Sedang => "sedang",
            Prioritas::Tinggi => "tinggi",
        }
    }
}

#[derive(Debug, Clone)]
struct Todo {
    id: u32,
    judul: String,
    selesai: bool,
    prioritas: Prioritas,
}

impl Todo {
    fn new(id: u32, judul: String, prioritas: Prioritas) -> Self {
        Todo {
            id,
            judul,
            selesai: false,
            prioritas,
        }
    }

    fn toggle(&mut self) {
        self.selesai = !self.selesai;
    }

    fn to_save_string(&self) -> String {
        format!(
            "{}|{}|{}|{}",
            self.id,
            self.judul,
            self.selesai,
            self.prioritas.to_save_str()
        )
    }

    fn from_save_string(s: &str) -> Result<Self, String> {
        let parts: Vec<&str> = s.split('|').collect();
        if parts.len() != 4 {
            return Err(format!("Format tidak valid: {}", s));
        }

        let id = parts[0]
            .parse::<u32>()
            .map_err(|e| format!("ID tidak valid: {}", e))?;
        let judul = parts[1].to_string();
        let selesai = parts[2] == "true";
        let prioritas = Prioritas::from_str(parts[3])?;

        Ok(Todo {
            id,
            judul,
            selesai,
            prioritas,
        })
    }
}

impl fmt::Display for Todo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let status = if self.selesai { "✅" } else { "⬜" };
        write!(
            f,
            "  {} [{}] {} ({})",
            status, self.id, self.judul, self.prioritas
        )
    }
}

// ══════════════════════════════════════════════════════════════
// TODO LIST MANAGER
// ══════════════════════════════════════════════════════════════

struct TodoList {
    todos: Vec<Todo>,
    next_id: u32,
    file_path: String,
}

impl TodoList {
    fn new(file_path: &str) -> Self {
        let mut list = TodoList {
            todos: Vec::new(),
            next_id: 1,
            file_path: file_path.to_string(),
        };
        list.load();
        list
    }

    fn tambah(&mut self, judul: &str, prioritas: Prioritas) -> &Todo {
        let todo = Todo::new(self.next_id, judul.to_string(), prioritas);
        self.next_id += 1;
        self.todos.push(todo);
        self.simpan();
        self.todos.last().unwrap()
    }

    fn toggle(&mut self, id: u32) -> Result<&Todo, String> {
        let todo = self
            .todos
            .iter_mut()
            .find(|t| t.id == id)
            .ok_or(format!("Todo dengan ID {} tidak ditemukan", id))?;
        todo.toggle();
        self.simpan();
        Ok(todo)
    }

    fn hapus(&mut self, id: u32) -> Result<Todo, String> {
        let pos = self
            .todos
            .iter()
            .position(|t| t.id == id)
            .ok_or(format!("Todo dengan ID {} tidak ditemukan", id))?;
        let removed = self.todos.remove(pos);
        self.simpan();
        Ok(removed)
    }

    fn edit(&mut self, id: u32, judul_baru: &str) -> Result<&Todo, String> {
        let todo = self
            .todos
            .iter_mut()
            .find(|t| t.id == id)
            .ok_or(format!("Todo dengan ID {} tidak ditemukan", id))?;
        todo.judul = judul_baru.to_string();
        self.simpan();
        Ok(todo)
    }

    fn daftar(&self, filter: Option<bool>) -> Vec<&Todo> {
        match filter {
            Some(selesai) => self.todos.iter().filter(|t| t.selesai == selesai).collect(),
            None => self.todos.iter().collect(),
        }
    }

    fn cari(&self, keyword: &str) -> Vec<&Todo> {
        let keyword_lower = keyword.to_lowercase();
        self.todos
            .iter()
            .filter(|t| t.judul.to_lowercase().contains(&keyword_lower))
            .collect()
    }

    fn statistik(&self) -> (usize, usize, usize) {
        let total = self.todos.len();
        let selesai = self.todos.iter().filter(|t| t.selesai).count();
        let belum = total - selesai;
        (total, selesai, belum)
    }

    fn bersihkan_selesai(&mut self) -> usize {
        let awal = self.todos.len();
        self.todos.retain(|t| !t.selesai);
        let dihapus = awal - self.todos.len();
        if dihapus > 0 {
            self.simpan();
        }
        dihapus
    }

    // ── File I/O ────────────────────────────────────────────
    fn simpan(&self) {
        let data: String = self
            .todos
            .iter()
            .map(|t| t.to_save_string())
            .collect::<Vec<String>>()
            .join("\n");

        if let Err(e) = fs::write(&self.file_path, data) {
            eprintln!("Gagal menyimpan: {}", e);
        }
    }

    fn load(&mut self) {
        match fs::read_to_string(&self.file_path) {
            Ok(data) => {
                self.todos.clear();
                self.next_id = 1;

                for line in data.lines() {
                    if line.trim().is_empty() {
                        continue;
                    }
                    match Todo::from_save_string(line) {
                        Ok(todo) => {
                            if todo.id >= self.next_id {
                                self.next_id = todo.id + 1;
                            }
                            self.todos.push(todo);
                        }
                        Err(e) => eprintln!("Warning: {}", e),
                    }
                }
            }
            Err(_) => {
                // File belum ada — tidak apa-apa
            }
        }
    }
}

// ══════════════════════════════════════════════════════════════
// CLI INTERFACE
// ══════════════════════════════════════════════════════════════

fn cetak_banner() {
    println!("╔══════════════════════════════════════╗");
    println!("║     🦀 RUST TODO APP 🦀              ║");
    println!("║     Mini Project Belajar Rust         ║");
    println!("╚══════════════════════════════════════╝");
}

fn cetak_help() {
    println!("\n📋 Perintah yang tersedia:");
    println!("  tambah <judul>         — Tambah todo baru");
    println!("  list                   — Tampilkan semua todo");
    println!("  list selesai           — Tampilkan yang selesai");
    println!("  list belum             — Tampilkan yang belum");
    println!("  toggle <id>            — Toggle status selesai");
    println!("  hapus <id>             — Hapus todo");
    println!("  edit <id> <judul baru> — Edit judul todo");
    println!("  cari <keyword>         — Cari todo");
    println!("  bersihkan              — Hapus semua yang selesai");
    println!("  stats                  — Tampilkan statistik");
    println!("  help                   — Tampilkan bantuan ini");
    println!("  quit                   — Keluar");
}

fn baca_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn cetak_daftar(todos: &[&Todo]) {
    if todos.is_empty() {
        println!("  (Tidak ada todo)");
    } else {
        for todo in todos {
            println!("{}", todo);
        }
    }
}

fn main() {
    cetak_banner();

    let file_path = "todos.txt";
    let mut list = TodoList::new(file_path);

    println!("\nData dimuat dari '{}'", file_path);
    let (total, _, _) = list.statistik();
    if total > 0 {
        println!("📦 {} todo ditemukan", total);
    }

    cetak_help();

    loop {
        let input = baca_input("\n🦀 > ");

        if input.is_empty() {
            continue;
        }

        // Parse command dan argumen
        let parts: Vec<&str> = input.splitn(2, ' ').collect();
        let command = parts[0].to_lowercase();
        let args = if parts.len() > 1 { parts[1] } else { "" };

        match command.as_str() {
            "tambah" | "add" | "t" => {
                if args.is_empty() {
                    println!("❌ Judul tidak boleh kosong!");
                    continue;
                }

                let prioritas_str = baca_input("  Prioritas (rendah/sedang/tinggi) [sedang]: ");
                let prioritas = if prioritas_str.is_empty() {
                    Prioritas::Sedang
                } else {
                    match Prioritas::from_str(&prioritas_str) {
                        Ok(p) => p,
                        Err(e) => {
                            println!("❌ {}", e);
                            continue;
                        }
                    }
                };

                let todo = list.tambah(args, prioritas);
                println!("✅ Ditambahkan: {}", todo);
            }

            "list" | "ls" | "l" => {
                let filter = match args {
                    "selesai" | "done" => Some(true),
                    "belum" | "pending" => Some(false),
                    _ => None,
                };
                let todos = list.daftar(filter);
                println!("\n📋 Daftar Todo:");
                cetak_daftar(&todos);
            }

            "toggle" | "done" | "d" => {
                match args.parse::<u32>() {
                    Ok(id) => match list.toggle(id) {
                        Ok(todo) => {
                            let status = if todo.selesai { "selesai" } else { "belum selesai" };
                            println!("🔄 Todo #{} sekarang {}", id, status);
                        }
                        Err(e) => println!("❌ {}", e),
                    },
                    Err(_) => println!("❌ ID harus berupa angka!"),
                }
            }

            "hapus" | "delete" | "rm" => {
                match args.parse::<u32>() {
                    Ok(id) => match list.hapus(id) {
                        Ok(todo) => println!("🗑️  Dihapus: {}", todo.judul),
                        Err(e) => println!("❌ {}", e),
                    },
                    Err(_) => println!("❌ ID harus berupa angka!"),
                }
            }

            "edit" | "e" => {
                let edit_parts: Vec<&str> = args.splitn(2, ' ').collect();
                if edit_parts.len() < 2 {
                    println!("❌ Format: edit <id> <judul baru>");
                    continue;
                }
                match edit_parts[0].parse::<u32>() {
                    Ok(id) => match list.edit(id, edit_parts[1]) {
                        Ok(todo) => println!("✏️  Diedit: {}", todo),
                        Err(e) => println!("❌ {}", e),
                    },
                    Err(_) => println!("❌ ID harus berupa angka!"),
                }
            }

            "cari" | "search" | "find" => {
                if args.is_empty() {
                    println!("❌ Keyword tidak boleh kosong!");
                    continue;
                }
                let hasil = list.cari(args);
                println!("\n🔍 Hasil pencarian '{}':", args);
                cetak_daftar(&hasil);
            }

            "bersihkan" | "clean" => {
                let jumlah = list.bersihkan_selesai();
                println!("🧹 {} todo yang selesai telah dihapus", jumlah);
            }

            "stats" | "statistik" => {
                let (total, selesai, belum) = list.statistik();
                println!("\n📊 Statistik:");
                println!("  Total   : {}", total);
                println!("  Selesai : {} ✅", selesai);
                println!("  Belum   : {} ⬜", belum);
                if total > 0 {
                    let persen = (selesai as f64 / total as f64 * 100.0) as u32;
                    println!("  Progress: {}%", persen);
                    let bar_len = 20;
                    let filled = (persen as usize * bar_len) / 100;
                    let empty = bar_len - filled;
                    println!(
                        "  [{}{}]",
                        "█".repeat(filled),
                        "░".repeat(empty)
                    );
                }
            }

            "help" | "h" | "?" => cetak_help(),

            "quit" | "exit" | "q" => {
                println!("👋 Sampai jumpa! Data tersimpan di '{}'", file_path);
                break;
            }

            _ => {
                println!("❓ Perintah tidak dikenal: '{}'. Ketik 'help' untuk bantuan.", command);
            }
        }
    }
}

// ============================================================
// 🎉 SELAMAT! Kamu sudah menyelesaikan semua tutorial!
//
// 🚀 LANGKAH SELANJUTNYA:
// 1. Tambahkan fitur: due date, kategori/tag, sub-tasks
// 2. Gunakan serde + JSON untuk format penyimpanan yang lebih baik
// 3. Tambahkan warna menggunakan crate `colored`
// 4. Buat versi web menggunakan Actix-web atau Axum
// 5. Buat versi TUI menggunakan crate `ratatui`
// 6. Publish sebagai crate di crates.io
//
// 📚 RESOURCES:
// - The Rust Programming Language Book: https://doc.rust-lang.org/book/
// - Rust by Example: https://doc.rust-lang.org/rust-by-example/
// - Rustlings (exercises): https://github.com/rust-lang/rustlings
// - Exercism Rust Track: https://exercism.org/tracks/rust
// - Crates.io: https://crates.io
// ============================================================
