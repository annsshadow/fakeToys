
## Elastic Network Adapter (ENA) 系列 Linux 内核驱动


## 概述


ENA 是一种网络接口，旨在充分利用现代 CPU 特性和系统架构。

ENA 设备通过一个轻量级管理接口暴露出来，该接口具有最少的存储器映射寄存器集合，并通过
管理队列（Admin Queue）提供可扩展的命令集。

该驱动支持一系列 ENA 设备，与链路速率无关（即 10GbE、25GbE、40GbE 等使用相同的驱动），
并具有经协商和可扩展的特性集合。

部分 ENA 设备支持 SR-IOV。本驱动同时用于 SR-IOV 物理功能（PF）和虚拟功能（VF）设备。

ENA 设备通过提供多个 Tx/Rx 队列对（最大数量由设备通过管理队列通告）、每个 Tx/Rx 队列对
一个专用 MSI-X 中断向量、自适应中断调节，以及针对 CPU 缓存行优化的数据放置，实现了高速、
低开销的网络流量处理。

ENA 驱动支持业界标准的 TCP/IP 卸载特性，例如校验和卸载。支持接收端缩放（RSS）以实现多核
扩展。

ENA 驱动及其相应设备实现了看门狗等健康监控机制，使设备和驱动能够以对应用程序透明的方式
恢复，并提供调试日志。

部分 ENA 设备支持一种称为低延迟队列（LLQ）的工作模式，可再节省几微秒。


## ENA 源代码目录结构


=================   ======================================================
ena_com.[ch]        管理通信层。该层负责处理设备与驱动之间的所有管理
                    （admin）通信。
ena_eth_com.[ch]    Tx/Rx 数据路径。
ena_admin_defs.h    ENA 管理接口的定义。
ena_eth_io_defs.h   ENA 数据路径接口的定义。
ena_common_defs.h   ena_com 层的通用定义。
ena_regs_defs.h     ENA PCI 存储器映射（MMIO）寄存器的定义。
ena_netdev.[ch]     Linux 内核主驱动。
ena_ethtool.c       ethtool 回调。
ena_xdp.[ch]        XDP 文件
ena_pci_id_tbl.h    支持的设备的 ID。
ena_phc.[ch]        PTP 硬件时钟基础设施（更多信息请参见 `PHC`_）
ena_devlink.[ch]    devlink 文件。
ena_debugfs.[ch]    debugfs 文件。
=================   ======================================================


## 管理接口：


ENA 管理接口通过以下方式暴露：

- PCIe 配置空间
- 设备寄存器
- 管理队列（AQ）和管理完成队列（ACQ）
- 异步事件通知队列（AENQ）

ENA 设备 MMIO 寄存器仅在驱动初始化期间访问，在后续正常设备运行期间不使用。

AQ 用于提交管理命令，结果/响应通过 ACQ 异步上报。

ENA 引入了一小组管理命令，并为厂商特定扩展留出空间。大多数管理操作都被封装在通用的
Get/Set feature 命令中。

支持以下管理队列命令：

- 创建 I/O 提交队列
- 创建 I/O 完成队列
- 销毁 I/O 提交队列
- 销毁 I/O 完成队列
- 获取特性
- 设置特性
- 配置 AENQ
- 获取统计信息

支持的 Get/Set Feature 属性列表请参见 ena_admin_defs.h。

异步事件通知队列（AENQ）是一个单向队列，由 ENA 设备用来向驱动发送无法通过 ACQ 上报的
事件。AENQ 事件细分为若干组。每组可能有多个综合征，如下所示

事件如下：

====================    ===============
Group                    Syndrome
====================    ===============
链路状态变更             **X**
致命错误                 **X**
通知                     Suspend traffic
通知                     Resume traffic
Keep-Alive              **X**
====================    ===============

ACQ 和 AENQ 共享同一个 MSI-X 向量。

Keep-Alive 是一种特殊机制，用于监控设备的健康状态。设备每秒传递一次 Keep-Alive 事件。
驱动维护一个看门狗（WD）处理程序，记录当前状态和统计信息。如果 Keep-Alive 事件未按预期
传递，WD 会重置设备和驱动。


