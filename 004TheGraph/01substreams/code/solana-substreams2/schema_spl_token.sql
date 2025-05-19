----- solana_substream_spl_token
CREATE TABLE solana_raydium.solana_substream_spl_token (
    id TEXT PRIMARY KEY,
    block_number BIGINT,
    block_time BIGINT,
    signature TEXT,
    spl_index INTEGER,
    spl_token_event_index INTEGER,
    amount numeric,
    authority TEXT,

    destination_address TEXT,
    destination_mint TEXT,
    destination_owner TEXT,
    destination_pre_balance numeric,
    destination_post_balance numeric,

    source_address TEXT,
    source_mint TEXT,
    source_owner TEXT,
    source_pre_balance numeric,
    source_post_balance numeric
);

CREATE INDEX idx_spl_block_number ON solana_substream_spl_token (block_number);
CREATE INDEX idx_spl_source_address ON solana_substream_spl_token (source_address);
CREATE INDEX idx_spl_source_mint ON solana_substream_spl_token (source_mint);
CREATE INDEX idx_spl_source_owner ON solana_substream_spl_token (source_owner);
CREATE INDEX idx_spl_destination_address ON solana_substream_spl_token (destination_address);
CREATE INDEX idx_spl_destination_mint ON solana_substream_spl_token (destination_mint);
CREATE INDEX idx_spl_destination_owner ON solana_substream_spl_token (destination_owner);

----- solana_substream_initialize_mint

