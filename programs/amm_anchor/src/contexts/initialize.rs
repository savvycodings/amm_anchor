use crate::errors::AmmError;
use crate::state::Config;
use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{Mint, TokenInterface},
};
use std::collections::BTreeMap;

#[derive(Accounts)]
#[instruction(seed: u64)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub initializer: Signer<'info>,
    pub mint_x: InterfaceAccount<'info, Mint>,
    pub mint_y: InterfaceAccount<'info, Mint>,
    #[account(
        init,
        payer = initializer,
        seeds = [b"lp", config.key.as_ref()],
        mint::decimals = 6,
        mint::authority = auth,
        bump,
    )]
    pub mint_lp: InterfaceAccount<'info, Mint>,

    #[account(
        init_if_needed,
        payer = initializer,
        associated_token::mint = mint_x,
        associated_token::authority = auth
    )]
    pub vault_x: Interface<'info, TokenInterface>,
    #[account(
        init_if_needed,
        payer = initializer,
        associated_token::mint = mint_y,
        associated_token::authority = auth
    )]
    pub vault_y: Interface<'info, TokenInterface>,

    #[account(
        seeds = [b"auth", config.key().as_ref()],
        bump
    )]
    // CHECK: this is safe because it's only used for signature
    pub auth: UncheckedAccount<'info>,
    #[account(
        init,
        payer = initializer,
        seeds = [b"config", initializer.key.as_ref(), seed.to_le_bytes().as_ref()],
        bump,
        space = Config::LEN
    )]
    pub config: Account<'info, Config>,
    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

impl<'info> Initialize<'info> {
    pub fn init(
        &mut self,
        bumps: BTreeMap<String, u8>,
        seed: u64,
        fee: u16,
        has_authority: bool,
    ) -> Result<()> {
        require!(fee < 10000, AmmError::InvalidFee);
        // Get all our bumps
        let (auth_bump, config_bump, lp_bump) = (
            *bumps.get("auth").ok_or(AmmError::BumpError)?,
            *bumps.get("config").ok_or(AmmError::BumpError)?,
            *bumps.get("mint_lp").ok_or(AmmError::BumpError)?,
        );
        self.config.init(
            seed,
            has_authority,
            self.mint_x.key(),
            self.mint_y.key(),
            fee,
            auth_bump,
            config_bump,
            lp_bump,
        );
        Ok(())
    }
}
