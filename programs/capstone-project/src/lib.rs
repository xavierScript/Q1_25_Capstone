use anchor_lang::prelude::*;

mod state;
mod instructions;
mod errors;

use instructions::*;

declare_id!("8D3kN9okU1Ck2bxyHp9ZzKhTmNiwJ1BAuaZyUxCanxrB");

#[program]
pub mod escrow {
    use super::*;

    pub fn make(ctx: Context<Make>, seed: u64, deposit: u64, receive: u64) -> Result<()> {
        ctx.accounts.init_escrow(seed, receive, &ctx.bumps)?;
        ctx.accounts.deposit(deposit)?;
        Ok(())
    }

    pub fn refund(ctx: Context<Refund>) -> Result<()> {
        ctx.accounts.refund_and_close_vault()?;
        Ok(())
    }
pub fn take(ctx: Context<Take>) -> Result<()> {
        ctx.accounts.withdraw_and_close_vault()?;
        Ok(())
    }
}