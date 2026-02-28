use anchor_lang::prelude::*;
use crate::state::Wallet;

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(
        init,
        payer = payer,
        space = Wallet::LEN,
        seeds = [b"wallet", payer.key().as_ref()],
        bump
    )]
    pub wallet: Account<'info, Wallet>,
    pub system_program: Program<'info, System>,
}

impl<'info> Initialize<'info> {
    pub fn process(ctx: Context<Initialize>, threshold: u8, bump: u8) -> Result<()> {
        let wallet = &mut ctx.accounts.wallet;
        wallet.owner = ctx.accounts.payer.key();
        wallet.guardians = Vec::new();
        wallet.threshold = threshold;
        wallet.recovery_in_progress = false;
        wallet.new_owner = None;
        wallet.recovery_initiated_at = None;
        wallet.approvals = Vec::new();
        wallet.bump = bump;

        Ok(())
    }
}
