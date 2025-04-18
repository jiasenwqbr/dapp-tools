--- block      id = block_number
create table IF NOT EXISTS ethereum_uniswap_v3.ethereum_unswap_v3_json (
    id text primary key,
    timestamp_sub bigint,
    data_type text,
    data text
);


create table IF NOT EXISTS ethereum_uniswap_v3.ethereum_uniswap_v3_factory (
    id text primary key,
    update_at bigint,
    data_type text,
    pool_count bigint,
    tx_count bigint,
    untracked_volume_usd numeric,
    total_fees_usd numeric,
    total_volume_eth numeric,
    total_fees_eth numeric,
    total_value_locked_usd numeric,
    total_value_locked_usd_untracked numeric,
    total_value_locked_eth numeric,
    total_value_locked_eth_untracked numeric
);

create table IF NOT EXISTS ethereum_uniswap_v3.ethereum_uniswap_v3_pools (
     id text primary key,
     created_at_timestamp bigint,
     created_at_block_number bigint,
     token0 text,
     token1 text,
     fee_tier text,
     liquidity bigint,
     sqrt_price bigint,
     fee_growth_global_0x128 bigint,
     fee_growth_global_1x128 bigint,
     token0_price numeric,
     token1_price numeric,
     tick bigint,
     observation_index bigint,
     volume_token0 numeric,
     volume_token1 numeric,
     volume_usd numeric,
     untracked_volume_usd numeric,
     fees_usd numeric,
     tx_count bigint,
     collected_fees_token0 numeric,
     collected_fees_token1 numeric,
     collected_fees_usd numeric,
     total_value_locked_token0 numeric,
     total_value_locked_token1 numeric,
     total_value_locked_eth numeric,
     total_value_locked_usd numeric,
     total_value_locked_usd_untracked numeric,
     total_value_locked_eth_untracked numeric,
     liquidity_provider_count bigint

);


create table IF NOT EXISTS ethereum_uniswap_v3.ethereum_uniswap_v3_tokens (
     id text primary key,
     symbol text,
     token_name text,
     token_decimals bigint,
     total_supply numeric,
     volume numeric,
     volume_usd numeric,
     untracked_volume_usd numeric,
     fees_usd numeric,
     tx_count bigint,
     pool_count bigint,
     total_value_locked numeric,
     total_value_locked_usd numeric,
     total_value_locked_usd_untracked numeric,
     derived_eth numeric
);