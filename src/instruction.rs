//! Program instructions

use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::program_error::ProgramError;
use crate::error::EscrowError;

#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug, Clone)]
pub enum EscrowInstruction {
    /// Starts the trade by creating and populating an escrow account and
    /// transferring ownership of the given token account to the PDA.
    ///
    /// The client is expected to have already created a temporary token account
    /// and transferred the tokens to be sold into it.
    ///
    /// Accounts expected:
    ///
    /// 0. `[signer]` The account of the person initializing the escrow (Alice).
    /// 1. `[writable]` The temporary token account holding the tokens Alice wants to trade.
    /// 2. `[]` Alice's source token account, used to validate the mint of the temporary account.
    /// 3. `[writable]` The escrow account, it will hold all necessary info about the trade.
    /// 4. `[]` The rent sysvar.
    /// 5. `[]` The token program.
    InitEscrow {
        /// The amount of SOL Alice wants for her tokens.
        amount: u64,
    },

    /// Accepts a trade. Bob (the taker) sends SOL to Alice, and the
    /// escrow program sends Alice's tokens to Bob.
    ///
    /// Accounts expected:
    ///
    /// 0. `[signer]` The account of the person taking the trade (Bob).
    /// 1. `[writable]` Bob's token account that will receive the tokens from Alice.
    /// 2. `[writable]` The PDA's temporary token account.
    /// 3. `[writable]` The escrow account.
    /// 4. `[writable]` The account of the initializer of the escrow (Alice).
    /// 5. `[]` The PDA account that owns the temporary token account.
    /// 6. `[]` The token program.
    /// 7. `[]` The system program.
    Exchange {
        /// The amount of tokens Bob expects to receive. This is validated against the amount in the PDA's token account.
        amount: u64,
    },

    /// Cancels a trade and returns ownership of the temporary token account to Alice.
    /// The escrow account is closed and its rent is returned to Alice.
    ///
    /// Accounts expected:
    ///
    /// 0. `[signer]` The account of the person initializing the escrow (Alice).
    /// 1. `[writable]` The PDA's temporary token account.
    /// 2. `[writable]` The escrow account.
    /// 3. `[]` The PDA account that owns the temporary token account.
    /// 4. `[]` The token program.
    Cancel,
}

impl EscrowInstruction {
    /// Unpacks a byte buffer into a EscrowInstruction.
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        Self::try_from_slice(input).map_err(|_| EscrowError::InvalidInstruction.into())
    }
}
