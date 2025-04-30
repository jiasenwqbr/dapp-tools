--- block 

--- block pairs 
create table IF NOT EXISTS bsc_pancake_v2.bsc_pancake_v2_pairs_create(
    id text primary key,
    block_number bigint,
    block_time bigint,
    token0_address text,
    token1_address text,
    creation_transaction_id text,
    log_ordinal bigint,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE
);

--- block swaps 
create table IF NOT EXISTS bsc_pancake_v2.bsc_pancake_v2_swaps(
    id text primary key,
    block_number bigint,
    block_time bigint,
    pair_address text,
    swap_sender text,
    swap_to text,
    amount0_in numeric,
    amount0_out numeric,
    amount1_in numeric,
    amount1_out numeric,
    transaction_from text,
    transaction_to text,
    transaction_gas_price numeric,
    transaction_gas_used numeric,
    transaction_hash text,
    transaction_public_key text,
    transaction_max_fee_per_gas numeric,
    transaction_max_priority_fee_per_gas numeric,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE
);


--- block pair tokens 
create table IF NOT EXISTS bsc_pancake_v2.bsc_pancake_v2_graph_pair_tokens(
    id text primary key,
    created_at_block_number text,
    created_at_timestamp text,
    liquidity_provider_count text,
    reserve0 numeric,
    reserve1 numeric,
    reserve_bnb numeric,
    reserve_usd numeric,
    token0_price numeric,
    token1_price numeric,
    total_supply numeric,
    tracked_reserve_bnb numeric,
    tx_count bigint,
    untracked_volume_usd numeric,
    volume_token0 numeric,
    volume_token1 numeric,
    volume_usd numeric,
    token0_id text,
    token0_name text,
    token0_symbol text,
    token0_decimals text,
    token0_derived_bnb  numeric,
    token0_total_liquidity numeric,
    token0_total_supply numeric,
    token0_trade_volume numeric,
    token0_trade_volume_usd numeric,
    token0_tx_count bigint,
    token0_untracked_volume_usd numeric,
    token1_id text,
    token1_name text,
    token1_symbol text,
    token1_decimals text,
    token1_derived_bnb  numeric,
    token1_total_liquidity numeric,
    token1_total_supply numeric,
    token1_trade_volume numeric,
    token1_trade_volume_usd numeric,
    token1_tx_count bigint,
    token1_untracked_volume_usd numeric,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE
);






