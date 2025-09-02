# SP1 Toolchain Installation Guide for ConnectSphere

This guide explains how to set up the environment required to generate Zero-Knowledge Proofs (ZK Proofs) for the ConnectSphere dApp.  
The process is designed for advanced users who are comfortable working with the command line.

---

## 📦 Step 1: Basic Prerequisites (Rust & Sui)

Make sure you have **Rust** and the **Sui Client CLI** installed.

### 1. Install Rust
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env
```

### 2. Install Sui Client
This will also install the Rust dependencies required by Sui.
```bash
curl -L https://get.sui.io | bash -s -- --install-rust-deps
source ~/.sui/env
```

---

## ⚙️ Step 2: Manual Installation of the SP1 Toolchain

We will install SP1 manually (instead of using the default `sp1up` script) to ensure stability.

### 1. Clone SP1 Repository & Install CLI
```bash
# Navigate to your home directory
cd ~

# Clone the official SP1 repository
git clone https://github.com/succinctlabs/sp1.git

# Enter the CLI directory and install
cd sp1/crates/cli
cargo install --path .
```

### 2. Verify Installation
```bash
cargo prove --version
```
You should see the installed version of `cargo-prove`.

### 3. Manually Link the Toolchain
```bash
# Create a directory for the toolchain
mkdir -p ~/.sp1/toolchains/succinct

# Extract the downloaded toolchain into the directory
# Note: If the file doesn’t exist, run `sp1up` once to fetch it, cancel midway, and retry.
tar -xvf ~/.sp1/rust-toolchain-x86_64-unknown-linux-gnu.tar.gz -C ~/.sp1/toolchains/succinct --strip-components=1

# Register the 'succinct' toolchain with rustup
rustup toolchain link succinct ~/.sp1/toolchains/succinct
```

At this point, the SP1 environment should be fully set up.

---

## 🔧 Step 3: Setting Up the ConnectSphere Prover Repository

### 1. Clone the Prover Repository
```bash
# Replace YOUR_GITHUB_URL with the actual GitHub repo URL
git clone https://github.com/YOUR_GITHUB_URL/sp1_connectsphere.git
cd sp1_connectsphere
```

### 2. Build Dependencies
```bash
cargo build --release
```

---

## 🚀 Usage: Generating Proofs

Once everything is installed, you can generate your own proofs:

1. Open the ConnectSphere dApp.  
2. Navigate to the verification menu and enter your two secret numbers.  
3. The dApp will display a unique `cargo run` command.  
4. Open a terminal in the `sp1_connectsphere` directory.  
5. Run the provided command. Example:
   ```bash
   cargo run --release -p proof-of-secret-script -- --n 791918400 --a 282828 --b 2800 && cargo run --release -p proof-converter-script
   ```
6. Once the process completes, the terminal will output:
   - `pvk`
   - `public_inputs`
   - `proof_points`
7. Copy these values back into the dApp to finish verification.

---

## 📖 Notes

- Keep this file (`README.md`) inside your `sp1_connectsphere` repository.  
- These instructions are intended for **advanced developers** who need to generate proofs manually.  
- If you encounter errors with toolchain setup, re-run `sp1up` once to download missing files before retrying manual linking.  

---
