use anchor_lang::prelude::*;

declare_id!("61WSjewfxM2gcD5dfgDXSCJB1La8zzam4tQi83yKjpFA");

#[program]
pub mod solana_dapp_with_react {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
