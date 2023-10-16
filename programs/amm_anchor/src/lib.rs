use anchor_lang::prelude::*;

mod constants;
mod contexts;
mod errors;
mod helpers;
mod state;

use contexts::*;
declare_id!("58hiw2qGCoDbqCiKvhzB9eiX3JkL8D6Q1BNUvJpTbKgN");

#[program]
pub mod amm_anchor {
    use super::*;

    pub fn initialize(
        ctx: Context<Initialize>,
        seed: u64,
        fee: u16,
        has_authority: bool,
    ) -> Result<()> {
        ctx.accounts.init(ctx.bumps, seed, fee, has_authority)
    }

    pub fn update(ctx: Context<Update>, locked: bool) -> Result<()> {
        ctx.accounts.update(locked);
        unimplemented!()
    }

    pub fn deposit(
        ctx: Context<Deposit>,
        amount: u64,
        max_x: u64,
        max_y: u64,
        expiration: i64,
    ) -> Result<()> {
        ctx.accounts.deposit(amount, max_x, max_y, expiration);
    }
}
