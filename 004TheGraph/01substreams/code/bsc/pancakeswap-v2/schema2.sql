--- block 

--- block token 
create table IF NOT EXISTS bsc_pancake_v2.bsc_pancake_v2_pcs_substream_token(
    id text primary key,
    token_name text,
    symbol text,
    token_decimals bigint,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE
);

--- block pairs
create table IF NOT EXISTS bsc_pancake_v2.bsc_pancake_v2_pcs_substream_pair(
    id text primary key,
    token_name text,
    symbol text,
    token_decimals bigint,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE
);

--- block swaps 
create table IF NOT EXISTS bsc_pancake_v2.bsc_pancake_v2_pcs_substream_swaps(
    id text primary key,
    pair_address text,
    block_timestamp bigint,
    token0 text,
    token1 text,
    transaction_id text,
    swap_id text,
    amount0_in text,
    amount0_out text,
    amount1_in text,
    amount1_out text,
    amount_bnb text,
    amount_usd text,
    swap_from text,
    swap_to text,
    log_address text,
    sender text,
    trade_volume0 text,
    trade_volume1 text,
    trade_volume_usd0 text,
    trade_volume_usd1 text,
    volume_token0 text,
    volume_token1 text,
    volume_usd text,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE
);







