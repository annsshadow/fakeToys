## Meta 平台主机网络接口


### 固件版本


fbnic 在闪存上存储了三个由单个 PLDM 镜像提供的组件：

1. fw - 用于控制固件，用于查看和修改固件设置、请求固件动作，以及在数据路径之外获取固件计数器。这是 `fbnic_fw.c` 所交互的固件。
2. bootloader - 用于校验固件安全性并控制基本操作（包括加载和更新固件）的固件。这也被称为 cmrt 固件。
3. undi - 基于 Linux 驱动构建的 UEFI 驱动。

fbnic 在闪存上为这三个组件各存储两份副本。这使得 fbnic 在固件启动失败时能自动回退到旧版本的固件。两者的版本信息都以 running 和 stored 的形式提供。undi 仅以 stored 形式提供，因为一旦 Linux 驱动接管，它就不再主动运行。

`devlink dev info` 提供全部三个组件的版本信息。除了版本之外，构建的 hg commit hash 也作为单独的条目一并包含。

### 配置


#### Ring 参数（ethtool -g / -G）


fbnic 为每个完成（设备 -> 主机）环配备两个提交（主机 -> 设备）环。这三个环对象共同组成上层软件使用的一个“队列”（一个 Rx 队列或 Tx 队列）。

对于 Rx，两个提交环用于将空页传递给 NIC。环 0 是 Header Page Queue（HPQ，头页队列），NIC 将使用其页面来放置 L2-L4 头部（如果帧不是头-数据分离，则放置整帧）。环 1 是 Payload Page Queue（PPQ，载荷页队列），用于包载荷。完成环用于接收包通知/元数据。ethtool 的 `rx` ring 参数对应完成环的大小，`rx-mini` 对应 HPQ，`rx-jumbo` 对应 PPQ。

对于 Tx，两个提交环都可用于提交包，完成环携带两者的通知。fbnic 使用一个提交环来处理来自协议栈的普通流量，第二个用于处理 XDP 帧。ethtool 的 `tx` ring 参数同时控制提交环和完成环的大小。

HPQ 和 PPQ（`rx-mini`、`rx-jumbo`）上的每一个表项对应 4kB 的已分配内存，而其余环上的表项以描述符（8B）为单位。提交环与完成环大小之间的理想比例取决于工作负载，因为对于小包，多个包可以放入单个页面。

### 升级固件


fbnic 支持使用带有签名的 PLDM 镜像通过 `devlink dev flash` 来更新固件。PLDM 镜像被写入闪存。刷写过程不会中断设备的运行。

主机启动时始终使用最新的 UEFI 驱动，无需显式激活。运行新的控制固件需要激活固件。cmrt 固件只能通过给 NIC 断电再上电（power cycle）来激活。

### 健康报告器（Health reporters）


#### fw 报告器


`fw` 健康报告器跟踪 FW 崩溃。转储该报告器将显示最近一次 FW 崩溃的核心转储；如果自断电以来未发生 FW 崩溃，则显示一份 FW 内存快照。诊断回调根据最近接收到的心跳消息显示 FW 已运行时间（崩溃通过检查运行时间是否下降来检测）。

#### otp 报告器


OTP 内存（“熔丝”）用于安全启动和防回滚保护。OTP 内存受 ECC 保护，ECC 错误表明存在制造缺陷或部件随老化而退化。

### 统计信息


#### TX MAC 接口


 - `ptp_illegal_req`：设置了 PTP 请求位但被路由到 BMC/FW 的、发往 NIC 的包
 - `ptp_good_ts`：成功路由到 MAC 且设置了 PTP 请求位的包
 - `ptp_bad_ts`：目的为 MAC 且设置了 PTP 请求位、但因某种错误（例如 DMA 读错误）而中止的包

#### TX Extension（TEI）接口（TTI）


 - `tti_cm_drop`：因信用（credit）耗尽而在 TX Extension（TEI）接口处丢弃的控制消息
 - `tti_frame_drop`：因信用耗尽而在 TX Extension（TEI）接口处丢弃的包
 - `tti_tbi_drop`：因信用耗尽而在 TX BMC 接口（TBI）处丢弃的包

