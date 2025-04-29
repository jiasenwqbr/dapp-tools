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