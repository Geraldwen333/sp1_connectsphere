// Program ini tidak menggunakan standard library, jadi kita nonaktifkan.
#![no_main]

// Impor entrypoint macro dari sp1-zkvm.
sp1_zkvm::entrypoint!(main);

fn main() {
    // Program ZK ini membuktikan bahwa ia "mengetahui" dua angka (a, b)
    // yang jika dikalikan akan menghasilkan angka publik (n).

    // 1. Baca input dari prover.
    // `n` adalah angka publik (tantangan).
    // `a` dan `b` adalah angka privat (rahasia yang tidak akan terungkap).
    let n = sp1_zkvm::io::read::<u64>();
    let a = sp1_zkvm::io::read::<u64>();
    let b = sp1_zkvm::io::read::<u64>();

    // 2. Lakukan pembuktian.
    // Jika `a * b` tidak sama dengan `n`, program akan `panic`.
    // Ketika program panic, pembuatan bukti akan gagal.
    if a.checked_mul(b).unwrap_or(0) != n {
        panic!("Verification failed: factors do not multiply to n");
    }

    // 3. Commit input publik ke dalam bukti.
    // Ini penting agar smart contract tahu untuk tantangan `n` mana bukti ini berlaku.
    // Dalam kasus ini, kita hanya membuktikan bahwa komputasi berhasil, jadi kita commit angka 1.
    sp1_zkvm::io::commit(&1u32);
}