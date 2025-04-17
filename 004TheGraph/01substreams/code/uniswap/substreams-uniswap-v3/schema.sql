--- block      id = block_number
create table IF NOT EXISTS ethereum_uniswap_v3.ethereum_unswap_v3_json (
    id text primary key,
    timestamp_sub bigint,
    data_type text,
    data text
);