use anchor_lang::prelude::*;

use crate::errors::ErrorCode;
use crate::state::UserAccount;

#[derive(Accounts)]
pub struct UpdateUserAccount<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        mut,
        seeds = [b"user".as_ref(), user.key().as_ref()],
        bump = user_account.bump,
        constraint = user_account.owner == user.key()
    )]
    pub user_account: Account<'info, UserAccount>,
}

impl<'info> UpdateUserAccount<'info> {
    pub fn update_user_account(
        &mut self,
        name: Option<String>,
        bio: Option<String>,
        github_username: Option<String>,
    ) -> Result<()> {
        if let Some(name) = name {
            require!(name.len() > 0 && name.len() <= 32, ErrorCode::InvalidName);
            self.user_account.name = name;
        }

        if let Some(bio) = bio {
            require!(bio.len() <= 200, ErrorCode::BioTooLong);
            self.user_account.bio = bio;
        }

        if let Some(github_username) = github_username {
            require!(
                github_username.len() <= 32,
                ErrorCode::GitHubUsernameTooLong
            );
            self.user_account.github_username = github_username;
        }

        Ok(())
    }
}
