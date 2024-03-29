use anchor_lang::prelude::*;

#[account]
#[derive(Default, InitSpace)]
pub struct UserAccount {
    #[max_len(256)]
    pub name: String, // 4 + 256
    #[max_len(2048)]
    pub avatar: String, // 4 + 2048
    pub authority: Pubkey, // 32
    pub last_post_id: u8,  // 1
    pub post_count: u8,    // 1
}

#[account]
#[derive(Default, InitSpace)]
pub struct PostAccount {
    pub id: u8, // 1
    #[max_len(256)]
    pub title: String, // 4 + 256
    #[max_len(2048)]
    pub content: String, // 4 + 2048
    pub user: Pubkey, // 32
    pub authority: Pubkey, // 32
}