## 数据路径接口


I/O 操作基于 Tx 和 Rx 提交队列（分别对应 Tx SQ 和 Rx SQ）。每个 SQ 都有一个与之关联的
完成队列（CQ）。

SQ 和 CQ 实现为连续物理内存中的描述符环。

ENA 驱动对 Tx SQ 支持两种队列操作模式：

- **常规模式：**
  在此模式下，Tx SQ 驻留在主机内存中。ENA 设备从主机内存中获取 ENA Tx 描述符和包数据。

- **低延迟队列（LLQ）模式或“推送模式”：**
  在此模式下，驱动将发送描述符和包的前 96 字节直接推送到 ENA 设备内存空间。包的其余
  有效载荷由设备获取。对于此操作模式，驱动使用一个专用的 PCI 设备内存 BAR，并以写合并
  （write-combine）能力映射。

  **请注意**，并非所有 ENA 设备都支持 LLQ，此特性在初始化时与设备协商。如果 ENA 设备
  不支持 LLQ 模式，驱动会回退到常规模式。

Rx SQ 仅支持常规模式。

驱动对 Tx 和 Rx 都支持多队列。这有各种好处：

- 减少给定以太网接口上的 CPU/线程/进程争用。
- 降低完成时的缓存未命中率，尤其是对于保存 sk_buff 结构的数据缓存行。
- 提高处理接收数据包时的进程级并行度。
- 通过将数据包的内核处理引导到运行消费该数据包的应用线程的 CPU，提高数据缓存命中率。
- 硬件中断重定向。


## 中断模式


驱动为每个队列对（Tx 和 Rx 两个方向）分配一个 MSI-X 向量。驱动还分配一个额外的专用 MSI-X
向量用于管理（ACQ 和 AENQ）。

管理中断注册在 Linux 内核探测适配器时执行，并在适配器被移除时注销。I/O 队列中断注册在
适配器被打开时执行，并在接口关闭时注销。

```

   ena-mgmnt@pci:<PCI domain:bus:slot.function>

```
```

   <interface name>-Tx-Rx-<queue index>

```
ENA 设备运行在自动屏蔽和自动清除中断模式下。也就是说，一旦 MSI-X 被投递到主机，其 Cause
位会自动清除，并且中断被屏蔽。中断在 NAPI 处理完成后由驱动解除屏蔽。


## 中断调节


ENA 驱动和设备可以运行在常规或自适应中断调节模式下。

**在常规模式下**，驱动指示设备根据静态中断延迟值推迟中断投递。中断延迟值可通过
`ethtool(8)` 配置。驱动支持以下 `ethtool` 参数：`tx-usecs`、`rx-usecs`。

**在自适应中断**调节模式下，中断延迟值由驱动动态更新，并根据流量性质在每个 NAPI 周期进行
调整。

自适应聚合可以通过 `ethtool(8)` 的 `adaptive_rx on|off` 参数开启/关闭。

关于自适应中断调节（DIM）的更多信息可在 Documentation/networking/net_dim.rst 中找到。


## RX 复制中断（RX copybreak）


rx_copybreak 默认初始化为 ENA_DEFAULT_RX_COPYBREAK，并可通过 SIOCETHTOOL ioctl 的
ETHTOOL_STUNABLE 命令配置。

此选项控制其接收所在的 RX 描述符会被回收的最大包长度。当接收到小于 RX copybreak 字节的
数据包时，它会被复制到一个新的内存缓冲区，并且 RX 描述符被返回给硬件。


## PTP 硬件时钟（PHC）


ENA Linux 驱动支持 PTP 硬件时钟，提供时间戳参考以实现纳秒级分辨率。

**PHC 支持**

PHC 依赖于 PTP 模块，该模块需要作为模块加载或编译进内核。

验证 PTP 模块是否存在：


  grep -w '^CONFIG_PTP_1588_CLOCK=[ym]' /boot/config-`uname -r`

- 如果没有输出，则 ENA 驱动无法以 PHC 支持方式加载。

