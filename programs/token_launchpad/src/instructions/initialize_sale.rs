use anchor_lang::prelude::*;
use crate::errors::CustomError;
use crate::state::{InitSaleArgs, Sale};
use anchor_spl::token::Mint;
pub fn initialize_sale(ctx: Context<InitializeSale>, args: InitSaleArgs) -> Result<()> {
    let payer = &mut ctx.accounts.payer;
    let (sale_pda, _bump) = Pubkey::find_program_address(
        &[b"sale", payer.key().as_ref()],
        ctx.program_id,
    );
    require_keys_eq!(sale_pda, ctx.accounts.sale.key(), CustomError::ErrorWithPayer);

    ctx.accounts.sale.token_mint = ctx.accounts.mint.key();
    ctx.accounts.sale.authority = payer.key();
    ctx.accounts.sale.total_tokens = args.total_tokens;
    ctx.accounts.sale.start_time = args.start_time;
    ctx.accounts.sale.end_time = args.end_time;
    ctx.accounts.sale.price = args.price;
    ctx.accounts.sale.sold_tokens = 0;
    ctx.accounts.sale.bump = ctx.bumps.sale;

    Ok(())
}

#[derive(Accounts)]
pub struct InitializeSale<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(mut)]
    pub mint: Account<'info, Mint>,
    #[account(
        init,
        payer = payer,
        space = 8 + 32 + 32 + 8 + 8 + 8 + 8 + 8 + 1,
        seeds = [b"sale", payer.key().as_ref()],
        bump
    )]
    pub sale: Account<'info, Sale>,
    pub system_program: Program<'info, System>,
}