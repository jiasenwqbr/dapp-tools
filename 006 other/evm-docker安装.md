evm安装

## docker 、docker compose 

在 Ubuntu 上安装 Docker 和 Docker Compose，按照以下步骤操作：

### 一、安装 Docker

#### 1. 更新系统和依赖包

```bash
sudo apt upgrade -y
sudo apt install ca-certificates curl gnupg lsb-release -y
```

#### 2. 添加 Docker 的官方 GPG 密钥

```bash
sudo mkdir -p /etc/apt/keyrings
curl -fsSL https://download.docker.com/linux/ubuntu/gpg | \
  sudo gpg --dearmor -o /etc/apt/keyrings/docker.gpg
```

#### 3. 设置 Docker 仓库

```bash
echo \
  "deb [arch=$(dpkg --print-architecture) signed-by=/etc/apt/keyrings/docker.gpg] \
  https://download.docker.com/linux/ubuntu \
  $(lsb_release -cs) stable" | \
  sudo tee /etc/apt/sources.list.d/docker.list > /dev/null
```

#### 4. 安装 Docker 引擎

```bash
sudo apt update
sudo apt install docker-ce docker-ce-cli containerd.io docker-buildx-plugin docker-compose-plugin -y
```

验证：

```
docker version
```



### 二、安装 Docker Compose（独立版本）

> 如果你使用的是 Docker Desktop 或较新版本的 Docker，`docker compose` 已集成，不需独立安装。但如果需要单独版本，请参考：

#### 1. 下载最新版本（以 v2.24.6 为例）

```bash
DOCKER_COMPOSE_VERSION="v2.24.6"
sudo curl -SL https://github.com/docker/compose/releases/download/${DOCKER_COMPOSE_VERSION}/docker-compose-$(uname -s)-$(uname -m) \
  -o /usr/local/bin/docker-compose
```

#### 2. 赋予执行权限

```bash
sudo chmod +x /usr/local/bin/docker-compose
```

#### 3. 验证安装

```bash
docker --version
docker compose version   # 如果使用 plugin 模式
docker-compose version   # 如果安装的是独立二进制
```

### 三、可选：将当前用户加入 docker 组（无需 sudo）

```bash
sudo usermod -aG docker $USER
newgrp docker  # 或重新登录会话
```

---

如果你已经在运行 Docker Compose 项目（比如你的测试链），那说明安装已成功。如需我帮你写个自动安装脚本，也可以告诉我。



## Bash


### 安装geth
```shell
mkdir -p /blockchain/eth
cd /blockchain/eth
wget https://gethstore.blob.core.windows.net/builds/geth-linux-amd64-1.13.10-bc0be1b1.tar.gz
tar -xvf geth-linux-amd64-1.13.10-bc0be1b1.tar.gz
echo 'export PATH=$PATH:/blockchain/eth/geth-linux-amd64-1.13.10-bc0be1b1' >> ~/.bashrc
source ~/.bashrc
```

### 安装bootnode 可以查看enode链接
```shell
cd /blockchain/eth
git clone -b v1.13.10 https://github.com/ethereum/go-ethereum.git

```

安装make

```shell
sudo apt update
sudo apt install build-essential -y
```

安装go

```shell
wget https://go.dev/dl/go1.22.3.linux-amd64.tar.gz
sudo rm -rf /usr/local/go
sudo tar -C /usr/local -xzf go1.22.3.linux-amd64.tar.gz
echo "export PATH=\$PATH:/usr/local/go/bin" >> ~/.bashrc
source ~/.bashrc
go version
```



```shell
cd go-ethereum
make all
echo 'export PATH=$PATH:/blockchain/eth/go-ethereum/build/bin' >> ~/.bashrc
source ~/.bashrc
```



### 创建目录

01_create_dir.sh

```shell
mkdir nodes && cd nodes
mkdir node0 node1 node2 node3 node4
mkdir -p {node0,node1,node2,node3,node4}/data/geth
```



### 创建解锁密码文件（权限设置）

02_create_passwod.sh

```shell
mkdir -p /env/evm/ && chmod 700 /env/evm/
echo "11111" > /env/evm/password
chmod 400 /env/evm/password
```



### 生成账户（为每个节点创建以太坊账户）

03_create_account.sh

```shell
cd nodes

for i in {0..4}; do
  mkdir -p ./node$i/data/keystore  # 确保目录存在
  geth account new \
    --keystore ./node$i/data/keystore \
    --password /env/evm/password  # 直接使用宿主机上的密码文件
done

```



### 生成节点密钥，用于固定enode链接

04_gen_nodekey.sh

```shell
cd nodes
for i in {0..4}; do
  mkdir -p node$i/data/geth
  openssl rand -hex 32 > node$i/data/geth/nodekey
done

```



###  获取节点链接的前缀

```shell
# 获取节点链接的前缀
for i in {0..4}; do
  bootnode -nodekeyhex $(cat ./nodes/node$i/data/geth/nodekey) -writeaddress
done

```

```cmd
2fc4a6aee3cf94d7e7502d8151a8187f0a22c913837bbfb94639c966d61125709995b14f2330b65e5dc9817a1235f3aa2142070e9e81feba2fee79534c459f36
eb80e6936349cfdc82ad1bc10443e7634879bb8b90b0113a49e7edfc44d6063445f6bf1d589a51e89559077586800848eb1ea750d32a014e219c5a5e1facf482
077af3b95ed8771bda4d3bc3238a92507426bca59309da08100d1ce25d5ef602e6b4bcda1a58e06759771da6b605decb92ef3d9b11773b15ff4329fa540986ef
6df9dac15c70ea67a713b8f4c51d695e39a0e03282e4f7e24f50b0b7ff491fe32ab24c0c03a483d69f1e2667ec1b07c04236c4cc3f21a38dc0d68da335ba8465
e3c3af3457d56ae98e2fa13f60fe737a1a0b8798c5a04eb32f0a2b4b721d5988f3905ef6b9500f361c8097157e17d7d6b7e52729970135d6b6f52c716f25e77f

```



