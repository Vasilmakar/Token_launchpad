use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, Token, TokenAccount},
    metadata::{
        CreateMetadataAccountsV3,
        create_metadata_accounts_v3,
        mpl_token_metadata::types::{DataV2, Creator as MetaCreator},
    },
};
use crate::errors::CustomError;
use crate::state::*;
use crate::TOKEN_METADATA_PROGRAM_ID;

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
    #[account(
        mut,
        seeds = [b"mint-auth", payer.key().as_ref()],
        bump
    )]
    pub mint_auth: AccountInfo<'info>,
    #[account(mut)]
    pub token_account: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
    /// CHECK: This is the token metadata program
    pub token_metadata_program: Program<'info, anchor_spl::metadata::Metadata>,  // Виправлено тип
    #[account(mut)]
    pub metadata: UncheckedAccount<'info>,
}



// === Args, які передаєш з клієнта ===
// #[derive(AnchorSerialize, AnchorDeserialize, Clone)]
// pub struct CreateMintArgs {
//     pub name: String,      // "My Token"
//     pub symbol: String,    // "MYT"
//     pub uri: String,       // https://.../metadata.json (Arweave/IPFS/http)
//     pub decimals: u8,      // 6 / 9 / 0 ...
//     pub initial_supply: u64, // у "мінтових" одиницях (з урахуванням decimals)
//     pub freeze_authority: bool, // true = PDA також freeze authority; false = без freeze
//     // pub recipient_is_payer: bool, // якщо хочеш карбувати на платника
// }
//
// #[derive(Accounts)]
// pub struct MintToATA<'info>{
//     #[account(mut)]
//     pub payer: Signer<'info>,
//
//     #[account(
//     mut,
//     token::mint = mint,
//     token::authority = escrow_authority_pda,
//     )]
//     pub escrow: Account<'info, TokenAccount>,
//
//     #[account(
//     seeds = [b"escrow-authority", mint.key().as_ref()],
//     bump
//     )]
//     pub escrow_authority_pda: UncheckedAccount<'info>,
//
//     #[account(mut)]
//     pub mint: Account<'info, Mint>,
//
//     #[account(
//     mut,
//     payer = payer,
//     associated_token::mint = mint,
//     associated_token::authority = payer,
//     )]
//     pub recipient: Account<'info, TokenAccount>,
//
//     pub system_program: Program<'info, System>,
//     pub token_program: Program<'info, Token>,
//     pub associated_token_program: Program<'info, AssociatedToken>,
//     pub rent: Sysvar<'info, Rent>,
//
// }
// === Accounts ===
// #[derive(Accounts)]
// #[instruction(args: CreateMintArgs)]
// pub struct CreateCustomMint<'info> {
//     /// Платник rent + ініціатор
//     #[account(mut)]
//     pub payer: Signer<'info>,
//
//     /// PDA, який буде mint_authority (і опційно freeze_authority)
//     /// seeds: ["mint-auth", payer]
//     /// Ми не створюємо окремий акаунт — просто підпис PDA.
//     /// Тримай як unchecked, бо це просто адреса для підпису.
//     /// Безпека: контроль через seeds і програму.
//     /// Якщо хочеш — збережи цей ключ у своєму state.
//     /// (або використай існуючий програмний PDA на sale)
//     ///
//     /// NB: Anchor не вимагає тут init, це just PDA-сигнер.
//     ///
//     /// Важливо: насіння має бути стабільним/визначеним у твоєму проєкті.
//     /// Нижче в handler ми згенеруємо seeds для signer'а.
//     ///
//     /// Якщо ти ведеш глобальний state — заміни на справжній Account<...>.
//     ///
//     /// Для простоти: UncheckedAccount.
//     #[account(
//         seeds = [b"mint-auth", payer.key().as_ref()],
//         bump
//     )]
//     /// CHECK: PDA signer only
//     pub mint_authority_pda: UncheckedAccount<'info>,
//
//     /// **Mint** акаунт (ініціалізуємо тут)
//     #[account(
//         init,
//         payer = payer,
//         mint::decimals = args.decimals,
//         mint::authority = mint_authority_pda,
//         mint::freeze_authority = if args.freeze_authority { Some(mint_authority_pda.key()) } else { None },
//     )]
//     pub mint: Account<'info, Mint>,
//     #[account(
//     init,
//     payer = payer,
//     token::mint = mint,
//     token::authority = escrow_authority_pda
//     )]
//     pub escrow: Account<'info, TokenAccount>,
//
//     #[account(
//     seeds = [b"escrow-authority", mint.key().as_ref()],
//     bump
//     )]
//     pub escrow_authority_pda: UncheckedAccount<'info>,
//     // /// Кому карбувати початковий обсяг
//     // /// Якщо args.recipient_is_payer = true — це має бути = payer.
//     // /// Інакше — будь-який інший власник гаманця.
//     // ///
//     // /// SystemAccount достатньо
//     // pub recipient: SystemAccount<'info>,
//
//     /// ATA отримувача під цей mint (створимо автоматично)
//     // #[account(
//     //     init,
//     //     payer = payer,
//     //     associated_token::mint = mint,
//     //     associated_token::authority = recipient
//     // )]
//     // pub recipient_ata: Account<'info, TokenAccount>,
//
//     /// Метадані (PDA Metaplex'а): ["metadata", metadata_program_id, mint]
//     /// Ми можемо обчислити адресу в handler'і, але зручніше передати готову,
//     /// щоб на клієнті перевірити, що вона очікувана.
//     ///
//     /// Для простоти — unchecked і перевірка в коді.
//     /// (Можна також перевіряти через find_program_address)
//     ///
//     /// CHECK: Metaplex will own it
//     #[account(mut)]
//     pub metadata: UncheckedAccount<'info>,
//     pub token_program: Program<'info, Token>,
//     pub associated_token_program: Program<'info, AssociatedToken>,
//     pub system_program: Program<'info, System>,
//     /// CHECK: офіційний метапрограма
//     #[account(address = TOKEN_METADATA_PROGRAM_ID)]
//     pub token_metadata_program: UncheckedAccount<'info>,
//     pub rent: Sysvar<'info, Rent>,
// }
//
// pub fn mint_to_recipient(ctx:Context<MintToATA>, amount: u64) ->Result<()>{
//     let payer = &mut ctx.accounts.payer;
//     let escrow = &mut ctx.accounts.escrow;
//     let recipient = &mut ctx.accounts.recipient;
//
//     let seeds = &[
//         b"escrow-authority",
//         ctx.accounts.mint.key().as_ref(),
//         &[ctx.bumps.escrow_authority_pda],
//     ];
//
//     let signer = &[&seeds[..]];
//
//     let ctx_accounts = token::Transfer{
//         from: escrow.to_account_info(),
//         to: recipient.to_account_info(),
//         authority: ctx.accounts.escrow_authority_pda.to_account_info()
//     };
//     let cpi_program = ctx.accounts.token_program.to_account_info();
//     let cpi_cpx = CpiContext::new_with_signer(cpi_program, ctx_accounts, signer);
//     token::transfer(cpi_cpx, amount)?;
//     Ok(())
// }
//
// pub fn handler(ctx: Context<CreateCustomMint>, args: CreateMintArgs) -> Result<()> {
//     let payer = &ctx.accounts.payer;
//     let mint = &ctx.accounts.mint;
//     // let recipient = &ctx.accounts.recipient;
//     // let recipient_ata = &ctx.accounts.recipient_ata;
//
//     // --- 1) Перевірки прості/бізнес-логіка ---
//     // if args.recipient_is_payer {
//     //     require_keys_eq!(recipient.key(), payer.key(), CustomError::RecipientMustBePayer);
//     // }
//
//     // --- 2) Створення METADATA через CPI до mpl-token-metadata ---
//     // Derive і перевір адреси (не обов'язково, але корисно)
//     let (metadata_pda, _bump_md) = Pubkey::find_program_address(
//         &[
//             b"metadata",
//             &TOKEN_METADATA_PROGRAM_ID.to_bytes(),
//             &mint.key().to_bytes(),
//         ],
//         &TOKEN_METADATA_PROGRAM_ID,
//     );
//     require_keys_eq!(metadata_pda, ctx.accounts.metadata.key(), CustomError::MetadataPdaMismatch);
//
//     // DataV2: базові поля. Можеш додати "creators", "collection", "uses" за потреби.
//     let data_v2 = DataV2 {
//         name: args.name.clone(),
//         symbol: args.symbol.clone(),
//         uri: args.uri.clone(),
//         seller_fee_basis_points: 0, // для fungible зазвичай 0
//         creators: None,             // або Some(vec![...]) якщо треба
//         collection: None,
//         uses: None,
//     };
//
//     // CPI інструкція створення Metadata
//     // Власник mint'a (authority) нам не потрібен тут як підписант — це mint_authority_pda для minting.
//     // Для create_metadata_accounts_v3 підписує payer (update_authority = payer за замовч.)
//     // let ix = create_metadata_accounts_v3(
//     //     TOKEN_METADATA_PROGRAM_ID,
//     //     metadata_pda,
//     //     mint.key(),
//     //     ctx.accounts.mint_authority_pda.key(), // mint_authority (може бути будь-яким; для перевірок метаплексу)
//     //     payer.key(),                            // payer
//     //     payer.key(),                            // update_authority (зазвичай власник/DAO)
//     //     data_v2,
//     //     true,   // is_mutable
//     //     true,   // update_authority_is_signer
//     //     None,   // collection_details
//     // );
//
//     // Виклик CPI
//     let cpi_accounts = CreateMetadataAccountsV3 {
//         metadata: ctx.accounts.metadata.to_account_info(),
//         mint: ctx.accounts.mint.to_account_info(),
//         mint_authority: ctx.accounts.mint_authority_pda.to_account_info(),
//         payer: ctx.accounts.payer.to_account_info(),
//         update_authority: ctx.accounts.payer.to_account_info(),
//         system_program: ctx.accounts.system_program.to_account_info(),
//         rent: ctx.accounts.rent.to_account_info(),
//     };
//
//     let cpi_ctx = CpiContext::new(
//         ctx.accounts.token_metadata_program.to_account_info(),
//         cpi_accounts,
//     );
//
//     create_metadata_accounts_v3(
//         cpi_ctx,
//         data_v2,
//         true,  // is_mutable
//         true,  // update_authority_is_signer
//         None,  // collection_details
//     )?;
//
//
//     // anchor_lang::solana_program::program::invoke_signed(
//     //     &ix,
//     //     &[
//     //         ctx.accounts.metadata.to_account_info(),
//     //         mint.to_account_info(),
//     //         ctx.accounts.mint_authority_pda.to_account_info(),
//     //         payer.to_account_info(),
//     //         payer.to_account_info(),
//     //         ctx.accounts.system_program.to_account_info(),
//     //         ctx.accounts.rent.to_account_info(),
//     //     ],
//     //     &[], // тут не потрібен signer seeds, підписує payer
//     // )?;
//
//     // --- 3) Карбування початкового обсягу (якщо > 0) ---
//     if args.initial_supply > 0 {
//         // seeds для PDA-підпису (mint_authority)
//         let seeds: &[&[u8]] = &[
//             b"mint-auth",
//             payer.key.as_ref(),
//         ];
//         let (pda, bump) = Pubkey::find_program_address(&[b"mint-auth", payer.key.as_ref()], ctx.program_id);
//         require_keys_eq!(pda, ctx.accounts.mint_authority_pda.key(), CustomError::PdaMismatch);
//         // let signer = &[&[b"mint-auth", payer.key.as_ref(), &[bump]][..]];
//
//         // CPI mint_to
//     //     let cpi_ctx = CpiContext::new_with_signer(
//     //         ctx.accounts.token_program.to_account_info(),
//     //         MintTo {
//     //             mint: mint.to_account_info(),
//     //             to: recipient_ata.to_account_info(),
//     //             authority: ctx.accounts.mint_authority_pda.to_account_info(),
//     //         },
//     //         signer,
//     //     );
//     //     token::mint_to(cpi_ctx, args.initial_supply)?;
//      }
//
//     Ok(())
// }
//
// // === Помилки для читабельності ===
//
// #[error_code]
// pub enum CustomError {
//     #[msg("Recipient must be the payer as requested.")]
//     RecipientMustBePayer,
//     #[msg("Metadata PDA mismatch.")]
//     MetadataPdaMismatch,
//     #[msg("Mint authority PDA mismatch.")]
//     PdaMismatch,
// }



