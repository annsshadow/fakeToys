
## 用于 Synopsys(R) 以太网控制器 "stmmac" 的 Linux 驱动

作者：Giuseppe Cavallaro <peppe.cavallaro@st.com>、
Alexandre Torgue <alexandre.torgue@st.com>、Jose Abreu <joabreu@synopsys.com>

## 目录

- In This Release（本版本说明）
- Feature List（特性列表）
- Kernel Configuration（内核配置）
- Command Line Parameters（命令行参数）
- Driver Information and Notes（驱动信息与说明）
- Debug Information（调试信息）
- Support（支持）

## 本版本说明

本文件描述了用于所有 Synopsys(R) 以太网控制器的 stmmac Linux 驱动。

目前，这个网络设备驱动适用于所有 STi 嵌入式 MAC/GMAC（即 7xxx/5xxx SoC）、
SPEAr（arm）、Loongson1B（mips）以及 XILINX XC2V3000 FF1152AMT0221
D1215994A VIRTEX FPGA 板。Synopsys Ethernet QoS 5.0 IPK 也受支持。

开发此驱动时使用了 DesignWare(R) Cores Ethernet MAC 10/100/1000 Universal
版本 3.70a（及更早版本）和 DesignWare(R) Cores Ethernet Quality-of-Service
版本 4.0（及更高版本），以及 DesignWare(R) Cores XGMAC - 10G Ethernet MAC 和
DesignWare(R) Cores Enterprise MAC - 100G Ethernet MAC。

此驱动同时支持 platform 总线和 PCI。

此驱动包含对以下 Synopsys(R) DesignWare(R) Cores 以太网控制器以及对应最小和
最大版本的支持：

+-------------------------------+--------------+--------------+--------------+
| Controller Name               | Min. Version | Max. Version | Abbrev. Name |
+===============================+==============+==============+==============+
| Ethernet MAC Universal        | N/A          | 3.73a        | GMAC         |
+-------------------------------+--------------+--------------+--------------+
| Ethernet Quality-of-Service   | 4.00a        | N/A          | GMAC4+       |
+-------------------------------+--------------+--------------+--------------+
| XGMAC - 10G Ethernet MAC      | 2.10a        | N/A          | XGMAC2+      |
+-------------------------------+--------------+--------------+--------------+
| XLGMAC - 100G Ethernet MAC    | 2.00a        | N/A          | XLGMAC2+     |
+-------------------------------+--------------+--------------+--------------+

有关硬件要求的问题，请参考随你的以太网适配器一起提供的文档。列出的所有硬件
要求都适用于在 Linux 下使用。

## 特性列表

此驱动提供以下特性：
 - GMII/MII/RGMII/SGMII/RMII/XGMII/XLGMII 接口
 - 半双工 / 全双工操作
 - 节能以太网（EEE）
 - IEEE 802.3x PAUSE 包（流控制）
 - RMON/MIB 计数器
 - IEEE 1588 时间戳（PTP）
 - 每秒脉冲输出（PPS）
 - MDIO Clause 22 / Clause 45 接口
 - MAC 回环
 - ARP 卸载（Offloading）
 - 自动 CRC / PAD 插入与检查
 - 接收与发送数据包的校验和卸载
 - 标准或巨型（Jumbo）以太网包
 - 源地址插入 / 替换
 - VLAN TAG 插入 / 替换 / 删除 / 过滤（HASH 和 PERFECT）
 - 可编程的 TX 和 RX 看门狗与合并（Coalesce）设置
 - 目的地址过滤（PERFECT）
 - HASH 过滤（多播）
 - Layer 3 / Layer 4 过滤
 - 远程唤醒检测
 - 接收侧缩放（RSS）
 - TX 和 RX 的帧抢占（Frame Preemption）
 - 可编程突发长度、阈值、队列大小
 - 多队列（最多 8 个）
 - 多种调度算法（TX：WRR、DWRR、WFQ、SP、CBS、EST、TBS；RX：WRR、SP）
 - 灵活的 RX 解析器
 - TCP / UDP 分段卸载（TSO、USO）
 - 分割头部（SPH）
 - 安全特性（ECC 保护、数据奇偶保护）
 - 使用 Ethtool 的自检测试

## 内核配置

内核配置选项是 `CONFIG_STMMAC_ETH`：
 - `CONFIG_STMMAC_PLATFORM`：用于启用 platform 驱动。
 - `CONFIG_STMMAC_PCI`：用于启用 pci 驱动。

