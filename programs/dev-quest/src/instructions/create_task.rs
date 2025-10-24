use anchor_lang::prelude::*;

use crate::errors::ErrorCode;
use crate::state::{Config, Task};

#[derive(Accounts)]
pub struct CreateTask<'info> {
    #[account(
        mut,
        constraint = admin.key() == config.admin
    )]
    pub admin: Signer<'info>,
    #[account(
        mut,
        seeds = [b"config".as_ref()],
        bump = config.bump,
    )]
    pub config: Account<'info, Config>,
    #[account(
        init,
        payer = admin,
        seeds = [b"task", config.task_counter.to_le_bytes().as_ref()],
        bump,
        space = 8 + Task::INIT_SPACE,
    )]
    pub task: Account<'info, Task>,
    pub system_program: Program<'info, System>,
}

impl<'info> CreateTask<'info> {
    pub fn create_task(
        &mut self,
        tittle: String,
        description: String,
        points_reward: u32,
        is_active: bool,
        bumps: &CreateTaskBumps,
    ) -> Result<()> {
        require!(
            tittle.len() > 0 && tittle.len() <= 64,
            ErrorCode::TitleTooLongOrTooShort
        );
        require!(description.len() <= 300, ErrorCode::DescriptionTooLong);
        require!(points_reward > 0, ErrorCode::InvalidPoints);

        self.task.set_inner(Task {
            task_id: self.config.task_counter,
            title: tittle,
            description: description,
            points_reward: points_reward,
            is_active: is_active,
            bump: bumps.task,
        });

        self.config.task_counter += 1;

        msg!(
            "task created: Task_id={}, Title={}",
            self.task.task_id,
            self.task.title
        );

        Ok(())
    }
}
