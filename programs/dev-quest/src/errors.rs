use anchor_lang::error_code;

#[error_code]
pub enum ErrorCode {
    #[msg("Title to long")]
    TitleTooLongOrTooShort,
    #[msg("Description to long")]
    DescriptionTooLong,
    #[msg("invalid reward points")]
    InvalidPoints,
    #[msg("invalid repo url")]
    InvalidRepoUrl,
}
