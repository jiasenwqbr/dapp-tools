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

--- block component_changes

create table IF NOT EXISTS ethereum_uniswap_v2.ethereum_block_uniswapv2_component_change(
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



--- block component_changes 
create table IF NOT EXISTS ethereum_uniswap_v2.ethereum_block_uniswapv2_component_changes(
    id text primary key,
    block_number bigint,
    block_time bigint,
    transaction_change_index bigint,
    transcation_index bigint,
    contract_change_index bigint,
    contract_change_id bigint,
    change bigint,
    protocol_type_financial_type bigint,
    protocol_type_implementation_type bigint,
    protocol_type_name text,
    protocol_type_attribute_schema text,
    contract_index bigint,
    contract text,
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
























