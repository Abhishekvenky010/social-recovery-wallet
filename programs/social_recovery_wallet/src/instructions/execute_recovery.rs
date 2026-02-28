use anchor_lang::prelude::*;
use crate::state::Wallet;
use crate::error::SocialRecoveryWalletError;

#[derive(Accounts)]
pub struct ExecuteRecovery<'info> {
    pub guardian: Signer<'info>,
    #[account(
        mut,
        seeds = [b"wallet", wallet.owner.as_ref()],
        bump = wallet.bump,
        constraint = wallet.recovery_in_progress @ SocialRecoveryWalletError::NoRecoveryInProgress,
        constraint = wallet.approvals.len() as u8 >= wallet.threshold @ SocialRecoveryWalletError::InsufficientGuardians
    )]
    pub wallet: Account<'info, Wallet>,
}

impl<'info> ExecuteRecovery<'info> {
    pub fn process(ctx: Context<ExecuteRecovery>) -> Result<()> {
        let wallet = &mut ctx.accounts.wallet;

        // Check that recovery period has elapsed (24 hours)
        let now = Clock::get()?.unix_timestamp;
        let recovery_initiated_at = wallet.recovery_initiated_at.ok_or(SocialRecoveryWalletError::NoRecoveryInProgress)?;
        let recovery_period = 24 * 60 * 60; // 24 hours in seconds

        if now - recovery_initiated_at < recovery_period {
            return Err(SocialRecoveryWalletError::RecoveryPeriodNotElapsed.into());
        }

        // Execute recovery - transfer ownership to new owner
        let new_owner = wallet.new_owner.ok_or(SocialRecoveryWalletError::InvalidNewOwner)?;
        wallet.owner = new_owner;
        wallet.recovery_in_progress = false;
        wallet.new_owner = None;
        wallet.recovery_initiated_at = None;
        wallet.approvals = Vec::new();

        Ok(())
    }
}
