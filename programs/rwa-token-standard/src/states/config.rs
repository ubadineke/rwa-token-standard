use anchor_lang::prelude::*;

#[account]
pub struct Config {
    asset_count: u64,
}
