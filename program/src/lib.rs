use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program::{invoke},
    program_error::ProgramError,
    pubkey::Pubkey,
    system_instruction,
};
use spl_token::instruction as token_instruction;

entrypoint!(process_instruction);

pub fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    match instruction_data[0] {
        0 => create_mint(accounts),
        1 => transfer_tokens(accounts, instruction_data),
        _ => Err(ProgramError::InvalidInstructionData),
    }
}

fn create_mint(accounts: &[AccountInfo]) -> ProgramResult {
    msg!("Creating new token mint...");

    let accounts_iter = &mut accounts.iter();

    let payer = next_account_info(accounts_iter)?;
    let mint = next_account_info(accounts_iter)?;
    let mint_authority = next_account_info(accounts_iter)?;
    let token_program = next_account_info(accounts_iter)?;
    let system_program = next_account_info(accounts_iter)?;
    let rent = next_account_info(accounts_iter)?;

    // Create mint account
    invoke(
        &system_instruction::create_account(
            payer.key,
            mint.key,
            1_000_000_000,
            spl_token::state::Mint::LEN as u64,
            token_program.key,
        ),
        &[payer.clone(), mint.clone(), system_program.clone()],
    )?;

    // Initialize mint
    invoke(
        &token_instruction::initialize_mint(
            token_program.key,
            mint.key,
            mint_authority.key,
            None,
            9,
        )?,
        &[mint.clone(), rent.clone(), token_program.clone()],
    )?;

    msg!("Mint created successfully!");
    Ok(())
}

fn transfer_tokens(accounts: &[AccountInfo], data: &[u8]) -> ProgramResult {
    let amount = u64::from_le_bytes(data[1..9].try_into().unwrap());

    let accounts_iter = &mut accounts.iter();

    let from_account = next_account_info(accounts_iter)?;
    let to_account = next_account_info(accounts_iter)?;
    let authority = next_account_info(accounts_iter)?;
    let token_program = next_account_info(accounts_iter)?;

    invoke(
        &token_instruction::transfer(
            token_program.key,
            from_account.key,
            to_account.key,
            authority.key,
            &[],
            amount,
        )?,
        &[
            from_account.clone(),
            to_account.clone(),
            authority.clone(),
            token_program.clone(),
        ],
    )?;

    msg!("Transferred {} tokens", amount);
    Ok(())
}
