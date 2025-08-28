// src/error.rs
use anchor_lang::prelude::*;
#[error_code]
pub enum CustomError {
    #[msg("Payer is not authority of mint")]
    ErrorWithPayer,
    #[msg("PDA of user is wrong")]
    ErrorWithPDA,
    #[msg("Invalid mint")]
    InvalidMint,
    #[msg("Overflow")]
    Overflow,
    #[msg("Sale not started")]
    SaleNotStarted,
    #[msg("Sale ended")]
    SaleEnded,
    #[msg("URI is too long (max 200 characters)")]
    UriTooLong,
    #[msg("URI is empty")]
    UriEmpty,
    #[msg("Invalid URI format")]
    InvalidUriFormat,
    #[msg("Name is too long")]
    BoundsNameError,
    #[msg("Symbol is not valid")]
    ErrorWithSymbol,
    #[msg("Wrong PDA")]
    MetadataPdaMismatch,
    #[msg("Name is empty")]
    NameEmpty,
    #[msg("Invalid name format")]
    InvalidNameFormat,
    #[msg("Symbol is empty")]
    SymbolEmpty,
    #[msg("Invalid symbol format")]
    InvalidSymbolFormat,
    #[msg("No creators specified for non-zero royalty")]
    NoCreatorsForRoyalty,
    #[msg("Creator shares must sum to 100")]
    InvalidCreatorShares,
    #[msg("Invalid royalty amount")]
    InvalidRoyalty,
    #[msg("Too many creators (max 5)")]
    TooManyCreators,
    #[msg("Non-zero royalty requires creators")]
    NoRoyaltyForCreators,
}
