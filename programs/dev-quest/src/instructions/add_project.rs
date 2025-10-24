use anchor_lang::prelude::*;

use crate::errors::ErrorCode;
use crate::state::{Project, UserAccount};

#[derive(Accounts)]
pub struct AddProject<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        mut,
        seeds = [b"user".as_ref(), user.key().as_ref()],
        bump = user_account.bump,
    )]
    pub user_account: Account<'info, UserAccount>,
    #[account(
        init,
        payer = user,
        seeds = [
            b"project",
            user.key().as_ref(),
            user_account.project_counter.to_le_bytes().as_ref()
        ],
        bump,
        space = 8 + Project::INIT_SPACE,
    )]
    pub project: Account<'info, Project>,
    pub system_program: Program<'info, System>,
}

impl<'info> AddProject<'info> {
    pub fn add_project(
        &mut self,
        name: String,
        description: String,
        repo_name: String,
        website_url: String,
        bumps: &AddProjectBumps,
    ) -> Result<()> {
        require!(
            name.len() > 0 && name.len() <= 64,
            ErrorCode::InvalidProjectName
        );
        require!(
            description.len() > 0 && description.len() <= 300,
            ErrorCode::TitleTooLongOrTooShort
        );
        require!(
            repo_name.len() > 0 && repo_name.len() <= 100,
            ErrorCode::InvalidRepoName
        );
        require!(website_url.len() <= 100, ErrorCode::InvalidWebsiteUrl);
        require!(
            self.user_account.project_counter < 255,
            ErrorCode::TooManyProjects
        );

        self.project.set_inner(Project {
            user: self.user.key(),
            project_id: self.user_account.project_counter,
            name: name,
            description: description,
            repo_name,
            website_url: website_url,
            created_at: Clock::get()?.unix_timestamp,
            bump: bumps.project,
        });

        Ok(())
    }
}
