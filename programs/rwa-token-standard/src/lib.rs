use crate::instructions::*;
use crate::states::*;
use anchor_lang::prelude::*;

pub mod constants;
pub mod instructions;
pub mod states;
pub mod utils;

declare_id!("B56D3RPexpgAwgSbPV9Upwnu1wKN25eChYMB9AhuL9Nb");

#[program]
pub mod rwa_token_standard {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }

    pub fn create_asset(ctx: Context<CreateAsset>, params: CreateAssetParams) -> Result<()> {
        CreateAsset::handler(ctx, params)?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
