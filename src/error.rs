//! Program-specific errors

use solana_program::program_error::ProgramError;
use thiserror::Error;

#[derive(Error, Debug, Copy, Clone, PartialEq)]
pub enum EscrowError {
    /// Invalid instruction data
    #[error("Invalid Instruction")]
    InvalidInstruction,

    /// The account is not rent exempt
    #[error("Account is not rent exempt")]
    NotRentExempt,

    /// The expected amount does not match the actual amount
    #[error("Expected amount mismatch")]
    ExpectedAmountMismatch,

    /// The authority does not match the expected authority
    #[error("Authority mismatch")]
    AuthorityMismatch,

    /// The escrow is not in a valid state for the requested action
    #[error("Invalid escrow state")]
    InvalidState,

    /// The owner of the token account is invalid
    #[error("Invalid token account owner")]
    InvalidOwner,

    /// The escrow has already been initialized
    #[error("Escrow already initialized")]
    AlreadyInitialized,
}

impl From<EscrowError> for ProgramError {
    fn from(e: EscrowError) -> Self {
        ProgramError::Custom(e as u32)
    }
}
