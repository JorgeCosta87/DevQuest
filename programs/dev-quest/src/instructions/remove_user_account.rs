use crate::state::UserAccount;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct RemoveUserAccount<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        mut,
        close = user,
        seeds = [b"user".as_ref(), user.key().as_ref()],
        bump = user_account.bump,
    )]
    pub user_account: Account<'info, UserAccount>,
}

impl<'info> RemoveUserAccount<'info> {
    pub fn remove_user_account(&mut self) -> Result<()> {
        msg!("User account removed for: {}", self.user.key());

        Ok(())
    }
}
