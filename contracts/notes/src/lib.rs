#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, symbol_short, Env, String, Symbol, Vec};

// Struktur data yang akan menyimpan kicauan/menfess
#[contracttype]
#[derive(Clone, Debug)]
pub struct Keluhan {
    pub id: u64,
    pub nim: String,       // Menggunakan String karena NIM IPB ada hurufnya (ex: G64...)
    pub konten: String,    // Isi dari kicauan atau menfess mahasiswa
    pub timestamp: u64,    // Waktu saat keluhan dibuat
}

// Storage key untuk data keluhan
const KELUHAN_DATA: Symbol = symbol_short!("KELUHAN");

#[contract]
pub struct IpbMenfessContract;

#[contractimpl]
impl IpbMenfessContract {
    // Fungsi untuk melihat semua kicauan (bisa diakses masyarakat IPB)
    pub fn get_semua_keluhan(env: Env) -> Vec<Keluhan> {
        // 1. Ambil data keluhan dari storage, jika kosong kembalikan vector kosong
        env.storage().instance().get(&KELUHAN_DATA).unwrap_or(Vec::new(&env))
    }

    // Fungsi untuk mengirim menfess/keluhan baru
    pub fn tambah_keluhan(env: Env, nim: String, konten: String) -> String {
        // 1. Ambil data kicauan yang sudah ada dari storage
        let mut daftar_keluhan: Vec<Keluhan> = env.storage().instance().get(&KELUHAN_DATA).unwrap_or(Vec::new(&env));
        
        // 2. Buat object keluhan baru
        let keluhan_baru = Keluhan {
            id: env.prng().gen::<u64>(),         // Bikin ID unik secara acak
            nim,                                 // Simpan NIM
            konten,                              // Simpan isi curhatan
            timestamp: env.ledger().timestamp(), // Catat waktu dari ledger blockchain
        };
        
        // 3. Tambahkan keluhan baru ke daftar yang sudah ada
        daftar_keluhan.push_back(keluhan_baru);
        
        // 4. Simpan kembali daftar keluhan yang udah di-update ke storage
        env.storage().instance().set(&KELUHAN_DATA, &daftar_keluhan);
        
        String::from_str(&env, "Menfess kamu berhasil dikirim, semangat terus kuliahnya!")
    }

    // Fungsi untuk menghapus kicauan berdasarkan id (misal kalau salah ngetik/nyesel)
    pub fn hapus_keluhan(env: Env, id: u64) -> String {
        // 1. Ambil data keluhan dari storage 
        let mut daftar_keluhan: Vec<Keluhan> = env.storage().instance().get(&KELUHAN_DATA).unwrap_or(Vec::new(&env));

        // 2. Cari index kicauan yang mau dihapus lewat perulangan
        for i in 0..daftar_keluhan.len() {
            if daftar_keluhan.get(i).unwrap().id == id {
                daftar_keluhan.remove(i); // Hapus elemen di index tersebut

                // 3. Update data di storage setelah dihapus
                env.storage().instance().set(&KELUHAN_DATA, &daftar_keluhan);
                return String::from_str(&env, "Kicauan berhasil ditarik/dihapus");
            }
        }

        // Kalau ID nggak ketemu
        String::from_str(&env, "Kicauan tidak ditemukan")
    }
}

mod test;