### 初始化创世文件  地址去掉0x 全小写

```shell
cat <<EOF > genesis.json
{
  "config": {
    "chainId": 20250521,
    "terminalTotalDifficulty": 18446744073709551615,
    "homesteadBlock": 0,
    "eip150Block": 0,
    "eip155Block": 0,
    "eip158Block": 0,
    "byzantiumBlock": 0,
    "constantinopleBlock": 0,
    "petersburgBlock": 0,
    "istanbulBlock": 0,
    "clique": {
      "period": 2,
      "epoch": 30000
    }
  },
  "difficulty": "1",
  "gasLimit": "8000000",
  "extradata": "0x000000000000000000000000000000000000000000000000000000000000000062e333555cbe3dfa2d16f4b214ca3ecd420a088db8f72b3b4f783cda7c4c61bb58c16ba92becb405d4a2118da939f23ab9dfa2d91cd819f2135f54880000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000",                          
  "alloc": {
  "0x62e333555cbe3dfa2d16f4b214ca3ecd420a088d": { 
    "balance": "0x20000000000000000000000000000" 
  },
  "0xb8f72b3b4f783cda7c4c61bb58c16ba92becb405": { 
    "balance": "0x20000000000000000000000000000" 
  },
  "0xd4a2118da939f23ab9dfa2d91cd819f2135f5488": { 
    "balance": "0x20000000000000000000000000000" 
  }
}
}
EOF

```





### 节点初始化创世文件
```shell
cd nodes
for i in {0..4}; do
  geth --datadir node$i/data init ../genesis.json
done
```





### dumpconfig

```shell
for i in {0..4}; do
  geth --datadir ./nodes/node$i/data dumpconfig > ./nodes/node$i/data/config.toml
done

```



### 填写staticNode

通过运行：

```shell
root@jason-VMware-Virtual-Platform:/blockchain/eth# ./05_get_node_hex.sh 
06c796b842895967be4b414067d4a7c7c675bf938f9922ec352002153a5087eb2f2259b7805badac2136424ed555c721c3a1f2e6949397580fba7ee942056e8e
3919b14bb30071079a97c9ff28b23a49e6cc06569a7cf1b98c6543c6d26e0f438e5ae61f91bb467a4aa4ee5b16684f5688940a0b983e3e7debdf1d3071f6e42b
88ba0c4fcc172c31b2db85e58d19f3bc95ee268ed1510847ae9e7db81cd4ed41dce4c00d0c05bc2e320dcbe4f4e13f90c65694a5b3181ef2b0066aacbf1ebf0e
72887d37f125dd7cb4722027706ed3c8c8f24fee36f06d2eb8ebeb0e27910af66888b7c4b683f89bc5fdc9ca72095049364bc480af69823812bba2e351db3180
247b1be75ac4d0c02b8453ddff8345ebe0a2b657cd54e1d1db5ab6d061b7461282a804419e1e73a9aa2205a5c1e9b8157cca76a9767a0e04714f6487e7a2b09c

```



```
StaticNodes=[
"enode://87a2e1a230a37f774a5c684a16aeb3f13334bcd6537c7949e4d61110a2a99e828a849f699ebaabb3530a8c736f95362d88d95a1d0d1865dbc0481ee66c041c30@134.122.135.200:30300?discport=0",
"enode://61a2af91ff9a997f01fe1d317c2f1c4de0a7b618e2f40df18daac51c0354638cffbd381ee5aa18367bd8e3a0dadbb833149b595f019d1aa839ebe1daa391b205@134.122.135.200:30301?discport=0",
"enode://41e83b4b32b6e3b2ac48778edd8bef88aef4d26f875b6c575f3d1e241d488368b0a6c14eb1dad3bf65822b6a049d029be39088a576cf7d4dee92c712bb3b4bc9@134.122.135.200:30302?discport=0",
"enode://89bcdde710c9e47b24fea4a4c393d86d394948ef038da9ce6235cea17b800686455c8a71031016bc8b4465e06cb87f995b9b686cc2e56c2f391d84cdc5fb50c6@172.16.238.13:30305?discport=0",
"enode://6511e126addaf457d26f48e7242778fac351dc05a1f1a21b52d6a260176e16b0545470abf7ea807609a7476525d55cea931bf26dea5af9014c19801e39b50ce3@172.16.238.14:30306?discport=0"
]
```





## docker



### yml