## 命令行参数

如果驱动被构建为模块，可以使用以下可选参数，通过 modprobe 命令把它们输入到
命令行中，使用如下形式
```

    modprobe stmmac_pci [<option>=<VAL1>,<VAL2>,...]

```
```

    stmmaceth=watchdog:100,chain_mode=1

```
每个参数的默认值通常就是推荐设置，除非另有说明。

### watchdog

:Valid Range: 5000-None
:Default Value: 5000

此参数覆盖以毫秒为单位的发送超时。

### debug

:Valid Range: 0-16 (0=none,...,16=all)
:Default Value: 0

此参数调整显示在系统日志中的调试消息级别。

### phyaddr

:Valid Range: 0-31
:Default Value: -1

此参数覆盖 PHY 设备的物理地址。

### flow_ctrl

:Valid Range: 0-3 (0=off,1=rx,2=tx,3=rx/tx)
:Default Value: 3

此参数改变默认的流控制能力。

### pause

:Valid Range: 0-65535
:Default Value: 65535

此参数改变默认的流控制暂停时间。

### tc

:Valid Range: 64-256
:Default Value: 64

此参数改变默认的 HW FIFO 阈值控制值。

### buf_sz

:Valid Range: 1536-16384
:Default Value: 1536

此参数改变默认的 RX DMA 包缓冲区大小。

### eee_timer

:Valid Range: 0-None
:Default Value: 1000

此参数改变默认的 LPI TX 过期时间（毫秒）。

### chain_mode

:Valid Range: 0-1 (0=off,1=on)
:Default Value: 0

此参数把默认的操作模式从 Ring 模式改为 Chain 模式。

## 驱动信息与说明

### 发送过程

当内核需要发送一个包时，会调用 xmit 方法；它设置环（ring）中的描述符，并
通知 DMA 引擎有一个包已准备好发送。

默认情况下，驱动在 `net_device` 结构的 features 字段中设置 `NETIF_F_SG` 位，
从而启用散聚（scatter-gather）特性。在那些校验和可以在硬件中完成的芯片和
配置上是如此。

一旦控制器完成包的发送，就会调度一个定时器来释放发送资源。

### 接收过程

当一个或多个包被收到时，会发生一次中断。中断不会被排队，因此驱动在接收过程中
必须扫描环中的所有描述符。

这基于 NAPI，因此中断处理程序只在有工作要做时才发出信号，然后退出。接着 poll
方法会在将来的某个时刻被调度。

DMA 把收到的包存储在预先分配的 socket 缓冲区列表中，以避免 memcpy（零拷贝）。

### 中断缓解

驱动能够使用 NAPI 来缓减（mitigate）其 DMA 中断的数量，用于 3.50 之前芯片的
接收。新芯片有一个用于这种缓减的 HW RX 看门狗。

缓减参数可以通过 ethtool 调整。

### WoL

通过 Magic 帧和 Unicast 帧实现的局域网唤醒（Wake up on Lan）特性，在 GMAC、
GMAC4/5 和 XGMAC 核心上受支持。

### DMA 描述符

驱动处理普通描述符和备用描述符。后者仅在 DesignWare(R) Cores Ethernet MAC
Universal 版本 3.41a 及以后的版本上被测试过。

stmmac 支持 DMA 描述符在双缓冲（RING）和链表（CHAINED）两种模式下操作。在
RING 模式下，每个描述符指向两个数据缓冲区指针，而在 CHAINED 模式下它们只指向
一个数据缓冲区指针。RING 模式是默认模式。

在 CHAINED 模式下，每个描述符会有一个指向下一个描述符的指针，从而在描述符
自身中创建显式的链接；而这样的显式链接在 RING 模式下是不可能的。

### 扩展描述符

扩展描述符在承载 PTP 包或 IP 上的 TCP/UDP/ICMP 时，给我们提供关于以太网载荷
的信息。在早于 3.50 的 GMAC Synopsys(R) 芯片上这些不可用。在探测（probe）时
驱动会决定是否可以真正使用它们。这一支持对于 PTPv2 也是强制需要的，因为额外
的描述符被用来保存硬件时间戳和扩展状态。

### Ethtool 支持

支持 Ethtool。例如，驱动统计信息（包括 RMON）：
```

    ethtool -S ethX

```
Ethtool 自检测试也受支持。这允许做一些早期的健全性检查
```

    ethtool -t ethX

```
### 巨型帧与分段卸载

