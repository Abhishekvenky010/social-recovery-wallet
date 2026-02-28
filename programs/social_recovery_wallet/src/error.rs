use anchor_lang::prelude::*;

#[error_code]
pub enum SocialRecoveryWalletError {
    #[msg("Invalid owner")]
    InvalidOwner,
    #[msg("Invalid signer")]
    InvalidSigner,
    #[msg("Insufficient number of guardians")]
    InsufficientGuardians,
    #[msg("Guardian already exists")]
    GuardianAlreadyExists,
    #[msg("Guardian not found")]
    GuardianNotFound,
    #[msg("Recovery period not elapsed")]
    RecoveryPeriodNotElapsed,
    #[msg("Recovery already in progress")]
    RecoveryAlreadyInProgress,
    #[msg("No recovery in progress")]
    NoRecoveryInProgress,
    #[msg("Invalid threshold")]
    InvalidThreshold,
    #[msg("Invalid new owner")]
    InvalidNewOwner,
    #[msg("Invalid transaction")]
    InvalidTransaction,
}