**PHC 激活**

该特性默认关闭，要开启该特性，可以按以下方式加载 ENA 驱动：

- devlink：


  sudo devlink dev param set pci/<domain:bus:slot.function> name enable_phc value true cmode driverinit
  sudo devlink dev reload pci/<domain:bus:slot.function>
  # for example:
  sudo devlink dev param set pci/0000:00:06.0 name enable_phc value true cmode driverinit
  sudo devlink dev reload pci/0000:00:06.0

所有可用的 PTP 时钟源可在此处查看：


  ls /sys/class/ptp

PHC 支持和能力可使用 ethtool 验证：


  ethtool -T <interface>

**PHC 时间戳**

要获取 PHC 时间戳，请使用 `ptp-userspace-api`_，使用 `testptp`_ 的示例如下：


  testptp -d /dev/ptp$(ethtool -T <interface> | awk '/PTP Hardware Clock:/ {print $NF}') -k 1

PHC 获取时间请求应保持在合理范围内，避免过度使用以确保最佳的性能和效率。ENA 设备将 PHC
获取时间请求的频率限制为每秒最多 125 次。如果超过此限制，获取时间请求将失败，导致
phc_err_ts 统计值增加。

**PHC 统计**

PHC 可使用 debugfs（如果已挂载）监控：


  sudo cat /sys/kernel/debug/<domain:bus:slot.function>/phc_stats

  # for example:
  sudo cat /sys/kernel/debug/0000:00:06.0/phc_stats

PHC 错误必须保持在所有 PHC 请求的 1% 以下，以维持所需的准确性和可靠性水平

=================   ======================================================
**phc_cnt**         | 成功获取的时间戳数量（在过期超时以内）。
**phc_exp**         | 已过期的获取时间戳数量（超过过期超时）。
**phc_skp**         | 跳过的获取时间尝试次数（在阻塞期间）。
**phc_err_dv**      | 由于设备错误导致的获取时间失败次数（进入阻塞状态）。
**phc_err_ts**      | 由于时间戳错误导致的获取时间失败次数（进入阻塞状态），
                    | 若驱动超过请求限制或设备收到无效时间戳则会发生。
=================   ======================================================

PHC 超时：

=================   ======================================================
**expire**          | 有效时间戳获取的最长时间，超过此阈值将导致获取时间请求
                    | 失败，并阻塞新请求直到阻塞超时。
**block**           | 阻塞期在获取时间请求过期或失败后立即开始，
                    | 阻塞期间的所有获取时间请求都将被跳过。
=================   ======================================================


## 统计信息


用户可以使用 `ethtool` 获取 ENA 设备和驱动统计信息。驱动可以从设备收集常规或扩展统计信息
（包括每队列统计）。

此外，驱动在设备重置时将统计信息记录到 syslog。

在受支持的实例类型上，统计信息还将包含 ENA Express 数据（以 `ena_srd` 为前缀的字段）。
有关 ENA Express 数据的完整文档，请参考
https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/ena-express.html#ena-express-monitor


## MTU


驱动支持任意大的 MTU，其最大值与设备协商确定。驱动使用 SetFeature 命令（ENA_ADMIN_MTU
属性）配置 MTU。用户可以通过 `ip(8)` 及类似的遗留工具更改 MTU。


## 无状态卸载


ENA 驱动支持：

- IPv4 头校验和卸载
- 基于 IPv4/IPv6 的 TCP/UDP 校验和卸载


## RSS


- ENA 设备支持 RSS，允许灵活的 Rx 流量引导。
- 支持 Toeplitz 和 CRC32 哈希函数。
- 可以将 L2/L3/L4 字段的不同组合配置为哈希函数的输入。
- 驱动使用 AQ SetFeature 命令（ENA_ADMIN_RSS_HASH_FUNCTION、ENA_ADMIN_RSS_HASH_INPUT
  和 ENA_ADMIN_RSS_INDIRECTION_TABLE_CONFIG 属性）配置 RSS 设置。
