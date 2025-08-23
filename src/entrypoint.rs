//! Program entrypoint

use solana_program::{
    account_info::AccountInfo,
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    pubkey::Pubkey,
};
use crate::processor::Processor;

// Declares the program's entrypoint. The `entrypoint!` macro sets up the necessary
// boilerplate for the program to be callable by the Solana runtime.
entrypoint!(process_instruction);

/// The main processing function for the program.
///
/// The Solana runtime calls this function for each instruction sent to the program.
fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    msg!("Entrypoint");
    Processor::process(program_id, accounts, instruction_data)
}
