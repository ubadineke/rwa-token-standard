use crate::constants::ASSET;
use crate::states::*;
use anchor_lang::prelude::*;
use anchor_lang::system_program::{create_account, CreateAccount};

use crate::utils::update_account_lamports_to_minimum_balance;
use anchor_spl::{
    token_2022::{
        initialize_mint2,
        spl_token_2022::{
            extension::{metadata_pointer::MetadataPointer, ExtensionType},
            pod::PodMint,
        },
        InitializeMint2,
    },
    token_interface::{
        metadata_pointer_initialize, token_metadata_initialize, MetadataPointerInitialize, Mint,
        Token2022, TokenMetadataInitialize,
    },
};

#[derive(Accounts)]
pub struct CreateAsset<'info> {
    #[account(mut)]
    authority: Signer<'info>,

    #[account(
        init,
        signer,
        payer = authority,
        mint::token_program = token_program,
        mint::decimals = 6,
        mint::authority = asset.key(),
        mint::freeze_authority = asset.key(),
        extensions::metadata_pointer::authority = asset.key(),
        extensions::metadata_pointer::metadata_address = mint.key(),
    )]
    mint: Box<InterfaceAccount<'info, Mint>>,

    #[account(
    init,
    payer = authority,
    space = 8 + Asset::INIT_SPACE,
    seeds = [ASSET.as_bytes(), mint.key().as_ref()],
    bump
  )]
    asset: Box<Account<'info, Asset>>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token2022>,
}

impl CreateAsset<'_> {
    pub fn handler(
        ctx: Context<CreateAsset>,
        name: String,
        symbol: String,
        uri: String,
    ) -> Result<()> {
        let asset = &mut ctx.accounts.asset;
        asset.authority = ctx.accounts.authority.key();
        asset.mint = ctx.accounts.mint.key();
        asset.status = true;
        // asset.delegate = delegate;

        // let mint = ctx.accounts.mint.key();

        let mint_key = ctx.accounts.mint.key();
        let mint_auth_signer_seeds: &[&[&[u8]]] =
            &[&[ASSET.as_bytes(), &mint_key.as_ref(), &[ctx.bumps.asset]]];

        ctx.accounts.initialize_token_metadata(
            &name,
            &symbol,
            &uri,
            mint_auth_signer_seeds,
        )?;

        ctx.accounts.mint.reload()?;

        // transfer minimum rent to mint account
        update_account_lamports_to_minimum_balance(
            ctx.accounts.mint.to_account_info(),
            ctx.accounts.authority.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
        )?;

        emit!(AssetMetadataEvent {
            mint: ctx.accounts.mint.key().to_string(),
            name: Some(name),
            symbol: Some(symbol),
            uri: Some(uri),
            decimals: Some(6)
        });

        Ok(())
    }

    fn initialize_token_metadata(
        &self,
        name: &String,
        symbol: &String,
        uri: &String,
        signer_seeds: &[&[&[u8]]],
    ) -> Result<()> {
        let cpi_accounts = TokenMetadataInitialize {
            program_id: self.token_program.to_account_info(),
            mint: self.mint.to_account_info(),
            metadata: self.mint.to_account_info(), // metadata account is the mint, since data is stored in mint
            mint_authority: self.asset.to_account_info(),
            update_authority: self.asset.to_account_info(),
        };
        let cpi_ctx = CpiContext::new_with_signer(
            self.token_program.to_account_info(),
            cpi_accounts,
            signer_seeds,
        );
        token_metadata_initialize(cpi_ctx, name.clone(), symbol.clone(), uri.clone(), )?;
        Ok(())
    }
}
