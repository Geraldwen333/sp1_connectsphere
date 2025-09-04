// --- PERUBAHAN: Impor yang diperlukan ---
use sp1_sdk::{include_elf, ProverClient, SP1Stdin};
use std::path::Path;
use clap::Parser;

// Gunakan `include_elf!` dengan nama crate, bukan path.
const ELF: &[u8] = include_elf!("proof-of-secret-program");

// --- PERUBAHAN: Definisikan argumen command-line menggunakan clap ---
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(long)]
    n: u64,

    #[arg(long)]
    a: u64,

    #[arg(long)]
    b: u64,
}

fn main() {
    sp1_sdk::utils::setup_logger();
    
    // --- PERUBAHAN: Parse argumen dari command line ---
    let args = Args::parse();

    // Lakukan validasi sederhana sebelum menjalankan proses proving yang mahal
    assert_eq!(
        args.n,
        args.a.checked_mul(args.b).expect("overflow"),
        "Error: n must be the product of a and b."
    );

    // --- PERUBAHAN: Gunakan input dari argumen, bukan nilai hardcoded ---
    let mut stdin = SP1Stdin::new();
    stdin.write(&args.n);
    stdin.write(&args.a);
    stdin.write(&args.b);

    //--- Proses Hasilkan Bukti (Sama seperti sebelumnya) ---
    println!("Generating proof... (this may take a few minutes)");
    let client = ProverClient::from_env();
    
    println!("Setting up proving key...");
    let (pk, _vk) = client.setup(ELF);

    println!("Generating Groth16 proof...");
    let proof = client
        .prove(&pk, &stdin)
        .groth16()
        .run()
        .expect("proving failed");
    
    println!("Proof generated successfully!");

    // Simpan bukti ke file di direktori utama workspace.
    let proof_path = Path::new("./my_proof.bin");
    proof.save(proof_path).expect("saving proof failed");
    println!("Proof saved to: {:?}", proof_path.canonicalize().unwrap());
}