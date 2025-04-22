//owner(authority)
//type
//policies
//status
//asset mint
//delegate
use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Asset {
    //creator
    pub authority: Pubkey,
    //asset mint
    pub mint: Pubkey,
    //active or disabled
    pub status: bool,
    //delegate, carry out instructions on behalf of the creator
    pub delegate: Option<Pubkey>,

    pub bump: u8,
}

#[derive(AnchorSerialize, AnchorDeserialize, Debug, Clone)]
pub struct CreateAssetParams {
    pub name: String,
    pub symbol: String,
    pub uri: String,
    pub delegate: Option<String>,
}

#[event]
pub struct AssetMetadataEvent {
    pub mint: String,
    pub name: Option<String>,
    pub symbol: Option<String>,
    pub uri: Option<String>,
    pub decimals: Option<u8>,
}
