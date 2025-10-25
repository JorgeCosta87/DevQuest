use anchor_lang::prelude::*;
use crate::state::{UserAccount, TaskSubmission};

#[derive(Accounts)]
#[instruction(task_id: u64)]
pub struct RemoveTaskSubmission<'info> {
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
            b"submission".as_ref(),
            user.key().as_ref(),
            task_id.to_le_bytes().as_ref()
        ],
        bump = task_submission.bump,
        constraint = task_submission.user == user.key()
    )]
    pub task_submission: Account<'info, TaskSubmission>,
}

impl<'info> RemoveTaskSubmission<'info> {
    pub fn remove_task_submission(&mut self) -> Result<()> {
        msg!(
            "Task submission removed for user: {}, task: {}",
            self.user.key(),
            self.task_submission.task_id
        );
        Ok(())
    }
}