#### RXB（RX Buffer）入队


 - `rxb_integrity_err[i]`：在 RXB 输入 i 上以完整性错误（例如多位 ECC 错误）入队的帧
 - `rxb_mac_err[i]`：在 RXB 输入 i 上以 MAC 帧尾错误（例如坏 FCS）入队的帧
 - `rxb_parser_err[i]`：经历了 RPC 解析器错误的帧
 - `rxb_frm_err[i]`：在 RXB 输入 i 上经历了信号错误（例如缺少包尾/包首）的帧
 - `rxb_drbo[i]_frames`：在 RXB 输入 i 上接收到的帧
 - `rxb_drbo[i]_bytes`：在 RXB 输入 i 上接收到的字节

#### RXB（RX Buffer）FIFO


 - `rxb_fifo[i]_drop`：进入 RXB 池 i 丢弃状态的次数
 - `rxb_fifo[i]_dropped_frames`：在 RXB 池 i 上被丢弃的帧
 - `rxb_fifo[i]_ecn`：进入 RXB 池 i ECN 标记状态的次数
 - `rxb_fifo[i]_level`：RXB 池 i 的当前占用

#### RXB（RX Buffer）出队


   - `rxb_intf[i]_frames`：发往输出 i 的帧
   - `rxb_intf[i]_bytes`：发往输出 i 的字节
   - `rxb_pbuf[i]_frames`：从内部包缓冲视角发往输出 i 的帧
   - `rxb_pbuf[i]_bytes`：从内部包缓冲视角发往输出 i 的字节

#### RPC（Rx 解析器）


 - `rpc_unkn_etype`：包含未知 EtherType 的帧
 - `rpc_unkn_ext_hdr`：包含未知 IPv6 扩展头的帧
 - `rpc_ipv4_frag`：包含 IPv4 分片的帧
 - `rpc_ipv6_frag`：包含 IPv6 分片的帧
 - `rpc_ipv4_esp`：带有 IPv4 ESP 封装的帧
 - `rpc_ipv6_esp`：带有 IPv6 ESP 封装的帧
 - `rpc_tcp_opt_err`：遇到 TCP 选项解析错误的帧
 - `rpc_out_of_hdr_err`：头部大于可解析区域的帧
 - `ovr_size_err`：超长帧

#### 硬件队列


1. RX DMA 引擎：

 - `rde_[i]_pkt_err`：带有 MAC EOP、RPC 解析器、RXB 截断或 RDE 帧截断错误的包。这些错误在包元数据中标记，因为支持 cut-through，但实际丢弃发生在到达 PCIE/RDE 时。
 - `rde_[i]_pkt_cq_drop`：因 RCQ 满而被丢弃的包
 - `rde_[i]_pkt_bdq_drop`：因 HPQ 或 PPQ 耗尽主机缓冲区而被丢弃的包

#### PCIe


fbnic 驱动通过 debugfs（`pcie_stats`）暴露 PCIe 硬件性能统计信息。这些统计信息有助于了解 PCIe 事务行为和潜在的性能瓶颈。

1. PCIe 事务计数器：

   这些计数器跟踪 PCIe 事务活动：
        - `pcie_ob_rd_tlp`：出站读 TLP（Transaction Layer Packets）计数
        - `pcie_ob_rd_dword`：出站读事务中传输的 DWORD 数
        - `pcie_ob_wr_tlp`：出站写 TLP 计数
        - `pcie_ob_wr_dword`：出站写事务中传输的 DWORD 数
	  transactions
        - `pcie_ob_cpl_tlp`：出站完成 TLP 计数
        - `pcie_ob_cpl_dword`：出站完成 TLP 中传输的 DWORD 数

2. PCIe 资源监控：

   这些计数器表示 PCIe 资源耗尽事件：
        - `pcie_ob_rd_no_tag`：因 tag 不可用而丢弃的读请求
        - `pcie_ob_rd_no_cpl_cred`：因完成信用（completion
	  credit）耗尽而丢弃的读请求
        - `pcie_ob_rd_no_np_cred`：因非发布（non-posted）信用耗尽
	  而丢弃的读请求

#### XDP 长度错误：


对于不支持 frags 的 XDP 程序，fbnic 会尝试确保 MTU 能放入单个缓冲区。如果收到超大帧并被分片，它将被丢弃，并更新以下 netlink 计数器：

   - `rx-length`：由于所附加的 XDP 程序缺少分片支持而被丢弃的帧数
   - `rx-errors`：接口上收到的错误包总数
