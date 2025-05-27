在已创建的 Geth 私有链的基础上，你可以**在另一台机器上新增一个只提供 RPC 服务的节点**，用于承载高并发 RPC 请求。这个节点不需要挖矿，也可以不开启 P2P 出块，只同步区块并提供 HTTP 服务。

------

### ✅ 一、准备条件

1. **你已有一条私链运行中**（假设 networkid 是 `20250521`）。
2. **已有创世区块文件 `genesis.json`**。
3. **主链节点之间互通（IP 可达）**。
4. 新机器安装了 `geth` 和 `docker`（如果使用 Docker）。

------

### ✅ 二、新节点的配置目标

- 不挖矿
- 不出块
- 只作为 RPC 网关
- 可选：不开启 P2P 入站，仅作为轻量节点同步

------

### 🚀 三、部署步骤（以 Docker 为例）

#### 🧩 1. 拷贝创世区块文件

从主链任意节点复制 `genesis.json` 到新机器上。

```bash
scp root@<主节点IP>:/path/to/genesis.json ./genesis.json
```

#### 🧱 2. 初始化链数据目录（一次性操作）

```bash
docker run --rm \
  -v $PWD/data:/data \
  -v $PWD/genesis.json:/genesis.json \
  ethereum/client-go:v1.13.10 \
  init /genesis.json --datadir /data
```

#### ⚙️ 3. 启动 Geth 容器（只提供 RPC，不出块）

```bash
docker run -d --name rpc-node \
  -v $PWD/data:/data \
  -p 8545:8545 \
  ethereum/client-go:v1.13.10 \
  --datadir /data \
  --networkid 20250521 \
  --http \
  --http.addr 0.0.0.0 \
  --http.port 8545 \
  --http.vhosts=* \
  --http.api eth,net,web3 \
  --syncmode=full \
  --bootnodes=enode://....@<主节点IP>:30303 \
  --cache=2048 \
  --maxpeers=5
```

> 注意：
>
> - `--bootnodes` 指定主链中的任意节点（你可以从主节点 `admin.nodeInfo.enode` 获取）。
> - 不设置 `--mine` 和 `--unlock`，表示不参与出块和账户操作。

------

### ✅ 四、验证新节点同步状态

进入容器或 attach：

```bash
docker exec -it rpc-node geth attach /data/geth.ipc
```

查看同步状态：

```js
eth.syncing
eth.blockNumber
net.peerCount
```

------

### ✅ 五、客户端使用这个 RPC 节点

你可以用 `http://<新节点IP>:8545` 对外提供 API，比如用于：

- web3.js
- ethers.js
- curl 测试
- Substreams 流式读取等

------

### ✅ 六、建议优化（高并发使用）

在 Docker 启动命令中添加：

```bash
--rpc.gascap=10000000
--rpc.txfeecap=10
--rpc.batch-request-limit=5000
```

系统层面还建议：

```bash
ulimit -n 1048576
sysctl -w net.core.somaxconn=65535
sysctl -w fs.file-max=2097152
```

------

如你希望我生成完整的 `docker-compose.yml` 来部署这个 RPC 节点，也可以告诉我，我可以快速帮你生成。