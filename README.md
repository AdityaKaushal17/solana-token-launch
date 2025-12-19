# solana-token-launch
🧠 Program Functionality

This Solana program exposes two instructions, encoded in the first byte of instruction_data:

Instruction 0 → Launch Token

Handles the logic for creating or initializing a token.

0 => 
{
    msg!("Creating new token...");
}

Instruction 1 → Transfer Token

Handles transferring tokens from one account to another.

1 => 
{
    msg!("Transferring tokens...");
}

🔧 Build Instructions

then build the Solana program, install BPF tools first:

cargo install solana-bpf-tools