```yml
version: '3'


networks:
  evm_net:
    driver: bridge
    ipam:
      config:
        - subnet: 172.16.238.0/24

services:

  #初始节点
  node0:
    
    image: ethereum/client-go:v1.13.10  # 使用官方Geth镜像
    container_name: node0            # 容器名称
    ports:
      #- "8540:8540"                  # HTTP-RPC端口映射（宿主机:容器）
      - "30300:30300/tcp"                # P2P网络端口
      - "30300:30300/udp" 
    volumes:                         # 目录挂载配置（关键参数）
      - ./nodes/node0/data:/data           # 区块链数据目录（持久化存储）
      - ./nodes/node0/keystore:/keystore   # 账户密钥存储目录
      - /env/evm/password:/password
    command:                         # Geth启动参数
      - --port=30300
      - --nat=extip:172.16.238.10
      #- --nodiscover                 # 禁用自动发现
      - --datadir=/data              # 数据存储路径
      - --networkid=20250521            # 网络标识符（必须相同）
      - --syncmode=full              # 全同步模式
      - --mine                       # 启用挖矿（POA出块）
      - --miner.gaslimit=12000000
      - --miner.gasprice=0
      #- --miner.threads=1            # 挖矿线程数
      - --miner.etherbase=0x62e333555cbe3dfa2d16f4b214ca3ecd420a088d      # 出块奖励地址
      - --unlock=0x62e333555cbe3dfa2d16f4b214ca3ecd420a088d               # 解锁地址
      - --password=/password         # 解锁密码
      - --allow-insecure-unlock   # 允许非安全环境解锁
      #- --http                       # 启用HTTP-RPC
      #- --http.port=8540
      #- --http.addr=0.0.0.0          # 监听所有IP
      #- --http.api=eth,net,web3,debug # 开放的API
      #- --http.corsdomain=*
      #- --http.vhosts=*
      #- --vm.mode=archive            # 存档模式（记录所有交易细节）
      ##- --vm.evm=debug               (支持DEBUG)
      - --vmdebug=true
      - --log.rotate 
      - --log.maxsize=50 
      - --log.maxage=7 
      - --log.compress
      - --cache=2048
      - --cache.database=30
      - --cache.trie=25
      - --db.engine=pebble
      - --config=/data/config.toml
      
    networks:
      evm_net:
        ipv4_address: 172.16.238.10  # 固定IP（保证节点稳定互联）
        aliases:  [node0]


  #节点1 只进行同步以及验证，不启用RPC
  node1:
    
    image: ethereum/client-go:v1.13.10
    container_name: node1
    ports:
      - "30301:30301/tcp"
      - "30301:30301/udp"
    volumes:                         # 目录挂载配置（关键参数）
      - ./nodes/node1/data:/data           # 区块链数据目录（持久化存储）
      - ./nodes/node1/keystore:/keystore   # 账户密钥存储目录
      - /env/evm/password:/password
    command:                         # Geth启动参数
      - --nat=extip:172.16.238.11
      - --port=30301
      #- --nodiscover                 # 禁用自动发现
      - --datadir=/data              # 数据存储路径
      - --networkid=20250521            # 网络标识符（必须相同）
      - --syncmode=full              # 全同步模式
      - --mine                       # 启用挖矿（POA出块）
      - --miner.gasprice=0
      - --miner.gaslimit=12000000
      #- --miner.threads=1            # 挖矿线程数
      - --miner.etherbase=0xb8f72b3b4f783cda7c4c61bb58c16ba92becb405      # 出块奖励地址
      - --unlock=0xb8f72b3b4f783cda7c4c61bb58c16ba92becb405               # 解锁地址
      - --password=/password         # 解锁密码
      - --allow-insecure-unlock   # 允许非安全环境解锁
      #- --http                       # 启用HTTP-RPC
      #- --http.port=8540
      #- --http.addr=0.0.0.0          # 监听所有IP
      #- --http.api=eth,net,web3,debug # 开放的API
      #- --http.corsdomain=*
      #- --http.vhosts=*
      #- --vm.mode=archive            # 存档模式（记录所有交易细节）
      #- --vm.evm=debug               #(支持DEBUG)
      - --vmdebug=true
      - --log.rotate 
      - --log.maxsize=50 
      - --log.maxage=7 
      - --log.compress
      - --cache=2048
      - --cache.database=30
      - --cache.trie=25
      - --db.engine=pebble
      - --config=/data/config.toml
      
    networks:
      evm_net:
        ipv4_address: 172.16.238.11  # 固定IP（保证节点稳定互联）
        aliases:  [node1]

  #节点2 只进行同步以及验证，不启用RPC
  node2:
    
    image: ethereum/client-go:v1.13.10
    container_name: node2
    ports:
      - "30302:30302/tcp"
      - "30302:30302/udp"
    volumes:                         # 目录挂载配置（关键参数）
      - ./nodes/node2/data:/data           # 区块链数据目录（持久化存储）
      - ./nodes/node2/keystore:/keystore   # 账户密钥存储目录
      - /env/evm/password:/password
    command:                         # Geth启动参数
      - --nat=extip:172.16.238.12
      - --port=30302
      #- --nodiscover
      - --datadir=/data              # 数据存储路径
      - --networkid=20250521            # 网络标识符（必须相同）
      - --syncmode=full              # 全同步模式
      - --mine                       # 启用挖矿（POA出块）
      - --miner.gaslimit=12000000
      - --miner.gasprice=0
      ##- --miner.threads=1            # 挖矿线程数
      - --miner.etherbase=0xd4a2118da939f23ab9dfa2d91cd819f2135f5488      # 出块奖励地址
      - --unlock=0xd4a2118da939f23ab9dfa2d91cd819f2135f5488               # 解锁地址
      - --password=/password         # 解锁密码
      - --allow-insecure-unlock      # 允许非安全环境解锁
      #- --vm.mode=archive            # 存档模式（记录所有交易细节）
      #- --vm.evm=debug               #(支持DEBUG)
      - --vmdebug=true
      - --log.rotate 
      - --log.maxsize=50 
      - --log.maxage=7 
      - --log.compress
      - --cache=2048
      - --cache.database=30
      - --cache.trie=25
      - --db.engine=pebble
      - --config=/data/config.toml
      
    networks:
      evm_net:
        ipv4_address: 172.16.238.12  # 固定IP（保证节点稳定互联）
        aliases:  [node2]

  node3:
    
    image: ethereum/client-go:v1.13.10  # 使用官方Geth镜像
    container_name: node3            # 容器名称
    ports:
      - "8543:8543"                  # HTTP-RPC端口映射（宿主机:容器）
      - "30303:30303/tcp"                # P2P网络端口
      - "30303:30303/udp"                # P2P网络端口
    volumes:                         # 目录挂载配置（关键参数）
      - ./nodes/node3/data:/data           # 区块链数据目录（持久化存储）
      - ./nodes/node3/keystore:/keystore   # 账户密钥存储目录
      - /env/evm/password:/password
    command:                         # Geth启动参数
      - --nat=extip:172.16.238.13
      - --port=30303
      #- --nodiscover                 # 禁用自动发现
      - --datadir=/data              # 数据存储路径
      - --networkid=20250521            # 网络标识符（必须相同）
      - --syncmode=full              # 全同步模式
      #- --mine                       # 启用挖矿（POA出块）
      ##- --miner.threads=1            # 挖矿线程数
      #- --miner.etherbase=0xb1b9ccC71C564A47065c2B3fD5ee865d7AFB928b      # 出块奖励地址
      #- --unlock=0xb1b9ccC71C564A47065c2B3fD5ee865d7AFB928b               # 解锁地址
      #- --password=/password         # 解锁密码
      #- --allow-insecure-unlock   # 允许非安全环境解锁
      - --http                       # 启用HTTP-RPC
      - --http.port=8543
      - --http.addr=0.0.0.0          # 监听所有IP
      - --http.api=eth,net,web3 # 开放的API
      - --http.corsdomain=*
      - --http.vhosts=*
      - --rpc.gascap=10000000
      - --rpc.txfeecap=10
      - --rpc.batch-request-limit=500
      ##- --vm.mode=archive            # 存档模式（记录所有交易细节）
      ##- --vm.evm=debug               (支持DEBUG)
      - --vmdebug=true
      - --log.rotate 
      - --log.maxsize=50 
      - --log.maxage=7 
      - --log.compress
      - --cache=2048
      - --cache.database=30
      - --cache.trie=25
      - --db.engine=pebble
      - --config=/data/config.toml
      
    networks:
      evm_net:
        ipv4_address: 172.16.238.13  # 固定IP（保证节点稳定互联）
        aliases:  [node3]
  
  node4:
    
    image: ethereum/client-go:v1.13.10  # 使用官方Geth镜像
    container_name: node4            # 容器名称
    ports:
      - "8544:8544"                  # HTTP-RPC端口映射（宿主机:容器）
      - "30304:30304/tcp"                # P2P网络端口
      - "30304:30304/udp"                # P2P网络端口
    volumes:                         # 目录挂载配置（关键参数）
      - ./nodes/node4/data:/data           # 区块链数据目录（持久化存储）
      - ./nodes/node4/keystore:/keystore   # 账户密钥存储目录
      - /env/evm/password:/password
    command:                         # Geth启动参数
      - --nat=extip:172.16.238.14
      - --port=30304
      #- --nodiscover                 # 禁用自动发现
      - --datadir=/data              # 数据存储路径
      - --networkid=20250521            # 网络标识符（必须相同）
      - --syncmode=full              # 全同步模式
      #- --mine                       # 启用挖矿（POA出块）
      ##- --miner.threads=1            # 挖矿线程数
      #- --miner.etherbase=0x33F1Db858b2D11292b26ADbd5E25ce9345BA7Cd7      # 出块奖励地址
      #- --unlock=0x33F1Db858b2D11292b26ADbd5E25ce9345BA7Cd7               # 解锁地址
      #- --password=/password         # 解锁密码
      #- --allow-insecure-unlock   # 允许非安全环境解锁
      - --http                       # 启用HTTP-RPC
      - --http.port=8544
      - --http.addr=0.0.0.0          # 监听所有IP
      - --http.api=eth,net,web3 # 开放的API
      - --http.corsdomain=*
      - --http.vhosts=*
      ##- --vm.mode=archive            # 存档模式（记录所有交易细节）
      ##- --vm.evm=debug               (支持DEBUG)
      - --rpc.gascap=10000000
      - --rpc.txfeecap=10
      - --rpc.batch-request-limit=500
      - --vmdebug=true
      - --log.rotate 
      - --log.maxsize=50 
      - --log.maxage=7 
      - --log.compress
      - --cache=2048
      - --cache.database=30
      - --cache.trie=25
      - --db.engine=pebble
      - --config=/data/config.toml
      
    networks:
      evm_net:
        ipv4_address: 172.16.238.14  # 固定IP（保证节点稳定互联）
        aliases:  [node4]
```



