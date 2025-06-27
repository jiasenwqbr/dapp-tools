--- block 
create table IF NOT EXISTS ethereum_uniswap_v2.ethereum_block_all (id text primary key, data text);

--- block transcation_change 
create table IF NOT EXISTS ethereum_uniswap_v2.ethereum_block_uniswapv2_transcation_change(
    id text primary key,
    block_number bigint,
    block_time bigint,
    transaction_change_index bigint,
    transcation_index bigint,
    balance_change_index bigint,

    token text,
    trans_from text,
    trans_to text,
    balance text,
    component_id text,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);



--- block entity_changes

create table IF NOT EXISTS ethereum_uniswap_v2.ethereum_block_uniswapv2_entity_changes(
    id text primary key,
    block_number bigint,
    block_time bigint,
    transaction_change_index bigint,
    transcation_index bigint,
    entity_change_index bigint,
    component_id text,
    attribute_index bigint,

    reserve0 text,
    reserve0_value numeric,
    reserve0_change bigint,
    reserve1 text,
    reserve1_value numeric,
    reserve1_change bigint,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);



--- block balance_changes

create table IF NOT EXISTS ethereum_uniswap_v2.ethereum_block_uniswapv2_balance_changes(
    id text primary key,
    block_number bigint,
    block_time bigint,
    transaction_change_index bigint,
    transcation_index bigint,

    token0 text,
    token0_balance numeric,
    token0_component_id text,

    token1 text,
    token1_balance numeric,
    token1_component_id text,

    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);






--- block component_changes

create table IF NOT EXISTS ethereum_uniswap_v2.ethereum_block_uniswapv2_component_changes(
    id text primary key,
    block_number bigint,
    block_time bigint,
    transaction_change_index bigint,
    transcation_index bigint,
    component_change_index bigint,

    component_change_id text,
    token0 text,
    token1 text,
    fee_value bigint,
    fee_change bigint,
    pool_address text,
    pool_change bigint,
    change bigint,
    protocol_type_name text,
    protocol_financial_type bigint,
    protocol_type_implementation_type bigint,
    tx_from text,
    tx_to text,
    tx_hash text,
    tx_index bigint,

    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);


--- block swaps
create table IF NOT EXISTS ethereum_uniswap_v2.ethereum_block_uniswapv2_swaps(
    id text primary key,
    block_number bigint,
    block_time bigint,
    transaction_from text, --钱包地址
    transaction_to text,  --合约地址
    transaction_gas_price numeric,
    transaction_gas_used numeric,
    transaction_hash text,
    transaction_public_key text,
    transaction_max_fee_per_gas numeric,
    transaction_max_priority_fee_per_gas numeric,
    pair_address text,
    swap_sender text,
    swap_to text,
    amount0_in numeric,
    amount0_out numeric,
    amount1_in numeric,
    amount1_out numeric,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);


--- block reserve
create table IF NOT EXISTS ethereum_uniswap_v2.ethereum_block_uniswapv2_reserves(
    id text primary key,
    block_number bigint,
    block_time bigint,
    transaction_hash text,
    reserve0 numeric,
    reserve1 numeric,
     created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);
--- block tokens
create table IF NOT EXISTS ethereum_uniswap_v2.ethereum_block_uniswapv2_tokens(
    id text primary key,
    symbol text,
    token_name text,
    decimals bigint, 
    total_liquidity numeric,  
    total_supply numeric,
    trade_volume_usd numeric,
    tx_count bigint,
    untracked_volume_usd numeric,
    derivedETH numeric,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE
);

--- block pairs
create table IF NOT EXISTS ethereum_uniswap_v2.ethereum_block_uniswapv2_pairs(
    id text primary key,
    token0_id text,
    token0_name text,
    token1_id text,
    token1_name text,
    liquidity_provider_count bigint,
    reserve0 numeric,
    reserve1 numeric,
    reserve_eth numeric,
    reserve_usd numeric,
    token0_price numeric,
    token1_price numeric,
    total_supply numeric,
    tracked_reserve_eth numeric,
    tx_count bigint,
    untracked_volume_usd numeric,
    volume_token0 numeric,
    volume_token1 numeric,
    volume_usd numeric,
    created_at_block_number bigint,
    created_at_timestamp bigint,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE
);

-- block erc20
create table IF NOT EXISTS  ethereum_block_erc20(
    id text primary key,
    token_name text,
    token_symbol text,
    token_decimals bigint,
    total_supply text,
     created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- ethereum_block_uniswapv2_substream_pairs
create table IF NOT EXISTS ethereum_uniswap_v2.ethereum_block_uniswapv2_substream_pairs(
     id text primary key,
     pair_address text,
     token0_address text,
     token1_address text,
     transaction_hash text,
     block_number bigint,
     block_time bigint,
     created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);


























