use crate::errors::AmmError;
use crate::state::Config;
use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::TokenAccount,
    token_interface::{Mint, TokenInterface},
};
use std::collections::BTreeMap;

#[derive(Accounts)]
pub struct deposit<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    // Mints
    pub mint_x: Box<InterfaceAccount<'info, Mint>>,
    pub mint_y: Box<InterfaceAccount<'info, Mint>>,
    #[account(
        mut,
        seeds = [b"lp", config.key().as_ref()],
        bump = config.lp_bump,
    )]
    pub lp_mint: Box<InterfaceAccount<'info, Mint>>,
    // Vaults
    #[account(
        mut,
        associated_token::mint = config.mint_x,
        associated_token::authority = auth,
    )]
    pub vault_x: Box<InterfaceAccount<'info, TokenAccount>>,
    #[account(
        mut,
        associated_token::mint = config.mint_y,
        associated_token::authority = auth,
    )]
    pub vault_y: Box<InterfaceAccount<'info, TokenAccount>>,

    //User tokens
    #[account(
        mut,
        associated_token::mint = config.mint_x,
        associated_token::authority = user,
    )]
    pub user_x: Box<InterfaceAccount<'info, TokenAccount>>,
    #[account(
        mut,
        associated_token::mint = config.mint_y,
        associated_token::authority = user,
    )]
    pub user_y: Box<InterfaceAccount<'info, TokenAccount>>,
    // User LP tokens => get LP token back
    #[account(
        init_if_needed,
        payer = user,
        associated_token::mint = mint_lp,
        associated_token::authority = user,
    )]
    pub user_lp: Box<InterfaceAccount<'info, TokenAccount>>,

    // Auth PDA
    #[account(
        seeds = [b"auth", config.key().as_ref()],
        bump = config.auth_bump
    )]
    // Check: this is just for signature
    pub auth: UncheckedAccount<'info>,
    // TODO: remove authority from PDA seeds.
    #[account(
        has_one = mint_x,
        has_one = mint_y,
        seeds = [
            b"config",
            config.key().as_ref(),
            config.seed.to_le_bytes().as_ref()
        ],
        bump = config.config_bump
    )]
    pub config: Account<'info, Config>,
    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}