### 操作



#### 启动：

```shell
docker-compose up      （-d 不占用当前screen）
```



#### 停止：

```shell
docker-compose stop  (dockerName  不填就停止所有)
```



#### 重启： 

```shell
docker-compose restart
```

#### 删除内在文件

```shell
# 停止并删除所有容器、网络（保留卷）
docker-compose down

# 如果要同时删除卷（永久删除数据）
docker-compose down -v

# 如果要删除所有内容（容器、网络、卷和镜像）
docker-compose down --rmi all -v
```





#### 链接geth控制台：

```shell
docker exec -it <dockerName>  geth attach /data/geth.ipc
```

docker exec -it node0  geth attach /data/geth.ipc



查看日志：

```shell
docker-compose logs -f
docker-compose logs -f --tail 50 node0
docker-compose logs --tail=100 node0
```





常用命令：

以下是 **Geth 控制台常用命令** 的完整分类指南，涵盖账户管理、区块查询、交易操作、网络监控等场景：

##### **1. 基础信息查询**

| 命令                 | 功能             | 示例                 |
| -------------------- | ---------------- | -------------------- |
| `eth.blockNumber`    | 查看最新区块高度 | `eth.blockNumber`    |
| `eth.syncing`        | 检查同步状态     | `eth.syncing`        |
| `net.version`        | 查看网络ID       | `net.version`        |
| `net.peerCount`      | 查看连接节点数   | `net.peerCount`      |
| `web3.clientVersion` | 查看客户端版本   | `web3.clientVersion` |

##### **2. 账户与钱包管理**

