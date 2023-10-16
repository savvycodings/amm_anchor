use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, Token};
use std::collections::BTreeMap;

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub initializer: Signer<'info>,
    pub mint_x: Account<'info, Mint>,
    pub mint_y: Account<'info, Mint>,
    #[account(
        init,
        payer = initializer,
        seeds = [b"lp", config.key.as_ref()],
        mint::decimals = 6,
        mint::authority = auth,
        bump,
    )]
    pub lp_mint: Account<'info, Mint>,
}
