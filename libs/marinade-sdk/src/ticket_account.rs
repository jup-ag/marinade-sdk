use borsh::{BorshDeserialize, BorshSerialize};
use micro_anchor::{Discriminator, Owner};
use solana_program::pubkey::Pubkey;



#[derive(Debug, BorshDeserialize, BorshSerialize)]
pub struct TicketAccountData {
    pub state_address: Pubkey, // instance of marinade state this ticket belongs to
    pub beneficiary: Pubkey,   // main account where to send SOL when claimed
    pub lamports_amount: u64,  // amount this ticked is worth
    pub created_epoch: u64, // epoch when this acc was created (epoch when delayed-unstake was requested)
}

impl Discriminator for TicketAccountData {
    const DISCRIMINATOR: [u8; 8] = [133, 77, 18, 98, 211, 1, 231, 3];
}

impl Owner for TicketAccountData {
    fn owner() -> Pubkey {
        crate::ID
    }
}