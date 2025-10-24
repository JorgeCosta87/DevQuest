use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Task {
    pub task_id: u64,
    #[max_len(64)]
    pub title: String,
    #[max_len(250)]
    pub description: String,
    #[max_len(64)]
    pub dificulty: String,
    pub points_reward: u32,
    pub is_active: bool,
    pub bump: u8,
}
