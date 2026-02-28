use anchor_lang::prelude::*;
use crate::state::Wallet;
use crate::error::SocialRecoveryWalletError;

#[derive(Accounts)]
pub struct ApproveRecovery<'info> {
    pub guardian: Signer<'info>,
    #[account(
        mut,
        seeds = [b"wallet", wallet.owner.as_ref()],
        bump = wallet.bump,
        constraint = wallet.recovery_in_progress @ SocialRecoveryWalletError::NoRecoveryInProgress
    )]
    pub wallet: Account<'info, Wallet>,
}

impl<'info> ApproveRecovery<'info> {
    pub fn process(ctx: Context<ApproveRecovery>) -> Result<()> {
        let wallet = &mut ctx.accounts.wallet;

        // Check that guardian is valid
        if !wallet.guardians.contains(&ctx.accounts.guardian.key()) {
            return Err(SocialRecoveryWalletError::InvalidSigner.into());
        }

        // Check that guardian hasn't already approved
        if wallet.approvals.contains(&ctx.accounts.guardian.key()) {
            return Err(SocialRecoveryWalletError::InvalidTransaction.into());
        }

        wallet.approvals.push(ctx.accounts.guardian.key());

        Ok(())
    }
}