- 如果设置了 NETIF_F_RXHASH 标志，则在 Rx CQ 描述符中传递的哈希函数的 32 位结果会被设置
  到接收到的 SKB 中。
- 用户可以通过 `ethtool(8)` 提供哈希密钥、哈希函数，并配置间接表。


## DEVLINK 支持


`devlink`_ 支持重新加载驱动并启动与 ENA 设备的重新协商


  sudo devlink dev reload pci/<domain:bus:slot.function>
  # for example:
  sudo devlink dev reload pci/0000:00:06.0


## 数据路径


### Tx


`ena_start_xmit()` 由协议栈调用。该函数执行以下操作：

- 映射数据缓冲区（`skb->data` 和分片）。
- 填充 `ena_buf` 用于推送缓冲区（如果驱动和设备处于推送模式）。
- 为其余分片准备 ENA 缓冲区。
- 从空的 `req_id` 环分配一个新的请求 ID。请求 ID 是数据包在 Tx 信息中的索引。这用于乱序的
  Tx 完成。
- 将数据包添加到 Tx 环中的适当位置。
- 调用 `ena_com_prepare_tx()`，这是一个 ENA 通信层，将 `ena_bufs` 转换为 ENA 描述符（并按
  需添加元数据 ENA 描述符）。

  - 该函数还将 ENA 描述符和推送缓冲区复制到设备内存空间（如果处于推送模式）。

- 向 ENA 设备写入门铃（doorbell）。
- 当 ENA 设备完成发送数据包时，产生一个完成中断。
- 中断处理程序调度 NAPI。
- 调用 `ena_clean_tx_irq()` 函数。该函数处理 ENA 生成的完成描述符，每个完成的数据包对应
  一个完成描述符。

  - 从完成描述符中检索 `req_id`。通过 `req_id` 检索数据包的 `tx_info`。数据缓冲区被解除
    映射，并且 `req_id` 被返回到空的 `req_id` 环。
  - 当完成描述符处理完毕或达到预算时，函数停止。

### Rx


- 当从 ENA 设备接收到数据包时。
- 中断处理程序调度 NAPI。
- 调用 `ena_clean_rx_irq()` 函数。该函数调用 ENA 通信层函数 `ena_com_rx_pkt()`，后者返回
  用于新数据包的描述符数量，如果没有找到新数据包则返回零。
- `ena_rx_skb()` 检查包长度：

  - 如果包较小（len < rx_copybreak），驱动为新数据包分配一个 SKB，并将包有效载荷复制到
    SKB 数据缓冲区。

    - 这样原始数据缓冲区不会被传递给协议栈，而是被复用于将来的 Rx 数据包。

  - 否则该函数解除 Rx 缓冲区的映射，将第一个描述符设为 `skb` 的线性部分，其他描述符设为
    `skb` 的分片。

- 新的 SKB 会被更新必要信息（协议、校验和硬件校验结果等），然后通过 NAPI 接口函数
  `napi_gro_receive()` 传递给网络协议栈。

### 动态 RX 缓冲区（DRB）


RX 环中的每个 RX 描述符都是单个内存页（根据系统配置为 4KB 或 16KB 长）。为了减少处理高速率
小数据包时所需的内存分配，如果此页剩余超过 2KB 未使用，驱动会尝试复用剩余的 RX 描述符
空间。

该机制的一个简单示例如下事件序列：

```

        1. Driver allocates page-sized RX buffer and passes it to hardware
                +----------------------+
                |4KB RX Buffer         |
                +----------------------+

        2. A 300Bytes packet is received on this buffer

        3. The driver increases the ref count on this page and returns it back to
           HW as an RX buffer of size 4KB - 300Bytes = 3796 Bytes
               +----+--------------------+
               |****|3796 Bytes RX Buffer|
               +----+--------------------+

```
当加载 XDP 程序时，或当 RX 数据包小于 rx_copybreak 字节时（在这种情况下，数据包被复制出
RX 缓冲区到为其新分配的 skb 的线性部分，且 RX 缓冲区保持相同大小，参见 `RX copybreak`_），
不使用此机制。