巨型帧受支持，并且针对 GMAC 测试过。GSO 也被加入，但它是以软件方式执行的。
LRO 不受支持。

### TSO 支持

TSO（TCP 分段卸载）特性受 GMAC > 4.x 和 XGMAC 芯片系列支持。当一个包通过 TCP
协议发送时，TCP 栈确保提供给底层驱动（在我们的例子中是 stmmac）的 SKB 与最大
帧长相匹配（IP 头 + TCP 头 + 载荷 <= 1500 字节（对于设置为 1500 的 MTU））。这
意味着，如果使用 TCP 的应用程序想要发送一个在加上头部之后长度 > 1514 的包，该
包将被拆分成多个 TCP 包：数据载荷被拆分，而头部（TCP/IP ..）被添加。这是由软件
完成的。

当启用 TSO 时，TCP 栈不关心最大帧长，而是原样把 SKB 包提供给 stmmac。GMAC IP
将不得不自己执行分段，以匹配最大帧长。

这个特性可以通过设备树中的 `snps,tso` 项来启用。

### 节能以太网

节能以太网（EEE）使 IEEE 802.3 MAC 子层连同一系列物理层一起在低功耗空闲（LPI）
模式下运行。EEE 模式支持 IEEE 802.3 MAC 在 100Mbps、1000Mbps 和 1Gbps 下的
操作。

LPI 模式通过在没有数据要收发时关闭通信设备功能的一部分来节省功耗。链路两端的
系统都可以禁用某些功能，并在低链路利用率期间节省功耗。MAC 控制着系统是否应该
进入或退出 LPI 模式，并把这一点通知给 PHY。

一旦接口被打开，驱动就验证 EEE 是否可以被支持。这是通过查看 DMA HW 能力寄存器
和 PHY 设备的 MCD 寄存器来完成的。

为了进入 TX LPI 模式，驱动需要一个软件定时器，在没有东西要发送时启用和禁用
LPI 模式。

### 精确时间协议（PTP）

驱动支持 IEEE 1588-2002 精确时间协议（PTP），它使得在使用诸如网络通信等技术
实现的测量和控制系统中，时钟能够被精确同步。

除了 IEEE 1588-2002 时间戳中提到的那些基础时间戳特性外，新的 GMAC 核心支持
高级时间戳特性。IEEE 1588-2008 可以在配置内核时启用。

### SGMII/RGMII 支持

新的 GMAC 设备提供了自己的方式来管理的 RGMII/SGMII。这一信息在运行时通过查看
HW 能力寄存器就可以获得。这意味着 stmmac 可以无需使用 PHYLIB 的那套机制，就能
管理自协商和链路状态。实际上，HW 提供了一组扩展寄存器来重新启动 ANE、验证
全/半双工模式和速度。多亏了这些寄存器，才得以查看自协商的链路伙伴能力。

### 物理层

驱动与物理抽象层（Physical Abstraction Layer）兼容，以连接到 PHY 和 GPHY 设备。

### 平台信息

一些信息可以通过 platform 和设备树传递。

