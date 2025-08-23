use solana_program::program_error::ProgramError;
use thiserror::Error;
#[derive(Error, Debug,Copy, Clone)]
pub enum EscrowError {
     #[error("Invalid instruction")]
    InvalidInstruction,
    #[error("Unauthorized")]
    Unauthorized,
    #[error("Seller not confirmed")]
    NotConfirmed,
    #[error("Already confirmed")]
    AlreadyConfirmed,
}

impl From<EscrowError> for ProgramError {
    fn from(e: EscrowError) -> Self {
        ProgramError::Custom(e as u32)
    }
}

