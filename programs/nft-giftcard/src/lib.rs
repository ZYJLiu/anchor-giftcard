use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{self, Mint, MintTo, Token, TokenAccount},
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
