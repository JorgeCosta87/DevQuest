use anchor_lang::prelude::*;

use crate::errors::ErrorCode;
use crate::state::{Task, TaskSubmission, UserAccount};

#[derive(Accounts)]
#[instruction(task_id: u64)]
pub struct SubmitTask<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        mut,
        seeds = [b"user".as_ref(), user.key().as_ref()],
        bump = user_account.bump,
    )]
    pub user_account: Account<'info, UserAccount>,
    #[account(
        mut,
        seeds = [b"task".as_ref(), task_id.to_le_bytes().as_ref()],
        bump = task.bump,
        constraint = task.is_active
    )]
    pub task: Account<'info, Task>,
    #[account(
        init,
        payer = user,
        seeds = [
            b"submission".as_ref(),
            user.key().as_ref(),
            task_id.to_le_bytes().as_ref()
        ],
        bump,
        space = 8 + TaskSubmission::INIT_SPACE,
    )]
    pub task_submission: Account<'info, TaskSubmission>,
    pub system_program: Program<'info, System>,
}

impl<'info> SubmitTask<'info> {
    pub fn submit_task(
        &mut self,
        task_id: u64,
        repo_name: String,
        bumps: &SubmitTaskBumps,
    ) -> Result<()> {
        require!(
            repo_name.len() > 0 && repo_name.len() <= 100,
            ErrorCode::InvalidRepoName
        );

        self.task_submission.set_inner(TaskSubmission {
            user: self.user.key(),
            task_id: task_id,
            repo_name,
            submitted_at: Clock::get()?.unix_timestamp,
            bump: bumps.task_submission,
        });

        self.user_account.total_points += self.task.points_reward as u64;

        msg!(
            "Task submitted! User: {}, Task: {}, Points earned: {}, Total: {}",
            self.user_account.owner,
            self.task.task_id,
            self.task.points_reward,
            self.user_account.total_points
        );

        Ok(())
    }
}
