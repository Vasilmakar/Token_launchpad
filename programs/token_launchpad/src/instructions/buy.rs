use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{self, Mint, Token, TokenAccount},
};
use crate::errors::CustomError;
use crate::state::{Sale, User};

pub fn buy(ctx: Context<BuyStruct>, amount: u64, _program_id: &Pubkey) -> Result<()> {
    let total_tokens = ctx.accounts.sale.total_tokens;
    require!(total_tokens >= amount, CustomError::InvalidCreatorShares);

    // if ctx.accounts.token_user_account.data_is_empty() {
    //     anchor_spl::associated_token::create(
    //         CpiContext::new(
    //             ctx.accounts.associated_token_program.to_account_info(),
    //             anchor_spl::associated_token::Create {
    //                 payer: ctx.accounts.payer.to_account_info(),
    //                 associated_token: ctx.accounts.token_user_account.to_account_info(),
    //                 authority: ctx.accounts.payer.to_account_info(),
    //                 mint: ctx.accounts.mint.to_account_info(),
    //                 system_program: ctx.accounts.system_program.to_account_info(),
    //                 token_program: ctx.accounts.token_program.to_account_info(),
    //             },
    //         ),
    //     )?;
    // }

    ctx.accounts.sale.total_tokens = ctx.accounts.sale
        .total_tokens
        .checked_sub(amount)
        .ok_or(CustomError::Overflow)?;
    ctx.accounts.sale.sold_tokens = ctx.accounts.sale
        .sold_tokens
        .checked_add(amount)
        .ok_or(CustomError::Overflow)?;

    // if ctx.accounts.user_account.data_is_empty() {
    //     ctx.accounts.user_account.set_inner(User {
    //         tokens_claimed: 0,
    //         token_to_claim: amount,
    //         address: ctx.accounts.payer.key(),
    //         token_account: ctx.accounts.token_user_account.key(),
    //     });
    // } else {
    //     ctx.accounts.user_account.token_to_claim = ctx.accounts.user_account
    //         .token_to_claim
    //         .checked_add(amount)
    //         .ok_or(CustomError::Overflow)?;
    // }
    ctx.accounts.user_account.tokens_claimed = 0;
    ctx.accounts.user_account.token_to_claim = 0;
    ctx.accounts.user_account.address = ctx.accounts.payer.key();
    ctx.accounts.user_account.token_account = ctx.accounts.token_user_account.key();

    Ok(())
}

#[derive(Accounts)]
#[instruction(amount: u64)]
pub struct BuyStruct<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(mut)]
    pub mint: Account<'info, Mint>,
    #[account(
        mut,
        seeds = [b"mint-auth"],
        bump
    )]
    pub mint_authority: AccountInfo<'info>,
    #[account(mut)]
    pub sale: Account<'info, Sale>,
    pub token_program: Program<'info, Token>,
    #[account(
    init,
    payer = payer,
    associated_token::mint = mint,
    associated_token::authority = payer
)]
    pub token_user_account: Account<'info, TokenAccount>,
    #[account(
        init,
        payer = payer,
        space = 8 + 32 + 8 + 8 + 32,  // Переконайтеся, що розмір відповідає структурі User
        seeds = [b"user", payer.key().as_ref()],
        bump
    )]
    pub user_account: Account<'info, User>,  // Повернули до Account
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

 