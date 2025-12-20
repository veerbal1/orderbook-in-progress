use solana_program::{
    account_info::AccountInfo, entrypoint, entrypoint::ProgramResult, pubkey::Pubkey,
};
use solana_security_txt::security_txt;
use solana_program::declare_id;

security_txt! {
    name: "Orderbook DEX",
    project_url: "https://github.com/veerbal1/orderbook",
    contacts: "veerbal.singh369@gmail.com",
    policy: "https://github.com/veerbal1/orderbook",
    source_code: "https://github.com/veerbal1/orderbook"
}

/// Log Authority PDA - only this program can sign for it
pub mod log_authority {
    use solana_program::pubkey::Pubkey;
    
    /// Seeds used to derive the PDA
    pub const SEED: &[u8] = b"log";
    
    /// Get the log authority PDA and bump
    pub fn get_log_authority(program_id: &Pubkey) -> (Pubkey, u8) {
        Pubkey::find_program_address(&[SEED], program_id)
    }
}

declare_id!("rtZo6YDEfQwvywWwaEiQbpbbFFmWBbmieHgsU52LSE6");

entrypoint!(process_instruction);
pub fn process_instruction(
    _program_id: &Pubkey,
    _accounts: &[AccountInfo],
    _instruction_data: &[u8],
) -> ProgramResult {
    Ok(())
}
