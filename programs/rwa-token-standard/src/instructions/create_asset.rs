use crate::constants::ASSET;
use crate::states::*;
use anchor_lang::prelude::*;
use anchor_spl::associated_token::Create;
use anchor_spl::metadata::Metadata;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_2022::spl_token_2022::extension::{
        interest_bearing_mint::InterestBearingConfig, ExtensionType,
    },
    token_interface::{
        get_mint_extension_data, token_metadata_initialize, Mint, Token2022,
        TokenMetadataInitialize,
    },
};

#[derive(Accounts)]
pub struct CreateAsset<'info> {
    #[account(mut)]
    authority: Signer<'info>,

    #[account(mut)]
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
    pub fn handler(ctx: Context<CreateAsset>, params: CreateAssetParams) -> Result<()> {
        let mint = ctx.accounts.mint.key();

        let mint_key = ctx.accounts.mint.key();
        let mint_auth_signer_seeds: &[&[&[u8]]] =
            &[&[ASSET.as_bytes(), &mint_key.as_ref(), &[ctx.bumps.asset]]];

        let asset = &mut ctx.accounts.asset;
        ctx.accounts
            .initialize_token_metadata(&params, mint_auth_signer_seeds)?;

        emit!(AssetMetadataEvent {
            mint: ctx.accounts.mint.key().to_string(),
            name: Some(params.name),
            symbol: Some(params.symbol),
            uri: Some(params.uri),
            decimals: Some(ctx.accounts.mint.decimals)
        });

        Ok(())
    }

    fn initialize_token_metadata(
        &self,
        params: &CreateAssetParams,
        signer_seeds: &[&[&[u8]]],
    ) -> Result<()> {
        let cpi_accounts = TokenMetadataInitialize {
            program_id: self.token_program.to_account_info(),
            mint: self.mint.to_account_info(),
            metadata: self.mint.to_account_info(), // metadata account is the mint, since data is stored in mint
            mint_authority: self.authority.to_account_info(),
            update_authority: self.authority.to_account_info(),
        };
        let cpi_ctx = CpiContext::new_with_signer(
            self.token_program.to_account_info(),
            cpi_accounts,
            signer_seeds,
        );
        token_metadata_initialize(
            cpi_ctx,
            params.name.clone(),
            params.symbol.clone(),
            params.uri.clone(),
        )?;
        Ok(())
    }
}
