use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Escrow {
    pub seed: u64,
    pub maker: Pubkey,
    // pub taker: Pubkey, // Store the eligible taker (student in this case)
    // pub taker: Pubkey,
    pub mint_a: Pubkey,
    pub receive: u64,
    pub bump: u8,
}