| 命令                        | 功能           | 示例                                        |
| --------------------------- | -------------- | ------------------------------------------- |
| `eth.accounts`              | 列出所有账户   | `eth.accounts`                              |
| `personal.listWallets`      | 查看已解锁账户 | `personal.listWallets`                      |
| `personal.newAccount()`     | 创建新账户     | `personal.newAccount("密码")`               |
| `personal.unlockAccount()`  | 解锁账户       | `personal.unlockAccount(addr, "密码", 300)` |
| `eth.getBalance()`          | 查询余额       | `eth.getBalance("0x...")`                   |
| `eth.getTransactionCount()` | 查询交易数     | `eth.getTransactionCount("0x...")`          |

##### **3. 区块与交易操作**

| 命令                          | 功能           | 示例                                                         |
| ----------------------------- | -------------- | ------------------------------------------------------------ |
| `eth.getBlock()`              | 获取区块信息   | `eth.getBlock("latest")`                                     |
| `eth.getTransaction()`        | 查询交易详情   | `eth.getTransaction("0xhash...")`                            |
| `eth.getTransactionReceipt()` | 查询交易收据   | `eth.getTransactionReceipt("0xhash...")`                     |
| `eth.sendTransaction()`       | 发送交易       | `eth.sendTransaction({from:addr, to:addr, value:web3.toWei(1)})` |
| `txpool.status`               | 查看交易池状态 | `txpool.status`                                              |

##### **4. 挖矿控制（PoW/PoA）**

| 命令                   | 功能                  | 示例                                  |
| ---------------------- | --------------------- | ------------------------------------- |
| `miner.start()`        | 启动挖矿              | `miner.start(1)`                      |
| `miner.stop()`         | 停止挖矿              | `miner.stop()`                        |
| `miner.setEtherbase()` | 设置挖矿收益地址      | `miner.setEtherbase(eth.accounts[0])` |
| `clique.getSigners()`  | （POA）查看签名者     | `clique.getSigners()`                 |
| `clique.propose()`     | （POA）投票修改签名者 | `clique.propose("0xaddr", true)`      |

##### **5. 合约交互**

| 命令                                 | 功能           | 示例                                                         |
| ------------------------------------ | -------------- | ------------------------------------------------------------ |
| `eth.getCode()`                      | 获取合约字节码 | `eth.getCode("0xcontractAddr")`                              |
| `eth.estimateGas()`                  | 估算Gas消耗    | `eth.estimateGas({to:"0x...", data:"0x..."})`                |
| `contract.methods.myMethod().call()` | 调用只读方法   | `myContract.methods.balanceOf(addr).call()`                  |
| `contract.methods.myMethod().send()` | 发送合约交易   | `myContract.methods.transfer(addr, 100).send({from:sender})` |

##### **6. 网络与调试**

| 命令                       | 功能         | 示例                                   |
| -------------------------- | ------------ | -------------------------------------- |
| `admin.peers`              | 查看对等节点 | `admin.peers`                          |
| `admin.addPeer()`          | 手动添加节点 | `admin.addPeer("enode://...@ip:port")` |
| `debug.traceTransaction()` | 追踪交易执行 | `debug.traceTransaction("0xhash")`     |
| `debug.verbosity()`        | 设置日志级别 | `debug.verbosity(3)`                   |

##### **7. 单位转换**

| 命令             | 功能             | 示例                                  |
| ---------------- | ---------------- | ------------------------------------- |
| `web3.fromWei()` | Wei → Ether      | `web3.fromWei("1000000000000000000")` |
| `web3.toWei()`   | Ether → Wei      | `web3.toWei("1", "ether")`            |
| `web3.toHex()`   | 十进制转十六进制 | `web3.toHex(255)`                     |

##### **8. 高级管理**

| 命令                  | 功能                   | 示例                                    |
| --------------------- | ---------------------- | --------------------------------------- |
| `admin.datadir`       | 查看数据目录           | `admin.datadir`                         |
| `admin.exportChain()` | 导出区块链数据         | `admin.exportChain("/path/backup.txt")` |
| `admin.setSolc()`     | 设置Solidity编译器路径 | `admin.setSolc("/usr/bin/solc")`        |

##### **9. 实用脚本片段**

###### **批量转账**

```javascript
function batchTransfer(toList, amount) {
  toList.forEach(addr => {
    eth.sendTransaction({from:eth.accounts[0], to:addr, value:web3.toWei(amount)})
  });
}
```

###### **监控新区块**

```javascript
function watchBlocks() {
  eth.filter("latest", (err, block) => {
    console.log("New block:", eth.getBlock(block).number);
  });
}
```

##### **10. 退出控制台**

| 命令               | 功能       |
| ------------------ | ---------- |
| `exit` 或 `ctrl+D` | 退出控制台 |

##### **注意事项**

1. **POA链专用命令**：`clique.*` 仅适用于 Clique 共识链
2. **账户安全**：生产环境避免在控制台明文输入密码
3. **Gas设置**：交易前建议用 `eth.estimateGas()` 估算
4. **异步操作**：部分命令返回 Promise，可用 `.then()` 处理

