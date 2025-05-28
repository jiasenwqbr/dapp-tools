--- solana_sunstreams_raydium_pair_initialize
create table IF NOT EXISTS solana_raydium_meta.solana_sunstreams_raydium_pair_initialize (
    id text primary key,
    signature text,
    block_time bigint,
    block_number BIGINT,
    transaction_index text,
    amm text,
    initialize_user text,
    pc_init_amount BIGINT,
    coin_init_amount numeric,
    lp_init_amount numeric,
    pc_mint text,
    coin_mint text,
    lp_mint text,
    nonce numeric,
    market text,
    user_pc_pre_balance numeric,
    user_coin_pre_balance numeric,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_raydium_pair_initialize_block_number ON solana_sunstreams_raydium_pair_initialize (block_number);
CREATE INDEX idx_raydium_pair_initialize_block_timer ON solana_sunstreams_raydium_pair_initialize (block_time);
CREATE INDEX idx_raydium_pair_initialize_amm ON solana_sunstreams_raydium_pair_initialize (amm);

--- solana_raydium_substreams_pump_fun_create
create table IF NOT EXISTS solana_raydium_meta.solana_raydium_substreams_pump_fun_create (
    id text primary key,
    signature text,
    block_time bigint,
    block_number BIGINT,
    transaction_index text,
    fun_name text,
    symbol text,
    uri text,
    mint text,
    bonding_curve text,
    associated_bonding_curve text,
    metadata text,
    user text,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_pump_fun_create_block_number ON solana_raydium_substreams_pump_fun_create (block_number);
CREATE INDEX idx_pump_fun_create_block_time ON solana_raydium_substreams_pump_fun_create (block_time);
CREATE INDEX idx_pump_fun_create_mint ON solana_raydium_substreams_pump_fun_create (mint);

--- solana_substream_initialize_account
CREATE TABLE solana_raydium_meta.solana_substream_spl_token_initialize_account (
    id TEXT PRIMARY KEY,
    block_number BIGINT,
    block_time BIGINT,
    signature TEXT,
    spl_index INTEGER,
    spl_token_event_index INTEGER,
    address TEXT,
    mint TEXT,
    owner TEXT,
    pre_balance BIGINT,
    post_balance BIGINT
);

CREATE INDEX idx_token_initialize_account_block_number ON solana_substream_spl_token_initialize_account (block_number);
CREATE INDEX idx_token_initialize_account_address ON solana_substream_spl_token_initialize_account (address);
CREATE INDEX idx_token_initialize_mint ON solana_substream_spl_token_initialize_account (mint);
CREATE INDEX idx_token_initialize_owner ON solana_substream_spl_token_initialize_account (owner);

----- solana_substream_initialize_mint
CREATE TABLE solana_raydium_meta.solana_substream_spl_token_initialize_mint (
    id TEXT PRIMARY KEY,
    block_number BIGINT,
    block_time BIGINT,
    signature TEXT,
    spl_index INTEGER,
    spl_token_event_index INTEGER,
    decimals INTEGER,
    freeze_authority TEXT,
    mint TEXT,
    mint_authority TEXT
);

-- 建立索引
CREATE INDEX idx_initialize_mint_block_number ON solana_substream_spl_token_initialize_mint (block_number);
CREATE INDEX idx_initialize_mint_mint ON solana_substream_spl_token_initialize_mint (mint);
CREATE INDEX idx_initialize_mint_mint_authority ON solana_substream_spl_token_initialize_mint (mint_authority);
CREATE INDEX idx_initialize_mint_freeze_authority ON solana_substream_spl_token_initialize_mint (freeze_authority);