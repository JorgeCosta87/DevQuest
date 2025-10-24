use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct TaskSubmission {
    pub user: Pubkey,
    pub task_id: u64,
    #[max_len(100)]
    pub repo_name: String,
    pub submitted_at: i64,
    pub bump: u8,
}