CREATE TABLE solana_raydium.solana_substream_spl_token_initialize_mint (
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

----- solana_substream_initialize_immutable_owner

CREATE TABLE solana_substream_spl_token_initialize_immutable_owner (
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

CREATE INDEX ON solana_substream_spl_token_initialize_immutable_owner (block_number);
CREATE INDEX ON solana_substream_spl_token_initialize_immutable_owner (address);
CREATE INDEX ON solana_substream_spl_token_initialize_immutable_owner (mint);
CREATE INDEX ON solana_substream_spl_token_initialize_immutable_owner (owner);


--- solana_substream_initialize_account
CREATE TABLE solana_substream_spl_token_initialize_account (
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

CREATE INDEX ON solana_substream_spl_token_initialize_account (block_number);
CREATE INDEX ON solana_substream_spl_token_initialize_account (address);
CREATE INDEX ON solana_substream_spl_token_initialize_account (mint);
CREATE INDEX ON solana_substream_spl_token_initialize_account (owner);

--- solana_substream_initialize_multisig
CREATE TABLE solana_substream_spl_token_initialize_multisig (
    id TEXT PRIMARY KEY,
    block_number BIGINT,
    block_time BIGINT,
    signature TEXT,
    spl_index BIGINT,
    spl_token_event_index BIGINT,
    multisig TEXT,
    m BIGINT,
    signer_index BIGINT,
    signer TEXT
);

-- 创建索引
CREATE INDEX idx_solana_init_multisig_block_number ON solana_substream_spl_token_initialize_multisig (block_number);
CREATE INDEX idx_solana_init_multisig_multisig ON solana_substream_spl_token_initialize_multisig (multisig);


--- solana_substream_spl_token_approve
CREATE TABLE solana_substream_spl_token_approve (
    id TEXT PRIMARY KEY,
    block_number BIGINT,
    block_time BIGINT,
    signature TEXT,
    spl_index BIGINT,
    spl_token_event_index BIGINT,
    amount BIGINT,
    delegate TEXT,
    approve_address TEXT,
    approve_mint TEXT,
    approve_owner TEXT,
    approve_post_balance BIGINT,
    approve_pre_balance BIGINT
);

-- 创建索引
CREATE INDEX idx_spl_token_approve_block_number ON solana_substream_spl_token_approve (block_number);
CREATE INDEX idx_spl_token_approve_address ON solana_substream_spl_token_approve (approve_address);
CREATE INDEX idx_spl_token_approve_mint ON solana_substream_spl_token_approve (approve_mint);
CREATE INDEX idx_spl_token_approve_owner ON solana_substream_spl_token_approve (approve_owner);



--- solana_substream_spl_mint_to
CREATE TABLE solana_substream_spl_token_mint_to (
    id TEXT PRIMARY KEY,
    block_number BIGINT,
    block_time BIGINT,
    signature TEXT,
    spl_index BIGINT,
    spl_token_event_index BIGINT,
    amount BIGINT,
    mint TEXT,
    mint_authority TEXT,
    mint_address TEXT,
    mint_mint TEXT,
    mint_owner TEXT,
    approve_post_balance BIGINT,
    approve_pre_balance BIGINT
);

-- 创建索引
CREATE INDEX idx_spl_mint_to_block_number ON solana_substream_spl_token_mint_to (block_number);
CREATE INDEX idx_spl_mint_to_amount ON solana_substream_spl_token_mint_to (amount);
CREATE INDEX idx_spl_mint_to_mint ON solana_substream_spl_token_mint_to (mint);
CREATE INDEX idx_spl_mint_to_mint_address ON solana_substream_spl_token_mint_to (mint_address);



------  solana_substream_spl_token_revoke

CREATE TABLE solana_substream_spl_token_revoke (
    id TEXT PRIMARY KEY,
    block_number BIGINT,
    block_time BIGINT,
    signature TEXT,
    spl_index BIGINT,
    spl_token_event_index BIGINT,
    revoke_address TEXT,
    revoke_mint TEXT,
    revoke_owner TEXT,
    revoke_post_balance BIGINT,
    revoke_pre_balance BIGINT
);

-- 创建索引
CREATE INDEX idx_spl_revoke_block_number ON solana_substream_spl_token_revoke (block_number);
CREATE INDEX idx_spl_revoke_address ON solana_substream_spl_token_revoke (revoke_address);
CREATE INDEX idx_spl_revoke_mint ON solana_substream_spl_token_revoke (revoke_mint);
CREATE INDEX idx_spl_revoke_owner ON solana_substream_spl_token_revoke (revoke_owner);

------  solana_substream_spl_token_set_authority
CREATE TABLE solana_substream_spl_token_set_authority (
    id TEXT PRIMARY KEY,
    block_number BIGINT,
    block_time BIGINT,
    signature TEXT,
    spl_index BIGINT,
    spl_token_event_index BIGINT,
    authority TEXT,
    authority_type INTEGER,
    mint TEXT,
    new_authority TEXT
);

-- 创建索引
CREATE INDEX idx_spl_set_authority_block_number ON solana_substream_spl_token_set_authority (block_number);


------ solana_substream_spl_token_burn
CREATE TABLE solana_substream_spl_token_burn (
    id TEXT PRIMARY KEY,
    block_number BIGINT,
    block_time BIGINT,
    signature TEXT,
    spl_index BIGINT,
    spl_token_event_index BIGINT,
    amount BIGINT,
    authority TEXT,
    burn_address TEXT,
    burn_mint TEXT,
    burn_owner TEXT,
    burn_post_balance BIGINT,
    burn_pre_balance BIGINT
);

-- 创建索引（按要求字段）
CREATE INDEX idx_spl_burn_block_number ON solana_substream_spl_token_burn (block_number);
CREATE INDEX idx_spl_burn_authority ON solana_substream_spl_token_burn (authority);
CREATE INDEX idx_spl_burn_address ON solana_substream_spl_token_burn (burn_address);
CREATE INDEX idx_spl_burn_mint ON solana_substream_spl_token_burn (burn_mint);
CREATE INDEX idx_spl_burn_owner ON solana_substream_spl_token_burn (burn_owner);


------ solana_substream_spl_token_close_account
CREATE TABLE solana_substream_spl_token_close_account (
    id TEXT PRIMARY KEY,
    block_number BIGINT,
    block_time BIGINT,
    signature TEXT,
    spl_index BIGINT,
    spl_token_event_index BIGINT,
    destination TEXT,
    source_address TEXT,
    source_mint TEXT,
    source_owner TEXT,
    source_post_balance BIGINT,
    source_pre_balance BIGINT
);

-- 创建索引（按要求字段）
CREATE INDEX idx_spl_close_account_block_number ON solana_substream_spl_token_close_account (block_number);
CREATE INDEX idx_spl_close_account_destination ON solana_substream_spl_token_close_account (destination);
CREATE INDEX idx_spl_close_account_source_address ON solana_substream_spl_token_close_account (source_address);
CREATE INDEX idx_spl_close_account_source_mint ON solana_substream_spl_token_close_account (source_mint);
CREATE INDEX idx_spl_close_account_source_owner ON solana_substream_spl_token_close_account (source_owner);


------ solana_substream_spl_token_freeze_account
CREATE TABLE solana_substream_spl_token_freeze_account (
    id TEXT PRIMARY KEY,
    block_number BIGINT,
    block_time BIGINT,
    signature TEXT,
    spl_index BIGINT,
    spl_token_event_index BIGINT,
    freeze_authority TEXT,
    freeze_address TEXT,
    freeze_mint TEXT,
    freeze_owner TEXT,
    freeze_post_balance BIGINT,
    freeze_pre_balance BIGINT
);

-- 创建索引（按要求字段）
CREATE INDEX idx_spl_freeze_block_number ON solana_substream_spl_token_freeze_account (block_number);
CREATE INDEX idx_spl_freeze_authority ON solana_substream_spl_token_freeze_account (freeze_authority);
CREATE INDEX idx_spl_freeze_address ON solana_substream_spl_token_freeze_account (freeze_address);
CREATE INDEX idx_spl_freeze_mint ON solana_substream_spl_token_freeze_account (freeze_mint);
CREATE INDEX idx_spl_freeze_owner ON solana_substream_spl_token_freeze_account (freeze_owner);


------- solana_substream_spl_token_thaw_account
CREATE TABLE solana_substream_spl_token_thaw_account (
    id TEXT PRIMARY KEY,
    block_number BIGINT,
    block_time BIGINT,
    signature TEXT,
    spl_index BIGINT,
    spl_token_event_index BIGINT,
    freeze_authority TEXT,
    source_address TEXT,
    source_mint TEXT,
    source_owner TEXT,
    source_post_balance BIGINT,
    source_pre_balance BIGINT
);

-- 创建索引（按要求字段）
CREATE INDEX idx_spl_thaw_block_number ON solana_substream_spl_token_thaw_account (block_number);
CREATE INDEX idx_spl_thaw_authority ON solana_substream_spl_token_thaw_account (freeze_authority);
CREATE INDEX idx_spl_thaw_source_address ON solana_substream_spl_token_thaw_account (source_address);
CREATE INDEX idx_spl_thaw_source_mint ON solana_substream_spl_token_thaw_account (source_mint);
CREATE INDEX idx_spl_thaw_source_owner ON solana_substream_spl_token_thaw_account (source_owner);


------- solana_substream_spl_token_sync_native
CREATE TABLE solana_substream_spl_token_sync_native (
    id TEXT PRIMARY KEY,
    block_number BIGINT,
    block_time BIGINT,
    signature TEXT,
    spl_index BIGINT,
    spl_token_event_index BIGINT,
    address TEXT,
    mint TEXT,
    owner TEXT,
    post_balance BIGINT,
    pre_balance BIGINT
);

-- 创建索引（按要求字段）
CREATE INDEX idx_spl_sync_native_block_number ON solana_substream_spl_token_sync_native (block_number);
CREATE INDEX idx_spl_sync_native_address ON solana_substream_spl_token_sync_native (address);
CREATE INDEX idx_spl_sync_native_mint ON solana_substream_spl_token_sync_native (mint);
CREATE INDEX idx_spl_sync_native_owner ON solana_substream_spl_token_sync_native (owner);
