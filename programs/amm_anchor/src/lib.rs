use anchor_lang::prelude::*;

mod constants;
mod contexts;
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
        authority: Option<Pubkey>,
    ) -> Result<()> {
        Ok(())
    }
}
