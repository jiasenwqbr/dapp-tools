create table IF NOT EXISTS solana_raydium.solana_raydium_transfer (
    id text primary key,
    signature text,
    block_time text,
    transaction_index text,
    funding_account text,
    recipient_account text,
    block_number bigint,
    lamports text,
    funding_account_balance_pre_balance text,
    funding_account_balance_post_balance text,
    recipient_account_balance_pre_balance text,
    recipient_account_balance_post_balance text,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);
create table IF NOT EXISTS solana_raydium.solana_raydium_swap (
    id text primary key,
    signature text,
    block_time text,
    block_number bigint,
    transaction_index text,
    amm text,
    user_swap text,
    mint_in text,
    mint_out text,
    amount_in text,
    amount_out text,
    direction text,
    pool_pc_amount text,
    pool_coin_amount text,
    pc_mint text,
    coin_mint text,
    user_pre_balance_in text,
    user_pre_balance_out text,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);
create table IF NOT EXISTS solana_raydium.solana_raydium_initialize (
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
create table IF NOT EXISTS solana_raydium.solana_raydium_deposite (
    id text primary key,
    signature text,
    block_time bigint,
    block_number BIGINT,
    transaction_index text,
    amm text,
    deposite_user text,
    pc_amount bigint,
    coin_amount bigint,
    lp_amount bigint,
    pc_mint text,
    coin_mint text,
    lp_mint text,
    pool_pc_amount numeric,
    pool_coin_amount numeric,
    pool_lp_amount numeric,
    user_pc_pre_balance numeric,
    user_coin_pre_balance numeric,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);
create table IF NOT EXISTS solana_raydium.solana_raydium_withdraw (
    id text primary key,
    signature text,
    block_time bigint,
    block_number BIGINT,
    transaction_index text,
    amm text,
    withdraw_user text,
    pc_amount bigint,
    coin_amount bigint,
    lp_amount bigint,
    pc_mint text,
    coin_mint text,
    lp_mint text,
    pool_pc_amount bigint,
    pool_coin_amount bigint,
    pool_lp_amount bigint,
    user_pc_pre_balance bigint,
    user_coin_pre_balance bigint,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);
create table IF NOT EXISTS solana_raydium.solana_raydium_withdraw_pnl (
    id text primary key,
    signature text,
    block_time bigint,
    block_number BIGINT,
    transaction_index text,
    amm text,
    withdraw_pnl_user text,
    pc_amount numeric,
    coin_amount numeric,
    pc_mint text,
    coin_mint text,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);
create table IF NOT EXISTS solana_raydium.solana_raydium_pump_fun_swap (
    id text primary key,
    signature text,
    block_time bigint,
    block_number BIGINT,
    transaction_index text,
    pump_fun_swap_user text,
    mint text,
    bonding_curve text,
    sol_amount numeric,
    token_amount numeric,
    direction text,
    virtual_sol_reserves numeric,
    virtual_token_reserves numeric,
    real_sol_reserves numeric,
    real_token_reserves numeric,
    user_token_pre_balance numeric,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);
create table IF NOT EXISTS solana_raydium.solana_raydium_pump_fun_withdraw (
    id text primary key,
    signature text,
    block_time bigint,
    block_number BIGINT,
    transaction_index text,
    mint text
);
create table IF NOT EXISTS solana_raydium.solana_raydium_pump_fun_create (
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
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);
create table IF NOT EXISTS solana_raydium.solana_raydium_transfer_with_seed (
    id text primary key,
    signature text,
    block_time bigint,
    block_number BIGINT,
    transaction_index text,
    funding_account text,
    base_account text,
    recipient_account text,
    lamports bigint,
    from_owner text,
    from_seed text,
    funding_account_pre_balance numeric,
    funding_account_post_balance numeric,
    recipient_account_pre_balance numeric,
    recipient_account_post_balance numeric,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- block sol_usd
create table IF NOT EXISTS solana_raydium.solana_block_sol_usd(
    id text primary key,
    block_number bigint,
    price numeric,
    price_text text,
    remark text,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

