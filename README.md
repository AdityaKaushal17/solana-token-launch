# solana-token-launch
🧠 Program Functionality

This Solana program exposes two instructions, encoded in the first byte of instruction_data:

Instruction 0 → Launch Token

Handles the logic for creating or initializing a token.

0 => {
    msg!("Creating new token...");
}

Instruction 1 → Transfer Token

Handles transferring tokens from one account to another.

1 => {
    msg!("Transferring tokens...");
}

🔧 Build Instructions

To build the Solana program, install BPF tools first:

cargo install solana-bpf-tools


Then build:

cd program
cargo build-sbf


If successful, the compiled .so file appears at:

program/target/deploy/token_program.so

🚀 Deploying to Local Solana Validator

1️⃣ Start local validator
solana-test-validator


Leave this running.

2️⃣ Deploy your program in another terminal:
solana program deploy target/deploy/token_program.so


You will receive a Program ID like:

Program Id: 5pJb2...XYZ


Save this — it identifies your on-chain program.
