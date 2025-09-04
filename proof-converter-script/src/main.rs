use sp1_sdk::SP1ProofWithPublicValues;
use std::path::Path;

// PERBAIKAN 1: Impor `convert_sp1_gnark_to_ark` dari root `sp1_sui`.
use sp1_sui::convert_sp1_gnark_to_ark;
// PERBAIKAN 2: Impor `encode` langsung dari crate `hex`.
use hex::encode;

fn main() {
    println!("Reading proof file from ./my_proof.bin...");
    let proof_path = Path::new("./my_proof.bin");
    let proof = SP1ProofWithPublicValues::load(proof_path).expect("failed to load proof file");
    
    println!("\nConverting proof to Sui transaction arguments...");
    let (pvk, public_inputs, proof_points) = convert_sp1_gnark_to_ark(proof);
    
    // PERBAIKAN 3: Panggil `hex::encode` secara langsung.
    let pvk_hex = encode(&pvk);
    let public_inputs_hex = encode(&public_inputs);
    let proof_points_hex = encode(&proof_points);

    println!("\n✅ Siap! Salin nilai-nilai berikut ke file `src/App.tsx` Anda:");
    println!("\n// Verification Key (pvk)\nconst pvk = \"{}\";", pvk_hex);
    println!("\n// Public Inputs\nconst public_inputs = \"{}\";", public_inputs_hex);
    println!("\n// Proof Points\nconst proof_points = \"{}\";", proof_points_hex);
}