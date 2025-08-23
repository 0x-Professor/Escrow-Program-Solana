//! Program state

use solana_program::{
    program_pack::{IsInitialized, Pack, Sealed},
    program_error::ProgramError,
    pubkey::Pubkey,
};
use arrayref::{array_mut_ref, array_ref, array_refs, mut_array_refs};

/// Escrow state
#[derive(Debug, PartialEq)]
pub struct Escrow {
    /// Indicates if the escrow is initialized.
    pub is_initialized: bool,
    /// Pubkey of the initializer (Alice).
    pub initializer_pubkey: Pubkey,
    /// Pubkey of the temporary token account holding the tokens to be sold.
    pub temp_token_account_pubkey: Pubkey,
    /// The amount of SOL the initializer expects to receive.
    pub expected_amount: u64,
}

impl Sealed for Escrow {}

impl IsInitialized for Escrow {
    fn is_initialized(&self) -> bool {
        self.is_initialized
    }
}

impl Pack for Escrow {
    const LEN: usize = 1 + 32 + 32 + 8; // 73 bytes

    fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
        let src = array_ref![src, 0, Escrow::LEN];
        let (
            is_initialized,
            initializer_pubkey,
            temp_token_account_pubkey,
            expected_amount,
        ) = array_refs![src, 1, 32, 32, 8];

        Ok(Escrow {
            is_initialized: is_initialized[0] == 1,
            initializer_pubkey: Pubkey::new_from_array(*initializer_pubkey),
            temp_token_account_pubkey: Pubkey::new_from_array(*temp_token_account_pubkey),
            expected_amount: u64::from_le_bytes(*expected_amount),
        })
    }

    fn pack_into_slice(&self, dst: &mut [u8]) {
        let dst = array_mut_ref![dst, 0, Escrow::LEN];
        let (
            is_initialized_dst,
            initializer_pubkey_dst,
            temp_token_account_pubkey_dst,
            expected_amount_dst,
        ) = mut_array_refs![dst, 1, 32, 32, 8];

        is_initialized_dst[0] = self.is_initialized as u8;
        initializer_pubkey_dst.copy_from_slice(self.initializer_pubkey.as_ref());
        temp_token_account_pubkey_dst.copy_from_slice(self.temp_token_account_pubkey.as_ref());
        *expected_amount_dst = self.expected_amount.to_le_bytes();
    }
}
