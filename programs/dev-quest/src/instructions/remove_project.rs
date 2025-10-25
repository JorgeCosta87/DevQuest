use crate::state::{Project, UserAccount};
use anchor_lang::prelude::*;

#[derive(Accounts)]
#[instruction(project_id: u8)]
pub struct RemoveProject<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        seeds = [b"user".as_ref(), user.key().as_ref()],
        bump = user_account.bump,
    )]
    pub user_account: Account<'info, UserAccount>,
    #[account(
        mut,
        close = user,
        seeds = [
            b"project".as_ref(),
            user.key().as_ref(),
            project_id.to_le_bytes().as_ref()
        ],
        bump = project.bump,
        constraint = project.user == user.key()
    )]
    pub project: Account<'info, Project>,
}

impl<'info> RemoveProject<'info> {
    pub fn remove_project(&mut self) -> Result<()> {
        msg!(
            "Project removed for user: {}, project: {}",
            self.user.key(),
            self.project.project_id
        );
        Ok(())
    }
}
