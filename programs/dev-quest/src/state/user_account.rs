use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct UserAccount {
    pub owner: Pubkey,
    #[max_len(32)]
    pub name: String,
    #[max_len(200)]
    pub bio: String,
    #[max_len(32)]
    pub github_username: String,
    pub total_points: u64,
    pub project_counter: u8,
    pub bump: u8,
}
