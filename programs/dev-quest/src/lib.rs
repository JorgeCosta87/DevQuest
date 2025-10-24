use anchor_lang::prelude::*;

mod errors;
mod instructions;
mod state;

pub use instructions::*;

declare_id!("G6kyX8C369qABor4ZVXDFXneZw9jqDpbfB79imLgEGfA");

#[program]
pub mod dev_quest {
    
    use super::*;

    pub fn initialize_config(ctx: Context<InitializeConfig>) -> Result<()> {
        ctx.accounts.initialize_config(&ctx.bumps)
    }

    pub fn initialize_user(
        ctx: Context<InitializeUser>,
        name: String,
        bio: String,
        github_username: String,
    ) -> Result<()> {
        ctx.accounts.initialize_user(name, bio, github_username, &ctx.bumps)
    }
    pub fn create_task(
        ctx: Context<CreateTask>,
        tittle: String,
        description: String,
        dificulty: String,
        points_reward: u32,
        is_active: bool,
    ) -> Result<()> {
        ctx.accounts
            .create_task(tittle, description, dificulty, points_reward, is_active, &ctx.bumps)
    }

    pub fn submit_task(
        ctx: Context<SubmitTask>,
        task_id: u64,
        repo_url: String
    ) -> Result<()> {
        ctx.accounts.submit_task(task_id, repo_url, &ctx.bumps)
    }
}
