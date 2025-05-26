要在 Docker 中部署你的 Geth 节点，并使用你之前提供的 `run.sh` 脚本启动，可以按以下步骤操作：

------

## ✅ 前提条件

确保你已经安装了：

- Docker
- `run.sh` 启动脚本
- Geth 使用的 `genesis.json`（如果是初始化节点）
- 一个用于解锁账户的 `pass.txt`
- 一个数据目录（如 `/data`）

------

## 🧱 项目结构建议（主机目录）

例如我们把所有文件放在主机的 `/root/node100` 下：

```bash
/root/node100/
├── run.sh        # 你写的启动脚本
├── genesis.json  # 创世块配置
├── pass.txt      # 解锁账户密码
└── keystore/     # 钱包密钥文件夹
```

------

## 🚀 一步步操作

### 1. 创建数据目录（如果未创建）

```bash
mkdir -p /root/node100
```

将你之前的 `run.sh`、`genesis.json`、`pass.txt` 等放进去。

### 2. 确保 run.sh 可执行

```bash
chmod +x /root/node100/run.sh
```

### 3. 初始化数据目录（只在首次启动前需要）

```bash
docker run --rm -v /root/node100:/data ethereum/client-go:alltools-v1.13.15 \
  init /data/genesis.json --datadir /data
```

### 4. 启动容器运行脚本

```bash
docker run -d \
  --name node100 \
  -v /root/node100:/data \
  -p 8545:8545 -p 8546:8546 -p 30303:30303 -p 30303:30303/udp \
  ethereum/client-go:alltools-v1.13.15 /data/run.sh
```

------

## ✅ 验证是否启动成功

查看容器日志：

```bash
docker logs -f node100
```

如果看到 Geth 正在出块或者同步，即表示启动成功。

------

## 🛑 停止 / 重启节点

```bash
docker stop node100      # 停止
docker start node100     # 重启
docker restart node100   # 重启（更推荐）
```

------

## 🔁 重建节点（如果 run.sh 修改了）

```bash
docker rm -f node100
docker run -d --name node100 \
  -v /root/node100:/data \
  -p 8545:8545 -p 8546:8546 -p 30303:30303 -p 30303:30303/udp \
  ethereum/client-go:alltools-v1.13.15 /data/run.sh
```

------







practise

### 初始化Geth数据目录

```
docker run --rm -v /Users/jason/Desktop/soft/data/node100:/data ethereum/client-go:alltools-v1.13.15 \
  init /Users/jason/Desktop/soft/data/genesis.json --datadir /Users/jason/Desktop/soft/data
```



导出镜像

```
docker save -o <导出文件名>.tar <镜像名>:<标签>
```

```
docker save -o /data/alltools-v1.13.15.tar ethereum/client-go:alltools-v1.13.15
```

导入镜像

```
docker load -i <导出的文件名>.tar
```

```
docker load -i /Users/jason/Desktop/soft/data/alltools-v1.13.15.tar 
```



















