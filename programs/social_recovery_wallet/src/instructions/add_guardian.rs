use anchor_lang::prelude::*;
use crate::state::Wallet;
use crate::error::SocialRecoveryWalletError;

#[derive(Accounts)]
pub struct AddGuardian<'info> {
    pub owner: Signer<'info>,
    #[account(
        mut,
        seeds = [b"wallet", owner.key().as_ref()],
        bump = wallet.bump
    )]
    pub wallet: Account<'info, Wallet>,
}

impl<'info> AddGuardian<'info> {
    pub fn process(ctx: Context<AddGuardian>, guardian: Pubkey) -> Result<()> {
        let wallet = &mut ctx.accounts.wallet;

        // Check that guardian does not already exist
        if wallet.guardians.contains(&guardian) {
            return Err(SocialRecoveryWalletError::GuardianAlreadyExists.into());
        }

        wallet.guardians.push(guardian);

        Ok(())
    }
}
