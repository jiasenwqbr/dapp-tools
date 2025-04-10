--- block 
create table IF NOT EXISTS ethereum_block_all (id text primary key, data text);

--- block uniswapv2 pool
create table IF NOT EXISTS ethereum_block_uniswapv2_pool(
    id text primary key,
    address text,
    token0 text,
    token1 text,
    created_tx_hash text,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);
COMMENT ON TABLE ethereum_block_uniswapv2_pool IS 'ethereum_block_uniswapv2_pool';
COMMENT ON COLUMN ethereum_block_uniswapv2_pool.id IS 'ID';


--- block uniswapv2 event
create table IF NOT EXISTS ethereum_block_uniswapv2_event(
    id text primary key,
    hash text,
    log_index bigint,
    log_ordinal bigint,
    address_to text,
    address_from text,
    block_number bigint,
    event_timestamp bigint,
    pool text,
    event_type bigint,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

--- block uniswapv2 event DepositEvent
create table IF NOT EXISTS ethereum_block_uniswapv2_event_depositevent(
    id text primary key,
    hash text,
    log_index bigint,
    log_ordinal bigint,
    address_to text,
    address_from text,
    block_number bigint,
    event_timestamp bigint,
    pool text,
    event_type bigint,
    input_token_amounts numberic,
    output_token_amount numberic,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);