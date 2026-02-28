use anchor_lang::prelude::*;
use crate::state::Wallet;
use crate::error::SocialRecoveryWalletError;

#[derive(Accounts)]
pub struct RemoveGuardian<'info> {
    pub owner: Signer<'info>,
    #[account(
        mut,
        seeds = [b"wallet", owner.key().as_ref()],
        bump = wallet.bump
    )]
    pub wallet: Account<'info, Wallet>,
}

impl<'info> RemoveGuardian<'info> {
    pub fn process(ctx: Context<RemoveGuardian>, guardian: Pubkey) -> Result<()> {
        let wallet = &mut ctx.accounts.wallet;

        // Check that guardian exists
        let position = match wallet.guardians.iter().position(|&g| g == guardian) {
            Some(pos) => pos,
            None => return Err(SocialRecoveryWalletError::GuardianNotFound.into()),
        };

        wallet.guardians.remove(position);

        Ok(())
    }
}
