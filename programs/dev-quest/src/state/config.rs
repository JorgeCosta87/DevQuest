use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Config {
    pub admin: Pubkey,
    pub task_counter: u64,
    pub bump: u8,
    pub points_per_task: u32,
}
