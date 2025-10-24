use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Project {
    pub user: Pubkey,
    pub project_id: u8,
    #[max_len(64)]
    pub name: String,
    #[max_len(300)]
    pub description: String,
    #[max_len(100)]
    pub repo_name: String,
    #[max_len(100)]
    pub website_url: String,
    pub created_at: i64,
    pub bump: u8,
}
