use anchor_lang::prelude::*;
use crate::state::Wallet;
use crate::error::SocialRecoveryWalletError;

#[derive(Accounts)]
pub struct UpdateThreshold<'info> {
    pub owner: Signer<'info>,
    #[account(
        mut,
        seeds = [b"wallet", owner.key().as_ref()],
        bump = wallet.bump
    )]
    pub wallet: Account<'info, Wallet>,
}

impl<'info> UpdateThreshold<'info> {
    pub fn process(ctx: Context<UpdateThreshold>, new_threshold: u8) -> Result<()> {
        let wallet = &mut ctx.accounts.wallet;

        // Check that new threshold is valid
        if new_threshold == 0 || new_threshold > wallet.guardians.len() as u8 {
            return Err(SocialRecoveryWalletError::InvalidThreshold.into());
        }

        wallet.threshold = new_threshold;

        Ok(())
    }
}
