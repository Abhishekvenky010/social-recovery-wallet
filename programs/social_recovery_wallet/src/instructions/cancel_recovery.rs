use anchor_lang::prelude::*;
use crate::state::Wallet;
use crate::error::SocialRecoveryWalletError;

#[derive(Accounts)]
pub struct CancelRecovery<'info> {
    pub owner: Signer<'info>,
    #[account(
        mut,
        seeds = [b"wallet", owner.key().as_ref()],
        bump = wallet.bump,
        constraint = wallet.recovery_in_progress @ SocialRecoveryWalletError::NoRecoveryInProgress
    )]
    pub wallet: Account<'info, Wallet>,
}

impl<'info> CancelRecovery<'info> {
    pub fn process(ctx: Context<CancelRecovery>) -> Result<()> {
        let wallet = &mut ctx.accounts.wallet;

        wallet.recovery_in_progress = false;
        wallet.new_owner = None;
        wallet.recovery_initiated_at = None;
        wallet.approvals = Vec::new();

        Ok(())
    }
}
