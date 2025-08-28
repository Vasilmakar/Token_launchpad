use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{self, Mint, MintTo, Token, TokenAccount},
    metadata::{
        create_metadata_accounts_v3,
        mpl_token_metadata::types::DataV2,  // DataV2 у модулі types
        CreateMetadataAccountsV3,
        Metadata as Metaplex,  // Аліас для програми Metaplex
    }
};

// pub fn treasure(ctx:Context<Treasure>) ->Result<()>{
//     msg!("Treasure succesfuly initialized");
//     Ok(())
// }
// #[derive(Accounts)]
// pub struct Treasure<'info>{
//     #[account(mut)]
//     pub payer: Signer<'info>,
//     #[account(mut)]
//     pub mint: Account<'info, Mint>,
//     #[account(
//     init,
//     associated_token::mint = mint,
//     associated_token::authority = payer
//     )]
//     pub treasury: Account<'info, TokenAccount>,
//     pub system_program: Program<'info, System>,
// }