use anchor_lang::prelude::*;

// Constants for maximum values
pub const MAX_GUARDIANS: usize = 10;

#[account]
pub struct Wallet {
    pub owner: Pubkey,
    pub guardians: Vec<Pubkey>,
    pub threshold: u8,
    pub recovery_in_progress: bool,
    pub new_owner: Option<Pubkey>,
    pub recovery_initiated_at: Option<i64>,
    pub approvals: Vec<Pubkey>,
    pub bump: u8,
}

impl Wallet {
    pub const LEN: usize = 32 + // owner (Pubkey)
        4 + (32 * MAX_GUARDIANS) + // guardians (Vec<Pubkey>)
        1 + // threshold (u8)
        1 + // recovery_in_progress (bool)
        33 + // new_owner (Option<Pubkey>)
        9 + // recovery_initiated_at (Option<i64>)
        4 + (32 * MAX_GUARDIANS) + // approvals (Vec<Pubkey>)
        1; // bump (u8)
}

// Guardian account struct currently unused but kept for future expansion
#[account]
pub struct Guardian {
    pub wallet: Pubkey,
    pub guardian: Pubkey,
    pub bump: u8,
}

// Recovery account struct currently unused but kept for future expansion
#[account]
pub struct Recovery {
    pub wallet: Pubkey,
    pub new_owner: Pubkey,
    pub initiated_at: i64,
    pub approvals: Vec<Pubkey>,
    pub bump: u8,
}
