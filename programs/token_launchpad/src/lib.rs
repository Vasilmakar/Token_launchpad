use anchor_lang::prelude::*;
use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{self, Mint, MintTo, Token, TokenAccount},
    metadata::{
        CreateMetadataAccountsV3,
        create_metadata_accounts_v3,
        mpl_token_metadata::types::{DataV2, Creator as MetaCreator},
    },
};

pub mod errors;
use crate::errors::CustomError;
pub const TOKEN_METADATA_PROGRAM_ID: Pubkey = pubkey!("metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s");

declare_id!("AcRe6mmG9TLciJfNVkxuSwnHGcex13KwaMdoCP9gYvba");

#[program]
pub mod token_launchpad {
    use super::*;

    pub fn create_custom_mint(ctx: Context<NewCustomMint>, args: CreateMintStruct) -> Result<()> {
        create_mint(ctx, args)
    }

    pub fn init_sale(ctx: Context<InitializeSale>, args: InitSaleArgs) -> Result<()> {
        initialize_sale(ctx, args)
    }

    pub fn buy_token(ctx: Context<BuyStruct>, amount: u64) -> Result<()> {
        let pid = ctx.program_id;
        buy(ctx, amount, pid)
    }

    pub fn claim_token(ctx: Context<ClaimStruct>) -> Result<()> {
        claim(ctx)
    }
}
//buy 
pub fn buy(ctx: Context<BuyStruct>, amount: u64, _program_id: &Pubkey) -> Result<()> {
    let total_tokens = ctx.accounts.sale.total_tokens;
    require!(total_tokens >= amount, CustomError::InvalidCreatorShares);


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

//initialize
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
//create mint 
pub fn create_mint(ctx: Context<NewCustomMint>, args: CreateMintStruct) -> Result<()> {
    let cpi_accounts = CreateMetadataAccountsV3 {
        metadata: ctx.accounts.metadata.to_account_info(),
        mint: ctx.accounts.mint.to_account_info(),
        mint_authority: ctx.accounts.mint_auth.to_account_info(),
        payer: ctx.accounts.payer.to_account_info(),
        update_authority: ctx.accounts.payer.to_account_info(),
        system_program: ctx.accounts.system_program.to_account_info(),
        rent: ctx.accounts.rent.to_account_info(),
    };
    let cpi_program = ctx.accounts.token_metadata_program.to_account_info();
    let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
    let data = DataV2 {
        name: args.name,
        symbol: args.symbol,
        uri: args.uri,
        seller_fee_basis_points: 0,
        creators: Some(vec![MetaCreator {
            address: ctx.accounts.payer.key(),
            verified: false,
            share: 100,
        }]),
        collection: None,
        uses: None,
    };
    create_metadata_accounts_v3(cpi_ctx, data, true, true, None)?;

    // let seeds = &[
    //     b"mint-auth",
    //     ctx.accounts.payer.key().as_ref(),
    //     &[ctx.bumps.mint_auth],
    // ];
    // let signer = &[&seeds[..]];
    // let cpi_accounts = anchor_spl::token::MintTo {
    //     mint: ctx.accounts.mint.to_account_info(),
    //     to: ctx.accounts.token_account.to_account_info(),
    //     authority: ctx.accounts.mint.mint_authority.to_account_info(),
    // };
    // let cpi_ctx = CpiContext::new_with_signer(
    //     ctx.accounts.token_program.to_account_info(),
    //     cpi_accounts,
    //     signer,
    // );
    // anchor_spl::token::mint_to(cpi_ctx, args.amount)?;

    Ok(())
}

#[derive(Accounts)]
#[instruction(args: CreateMintStruct)]
pub struct NewCustomMint<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(
        init,
        payer = payer,
        mint::decimals = 6,
        mint::authority = mint_auth,
        seeds = [b"mint", payer.key().as_ref()],
        bump
    )]
    pub mint: Account<'info, Mint>,
    ///CHECK: That account is safe
    #[account(
        mut,
        seeds = [b"mint-auth", payer.key().as_ref()],
        bump
    )]
    pub mint_auth: UncheckedAccount<'info>,
    /// CHECK: This is the SPL Token program
    #[account(address = token::ID)]
    pub token_program: AccountInfo<'info>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
    /// CHECK: This is the token metadata program
    pub token_metadata_program: Program<'info, anchor_spl::metadata::Metadata>,  // Виправлено тип
    #[account(mut)]
    pub metadata: UncheckedAccount<'info>,
}

//claim 
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

//accounts 
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