```

    struct plat_stmmacenet_data {

```
```
        int bus_id;

```
2) PHY 物理地址。如果设为 -1，驱动将选择它找到的第一个 PHY
```
        int phy_addr;

```
```
        int interface;

```
```
        struct stmmac_mdio_bus_data *mdio_bus_data;

```
```
        struct stmmac_dma_cfg *dma_cfg;

```
```
        int clk_csr;

```
```
        int has_gmac;

```
```
        int enh_desc;

```
```
        int tx_coe;
        int rx_coe;

```
11) 一些 HW 由于缓冲区大小有限，无法为超大帧在 HW 中执行 csum。设置此标志后，
csum 将在 SW 中执行
```
        int bugged_jumbo;

```
```
        int pmt;

```
```
        int force_sf_dma_mode;
        int force_thresh_dma_mode;

```
```
        int riwt_off;

```
```
        int max_speed;
        int maxmtu;

```
```
        int multicast_filter_bins;
        int unicast_filter_entries;

```
```
        int tx_fifo_size;
        int rx_fifo_size;

```
```
        u32 rx_queues_to_use;
        u32 tx_queues_to_use;

```
```
        u8 rx_sched_algorithm;
        u8 tx_sched_algorithm;

```
```
        struct stmmac_rxq_cfg rx_queues_cfg[MTL_MAX_RX_QUEUES];
        struct stmmac_txq_cfg tx_queues_cfg[MTL_MAX_TX_QUEUES];

```
24) 此回调用于修改某些 syscfg 寄存器（在 ST SoC 上）
```
        void (*fix_mac_speed)(void *priv, unsigned int speed);

```
25) 用于调用自定义初始化的回调；在某些平台（例如 ST 机顶盒）上这有时是必要的，
这些平台的 HW 需要设置一些 PIO 线或系统 cfg 寄存器。init/exit 回调不应使用
```
        int (*init)(struct platform_device *pdev, void *priv);
        void (*exit)(struct platform_device *pdev, void *priv);

```
26) 执行总线的 HW 设置。例如，在某些 ST 平台上这个字段
```
        struct mac_device_info *(*setup)(void *priv);
        void *bsp_priv;

```
```
        struct clk *stmmac_clk;
        struct clk *pclk;
        struct clk *clk_ptp_ref;
        unsigned int clk_ptp_rate;
        unsigned int clk_ref_rate;
        s32 ptp_max_adj;

```
```
        struct reset_control *stmmac_rst;

```
```
        struct stmmac_axi *axi;

```
```
        int has_gmac4;

```
```
        bool has_sun8i;

```
```
        bool tso_en;

```
```
        int rss_en;

```
```
        int mac_port_sel_speed;

```
```
        bool en_tx_lpi_clockgating;

```
```
        int has_xgmac;

```
```
    }

```
For MDIO bus data, we have:

```
    struct stmmac_mdio_bus_data {

```
```
        unsigned int phy_mask;

```
```
        int *irqs;

```
```
        int probed_phy_irq;

```
```
        bool needs_reset;

```
```
    }

```
For DMA engine configuration, we have:

```
    struct stmmac_dma_cfg {

```
```
        int pbl;

```
```
        int txpbl;
        int rxpbl;

```
```
        bool pblx8;

```
```
        int fixed_burst;
        int mixed_burst;

```
```
        bool aal;

```
```
        bool eame;

```
```
    }

```
For DMA AXI parameters, we have:

```
    struct stmmac_axi {

```
```
        bool axi_lpi_en;
        bool axi_xit_frm;

```
```
        u32 axi_wr_osr_lmt;
        u32 axi_rd_osr_lmt;

```
```
        bool axi_kbbe;

```
```
        u32 axi_blen[AXI_BLEN];

```
```
        bool axi_fb;
        bool axi_mb;

```
```
        bool axi_rb;

```
```
    }

```
For the RX Queues configuration, we have:

```
    struct stmmac_rxq_cfg {

```
```
        u8 mode_to_use;

```
```
        u32 chan;

```
```
        u8 pkt_route;

```
```
        bool use_prio;
        u32 prio;

```
```
    }

```
For the TX Queues configuration, we have:

```
    struct stmmac_txq_cfg {

```
```
        u32 weight;

```
```
        u8 mode_to_use;

```
```
        u32 send_slope;
        u32 idle_slope;
        u32 high_credit;
        u32 low_credit;

```
```
        bool use_prio;
        u32 prio;

```
```
    }

```
### 设备树信息

请参考以下文档：
Documentation/devicetree/bindings/net/snps,dwmac.yaml

### HW 能力

注意，从可用 HW 能力寄存器的新芯片开始，许多配置是在运行时发现的，例如用于
了解 EEE、HW csum、PTP、增强描述符等是否真正可用。作为此驱动所采用的策略，来自
HW 能力寄存器的信息可以取代从平台传来的信息。

## 调试信息

驱动导出了许多信息，例如内部统计、调试信息、MAC 和 DMA 寄存器等。

根据实际所需信息的类型，可以通过多种方式读取这些信息。

例如，用户可以使用 ethtool 支持来获取统计信息：例如使用 `ethtool -S ethX`
（如果支持则显示管理计数器（MMC）），或者查看 MAC/DMA 寄存器：例如使用
`ethtool -d ethX`。

用 `CONFIG_DEBUG_FS` 编译内核，驱动将导出以下 debugfs 项：

 - `descriptors_status`：用于显示 DMA TX/RX 描述符环
 - `dma_cap`：用于显示 HW 能力

开发者也可以使用 `debug` 模块参数来获取进一步的调试信息（请参阅：NETIF Msg
Level）。

## 支持

如果在受支持的内核上、使用受支持的适配器、在已发布的源代码中发现问题，请把
与该问题相关的具体信息通过电子邮件发送到 netdev@vger.kernel.org
