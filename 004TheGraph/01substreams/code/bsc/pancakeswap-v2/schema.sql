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

