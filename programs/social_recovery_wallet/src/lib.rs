use anchor_lang::prelude::*;

pub mod state;
pub mod error;

declare_id!("zKwDxvrQqXq1b5fNHUJv5CBqheQUsQRBEfF4dT8B1EV");

#[program]
pub mod social_recovery_wallet {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, threshold: u8, bump: u8) -> Result<()> {
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

    pub fn add_guardian(ctx: Context<AddGuardian>, guardian: Pubkey) -> Result<()> {
        let wallet = &mut ctx.accounts.wallet;

        // Check that guardian does not already exist
        if wallet.guardians.contains(&guardian) {
            return Err(error::SocialRecoveryWalletError::GuardianAlreadyExists.into());
        }

        // Check that maximum number of guardians is not reached
        if wallet.guardians.len() >= state::MAX_GUARDIANS {
            return Err(error::SocialRecoveryWalletError::InsufficientGuardians.into());
        }

        wallet.guardians.push(guardian);

        Ok(())
    }

    pub fn remove_guardian(ctx: Context<RemoveGuardian>, guardian: Pubkey) -> Result<()> {
        let wallet = &mut ctx.accounts.wallet;

        // Check that guardian exists
        let position = match wallet.guardians.iter().position(|&g| g == guardian) {
            Some(pos) => pos,
            None => return Err(error::SocialRecoveryWalletError::GuardianNotFound.into()),
        };

        wallet.guardians.remove(position);

        Ok(())
    }

    pub fn update_threshold(ctx: Context<UpdateThreshold>, new_threshold: u8) -> Result<()> {
        let wallet = &mut ctx.accounts.wallet;

        // Check that new threshold is valid
        if new_threshold == 0 || new_threshold > wallet.guardians.len() as u8 {
            return Err(error::SocialRecoveryWalletError::InvalidThreshold.into());
        }

        wallet.threshold = new_threshold;

        Ok(())
    }

    pub fn initiate_recovery(ctx: Context<InitiateRecovery>) -> Result<()> {
        let wallet = &mut ctx.accounts.wallet;

        // Check that guardian is valid
        if !wallet.guardians.contains(&ctx.accounts.guardian.key()) {
            return Err(error::SocialRecoveryWalletError::InvalidSigner.into());
        }

        // Check that no recovery is already in progress
        if wallet.recovery_in_progress {
            return Err(error::SocialRecoveryWalletError::RecoveryAlreadyInProgress.into());
        }

        // Start recovery process
        wallet.recovery_in_progress = true;
        wallet.new_owner = Some(ctx.accounts.new_owner.key());
        wallet.recovery_initiated_at = Some(Clock::get()?.unix_timestamp);
        wallet.approvals = vec![ctx.accounts.guardian.key()];

        Ok(())
    }

    pub fn approve_recovery(ctx: Context<ApproveRecovery>) -> Result<()> {
        let wallet = &mut ctx.accounts.wallet;

        // Check that guardian is valid
        if !wallet.guardians.contains(&ctx.accounts.guardian.key()) {
            return Err(error::SocialRecoveryWalletError::InvalidSigner.into());
        }

        // Check that guardian hasn't already approved
        if wallet.approvals.contains(&ctx.accounts.guardian.key()) {
            return Err(error::SocialRecoveryWalletError::InvalidTransaction.into());
        }

        wallet.approvals.push(ctx.accounts.guardian.key());

        Ok(())
    }

    pub fn execute_recovery(ctx: Context<ExecuteRecovery>) -> Result<()> {
        let wallet = &mut ctx.accounts.wallet;

        // Check that recovery period has elapsed (24 hours)
        let now = Clock::get()?.unix_timestamp;
        let recovery_initiated_at = wallet.recovery_initiated_at.ok_or(error::SocialRecoveryWalletError::NoRecoveryInProgress)?;
        let recovery_period = 24 * 60 * 60; // 24 hours in seconds

        if now - recovery_initiated_at < recovery_period {
            return Err(error::SocialRecoveryWalletError::RecoveryPeriodNotElapsed.into());
        }

        // Execute recovery - transfer ownership to new owner
        let new_owner = wallet.new_owner.ok_or(error::SocialRecoveryWalletError::InvalidNewOwner)?;
        wallet.owner = new_owner;
        wallet.recovery_in_progress = false;
        wallet.new_owner = None;
        wallet.recovery_initiated_at = None;
        wallet.approvals = Vec::new();

        Ok(())
    }

    pub fn cancel_recovery(ctx: Context<CancelRecovery>) -> Result<()> {
        let wallet = &mut ctx.accounts.wallet;

        wallet.recovery_in_progress = false;
        wallet.new_owner = None;
        wallet.recovery_initiated_at = None;
        wallet.approvals = Vec::new();

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(
        init,
        payer = payer,
        space = state::Wallet::LEN,
        seeds = [b"wallet", payer.key().as_ref()],
        bump
    )]
    pub wallet: Account<'info, state::Wallet>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct AddGuardian<'info> {
    pub owner: Signer<'info>,
    #[account(
        mut,
        seeds = [b"wallet", owner.key().as_ref()],
        bump = wallet.bump
    )]
    pub wallet: Account<'info, state::Wallet>,
}

#[derive(Accounts)]
pub struct RemoveGuardian<'info> {
    pub owner: Signer<'info>,
    #[account(
        mut,
        seeds = [b"wallet", owner.key().as_ref()],
        bump = wallet.bump
    )]
    pub wallet: Account<'info, state::Wallet>,
}

#[derive(Accounts)]
pub struct UpdateThreshold<'info> {
    pub owner: Signer<'info>,
    #[account(
        mut,
        seeds = [b"wallet", owner.key().as_ref()],
        bump = wallet.bump
    )]
    pub wallet: Account<'info, state::Wallet>,
}

#[derive(Accounts)]
pub struct InitiateRecovery<'info> {
    pub guardian: Signer<'info>,
    #[account(
        mut,
        seeds = [b"wallet", wallet.owner.as_ref()],
        bump = wallet.bump
    )]
    pub wallet: Account<'info, state::Wallet>,
    /// CHECK: This is the new owner being proposed
    pub new_owner: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct ApproveRecovery<'info> {
    pub guardian: Signer<'info>,
    #[account(
        mut,
        seeds = [b"wallet", wallet.owner.as_ref()],
        bump = wallet.bump,
        constraint = wallet.recovery_in_progress @ error::SocialRecoveryWalletError::NoRecoveryInProgress
    )]
    pub wallet: Account<'info, state::Wallet>,
}

#[derive(Accounts)]
pub struct ExecuteRecovery<'info> {
    pub guardian: Signer<'info>,
    #[account(
        mut,
        seeds = [b"wallet", wallet.owner.as_ref()],
        bump = wallet.bump,
        constraint = wallet.recovery_in_progress @ error::SocialRecoveryWalletError::NoRecoveryInProgress,
        constraint = wallet.approvals.len() as u8 >= wallet.threshold @ error::SocialRecoveryWalletError::InsufficientGuardians
    )]
    pub wallet: Account<'info, state::Wallet>,
}

#[derive(Accounts)]
pub struct CancelRecovery<'info> {
    pub owner: Signer<'info>,
    #[account(
        mut,
        seeds = [b"wallet", owner.key().as_ref()],
        bump = wallet.bump,
        constraint = wallet.recovery_in_progress @ error::SocialRecoveryWalletError::NoRecoveryInProgress
    )]
    pub wallet: Account<'info, state::Wallet>,
}