掌握这些命令后，您可以高效管理节点、调试合约和监控网络状态。如需更复杂操作，可结合 [Web3.js](https://web3js.readthedocs.io/) 或 [Ethers.js](https://docs.ethers.org/) 库开发脚本。



#### 链接bash：

```shell
docker exec -it <dockerName> /bin/bash
```

docker exec -it   /bin/bash

#### 查看各个节点区块



#### **安装 `expect` 工具**

在 Ubuntu/Debian 系统上运行：

```
sudo apt update
sudo apt install expect -y
```

运行脚本：

```shell
#!/usr/bin/expect -f

for {set i 0} {$i <= 4} {incr i} {
    puts "---- Node $i ----"
    spawn docker exec -it node$i geth attach /data/geth.ipc
    expect ">"
    send "eth.blockNumber\n"
    expect ">"
    send "exit\n"
    expect eof
}
```





### 解析私钥

```python
from web3 import Web3
import json

# 1. 读取 keystore 文件内容
with open('/blockchain/eth/node2.json') as f:
    keystore = json.load(f)

# 2. 输入密码
password = ''

# 3. 解密
private_key = Web3().eth.account.decrypt(keystore, password)
print(private_key.hex())
```

### 





##



docker run -d --name nginx-rpc-proxy -p 80:80  -v /blockchain/eth/nginx/nginx.conf:/etc/nginx/conf nginx



### 测试

#### rpc

```
curl -X POST \
  -H "Content-Type: application/json" \
  --data '{"jsonrpc":"2.0","method":"eth_blockNumber","params":[],"id":1}' \
  https://chain.pijswap.com

```

```

```

#### websocket

```
curl --include \
     --no-buffer \
     --header "Connection: Upgrade" \
     --header "Upgrade: websocket" \
     --header "Sec-WebSocket-Key: SGVsbG8sIHdvcmxkIQ==" \
     --header "Sec-WebSocket-Version: 13" \
     http://127.0.0.1:8546
```



```yml
  node5:
    
    image: ethereum/client-go:v1.13.10  # 使用官方Geth镜像
    container_name: node5            # 容器名称
    ports:
      - "8543:8543"                  # HTTP-RPC端口映射（宿主机:容器）
      - "30305:30305/tcp"                # P2P网络端口
      - "30305:30305/udp"                # P2P网络端口
      - "8546:8546"
    volumes:                         # 目录挂载配置（关键参数）
      - ./nodes/node5/data:/data           # 区块链数据目录（持久化存储）
      - ./nodes/node5/keystore:/keystore   # 账户密钥存储目录
      - /env/evm/password:/password
    command:                         # Geth启动参数
      - --nat=extip:172.16.238.15
      - --port=30305
      #- --nodiscover                 # 禁用自动发现
      - --datadir=/data              # 数据存储路径
      - --networkid=31419            # 网络标识符（必须相同）
      - --syncmode=full              # 全同步模式
      #- --mine                       # 启用挖矿（POA出块）
      ##- --miner.threads=1            # 挖矿线程数
      #- --miner.etherbase=0xb1b9ccC71C564A47065c2B3fD5ee865d7AFB928b      # 出块奖励地址
      #- --unlock=0xb1b9ccC71C564A47065c2B3fD5ee865d7AFB928b               # 解锁地址
      #- --password=/password         # 解锁密码
      #- --allow-insecure-unlock   # 允许非安全环境解锁
      - --ws                     # 启用 WebSocket
      - --ws.port=8546           # WebSocket 端口
      - --ws.addr=0.0.0.0        # 允许所有 IP 连接
      - --ws.api=eth,net,web3    # 开放的 API
      - --ws.origins=*         # 允许跨域
      - --http                       # 启用HTTP-RPC
      - --http.port=8543
      - --http.addr=0.0.0.0          # 监听所有IP
      - --http.api=eth,net,web3 # 开放的API
      - --http.corsdomain=*
      - --http.vhosts=*
      - --rpc.gascap=10000000
      - --rpc.txfeecap=10
      - --rpc.batch-request-limit=500
      ##- --vm.mode=archive            # 存档模式（记录所有交易细节）
      ##- --vm.evm=debug               (支持DEBUG)
      - --vmdebug=true
      - --log.rotate 
      - --log.maxsize=50 
      - --log.maxage=7 
      - --log.compress
      - --cache=2048
      - --cache.database=30
      - --cache.trie=25
      - --db.engine=pebble
      - --config=/data/config.toml
      
    networks:
      evm_net:
        ipv4_address: 172.16.238.15  # 固定IP（保证节点稳定互联）
        aliases:  [node5]
```





### 优化

编辑 `/etc/sysctl.conf` 添加如下配置（或用 `sysctl -w` 临时设置）：

```bash
fs.file-max = 1048576
net.core.somaxconn = 65535
net.core.netdev_max_backlog = 65535
net.ipv4.tcp_max_syn_backlog = 65535
net.ipv4.tcp_tw_reuse = 1
net.ipv4.ip_local_port_range = 1024 65000

```

然后运行：

```

sudo sysctl -p
```





### compose.yml文件

```yml
version: '3'


networks:
  evm_net:
    driver: bridge
    ipam:
      config:
        - subnet: 172.16.238.0/24

services:

  #初始节点
  node0:
    
    image: ethereum/client-go:v1.13.10  # 使用官方Geth镜像
    container_name: node0            # 容器名称
    ports:
      #- "8540:8540"                  # HTTP-RPC端口映射（宿主机:容器）
      - "30300:30300/tcp"                # P2P网络端口
      - "30300:30300/udp" 
    volumes:                         # 目录挂载配置（关键参数）
      - ./nodes/node0/data:/data           # 区块链数据目录（持久化存储）
      - ./nodes/node0/keystore:/keystore   # 账户密钥存储目录
      - /env/evm/password:/password
    command:                         # Geth启动参数
      - --port=30300
      - --nat=extip:172.16.238.10
      #- --nodiscover                 # 禁用自动发现
      - --datadir=/data              # 数据存储路径
      - --networkid=20250521            # 网络标识符（必须相同）
      - --syncmode=full              # 全同步模式
      #- --mine                       # 启用挖矿（POA出块）
      #- --miner.gaslimit=10000000
      #- --miner.gasprice=0
      #- --miner.threads=1            # 挖矿线程数
      #- --miner.etherbase=0xc29c2e39c9b2a4ad1a50f727e9ab843c27691a1c      # 出块奖励地址
      #- --unlock=0xc29c2e39c9b2a4ad1a50f727e9ab843c27691a1c              # 解锁地址
      #- --password=/password         # 解锁密码
      #- --allow-insecure-unlock   # 允许非安全环境解锁
      #- --http                       # 启用HTTP-RPC
      #- --http.port=8540
      #- --http.addr=0.0.0.0          # 监听所有IP
      #- --http.api=eth,net,web3,debug # 开放的API
      #- --http.corsdomain=*
      #- --http.vhosts=*
      #- --vm.mode=archive            # 存档模式（记录所有交易细节）
      ##- --vm.evm=debug               (支持DEBUG)
      - --vmdebug=true
      - --log.rotate 
      - --log.maxsize=50 
      - --log.maxage=7 
      - --log.compress
      - --cache=2048
      - --cache.database=30
      - --cache.trie=25
      - --db.engine=pebble
      - --config=/data/config.toml
      
    networks:
      evm_net:
        ipv4_address: 172.16.238.10  # 固定IP（保证节点稳定互联）
        aliases:  [node0]


  #节点1 只进行同步以及验证，不启用RPC
  node1:
    
    image: ethereum/client-go:v1.13.10
    container_name: node1
    ports:
      - "30301:30301/tcp"
      - "30301:30301/udp"
    volumes:                         # 目录挂载配置（关键参数）
      - ./nodes/node1/data:/data           # 区块链数据目录（持久化存储）
      - ./nodes/node1/keystore:/keystore   # 账户密钥存储目录
      - /env/evm/password:/password
    command:                         # Geth启动参数
      - --nat=extip:172.16.238.11
      - --port=30301
      #- --nodiscover                 # 禁用自动发现
      - --datadir=/data              # 数据存储路径
      - --networkid=20250521            # 网络标识符（必须相同）
      - --syncmode=full              # 全同步模式
      - --mine                       # 启用挖矿（POA出块）
      - --miner.gasprice=0
      - --miner.gaslimit=10000000
      #- --miner.threads=1            # 挖矿线程数
      - --miner.etherbase=0x79c097cf77a557e7ec020249bc9ab289e77ba7a0      # 出块奖励地址
      - --unlock=0x79c097cf77a557e7ec020249bc9ab289e77ba7a0               # 解锁地址
      - --password=/password         # 解锁密码
      - --allow-insecure-unlock   # 允许非安全环境解锁
      #- --http                       # 启用HTTP-RPC
      #- --http.port=8540
      #- --http.addr=0.0.0.0          # 监听所有IP
      #- --http.api=eth,net,web3,debug # 开放的API
      #- --http.corsdomain=*
      #- --http.vhosts=*
      #- --vm.mode=archive            # 存档模式（记录所有交易细节）
      #- --vm.evm=debug               #(支持DEBUG)
      - --vmdebug=true
      - --log.rotate 
      - --log.maxsize=50 
      - --log.maxage=7 
      - --log.compress
      - --cache=2048
      - --cache.database=30
      - --cache.trie=25
      - --db.engine=pebble
      - --config=/data/config.toml
      
    networks:
      evm_net:
        ipv4_address: 172.16.238.11  # 固定IP（保证节点稳定互联）
        aliases:  [node1]

  #节点2 只进行同步以及验证，不启用RPC
  node2:
    
    image: ethereum/client-go:v1.13.10
    container_name: node2
    ports:
      - "30302:30302/tcp"
      - "30302:30302/udp"
    volumes:                         # 目录挂载配置（关键参数）
      - ./nodes/node2/data:/data           # 区块链数据目录（持久化存储）
      - ./nodes/node2/keystore:/keystore   # 账户密钥存储目录
      - /env/evm/password:/password
    command:                         # Geth启动参数
      - --nat=extip:172.16.238.12
      - --port=30302
      #- --nodiscover
      - --datadir=/data              # 数据存储路径
      - --networkid=20250521            # 网络标识符（必须相同）
      - --syncmode=full              # 全同步模式
      #- --mine                       # 启用挖矿（POA出块）
      #- --miner.gaslimit=10000000
      #- --miner.gasprice=0
      ##- --miner.threads=1            # 挖矿线程数
      #- --miner.etherbase=0x1769c2c911c6caa870fd6a01ae2173c216110a30      # 出块奖励地址
      #- --unlock=0x1769c2c911c6caa870fd6a01ae2173c216110a30               # 解锁地址
      #- --password=/password         # 解锁密码
      #- --allow-insecure-unlock      # 允许非安全环境解锁
      #- --vm.mode=archive            # 存档模式（记录所有交易细节）
      #- --vm.evm=debug               #(支持DEBUG)
      - --vmdebug=true
      - --log.rotate 
      - --log.maxsize=50 
      - --log.maxage=7 
      - --log.compress
      - --cache=2048
      - --cache.database=30
      - --cache.trie=25
      - --db.engine=pebble
      - --config=/data/config.toml
      
    networks:
      evm_net:
        ipv4_address: 172.16.238.12  # 固定IP（保证节点稳定互联）
        aliases:  [node2]

  node3:
    
    image: ethereum/client-go:v1.13.10  # 使用官方Geth镜像
    container_name: node3            # 容器名称
    ports:
      - "8543:8543"                  # HTTP-RPC端口映射（宿主机:容器）
      - "30303:30303/tcp"                # P2P网络端口
      - "30303:30303/udp"                # P2P网络端口
    volumes:                         # 目录挂载配置（关键参数）
      - ./nodes/node3/data:/data           # 区块链数据目录（持久化存储）
      - ./nodes/node3/keystore:/keystore   # 账户密钥存储目录
      - /env/evm/password:/password
    ulimits:
      nofile:
        soft: 1048576
        hard: 1048576
    command:                         # Geth启动参数
      - --nat=extip:172.16.238.13
      - --port=30303
      #- --nodiscover                 # 禁用自动发现
      - --datadir=/data              # 数据存储路径
      - --networkid=20250521            # 网络标识符（必须相同）
      - --syncmode=full              # 全同步模式
      #- --mine                       # 启用挖矿（POA出块）
      ##- --miner.threads=1            # 挖矿线程数
      #- --miner.etherbase=0xb1b9ccC71C564A47065c2B3fD5ee865d7AFB928b      # 出块奖励地址
      #- --unlock=0xb1b9ccC71C564A47065c2B3fD5ee865d7AFB928b               # 解锁地址
      #- --password=/password         # 解锁密码
      #- --allow-insecure-unlock   # 允许非安全环境解锁
      - --http                       # 启用HTTP-RPC
      - --http.port=8543
      - --http.addr=0.0.0.0          # 监听所有IP
      - --http.api=eth,net,web3,txpool,debug # 开放的API
      - --http.corsdomain=*
      - --http.vhosts=*
      - --maxpeers=50                     # 提高最大对等节点数
      - --rpc.gascap=10000000
      - --rpc.txfeecap=10
      - --rpc.batch-request-limit=5000
      ##- --vm.mode=archive            # 存档模式（记录所有交易细节）
      ##- --vm.evm=debug               (支持DEBUG)
      - --vmdebug=true
      - --log.rotate 
      - --log.maxsize=50 
      - --log.maxage=7 
      - --log.compress
      - --cache=4096
      - --cache.database=60
      - --cache.trie=25
      - --db.engine=pebble
      - --config=/data/config.toml
       
      
    networks:
      evm_net:
        ipv4_address: 172.16.238.13  # 固定IP（保证节点稳定互联）
        aliases:  [node3]
  
  node4:
    
    image: ethereum/client-go:v1.13.10  # 使用官方Geth镜像
    container_name: node4            # 容器名称
    ports:
      - "8544:8544"                  # HTTP-RPC端口映射（宿主机:容器）
      - "30304:30304/tcp"                # P2P网络端口
      - "30304:30304/udp"                # P2P网络端口
    volumes:                         # 目录挂载配置（关键参数）
      - ./nodes/node4/data:/data           # 区块链数据目录（持久化存储）
      - ./nodes/node4/keystore:/keystore   # 账户密钥存储目录
      - /env/evm/password:/password
    command:                         # Geth启动参数
      - --nat=extip:172.16.238.14
      - --port=30304
      #- --nodiscover                 # 禁用自动发现
      - --datadir=/data              # 数据存储路径
      - --networkid=20250521            # 网络标识符（必须相同）
      - --syncmode=full              # 全同步模式
      #- --mine                       # 启用挖矿（POA出块）
      ##- --miner.threads=1            # 挖矿线程数
      #- --miner.etherbase=0x33F1Db858b2D11292b26ADbd5E25ce9345BA7Cd7      # 出块奖励地址
      #- --unlock=0x33F1Db858b2D11292b26ADbd5E25ce9345BA7Cd7               # 解锁地址
      #- --password=/password         # 解锁密码
      #- --allow-insecure-unlock   # 允许非安全环境解锁
      - --http                       # 启用HTTP-RPC
      - --http.port=8544
      - --http.addr=0.0.0.0          # 监听所有IP
      - --http.api=eth,net,web3 # 开放的API
      - --http.corsdomain=*
      - --http.vhosts=*
      ##- --vm.mode=archive            # 存档模式（记录所有交易细节）
      ##- --vm.evm=debug               (支持DEBUG)
      - --rpc.gascap=10000000
      - --rpc.txfeecap=10
      - --rpc.batch-request-limit=500
      - --vmdebug=true
      - --log.rotate 
      - --log.maxsize=50 
      - --log.maxage=7 
      - --log.compress
      - --cache=2048
      - --cache.database=30
      - --cache.trie=25
      - --db.engine=pebble
      - --config=/data/config.toml
      
    networks:
      evm_net:
        ipv4_address: 172.16.238.14  # 固定IP（保证节点稳定互联）
        aliases:  [node4]
```



### 其他

#### python

1. 更新包管理器

```bash

sudo apt update
sudo apt upgrade
```

2. 安装 Python3 和 pip（Python 包管理器）

```bash
sudo apt install python3 python3-pip -y
```

3. 验证安装

```bash
python3 --version
pip3 --version
```

测试ws

```python
import websocket

headers = {
    "Origin": "http://localhost"
}

ws = websocket.WebSocket()
ws.connect("ws://172.16.238.15:8546", header=headers)

ws.send('{"jsonrpc":"2.0","method":"eth_blockNumber","params":[],"id":1}')
print(ws.recv())
ws.close()
```



#### nginx映射rpc端口

```shell
docker run -p 80:80 --name mynginx -v D://docker//ngnix//html:/www -v D://docker//ngnix/conf/nginx.conf:/etc/nginx/nginx.conf -v D://docker//ngnix//logs:/wwwlogs -d nginx
```



```
worker_processes 1;

events {
    worker_connections 1024;
}

http {
    include       mime.types;
    default_type  application/json;

    sendfile        on;
    keepalive_timeout  65;

    server {
        listen       80;
        server_name  172.20.31.153;

        location / {
            proxy_pass http://192.168.10.132:8543;  # Docker 容器访问宿主机的方式（Mac/Windows）
            proxy_http_version 1.1;

            proxy_set_header Host $host;
            proxy_set_header X-Real-IP $remote_addr;
            proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
            proxy_set_header X-Forwarded-Proto $scheme;

            proxy_pass_request_headers on;
            proxy_pass_request_body on;
        }
    }
}

```











