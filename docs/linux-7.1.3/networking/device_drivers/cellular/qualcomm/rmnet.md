
## Rmnet 驱动


## 1. 简

rmnet 驱动用于支持复用与聚合协议（MAP，Multiplexing and aggregation Protocol）该协议被所有使Qualcomm Technologies, Inc. 调制解调器的最新芯片组所采用
本驱动可用于注册到任意处IP 模式的物理网络设备。物理传输包USB、HSICPCIe IP accelerator
复用允许创建逻辑网络设备（rmnet 设备）来处理多个私有数据网络（PDN），例如默认
互联网、网络共享（tethering）、彩信服务（MMS）或 IP 媒体子系统（IMS）。硬件将
带有 MAP 头的包发送给 rmnet。rmnet 根据复用id，在去除 MAP 头后将包路由相应PDN
要达到高数据速率需要聚合。这涉及硬件发送聚合的一MAP 帧。rmnet 驱动会将这些
MAP 帧去聚合，并发送到相应PDN
## 2. 包格

### a. MAP packet v1（数/ 控制

MAP 头字段采用大端（big endian）格式
```

  Bit             0             1           2-7      8-15           16-31
  Function   Command / Data   Reserved     Pad   Multiplexer ID    Payload length

  Bit            32-x
  Function      Raw bytes

```
Command (1)/ Data (0) 位用于指示该包是 MAP 命令包还是数据包。命令包用于传输层流控数据包是标准 IP 包
保留位发送时必须为零，接收时忽略
Padding 是追加到载荷末尾以确4 字节对齐的字节数
Multiplexer ID 用于指示数据要发送到PDN
载荷长度包含 padding 长度，但不包MAP 头长度
### b. Map packet v4（数/ 控制

MAP 头字段采用大端格式
```

  Bit             0             1           2-7      8-15           16-31
  Function   Command / Data   Reserved     Pad   Multiplexer ID    Payload length

  Bit            32-(x-33)      (x-32)-x
  Function      Raw bytes      Checksum offload header

```
Command (1)/ Data (0) 位用于指示该包是 MAP 命令包还是数据包。命令包用于传输层流控数据包是标准 IP 包
保留位发送时必须为零，接收时忽略
Padding 是追加到载荷末尾以确4 字节对齐的字节数
Multiplexer ID 用于指示数据要发送到PDN
载荷长度包含 padding 长度，但不包MAP 头长度
Checksum offload 头包含硬件所完成校验和处理的信息。Checksum offload 头字段采大端格式
```

  Bit             0-14        15              16-31
  Function      Reserved   Valid     Checksum start offset

  Bit                31-47                    48-64
  Function      Checksum length           Checksum value

```
保留位发送时必须为零，接收时忽略
Valid 位指示部分校验和是否已被计算且有效。若有效则置 1，否则置 0
Padding 是追加到载荷末尾以确4 字节对齐的字节数
Checksum start offset（校验和起始偏移）指示从 IP 头起始处起的字节偏移，调制解调器
从该偏移开始计算校验和
Checksum length（校验和长度）是CKSUM_START_OFFSET 起始的、计算了校验和的字节
长度
Checksum value（校验和值）指示计算得到的校验和
### c. MAP packet v5（数/ 控制

MAP 头字段采用大端格式
```

  Bit             0             1         2-7      8-15           16-31
  Function   Command / Data  Next header  Pad   Multiplexer ID   Payload length

  Bit            32-x
  Function      Raw bytes

```
Command (1)/ Data (0) 位用于指示该包是 MAP 命令包还是数据包。命令包用于传输层流控数据包是标准 IP 包
Next header 用于指示是否存在另一个头，目前仅限于校验和头
Padding 是追加到载荷末尾以确4 字节对齐的字节数
Multiplexer ID 用于指示数据要发送到PDN
载荷长度包含 padding 长度，但不包MAP 头长度
### d. Checksum offload header v5


Checksum offload 头字段采用大端格式
```

  Bit            0 - 6          7               8-15              16-31
  Function     Header Type    Next Header     Checksum Valid    Reserved

```
Header Type 用于指示头的类型，通常设为 CHECKSUM

Header types

= ===============
0 Reserved
1 Reserved
2 checksum header
= ===============

Checksum Valid 用于指示该头校验和是否有效。值为 1 表示已对本包计算校验和且有效值为 0 表示计算得到的包校验和无效
保留位发送时必须为零，接收时忽略
### e. MAP packet v1/v5（命令相关）


```

    Bit             0             1         2-7      8 - 15           16 - 31
    Function   Command         Reserved     Pad   Multiplexer ID    Payload length
    Bit          32 - 39        40 - 45    46 - 47       48 - 63
    Function   Command name    Reserved   Command Type   Reserved
    Bit          64 - 95
    Function   Transaction ID
    Bit          96 - 127
    Function   Command data

```
命令 1 表示禁用流控，2 表示启用流控
Command types

= ==========================================
0 for MAP command request
1 is to acknowledge the receipt of a command
2 is for unsupported commands
3 is for error during processing of commands
= ==========================================

### f. 聚合


聚合是在单个线skb 中传递给 rmnet 的多MAP 包（可以是数据或命令）。rmnet 处理各个包，并对 MAP 命令进行 ACK，或IP 包按需递交给网络栈
```

  MAP header|IP Packet|Optional padding|MAP header|IP Packet|Optional padding....

  MAP header|IP Packet|Optional padding|MAP header|Command Packet|Optional pad...

```
## 3. 用户空间配置


rmnet 的用户空间配置通过 netlink 使用 iproute2 完成
https://git.kernel.org/pub/scm/network/iproute2/iproute2.git/

驱动使用 rtnl_link_ops 进行通信