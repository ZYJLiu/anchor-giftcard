use anchor_lang::{prelude::*, solana_program::program::invoke_signed};
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{self, mint_to, Mint, MintTo, Token, TokenAccount},
};
use mpl_token_metadata::{
    instruction::{
        create_master_edition_v3, create_metadata_accounts_v3, sign_metadata,
        update_metadata_accounts_v2, verify_sized_collection_item,
    },
    pda::{find_master_edition_account, find_metadata_account},
    state::{Collection, CollectionDetails, Creator, DataV2, Metadata, TokenMetadataAccount},
    ID as MetadataID,
};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

const GIFTCARD_SEED: &str = "GIFTCARD";

#[program]
pub mod nft_giftcard {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        ctx.accounts.giftcard_state.giftcard_creator = ctx.accounts.giftcard_creator.key();
        ctx.accounts.giftcard_state.payment_destination = ctx.accounts.payment_destination.key();
        ctx.accounts.giftcard_state.gift_token_mint = ctx.accounts.gift_token_mint.key();
        ctx.accounts.giftcard_state.gift_token_mint_bump =
            *ctx.bumps.get("gift_token_mint").unwrap();
        Ok(())
    }

    // Todo
    pub fn create_giftcard(ctx: Context<CreateGiftcard>) -> Result<()> {
        // mint nft
        // mint tokens
        // transfer payment
        Ok(())
    }

    // mint nft reference
    pub fn nft(ctx: Context<Nft>, uri: String, name: String, symbol: String) -> Result<()> {
        let seeds = &["auth".as_bytes(), &[*ctx.bumps.get("auth").unwrap()]];
        let signer = [&seeds[..]];

        mint_to(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                MintTo {
                    authority: ctx.accounts.auth.to_account_info(),
                    to: ctx.accounts.token_account.to_account_info(),
                    mint: ctx.accounts.mint.to_account_info(),
                },
                &signer,
            ),
            1, // only 1 token minted
        )?;

        let account_info = vec![
            ctx.accounts.metadata.to_account_info(),
            ctx.accounts.mint.to_account_info(),
            ctx.accounts.auth.to_account_info(),
            ctx.accounts.payer.to_account_info(),
            ctx.accounts.token_metadata_program.to_account_info(),
            ctx.accounts.token_program.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
            ctx.accounts.rent.to_account_info(),
        ];

        let creator = vec![Creator {
            address: ctx.accounts.auth.key(),
            verified: false,
            share: 100,
        }];

        let collection_details = CollectionDetails::V1 { size: 0 };

        invoke_signed(
            &create_metadata_accounts_v3(
                ctx.accounts.token_metadata_program.key(), // token metadata program
                ctx.accounts.metadata.key(),               // metadata account PDA for mint
                ctx.accounts.mint.key(),                   // mint account
                ctx.accounts.auth.key(),                   // mint authority
                ctx.accounts.payer.key(),                  // payer for transaction
                ctx.accounts.auth.key(),                   // update authority
                name,                                      // name
                symbol,                                    // symbol
                uri,                                       // nft uri (offchain metadata)
                Some(creator),                             // (optional) creators
                0,                                         // seller free basis points
                true,                                      // (bool) update authority is signer
                true,                                      // (bool)is mutable
                None,                                      // (optional) collection
                None,                                      // (optional) uses
                Some(collection_details),                  // (optional) collection details
            ),
            account_info.as_slice(),
            &signer,
        )?;

        let master_edition_infos = vec![
            ctx.accounts.master_edition.to_account_info(),
            ctx.accounts.mint.to_account_info(),
            ctx.accounts.auth.to_account_info(),
            ctx.accounts.payer.to_account_info(),
            ctx.accounts.metadata.to_account_info(),
            ctx.accounts.token_metadata_program.to_account_info(),
            ctx.accounts.token_program.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
            ctx.accounts.rent.to_account_info(),
        ];

        invoke_signed(
            &create_master_edition_v3(
                ctx.accounts.token_metadata_program.key(), // token metadata program
                ctx.accounts.master_edition.key(),         // master edition account PDA
                ctx.accounts.mint.key(),                   // mint account
                ctx.accounts.auth.key(),                   // update authority
                ctx.accounts.auth.key(),                   // mint authority
                ctx.accounts.metadata.key(),               // metadata account
                ctx.accounts.payer.key(),                  //payer
                Some(0),                                   // (optional) max supply
            ),
            master_edition_infos.as_slice(),
            &signer,
        )?;

        let sign_metadata_info = vec![
            ctx.accounts.metadata.to_account_info(),
            ctx.accounts.auth.to_account_info(),
        ];

        invoke_signed(
            &sign_metadata(
                ctx.accounts.token_metadata_program.key(), // token metadata program
                ctx.accounts.metadata.key(),               // metadata account
                ctx.accounts.auth.key(),                   // collection update authority
            ),
            sign_metadata_info.as_slice(),
            &signer,
        )?;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreateGiftcard<'info> {
    pub giftcard_state: Account<'info, GiftcardState>,
    pub payment_destination: Account<'info, TokenAccount>,
    pub payment_source: Account<'info, TokenAccount>,
    #[account(mut)]
    pub giftcard_buyer: Signer<'info>,
}

#[derive(Accounts)]
pub struct Nft<'info> {
    #[account(
        init,
        payer = payer,
        mint::decimals = 0,
        mint::authority = auth,
        mint::freeze_authority = auth
    )]
    pub mint: Account<'info, Mint>,
    /// CHECK: metadata account
    #[account(
        mut,
        address=find_metadata_account(&mint.key()).0
    )]
    pub metadata: UncheckedAccount<'info>,
    /// CHECK: master edition account
    #[account(
        mut,
        address=find_master_edition_account(&mint.key()).0
    )]
    pub master_edition: UncheckedAccount<'info>,
    /// CHECK: mint authority
    #[account(
        mut,
        seeds = ["auth".as_bytes().as_ref()],
        bump,
    )]
    pub auth: UncheckedAccount<'info>,
    #[account(
        init,
        payer = payer,
        associated_token::mint = mint,
        associated_token::authority = user
    )]
    pub token_account: Account<'info, TokenAccount>,
    /// CHECK: user receiving mint
    pub user: UncheckedAccount<'info>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
    pub token_metadata_program: Program<'info, TokenMetaData>,
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        seeds = [GIFTCARD_SEED.as_bytes(), giftcard_creator.key().as_ref()],
        bump,
        payer = giftcard_creator,
        space = GiftcardState::LEN
    )]
    pub giftcard_state: Account<'info, GiftcardState>,
    // TODO: add check for mint
    pub payment_destination: Account<'info, TokenAccount>,
    #[account(
        init,
        seeds = [giftcard_state.key().as_ref()],
        bump,
        payer = giftcard_creator,
        mint::decimals = 6,
        mint::authority = giftcard_state,

    )]
    pub gift_token_mint: Account<'info, Mint>,
    #[account(mut)]
    pub giftcard_creator: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
}

#[account]
pub struct GiftcardState {
    pub giftcard_creator: Pubkey,    // 32
    pub payment_destination: Pubkey, //32
    pub gift_token_mint: Pubkey,     // 32
    pub gift_token_mint_bump: u8,    // 1
}

impl GiftcardState {
    pub const LEN: usize = 8 + 32 + 32 + 32 + 8;
}

#[derive(Clone)]
pub struct TokenMetaData;
impl anchor_lang::Id for TokenMetaData {
    fn id() -> Pubkey {
        MetadataID
    }
}
