use anchor_lang::prelude::*;
use crate::state::Wallet;
use crate::error::SocialRecoveryWalletError;

#[derive(Accounts)]
pub struct InitiateRecovery<'info> {
    pub guardian: Signer<'info>,
    #[account(
        mut,
        seeds = [b"wallet", wallet.owner.as_ref()],
        bump = wallet.bump
    )]
    pub wallet: Account<'info, Wallet>,
    /// CHECK: This is the new owner being proposed
    pub new_owner: AccountInfo<'info>,
}

impl<'info> InitiateRecovery<'info> {
    pub fn process(ctx: Context<InitiateRecovery>) -> Result<()> {
        let wallet = &mut ctx.accounts.wallet;

        // Check that guardian is valid
        if !wallet.guardians.contains(&ctx.accounts.guardian.key()) {
            return Err(SocialRecoveryWalletError::InvalidSigner.into());
        }

        // Check that no recovery is already in progress
        if wallet.recovery_in_progress {
            return Err(SocialRecoveryWalletError::RecoveryAlreadyInProgress.into());
        }

        // Start recovery process
        wallet.recovery_in_progress = true;
        wallet.new_owner = Some(ctx.accounts.new_owner.key());
        wallet.recovery_initiated_at = Some(Clock::get()?.unix_timestamp);
        wallet.approvals = vec![ctx.accounts.guardian.key()];

        Ok(())
    }
}
