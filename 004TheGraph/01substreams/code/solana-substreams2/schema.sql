create table IF NOT EXISTS solana_raydium_transfer
(
    id text primary key,
    signature          text,
    block_time          text,
    transaction_index      text,
    funding_account        text,
    recipient_account text,
    lamports   text,
    funding_account_balance_pre_balance text,
    funding_account_balance_post_balance text,
    recipient_account_balance_pre_balance text,
    recipient_account_balance_post_balance text
);

create table IF NOT EXISTS solana_raydium_swap
(
    id text primary key,
    signature          text,
    block_time          text,
    transaction_index      text,
    amm        text,
    user_swap text,
    mint_in   text,
    mint_out text,
    amount_in text,
    amount_out text,
    direction text,
    pool_pc_amount text,
    pool_coin_amount text,
    pc_mint text,
    coin_mint text,
    user_pre_balance_in  text,
    user_pre_balance_out text
);