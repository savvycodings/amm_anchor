use anchor_lang::{error::Error, error_code};

#[error_code]
pub enum AmmError {
    #[msg("Invalid Fee")]
    InvalidFee,
    #[msg("Invalid Auth Bump")]
    AuthBumpError,
    #[msg("Invalid Config Bump")]
    ConfigBumpError,
    #[msg("Invalid LP Mint Bump")]
    MintLpBumpError,
    #[msg("No Authority Set")]
    NoAuthoritySet,
}
