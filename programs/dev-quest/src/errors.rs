use anchor_lang::error_code;

#[error_code]
pub enum ErrorCode {
    #[msg("Invalid name")]
    InvalidName,
    #[msg("Invalid project name")]
    InvalidProjectName,
    #[msg("Title to long or to short")]
    TitleTooLongOrTooShort,
    #[msg("Description to long")]
    DescriptionTooLong,
    #[msg("Github username to long")]
    GitHubUsernameTooLong,
    #[msg("Invalid reward points")]
    InvalidPoints,
    #[msg("Invalid repo url")]
    InvalidRepoName,
    #[msg("Invalid dificulty")]
    InvalidDificulty,
    #[msg("Invalid website url")]
    InvalidWebsiteUrl,
    #[msg("Too many projects")]
    TooManyProjects,
    #[msg("Bio to long")]
    BioTooLong,
    
}