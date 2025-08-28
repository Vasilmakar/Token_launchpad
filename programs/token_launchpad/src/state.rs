use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, TokenAccount};

use anchor_lang::prelude::*;

#[account]
pub struct User {
    pub tokens_claimed: u64,
    pub token_to_claim: u64,
    pub address: Pubkey,
    pub token_account: Pubkey,
}

impl User {
    pub fn space() -> usize {
        8 + 8 + 8 + 32 + 32  // Дискримінатор + u64 + u64 + Pubkey + Pubkey
    }
}

#[account]
pub struct Sale {
    pub authority: Pubkey,
    pub token_mint: Pubkey,
    pub price: u64,
    pub start_time: i64,
    pub end_time: i64,
    pub total_tokens: u64,
    pub sold_tokens: u64,
    pub bump: u8,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct Creator {
    pub address: Pubkey,
    pub verified: bool,
    pub share: u8,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct CreateMintStruct {
    pub name: String,
    pub symbol: String,
    pub uri: String,
    pub decimals: u8,
    pub total_supply: u64,
    pub authority_freeze: Pubkey,
    pub recipient_payer: bool,
    pub creators: Option<Vec<Creator>>,
    pub seller_fee_basis_points: Option<u16>,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct InitSaleArgs {
    pub price: u64,
    pub start_time: i64,
    pub end_time: i64,
    pub total_tokens: u64,
}