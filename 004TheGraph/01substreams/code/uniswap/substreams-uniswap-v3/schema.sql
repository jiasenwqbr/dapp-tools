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
     liquidity numeric,
     sqrt_price numeric,
     fee_growth_global_0x128 numeric,
     fee_growth_global_1x128 numeric,
     token0_price numeric,
     token1_price numeric,
     tick numeric,
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

create table IF NOT EXISTS ethereum_uniswap_v3.ethereum_uniswap_v3_ticks (
     id text primary key,
     pool_address text,
     tick_idx text,
     pool text,
     liquidity_gross numeric,
     liquidity_net numeric,
     price0 numeric,
     price1 numeric,
     volume_token0 numeric,
     volume_token1 numeric,
     volume_usd numeric,
     untracked_volume_usd numeric,
     fees_usd numeric,
     collected_fees_token0 numeric,
     collected_fees_token1 numeric,
     collected_fees_usd numeric,
     created_at_timestamp bigint,
     created_at_block_number bigint,
     liquidity_provider_count bigint,
     fee_growth_outside_0x128 numeric,
     fee_growth_outside_1x128 numeric
);


create table IF NOT EXISTS ethereum_uniswap_v3.ethereum_uniswap_v3_positions (
     id text primary key,
     owner text,
     pool text,
     token0 text,
     token1 text,
     tick_lower text,
     tick_upper text,
     liquidity text,
     deposited_token0 numeric,
     deposited_token1 numeric,
     withdrawn_token0 numeric,
     withdrawn_token1 numeric,
     collected_fees_token0 numeric,
     collected_fees_token1 numeric,
     transaction text,
     fee_growth_inside0_last_x128 text,
     fee_growth_inside1_last_x128 text
);


create table IF NOT EXISTS  ethereum_uniswap_v3.ethereum_uniswap_v3_position_snapshot (
     id text primary key,
     owner text,
     pool text,
     position text,
     block_number text,
     position_timestamp bigint,
     liquidity numeric,
     deposited_token0 numeric,
     deposited_token1 numeric,
     withdrawn_token0 numeric,
     withdrawn_token1 numeric,
     collected_fees_token0 numeric,
     collected_fees_token1 numeric,
     transaction text,
     fee_growth_inside0_last_x128 numeric,
     fee_growth_inside1_last_x128 numeric
);

create table IF NOT EXISTS  ethereum_uniswap_v3.ethereum_uniswap_v3_transaction (
     id text primary key,
     block_number bigint,
     transaction_timestamp bigint,
     gas_used numeric,
     gas_price text

);

create table IF NOT EXISTS  ethereum_uniswap_v3.ethereum_uniswap_v3_swap (
   id text primary key,
   transaction text,
   swap_timestamp bigint,
   pool text,
   token0 text,
   token1 text,
   sender text,
   recipient text,
   origin text,
   amount0 numeric,
   amount1 numeric,
   amount_usd numeric,
   sqrt_price_x96 numeric,
   tick numeric,
   log_index bigint
);

create table IF NOT EXISTS  ethereum_uniswap_v3.ethereum_uniswap_v3_mint (
    id text primary key,
    transaction text,
    mint_timestamp bigint,
    pool text,
    token0 text,
    token1 text,
    owner text,
    sender text,
    origin text,
    amount_usd numeric,
    tick_lower text,
    tick_upper text,
    log_index bigint
);

create table IF NOT EXISTS  ethereum_uniswap_v3.ethereum_uniswap_v3_burn (
    id text primary key,
    transaction text,
    pool text,
    token0 text,
    token1 text,
    burn_timestamp bigint,
    owner text,
    origin text,
    amount numeric,
    amount0 numeric,
    amount1 numeric,
    amount_usd numeric,
    tick_lower text,
    tick_upper text
);



create table IF NOT EXISTS  ethereum_uniswap_v3.ethereum_uniswap_v3_uniswap_day_data (
    id text primary key,
    day_start_timestamp bigint,
    volume_eth numeric,
    volume_usd numeric,
    volume_usd_untracked numeric,
    total_value_locked_usd numeric,
    fees_usd numeric,
    tx_count numeric
);







