use anchor_lang::prelude::*;

use crate::state::{Config, Task};

#[derive(Accounts)]
pub struct UpdateTaskStatus<'info> {
    #[account(
        constraint = admin.key() == config.admin
    )]
    pub admin: Signer<'info>,
    #[account(
        seeds = [b"config".as_ref()],
        bump = config.bump,
    )]
    pub config: Account<'info, Config>,
    #[account(
        mut,
        seeds = [b"task".as_ref(), task.task_id.to_le_bytes().as_ref()],
        bump = task.bump,
    )]
    pub task: Account<'info, Task>,
}

impl<'info> UpdateTaskStatus<'info> {
    pub fn update_task_status(&mut self, is_active: bool) -> Result<()> {
        self.task.is_active = is_active;

        msg!(
            "The Task ID {}, status is: {}",
            self.task.task_id,
            self.task.is_active
        );

        Ok(())
    }
}
