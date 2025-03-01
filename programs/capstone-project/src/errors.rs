use anchor_lang::error_code;

#[error_code]
pub enum EscrowError {
    #[msg("The taker is not authorized to withdraw.")]
    UnauthorizedTaker,

    #[msg("The vault does not have enough tokens.")]
    VaultEmpty,

    #[msg("The maker does not have enough tokens to deposit.")]
    InsufficientFunds,
}
