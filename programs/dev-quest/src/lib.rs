use anchor_lang::prelude::*;

mod instructions;
mod state;

pub use instructions::*;

declare_id!("82g3uXx38QDULhTHLqRq24A9ZjM38cFjzYJAY8dhVPrf");

#[program]
pub mod dev_quest {
    use super::*;

    pub fn initialize_config(ctx: Context<InitializeConfig>) -> Result<()> {
        ctx.accounts.initialize_config(&ctx.bumps)
    }

    pub fn initialize_user(ctx: Context<InitializeUser>) -> Result<()> {
        ctx.accounts.initialize_user(&ctx.bumps)
    }
}
