use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    pubkey::Pubkey,
    msg,
    program_error::ProgramError,
    program_pack::Pack,
    program::invoke_signed,
};

use crate::instruction::EscrowInstruction;
use crate::state::Escrow;
use crate::error::EscrowError;
use borsh::{BorshDeserialize, BorshSerialize};

pub struct Processor;

impl Processor {
    pub fn process(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        instruction_data: &[u8],
    ) -> ProgramResult {
        let instruction = EscrowInstruction::try_from_slice(instruction_data)
            .map_err(|_| EscrowError::InvalidInstruction)?;

        match instruction {
            EscrowInstruction::InitEscrow { amount } => {
                msg!("Instruction: InitEscrow");
                Self::process_init_escrow(accounts, program_id, amount)
            }
            EscrowInstruction::SellerConfirm {} => {
                msg!("Instruction: SellerConfirm");
                Self::process_seller_confirm(accounts, program_id)
            }
            EscrowInstruction::Release {} => {
                msg!("Instruction: Release");
                Self::process_release(accounts, program_id)
            }
            EscrowInstruction::Cancel {} => {
                msg!("Instruction: Cancel");
                Self::process_cancel(accounts, program_id)
            }
        }
    }
    
}