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
    fn process_init_escrow(
        accounts: &[AccountInfo],
        program_id: &Pubkey,
        amount: u64,
    ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();

        let initializer = next_account_info(account_info_iter)?;
        let escrow_account = next_account_info(account_info_iter)?;
        let seller = next_account_info(account_info_iter)?;
        let mint = next_account_info(account_info_iter)?;

        let mut escrow_info = Escrow::try_from_slice(&escrow_account.data.borrow())?;
        if escrow_info.is_initialized {
            return Err(ProgramError::AccountAlreadyInitialized);
        }

        escrow_info.is_initialized = true;
        escrow_info.initializer = *initializer.key;
        escrow_info.seller = *seller.key;
        escrow_info.mint = *mint.key;
        escrow_info.amount = amount;
        escrow_info.seller_confirmed = false;

        escrow_info.serialize(&mut *escrow_account.data.borrow_mut())?;
        Ok(())
    }
    
}