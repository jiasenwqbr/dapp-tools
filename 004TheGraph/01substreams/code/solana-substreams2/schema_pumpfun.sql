----- solana_substream_pumpfun_initialize_user
CREATE TABLE solana_substream_pumpfun_initialize_user (
    id TEXT PRIMARY KEY,
    block_number BIGINT,
    block_time BIGINT,
    signature TEXT,
    pump_index BIGINT,
    event_index BIGINT,
    inital_user TEXT
);

-- 创建索引（按要求字段）
CREATE INDEX idx_pumpfun_init_user_block_number ON solana_substream_pumpfun_initialize_user (block_number);

----- solana_substream_pumpfun_set_params
CREATE TABLE solana_substream_pumpfun_set_params (
    id TEXT PRIMARY KEY,
    block_number BIGINT,
    block_time BIGINT,
    signature TEXT,
    pump_index BIGINT,
    event_index BIGINT,
    set_user TEXT,
    fee_basis_points numeric,
    fee_recipient TEXT,
    initial_real_token_reserves numeric,
    initial_virtual_sol_reserves numeric,
    initial_virtual_token_reserves numeric,
    token_total_supply numeric
);

-- 索引
CREATE INDEX idx_set_params_block_number ON solana_substream_pumpfun_set_params (block_number);
CREATE INDEX idx_set_params_user ON solana_substream_pumpfun_set_params (set_user);

----- solana_substream_pumpfun_swap
CREATE TABLE solana_substream_pumpfun_swap (
    id TEXT PRIMARY KEY,
    block_number BIGINT,
    block_time BIGINT,
    signature TEXT,
    pump_index INTEGER,
    event_index INTEGER,
    bonding_curve TEXT,
    direction TEXT,
    mint TEXT,
    zero INTEGER,
    real_sol_reserves BIGINT,
    real_token_reserves BIGINT,
    sol_amount BIGINT,
    token_amount BIGINT,
    swap_user TEXT,
    user_token_pre_balance BIGINT,
    virtual_sol_reserves BIGINT
);

CREATE INDEX idx_swap_block_number ON solana_substream_pumpfun_swap(block_number);
CREATE INDEX idx_swap_swap_user ON solana_substream_pumpfun_swap(swap_user);
CREATE INDEX idx_swap_mint ON solana_substream_pumpfun_swap(mint);
CREATE INDEX idx_swap_direction ON solana_substream_pumpfun_swap(direction);


----- solana_substream_pumpfun_withdraw

CREATE TABLE solana_substream_pumpfun_withdraw (
    id TEXT PRIMARY KEY,
    block_number BIGINT,
    signature TEXT,
    pump_index BIGINT,
    event_index BIGINT,
    mint TEXT
);

-- 创建索引（按要求字段）
CREATE INDEX idx_pumpfun_withdraw_block_number ON solana_substream_pumpfun_withdraw (block_number);
CREATE INDEX idx_pumpfun_withdraw_mint ON solana_substream_pumpfun_withdraw (mint);

----- solana_substream_pumpfun_create
CREATE TABLE solana_substream_pumpfun_create (
    id TEXT PRIMARY KEY,
    block_number BIGINT,
    block_time BIGINT,
    signature TEXT,
    pump_index INTEGER,
    event_index INTEGER,
    associated_bonding_curve TEXT,
    bonding_curve TEXT,
    metadata TEXT,
    mint TEXT,
    token_name TEXT,
    symbol TEXT,
    uri TEXT,
    carete_user TEXT
);

CREATE INDEX idx_create_block_number ON solana_substream_pumpfun_create(block_number);
CREATE INDEX idx_create_mint ON solana_substream_pumpfun_create(mint);
