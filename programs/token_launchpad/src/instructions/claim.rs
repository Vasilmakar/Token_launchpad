use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{self, Mint, Token, TokenAccount, MintTo},
};
use crate::errors::CustomError;
use crate::state::{Sale, User};

pub fn claim(ctx: Context<ClaimStruct>) -> Result<()> {
    let clock = Clock::get()?;
    require!(clock.unix_timestamp >= ctx.accounts.sale.start_time, CustomError::SaleNotStarted);
    require!(clock.unix_timestamp <= ctx.accounts.sale.end_time, CustomError::SaleEnded);

    let amount = ctx.accounts.user_account.token_to_claim;
    let seeds = &[
        b"mint-auth",
        ctx.accounts.payer.key.as_ref(),
        &[ctx.bumps.mint_auth],
    ];
    let signer = &[&seeds[..]];

    let cpi_accounts = MintTo {
        mint: ctx.accounts.mint.to_account_info(),
        to: ctx.accounts.associated_token_recipient.to_account_info(),
        authority: ctx.accounts.mint_auth.to_account_info(),
    };
    let cpi_ctx = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        cpi_accounts,
        signer,
    );
    token::mint_to(cpi_ctx, amount)?;

    ctx.accounts.user_account.tokens_claimed = ctx.accounts.user_account
        .tokens_claimed
        .checked_add(amount)
        .ok_or(CustomError::Overflow)?;
    ctx.accounts.user_account.token_to_claim = 0;

    Ok(())
}

#[derive(Accounts)]
pub struct ClaimStruct<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(mut)]
    pub mint: Account<'info, Mint>,
    #[account(mut)]
    pub associated_token_recipient: Account<'info, TokenAccount>,
    #[account(
        seeds = [b"mint-auth"],
        bump
    )]
    pub mint_auth: UncheckedAccount<'info>,
    #[account(
        mut,
        seeds = [b"sale", payer.key().as_ref()],
        bump
    )]
    pub sale: Account<'info, Sale>,
    #[account(mut)]
    pub user_account: Account<'info, User>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}