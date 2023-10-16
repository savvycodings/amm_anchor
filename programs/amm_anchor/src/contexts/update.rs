use crate::errors::AmmError;
use crate::state::Config;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct Update<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(
        seeds = [b"config", authority.key().as_ref(), config.seed.to_le_bytes().as_ref()],
        bump = config.config_bump
    )]
    pub config: Account<'info, Config>,
}

impl<'info> Update<'info> {
    pub fn update(&mut self, locked: bool) -> Result<()> {
        if self.config.has_authority == false {
            return err!(AmmError::NoAuthoritySet);
        }
        self.config.locked = locked;
        Ok(())
    }
}
