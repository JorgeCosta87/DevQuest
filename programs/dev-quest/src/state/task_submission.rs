use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct TaskSubmission {
    pub user: Pubkey,
    pub task_id: u64,
    #[max_len(200)]
    pub repo_url: String,
    pub submitted_at: i64,
    pub bump: u8,
}
