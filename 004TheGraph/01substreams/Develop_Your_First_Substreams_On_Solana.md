## Transactions & Instructions
### Step 1: Initialize Your Solana Substreams Project
```shell
mkdir start
cd start
substreams init
```

![image-20250310183020539](images/image-20250310183020539.png)

![image-20250310183142458](images/image-20250310183142458.png)

![image-20250310183326551](images/image-20250310183326551.png)

1. Open the [Dev Container](https://github.com/streamingfast/substreams-starter) and follow the on-screen steps to initialize your project.
2. Running `substreams init` will give you the option to choose between two Solana project options. Select the one that best fits your requirements:
   - **sol-minimal**: Creates a simple Substreams that extracts raw Solana block data and generates corresponding Rust code. This path will start you with the full raw block, you can navigate to the `substreams.yaml` (the manifest) to modify the input.
   - **sol-transactions**: Creates a Substreams that filters Solana transactions based on one or more Program IDs and/or Account IDs, using the cached [Solana Foundational Module](https://substreams.dev/streamingfast/solana-common/v0.3.0).
   - **sol-anchor-beta**: Given an Anchor IDL, create a Substreams that decodes instructions and events. If an IDL isn’t available using the `idl` subcommand within the [Anchor CLI](https://www.anchor-lang.com/docs/cli), you’ll need to provide it yourself.

The modules within Solana Common exclude voting transactions, to benefit from a 75% reduction in data processing size and costs, delay your stream by over 1000 blocks from head. This can be done using the [`sleep`](https://doc.rust-lang.org/std/thread/fn.sleep.html) function in Rust.

1. 打开 [Dev Container](https://github.com/streamingfast/substreams-starter) 并按照屏幕上的步骤初始化您的项目。
2. 运行 `substreams init` 将为您提供在两个 Solana 项目选项之间进行选择的选项。选择最适合您要求的选项：
- **sol-minimal**：创建一个简单的 Substreams，用于提取原始 Solana 区块数据并生成相应的 Rust 代码。此路径将以完整的原始区块开始，您可以导航到 `substreams.yaml`（清单）以修改输入。
- **sol-transactions**：创建一个 Substreams，使用缓存的 [Solana Foundational Module](https://substreams.dev/streamingfast/solana-common/v0.3.0) 根据一个或多个程序 ID 和/或帐户 ID 过滤 Solana 交易。
- **sol-anchor-beta**：给定一个 Anchor IDL，创建一个解码指令和事件的子流。如果无法使用 [Anchor CLI](https://www.anchor-lang.com/docs/cli) 中的 `idl` 子命令获取 IDL，则需要自行提供。

Solana Common 中的模块排除投票交易，为了从数据处理大小和成本减少 75% 中受益，请将您的流从头部延迟超过 1000 个区块。这可以使用 Rust 中的 [`sleep`](https://doc.rust-lang.org/std/thread/fn.sleep.html) 函数来完成。

### Step 2: Visualize the Data

1. Run `substreams auth` to create your [account](https://thegraph.market/) and generate an authentication token (JWT), then pass this token back as input.

```shell
substreams auth [authentication token]
```



1. Now you can freely use the `substreams gui` to visualize and iterate on your extracted data.

```shell
substreams gui
```

#### Step 2.5: (Optionally) Transform the Data

Within the generated directories, modify your Substreams modules to include additional filters, aggregations, and transformations, then update the manifest accordingly. To learn more about this, visit the [How-to-Guides](https://docs.substreams.dev/how-to-guides/develop-your-own-substreams/solana) Within the generated directories, modify your Substreams modules to include additional filters, aggregations, and transformations, then update the manifest accordingly. To learn more about this, visit the [How-to-Guides](https://docs.substreams.dev/how-to-guides/develop-your-own-substreams/solana)

### Step 3: Load the Data

To make your Substreams queryable (as opposed to [direct streaming](https://docs.substreams.dev/how-to-guides/sinks/stream)), you can automatically generate a Subgraph (known as a [Substreams-powered subgraph](https://thegraph.com/docs/en/substreams/sps/introduction/) or SQL-DB sink.

#### Subgraph

1. Run `substreams codegen subgraph` to initialize the sink, producing the necessary files and function definitions.
2. Create your [subgraph mappings](https://docs.substreams.dev/how-to-guides/sinks/subgraph/triggers) within the `mappings.ts` and associated entities within the `schema.graphql`.
3. Build and deploy locally or to [Subgraph Studio](https://thegraph.com/studio-pricing/) by running `deploy-studio`.

#### SQL

1. Run `substreams codegen sql` and choose from either ClickHouse or Postgres to initialize the sink, producing the necessary files.
2. Run `substreams build` build the [Substreams:SQL](https://docs.substreams.dev/how-to-guides/sinks/sql-sink) sink.
3. Run `substreams-sink-sql` to sink the data into your selected SQL DB.

```shell
substreams codegen sql
```

##### Install substreams-sink-sql

Install `substreams-sink-sql` by using the pre-built binary release [available in the official GitHub repository](https://github.com/streamingfast/substreams-sink-sql/releases).

Extract `substreams-sink-sql` into a folder and ensure this folder is referenced globally via your `PATH` environment variable.

In my macbook,I put it into  /usr/local/bin

##### Install PostgreSQL

##### Create Database

##### Create configuration file

Once the database has been created, you must now define the Substreams Sink Config in a Substreams manifest creating a deployable unit.

Let's create a folder `sink` and in it create a file called `substreams.dev.yaml` with the following content:

```
specVersion: v0.1.0
package:
  name: "<name>"
  version: <version>

imports:
  sql: https://github.com/streamingfast/substreams-sink-sql/releases/download/protodefs-v1.0.1/substreams-sink-sql-protodefs-v1.0.1.spkg
  main: ../substreams.yaml

network: 'mainnet'

sink:
  module: main:db_out
  type: sf.substreams.sink.sql.v1.Service
  config:
    schema: "../schema.sql"
    wire_protocol_access: true
```

The `package.name` and `package.version` are meant to be replaced to fit your project.

The `imports.main` defines your Substreams manifest that you want to sink. The `sink.module` defines which import key (`main` here) and which module's name (`db_out` here).

The `network` field defines which network this deployment should be part of, in our case `mainnet`

The `sink.type` defines the type of the config that we are expecting, in our case it's [sf.substreams.sink.sql.v1.Service](https://buf.build/streamingfast/substreams-sink-sql/docs/main:sf.substreams.sink.sql.v1#sf.substreams.sink.sql.v1.Service) (click on the link to see the message definition).

The `sink.config` is the instantiation of this `sink.type` with the config fully filled. Some config are special because they load from a file or from a folder. For example in our case the `sink.config.schema` is defined with a Protobuf option `load_from_file` which means the content of the `../schema.sql` will actually be inlined in the Substreams manifest.

> The final final can be found at [`sink/substreams.dev.yaml`](https://github.com/streamingfast/substreams-sink-sql/blob/develop/docs/tutorial/sink/substreams.dev.yaml)

##### Run setup command

Use the following command to run the `substreams-sink-sql` tool and set up the database for the tutorial.



```shell
substreams-sink-sql setup "psql://dev-node:insecure-change-me-in-prod@127.0.0.1:5432/substreams_example?sslmode=disable" ./sink/substreams.dev.yaml
```

The `"psql://..."` is the DSN (Database Source Name) containing the connection details to your database packed as an URL. The `scheme` (`psql` here) part of the DSN's url defines which driver to use, `psql` is what we are going to use here, see [Drivers](https://docs.substreams.dev/how-to-guides/sinks/sql-sink#drivers) section below to see what other DSN you can use here.

The DSN's URL defines the database IP address, username, and password, which depend on your PostgreSQL installation. Adjust `dev-node` to your own username `insecure-change-me-in-prod` to your password and `127.0.0.1:5432` to where your database can be reached.

##### Sink data to database

The `substreams-sink-sql` tool sinks data from the Substreams module to the SQL database. Use the tool's `run` command, followed by the endpoint to reach and your Substreams config file to use:

```
substreams-sink-sql run "psql://dev-node:insecure-change-me-in-prod@127.0.0.1:5432/substreams_example?sslmode=disable" ./sink/substreams.dev.yaml --header "x-api-key: YOUR_API_KEY"
```



The endpoint needs to match the blockchain targeted in the Substreams module. The example Substreams module uses the Ethereum blockchain.

Successful output from the `substreams-sink-sql` tool will resemble the following:

```text
2023-01-18T12:32:19.107-0800 INFO (sink-sql) starting prometheus metrics server {"listen_addr": "localhost:9102"}
2023-01-18T12:32:19.107-0800 INFO (sink-sql) sink from psql {"dsn": "psql://dev-node:insecure-change-me-in-prod@127.0.0.1:5432/substreams_example?sslmode=disable", "endpoint": "mainnet.eth.streamingfast.io:443", "manifest_path": "substreams.yaml", "output_module_name": "db_out", "block_range": ""}
2023-01-18T12:32:19.107-0800 INFO (sink-sql) starting pprof server {"listen_addr": "localhost:6060"}
2023-01-18T12:32:19.127-0800 INFO (sink-sql) reading substreams manifest {"manifest_path": "sink/substreams.dev.yaml"}
2023-01-18T12:32:20.283-0800 INFO (pipeline) computed start block {"module_name": "store_block_meta_start", "start_block": 0}
2023-01-18T12:32:20.283-0800 INFO (pipeline) computed start block {"module_name": "db_out", "start_block": 0}
2023-01-18T12:32:20.283-0800 INFO (sink-sql) validating output store {"output_store": "db_out"}
2023-01-18T12:32:20.285-0800 INFO (sink-sql) resolved block range {"start_block": 0, "stop_block": 0}
2023-01-18T12:32:20.287-0800 INFO (sink-sql) ready, waiting for signal to quit
2023-01-18T12:32:20.287-0800 INFO (sink-sql) starting stats service {"runs_each": "2s"}
2023-01-18T12:32:20.288-0800 INFO (sink-sql) no block data buffer provided. since undo steps are possible, using default buffer size {"size": 12}
2023-01-18T12:32:20.288-0800 INFO (sink-sql) starting stats service {"runs_each": "2s"}
2023-01-18T12:32:20.730-0800 INFO (sink-sql) session init {"trace_id": "4605d4adbab0831c7505265a0366744c"}
2023-01-18T12:32:21.041-0800 INFO (sink-sql) flushing table rows {"table_name": "block_data", "row_count": 2}
2023-01-18T12:32:21.206-0800 INFO (sink-sql) flushing table rows {"table_name": "block_data", "row_count": 2}
2023-01-18T12:32:21.319-0800 INFO (sink-sql) flushing table rows {"table_name": "block_data", "row_count": 0}
2023-01-18T12:32:21.418-0800 INFO (sink-sql) flushing table rows {"table_name": "block_data", "row_count": 0}
```



**Note**: If you have an error looking like `load psql table: retrieving table and schema: pq: SSL is not enabled on the server`, it's because SSL is not enabled to reach you database, add `?sslmode=disable` at the end of the `sink.config.dsn` value to connect without SSL.

You can view the database structure by using the following command, after launching PostgreSQL through the `psql` command.

```
<default_database_name>=# \c substreams_example
```

The table information is displayed in the terminal resembling the following:

```
           List of relations
 Schema |    Name    | Type  |  Owner
--------+------------+-------+----------
 public | block_data | table | postgres
 public | cursors    | table | postgres
(2 rows)
```

You can view the data extracted by Substreams and routed into the database table by using the following command:

```
substreams_example=# SELECT * FROM "block_data";
```

Output similar to the following is displayed in the terminal:

```
         id         | version |         at          | number |                               hash                               |                           parent_hash                            |      timestamp
--------------------+---------+---------------------+--------+------------------------------------------------------------------+------------------------------------------------------------------+----------------------
 day:first:19700101 |         | 1970-01-01 00:00:00 | 0      | d4e56740f876aef8c010b86a40d5f56745a118d0906a34e69aec8c0db1cb8fa3 | 0000000000000000000000000000000000000000000000000000000000000000 | 1970-01-01T00:00:00Z
 month:first:197001 |         | 1970-01-01 00:00:00 | 0      | d4e56740f876aef8c010b86a40d5f56745a118d0906a34e69aec8c0db1cb8fa3 | 0000000000000000000000000000000000000000000000000000000000000000 | 1970-01-01T00:00:00Z
 day:first:20150730 |         | 2015-07-30 00:00:00 | 1      | 88e96d4537bea4d9c05d12549907b32561d3bf31f45aae734cdc119f13406cb6 | d4e56740f876aef8c010b86a40d5f56745a118d0906a34e69aec8c0db1cb8fa3 | 2015-07-30T15:26:28Z
 month:first:201507 |         | 2015-07-01 00:00:00 | 1      | 88e96d4537bea4d9c05d12549907b32561d3bf31f45aae734cdc119f13406cb6 | d4e56740f876aef8c010b86a40d5f56745a118d0906a34e69aec8c0db1cb8fa3 | 2015-07-30T15:26:28Z
 day:first:20150731 |         | 2015-07-31 00:00:00 | 6912   | ab79f822909750f88dfb9dd0350c1ebe98d5495e9c969cdeb6e0ac993b80175b | 8ffd8c04cb89ef45e0e1163639d51d9ed4fa03dd169db90123a1e047361b46fe | 2015-07-31T00:00:01Z
 day:first:20150801 |         | 2015-08-01 00:00:00 | 13775  | 2dcecad4cf2079d18169ca05bc21e7ba0add7132b9382984760f43f2761bd822 | abaabb8f8b7f7fa07668fb38fd5a08da9814cd8ad18a793e54eef6fa9b794ab4 | 2015-08-01T00:00:03Z
 month:first:201508 |         | 2015-08-01 00:00:00 | 13775  | 2dcecad4cf2079d18169ca05bc21e7ba0add7132b9382984760f43f2761bd822 | abaabb8f8b7f7fa07668fb38fd5a08da9814cd8ad18a793e54eef6fa9b794ab4 | 2015-08-01T00:00:03Z
```

###  

















