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
    swap_from text,
    swap_to text,
    amount0_in numberic,
    amount0_out numberic,
    amount1_in numberic,
    amount1_out numberic,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);
























