use borsh::{BorshDeserialize, BorshSerialize};

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone)]
pub enum EscrowInstruction {
   InitEscrow { amount: u64 },
   SellerConfirm{},
   Release{},
   Cancel{},
}
