use substreams_solana_utils::pubkey::Pubkey;
use substreams_solana::b58;

pub const RAYDIUM_AMM_PROGRAM_ID: Pubkey = Pubkey(b58!("675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8"));

pub const JUPITER_AGG_PROGRAM_ID: Pubkey = Pubkey(b58!("JUP6LkbZbjS1jKKwapdHNy74zcZ3tLUZoi5QNyVTaV4"));

pub const SOL_MINIMUM_LAMPORTS: u64 = 100000;
