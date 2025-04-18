
```shell
make build
```

```shell
RUST_LOG=debug substreams-sink-sql setup "psql://postgres:root@172.20.31.18:5432/blockchain_data?sslmode=disable&search_path=ethereum_uniswap_v3&schema=ethereum_uniswap_v3" ./sink/substreams.dev.yaml 


RUST_LOG=debug  substreams-sink-sql run "psql://postgres:root@172.20.31.18:5432/blockchain_data?sslmode=disable&search_path=ethereum_uniswap_v3&schema=ethereum_uniswap_v3" ./sink/substreams.dev.yaml   --header "x-api-key:xxxxxxxxxxxxxxxxxxxxxxx" --on-module-hash-mistmatch=ignore --batch-block-flush-interval=1 --batch-row-flush-interval=10
```

