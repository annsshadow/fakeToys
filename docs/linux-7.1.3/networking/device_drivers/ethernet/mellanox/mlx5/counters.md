
## Ethtool 计数器


:Copyright: |copy| 2023, NVIDIA CORPORATION & AFFILIATES. 保留所有权利。

## 目录


- `Overview`_
- `Groups`_
- `Types`_
- `Descriptions`_

## 概述


存在若干个计数器分组，分组依据是计数器被统计的位置。此外，每一组计数器可能具有不同的计数器类型。

这些计数器分组基于网络设置中的哪个组件，
```

                                                  ----------------------------------------
                                                  |                                      |
    ----------------------------------------    ---------------------------------------- |
    |              Hypervisor              |    |                  VM                  | |
    |                                      |    |                                      | |
    | -------------------  --------------- |    | -------------------  --------------- | |
    | | Ethernet driver |  | RDMA driver | |    | | Ethernet driver |  | RDMA driver | | |
    | -------------------  --------------- |    | -------------------  --------------- | |
    |           |                 |        |    |           |                 |        | |
    |           -------------------        |    |           -------------------        | |
    |                   |                  |    |                   |                  |--
    ----------------------------------------    ----------------------------------------
                        |                                           |
            -------------               -----------------------------
            |                           |
         ------                      ------ ------ ------         ------      ------      ------
    -----| PF |----------------------| VF |-| VF |-| VF |-----  --| PF |--- --| PF |--- --| PF |---
    |    ------                      ------ ------ ------    |  | ------  | | ------  | | ------  |
    |                                                        |  |         | |         | |         |
    |                                                        |  |         | |         | |         |
    |                                                        |  |         | |         | |         |
    | eSwitch                                                |  | eSwitch | | eSwitch | | eSwitch |
    ----------------------------------------------------------  ----------- ----------- -----------
               -------------------------------------------------------------------------------
               |                                                                             |
               |                                                                             |
               | Uplink (no counters)                                                        |
               -------------------------------------------------------------------------------
                       ---------------------------------------------------------------
                       |                                                             |
                       |                                                             |
                       | MPFS (no counters)                                          |
                       ---------------------------------------------------------------
                                                     |
                                                     |
                                                     | Port

```
## 分组


Ring
  由驱动栈填充的软件计数器。

Netdev
  软件 Ring 计数器的聚合。

vPort counters
  因流控（steering）或无缓冲区导致流量计数与丢包。可能指示 NIC 存在问题。这些计数器包含以太网流量计数器（包括 Raw Ethernet）以及 RDMA/RoCE 流量计数器。

Physical port counters
  收集 PF 与 VF 相关统计信息的计数器。可能指示 NIC、链路或网络存在问题。该测量点保存了 IEEE 802.3、RFC2863、RFC 2819、RFC 3635 等标准化计数器，以及流控、FEC 等额外计数器的信息。Physical port counters 不会暴露给虚拟机。

Priority Port Counters
  一组物理端口计数器，按每个端口的每个优先级分别统计。

## 类型


计数器分为三种类型。

Traffic Informative Counters
  统计流量的计数器。这些计数器可用于负载估算或一般调试。

Traffic Acceleration Counters
  统计被 Mellanox 驱动或硬件加速过的流量的计数器。这些计数器是 informative 计数器集合之上的一层，同一份流量会同时被 informative 与 acceleration 计数器统计。


Error Counters
  这些计数器的增长可能指示问题。每个此类计数器都附带说明与纠正措施。

统计信息可以通过 `ip link` 或 `ethtool` 命令获取。`ethtool`
```

    ip –s link show <if-name>
    ethtool -S <if-name>

```
## 描述


XSK、PTP 与 QoS 计数器中，若与先前已定义的计数器类似，将不再单独列出。例如 `ptp_tx[i]_packets` 不会被显式说明，因为 `tx[i]_packets` 已描述两者的行为，唯一区别是 `ptp_tx[i]_packets` 仅在使用了精确时间协议（precision time protocol）时才计数。

### Ring / Netdev 计数器


以下计数器在每个 ring 或软件端口上均可用。

这些计数器提供关于被 NIC 加速的流量大小的信息。除了统计该流量的标准计数器外，这些计数器会额外统计加速流量（即加速流量会被统计两次）。

下表中列出的计数器名称同时指代 ring 与端口计数器。ring 计数器的记法包含不带花括号的 [i] 索引；端口计数器的记法则不包含 [i]。计数器名称 `rx[i]_packets` 在 ring 0 上会打印为 `rx0_packets`，在软件端口上会打印为 `rx_packets`。

   :widths: 2 3 1

   - - 计数器
     - 描述
     - 类型

   - - `rx[i]_packets`
     - ring i 上接收的数据包数量。
     - Informative

   - - `rx[i]_bytes`
     - ring i 上接收的字节数。
     - Informative

   - - `tx[i]_packets`
     - ring i 上发送的数据包数量。
     - Informative

   - - `tx[i]_bytes`
     - ring i 上发送的字节数。
     - Informative

   - - `tx[i]_recover`
     - SQ 被恢复的次数。
     - Error

   - - `tx[i]_cqes`
     - ring i 上 SQ 发出的 CQE 事件数量。
     - Informative

   - - `tx[i]_cqe_err`
     - ring i 上 SQ 遇到的错误 CQE 数量。
     - Error

   - - `tx[i]_tso_packets`
     - ring i 上发送的 TSO 数据包数量 [#accel]_。
     - Acceleration

   - - `tx[i]_tso_bytes`
     - ring i 上发送的 TSO 字节数 [#accel]_。
     - Acceleration

   - - `tx[i]_tso_inner_packets`
     - ring i 上发送的、被标记为携带内部封装的 TSO 数据包数量 [#accel]_。
     - Acceleration

   - - `tx[i]_tso_inner_bytes`
     - ring i 上发送的、被标记为携带内部封装的 TSO 字节数 [#accel]_。
     - Acceleration

   - - `rx[i]_gro_packets`
     - 使用硬件加速 GRO 处理的接收数据包数量，即 ring i 上接收的硬件 GRO 卸载数据包数量。仅统计真正的 GRO 数据包：仅统计位于 GRO 计数大于 1 的 SKB 中的数据包。
     - Acceleration

   - - `rx[i]_gro_bytes`
     - 使用硬件加速 GRO 处理的接收字节数，即 ring i 上接收的硬件 GRO 卸载字节数。仅统计真正的 GRO 数据包：仅统计位于 GRO 计数大于 1 的 SKB 中的数据包。
     - Acceleration

   - - `rx[i]_gro_skbs`
     - 由硬件加速 GRO 构建的 GRO SKB 数量。仅统计 GRO 计数大于 1 的 SKB。
     - Informative

   - - `rx[i]_gro_large_hds`
     - 使用硬件加速 GRO 且头部较大、需要额外分配内存的接收数据包数量。
     - Informative

   - - `rx[i]_hds_nodata_packets`
     - header/data split 模式下仅含头部的数据包数量 [#accel]_。
     - Informative

   - - `rx[i]_hds_nodata_bytes`
     - header/data split 模式下仅含头部的数据包的字节数 [#accel]_。
     - Informative
   - - `rx[i]_hds_nosplit_packets`
     - 在 header/data split 模式下未被拆分的数据包数量。当硬件不支持该协议拆分时，数据包不会被拆分，例如协议 ICMPv4/v6。目前 header/data split 仅支持 IPv4/IPv6 上的 TCP 与 UDP [#accel]_。
     - Informative

   - - `rx[i]_hds_nosplit_bytes`
     - 在 header/data split 模式下未被拆分的数据包的字节数。当硬件不支持该协议拆分时，数据包不会被拆分，例如协议 ICMPv4/v6。目前 header/data split 仅支持 IPv4/IPv6 上的 TCP 与 UDP [#accel]_。
     - Informative

   - - `rx[i]_lro_packets`
     - ring i 上接收的 LRO 数据包数量 [#accel]_。
     - Acceleration

   - - `rx[i]_lro_bytes`
     - ring i 上接收的 LRO 字节数 [#accel]_。
     - Acceleration

   - - `rx[i]_ecn_mark`
     - 接收数据包中 ECN 标记被置位的数量。
     - Informative

   - - `rx_oversize_pkts_buffer`
     - 因长度超出设备为入向流量分配的软件缓冲区大小、到达 RQ 后被丢弃的接收数据包数量。这可能意味着设备的 MTU 大于软件缓冲区大小。
     - Error

   - - `rx_oversize_pkts_sw_drop`
     - 因 CQE 数据大于 MTU 大小而在软件中被丢弃的接收数据包数量。
     - Error

   - - `rx[i]_csum_unnecessary`
     - ring i 上以 `CHECKSUM_UNNECESSARY` 接收的数据包 [#accel]_。
     - Acceleration

   - - `rx[i]_csum_unnecessary_inner`
     - ring i 上带有内部封装且以 `CHECKSUM_UNNECESSARY` 接收的数据包 [#accel]_。
     - Acceleration

   - - `rx[i]_csum_none`
     - ring i 上以 `CHECKSUM_NONE` 接收的数据包 [#accel]_。
     - Acceleration

   - - `rx[i]_csum_complete`
     - ring i 上以 `CHECKSUM_COMPLETE` 接收的数据包 [#accel]_。
     - Acceleration

   - - `rx[i]_csum_complete_tail`
     - 已进行校验和计算（可能需要填充）且能够使用 `CHECKSUM_PARTIAL` 完成计算的接收数据包数量。
     - Informative

   - - `rx[i]_csum_complete_tail_slow`
     - 校验和所需的填充大于 8 字节的接收数据包数量。
     - Informative

   - - `tx[i]_csum_partial`
     - ring i 上以 `CHECKSUM_PARTIAL` 发送的数据包 [#accel]_。
     - Acceleration

   - - `tx[i]_csum_partial_inner`
     - ring i 上带有内部封装且以 `CHECKSUM_PARTIAL` 发送的数据包 [#accel]_。
     - Acceleration

   - - `tx[i]_csum_none`
     - ring i 上未使用硬件校验和加速发送的数据包。
     - Informative

   - - `tx[i]_stopped` / `tx_queue_stopped` [#ring_global]_
     - ring i 上 SQ 已满的事件。若该计数器增长，请检查为发送分配的缓冲区数量。
     - Informative

   - - `tx[i]_wake` / `tx_queue_wake` [#ring_global]_
     - ring i 上 SQ 曾满后又变为非满的事件。
     - Informative

   - - `tx[i]_dropped` / `tx_queue_dropped` [#ring_global]_
     - ring i 上因 DMA 映射失败而被丢弃的发送数据包。若该计数器增长，请检查为发送分配的缓冲区数量。
     - Error
   - - `tx[i]_nop`
     - 由于到达循环缓冲区末尾而插入到 SQ（与 ring i 相关）的 nop WQE（空 WQE）数量。当接近循环缓冲区末尾时，驱动可能会添加这些空 WQE，以避免出现某个 WQE 在队列末尾开始、在队列开头结束的情况。这是正常现象。
     - Informative

   - - `tx[i]_timestamps`
     - 在设备 DMA 层被打上硬件时间戳的发送数据包。
     - Informative

   - - `tx[i]_added_vlan_packets`
     - vlan 标签插入被卸载到硬件的发送数据包数量。
     - Acceleration

   - - `rx[i]_removed_vlan_packets`
     - vlan 标签剥离被卸载到硬件的接收数据包数量。
     - Acceleration

   - - `rx[i]_wqe_err`
     - ring i 上接收到的错误操作码数量。
     - Error

   - - `rx[i]_mpwqe_frag`
     - 因无法分配复合页而失败、从而改用碎片化 MPWQE（Multi Packet WQE）的 WQE 数量，发生在 ring i 上。若该计数器上升，可能表明没有足够的大页内存，驱动分配了碎片化页。这不是异常状况。
     - Informative

   - - `rx[i]_mpwqe_filler_cqes`
     - ring i 上发出的 filler CQE 事件数量。
     - Informative

   - - `rx[i]_mpwqe_filler_strides`
     - ring i 上被 filler CQE 消耗的 stride 数量。
     - Informative

   - - `tx[i]_mpwqe_blks`
     - 从 Multi-Packet WQE（mpwqe）处理的发送块数量。
     - Informative

   - - `tx[i]_mpwqe_pkts`
     - 从 Multi-Packet WQE（mpwqe）处理的发送数据包数量。
     - Informative

   - - `rx[i]_cqe_compress_blks`
     - ring i 上带有 CQE 压缩的接收块数量 [#accel]_。
     - Acceleration

   - - `rx[i]_cqe_compress_pkts`
     - ring i 上带有 CQE 压缩的接收数据包数量 [#accel]_。
     - Acceleration

   - - `rx[i]_arfs_add`
     - 为 ring i 上的直接 RQ 流导向而添加到设备的 aRFS 流规则数量 [#accel]_。
     - Acceleration

   - - `rx[i]_arfs_request_in`
     - 已被请求移入 ring i 以进行直接 RQ 流导向的流规则数量 [#accel]_。
     - Acceleration

   - - `rx[i]_arfs_request_out`
     - 已被请求移出 ring i 的流规则数量 [#accel]_。
     - Acceleration

   - - `rx[i]_arfs_expired`
     - 已过期并被移除的流规则数量 [#accel]_。
     - Acceleration

   - - `rx[i]_arfs_err`
     - 未能成功添加到流表的流规则数量。
     - Error

   - - `rx[i]_recover`
     - RQ 被恢复的次数。
     - Error

   - - `tx[i]_xmit_more`
     - 在 skbuff 上设置了 `xmit_more` 指示（无需 doorbell）而发送的数据包数量。
     - Acceleration

   - - `ch[i]_poll`
     - 通道 i 上 NAPI poll 的调用次数。
     - Informative

   - - `ch[i]_arm`
     - 通道 i 上 NAPI poll 函数完成并为完成队列“布防”（arm）的次数。
     - Informative

   - - `ch[i]_aff_change`
     - 通道 i 上 NAPI poll 函数因亲和性变化而在某个 CPU 上显式停止执行的次数。
     - Informative
   - - `ch[i]_events`
     - 通道 i 的完成队列上发生的硬中断事件数量。
     - Informative

   - - `ch[i]_eq_rearm`
     - EQ 被恢复的次数。
     - Error

   - - `ch[i]_force_irq`
     - 通过向 ICOSQ 投递 NOP 来由 XSK 唤醒触发 NAPI 的次数。
     - Acceleration

   - - `rx[i]_congst_umr`
     - ring i 上因拥塞而延迟的未完成 UMR 请求次数。
     - Informative

   - - `rx_pp_alloc_fast`
     - 成功快速路径分配的次数。
     - Informative

   - - `rx_pp_alloc_slow`
     - 慢速路径 order-0 分配的次数。
     - Informative

   - - `rx_pp_alloc_slow_high_order`
     - 慢速路径高阶分配的次数。
     - Informative

   - - `rx_pp_alloc_empty`
     - 当 ptr ring 为空、从而被迫进行慢速路径分配时递增。
     - Informative

   - - `rx_pp_alloc_refill`
     - 当某次分配触发了缓存补充（refill）时递增。
     - Informative

   - - `rx_pp_alloc_waive`
     - 当从 ptr ring 获取的页因 NUMA 不匹配而无法加入缓存时递增。
     - Informative

   - - `rx_pp_recycle_cached`
     - 当回收将页放入 page pool 缓存时递增。
     - Informative

   - - `rx_pp_recycle_cache_full`
     - 当 page pool 缓存已满时递增。
     - Informative

   - - `rx_pp_recycle_ring`
     - 当页被放入 ptr ring 时递增。
     - Informative

   - - `rx_pp_recycle_ring_full`
     - 当因 ptr ring 已满而从 page pool 释放页时递增。
     - Informative

   - - `rx_pp_recycle_released_ref`
     - 当因 refcnt > 1 而释放页（而非回收）时递增。
     - Informative

   - - `rx[i]_xsk_buff_alloc_err`
     - 在 XSK RQ 上下文中分配 skb 或 XSK buffer 失败的次数。
     - Error

   - - `rx[i]_xdp_tx_xmit`
     - 因 XDP 程序的 `XDP_TX` 动作（回弹）而被转发回端口的数据包数量。这些数据包不会被其他软件计数器统计，但会被物理端口与 vPort 计数器统计。
     - Informative

   - - `rx[i]_xdp_tx_mpwqe`
     - 在网络设备上下文（RQ）中由 netdev 发送、并被 netdev 以 `XDP_TX` 处理的多个数据包 WQE 数量。
     - Acceleration

   - - `rx[i]_xdp_tx_inlnw`
     - 数据可内联于 WQE 中、并在 RQ 上下文中以 `XDP_TX` 处理的数据段 WQE 数量。
     - Acceleration

   - - `rx[i]_xdp_tx_nops`
     - 投递到 XDP SQ 的 NOP WQEBB（WQE 构建块）数量。
     - Acceleration

   - - `rx[i]_xdp_tx_full`
     - 本应因 `XDP_TX` 动作被转发回端口、却因发送队列已满而被丢弃的数据包数量。这些数据包不会被其他软件计数器统计，但会被物理端口与 vPort 计数器统计。可以打开更多 rx 队列并将 rx 流量分散到所有队列，和/或增大 rx ring 大小。
     - Error

   - - `rx[i]_xdp_tx_err`
     - 在 RX ring 的 `XDP_TX` ring 上发生的 `XDP_TX` 错误（如帧过长、帧过短）次数。
     - Error
   - - `rx[i]_xdp_tx_cqes` / `rx_xdp_tx_cqe` [#ring_global]_
     - 在 `XDP_TX` ring 的 CQ 上收到的完成数量。
     - Informative

   - - `rx[i]_xdp_drop`
     - 因 XDP 程序的 `XDP_DROP` 动作而被丢弃的数据包数量。这些数据包不会被其他软件计数器统计，但会被物理端口与 vPort 计数器统计。
     - Informative

   - - `rx[i]_xdp_redirect`
     - ring i 上触发 XDP redirect 动作的次数。
     - Acceleration

   - - `tx[i]_xdp_xmit`
     - 被重定向到接口（因 XDP redirect）的数据包数量。这些数据包不会被其他软件计数器统计，但会被物理端口与 vPort 计数器统计。
     - Informative

   - - `tx[i]_xdp_full`
     - 被重定向到接口（因 XDP redirect）、却因发送队列已满而被丢弃的数据包数量。这些数据包不会被其他软件计数器统计，可以增大 tx 队列。
     - Informative

   - - `tx[i]_xdp_mpwqe`
     - 从其他 netdev 以 `XDP_REDIRECT` 方式卸载到 NIC 的多个数据包 WQE 数量。
     - Acceleration

   - - `tx[i]_xdp_inlnw`
     - 数据可内联于 WQE 中、且数据段从其他 netdev 以 `XDP_REDIRECT` 方式来的 WQE 数据段数量。
     - Acceleration

   - - `tx[i]_xdp_nops`
     - 投递到 SQ、且从其他 netdev 以 `XDP_REDIRECT` 方式来的 NOP WQEBB（WQE 构建块）数量。
     - Acceleration

   - - `tx[i]_xdp_err`
     - 被重定向到接口（因 XDP redirect）、却因错误（如帧过长、帧过短）而被丢弃的数据包数量。
     - Error

   - - `tx[i]_xdp_cqes`
     - 在 CQ 上针对重定向到接口（因 XDP redirect）的数据包所收到的完成数量。
     - Informative

   - - `tx[i]_xsk_xmit`
     - 使用 XSK zerocopy 功能发送的数据包数量。
     - Acceleration

   - - `tx[i]_xsk_mpwqe`
     - 从其他 netdev 以 `XDP_REDIRECT` 方式卸载到 NIC 的多个数据包 WQE 数量。
     - Acceleration

   - - `tx[i]_xsk_inlnw`
     - 数据可内联于 WQE 中、且使用 XSK zerocopy 发送的数据段 WQE 数量。
     - Acceleration

   - - `tx[i]_xsk_full`
     - 在 XSK zerocopy 模式下 SQ 已满时响铃（doorbell）的次数。
     - Error

   - - `tx[i]_xsk_err`
     - 在 XSK zerocopy 模式下发生的错误数量，例如数据大小大于 MTU 大小。
     - Error

   - - `tx[i]_xsk_cqes`
     - 在 XSK zerocopy 模式下处理的 CQE 数量。
     - Acceleration

   - - `tx_tls_ctx`
     - 为加密而添加到设备的 TLS TX HW 卸载上下文数量。
     - Acceleration

   - - `tx_tls_del`
     - 从设备移除的 TLS TX HW 卸载上下文数量（连接关闭）。
     - Acceleration

   - - `tx_tls_pool_alloc`
     - 在 TLS HW 卸载池中成功分配一个工作单元的次数。
     - Acceleration

   - - `tx_tls_pool_free`
     - 在 TLS HW 卸载池中释放一个工作单元的次数。
     - Acceleration

   - - `rx_tls_ctx`
     - 为解密而添加到设备的 TLS RX HW 卸载上下文数量。
     - Acceleration
   - - `rx_tls_del`
     - 从设备删除的 TLS RX HW 卸载上下文数量（连接已结束）。
     - Acceleration

   - - `rx[i]_tls_decrypted_packets`
     - 属于 TLS 流且成功解密的 RX 数据包数量。
     - Acceleration

   - - `rx[i]_tls_decrypted_bytes`
     - RX 数据包中成功解密的 TLS 负载字节数。
     - Acceleration

   - - `rx[i]_tls_resync_req_pkt`
     - 带有重同步请求的接收 TLS 数据包数量。
     - Acceleration

   - - `rx[i]_tls_resync_req_start`
     - TLS 异步重同步请求被启动的次数。
     - Acceleration

   - - `rx[i]_tls_resync_req_end`
     - TLS 异步重同步请求正确结束、并提供了 HW 跟踪的 tcp-seq 的次数。
     - Acceleration

   - - `rx[i]_tls_resync_req_skip`
     - TLS 异步重同步请求过程被启动却未正确结束的次数。
     - Error

   - - `rx[i]_tls_resync_res_ok`
     - 对驱动的 TLS 重同步响应调用被成功处理的次数。
     - Acceleration

   - - `rx[i]_tls_resync_res_retry`
     - 当 ICOSQ 已满时，对驱动的 TLS 重同步响应调用被重试的次数。
     - Error

   - - `rx[i]_tls_resync_res_skip`
     - 对驱动的 TLS 重同步响应调用未成功终止的次数。
     - Error

   - - `rx[i]_tls_err`
     - CQE TLS 卸载出现问题的次数。
     - Error

   - - `tx[i]_tls_encrypted_packets`
     - 由内核进行 TLS 加密的发送数据包数量。
     - Acceleration

   - - `tx[i]_tls_encrypted_bytes`
     - 由内核进行 TLS 加密的发送字节数。
     - Acceleration

   - - `tx[i]_tls_ooo`
     - ring i 上处理的乱序 TLS SQE 分片次数。
     - Acceleration

   - - `tx[i]_tls_dump_packets`
     - 通过 DMA 从 NIC 拷贝过来的 TLS 解密数据包数量。
     - Acceleration

   - - `tx[i]_tls_dump_bytes`
     - 通过 DMA 从 NIC 拷贝过来的 TLS 解密字节数。
     - Acceleration

   - - `tx[i]_tls_resync_bytes`
     - 为解密而请求重同步的 TLS 字节数。
     - Acceleration

   - - `tx[i]_tls_skip_no_sync_data`
     - 可以安全跳过、无需解密的 TLS 发送数据量。
     - Acceleration

   - - `tx[i]_tls_drop_no_sync_data`
     - 因 TLS 数据重传而被丢弃的 TLS 发送数据量。
     - Acceleration

   - - `ptp_cq[i]_abort`
     - 在精确时间协议（precision time protocol）中，因端口时间戳与 CQE 时间戳之间的偏差大于 128 秒而必须跳过的 CQE 次数。
     - Error

   - - `ptp_cq[i]_abort_abs_diff_ns`
     - 在精确时间协议中，当端口时间戳与 CQE 时间戳之差大于 128 秒时，二者时间差值的累积。
     - Error

   - - `ptp_cq[i]_late_cqe`
     - 在 PTP 时间戳 CQ 上送达 CQE 的次数，而该 CQE 本不被预期——因为已过去一段时间，设备通常会确保不投递该 CQE。
     - Error
   - - `ptp_cq[i]_lost_cqe`
     - 设备预期因时间增量流逝而不会在 PTP 时间戳 CQE 上投递 CQE 的次数。如果这样的 CQE 仍被投递，则 `ptp_cq[i]_late_cqe` 会递增。
     - Error

                 相同名称（即不遵循通用命名方案）。

### vPort 计数器


连接到 eSwitch 的 NIC 端口上的计数器。

   :widths: 2 3 1

   - - 计数器
     - 描述
     - 类型

   - - `rx_vport_unicast_packets`
     - 接收的单播数据包，被导向到某个端口，包含 Raw Ethernet QP/DPDK 流量，不含 RDMA 流量。
     - Informative

   - - `rx_vport_unicast_bytes`
     - 接收的单播字节数，被导向到某个端口，包含 Raw Ethernet QP/DPDK 流量，不含 RDMA 流量。
     - Informative

   - - `tx_vport_unicast_packets`
     - 发送的单播数据包，从某个端口导向出去，包含 Raw Ethernet QP/DPDK 流量，不含 RDMA 流量。
     - Informative

   - - `tx_vport_unicast_bytes`
     - 发送的单播字节数，从某个端口导向出去，包含 Raw Ethernet QP/DPDK 流量，不含 RDMA 流量。
     - Informative

   - - `rx_vport_multicast_packets`
     - 接收的多播数据包，被导向到某个端口，包含 Raw Ethernet QP/DPDK 流量，不含 RDMA 流量。
     - Informative

   - - `rx_vport_multicast_bytes`
     - 接收的多播字节数，被导向到某个端口，包含 Raw Ethernet QP/DPDK 流量，不含 RDMA 流量。
     - Informative

   - - `tx_vport_multicast_packets`
     - 发送的多播数据包，从某个端口导向出去，包含 Raw Ethernet QP/DPDK 流量，不含 RDMA 流量。
     - Informative

   - - `tx_vport_multicast_bytes`
     - 发送的多播字节数，从某个端口导向出去，包含 Raw Ethernet QP/DPDK 流量，不含 RDMA 流量。
     - Informative

   - - `rx_vport_broadcast_packets`
     - 接收的广播数据包，被导向到某个端口，包含 Raw Ethernet QP/DPDK 流量，不含 RDMA 流量。
     - Informative

   - - `rx_vport_broadcast_bytes`
     - 接收的广播字节数，被导向到某个端口，包含 Raw Ethernet QP/DPDK 流量，不含 RDMA 流量。
     - Informative

   - - `tx_vport_broadcast_packets`
     - 发送的广播数据包，从某个端口导向出去，包含 Raw Ethernet QP/DPDK 流量，不含 RDMA 流量。
     - Informative

   - - `tx_vport_broadcast_bytes`
     - 发送的广播字节数，从某个端口导向出去，包含 Raw Ethernet QP/DPDK 流量，不含 RDMA 流量。
     - Informative

   - - `rx_vport_rdma_unicast_packets`
     - 接收的 RDMA 单播数据包，被导向到某个端口（计数器统计 RoCE/UD/RC 流量）[#accel]_。
     - Acceleration

   - - `rx_vport_rdma_unicast_bytes`
     - 接收的 RDMA 单播字节数，被导向到某个端口（计数器统计 RoCE/UD/RC 流量）[#accel]_。
     - Acceleration

   - - `tx_vport_rdma_unicast_packets`
     - 发送的 RDMA 单播数据包，从某个端口导向出去（计数器统计 RoCE/UD/RC 流量）[#accel]_。
     - Acceleration

   - - `tx_vport_rdma_unicast_bytes`
     - 发送的 RDMA 单播字节数，从某个端口导向出去（计数器统计 RoCE/UD/RC 流量）[#accel]_。
     - Acceleration

   - - `rx_vport_rdma_multicast_packets`
     - 接收的 RDMA 多播数据包，被导向到某个端口（计数器统计 RoCE/UD/RC 流量）[#accel]_。
     - Acceleration
   - - `rx_vport_rdma_multicast_bytes`
     - 接收的 RDMA 多播字节数，被导向到某个端口（计数器统计 RoCE/UD/RC 流量）[#accel]_。
     - Acceleration

   - - `tx_vport_rdma_multicast_packets`
     - 发送的 RDMA 多播数据包，从某个端口导向出去（计数器统计 RoCE/UD/RC 流量）[#accel]_。
     - Acceleration

   - - `tx_vport_rdma_multicast_bytes`
     - 发送的 RDMA 多播字节数，从某个端口导向出去（计数器统计 RoCE/UD/RC 流量）[#accel]_。
     - Acceleration

   - - `vport_loopback_packets`
     - 被环回（接收并发送）的单播、多播和广播数据包，IB/Eth [#accel]_。
     - Acceleration

   - - `vport_loopback_bytes`
     - 被环回（接收并发送）的单播、多播和广播字节数，IB/Eth [#accel]_。
     - Acceleration

   - - `rx_steer_missed_packets`
     - NIC 收到但因不匹配 NIC 流表中任何流而被丢弃的数据包数量。
     - Error

   - - `rx_packets`
     - 仅 representor：由 hypervisor 处理的接收数据包。
     - Informative

   - - `rx_bytes`
     - 仅 representor：由 hypervisor 处理的接收字节数。
     - Informative

   - - `tx_packets`
     - 仅 representor：由 hypervisor 处理的发送数据包。
     - Informative

   - - `tx_bytes`
     - 仅 representor：由 hypervisor 处理的发送字节数。
     - Informative

   - - `dev_internal_queue_oob`
     - 因内部设备 RQ 缺少接收 WQE 而被丢弃的数据包数量。
     - Error

### 物理端口计数器


物理端口计数器是连接适配器与网络的外部端口上的计数器。该测量点保存了 IEEE 802.3、RFC2863、RFC 2819、RFC 3635 等标准化计数器，以及流控、FEC 等额外计数器的信息。

   :widths: 2 3 1

   - - 计数器
     - 描述
     - 类型

   - - `rx_packets_phy`
     - 物理端口上接收的数据包数量。该计数器不包含因 FCS、帧大小及类似错误而被丢弃的数据包。
     - Informative

   - - `tx_packets_phy`
     - 物理端口上发送的数据包数量。
     - Informative

   - - `rx_bytes_phy`
     - 物理端口上接收的字节数，包含以太网头部与 FCS。
     - Informative

   - - `tx_bytes_phy`
     - 物理端口上发送的字节数。
     - Informative

   - - `rx_multicast_phy`
     - 物理端口上接收的多播数据包数量。
     - Informative

   - - `tx_multicast_phy`
     - 物理端口上发送的多播数据包数量。
     - Informative

   - - `rx_broadcast_phy`
     - 物理端口上接收的广播数据包数量。
     - Informative

   - - `tx_broadcast_phy`
     - 物理端口上发送的广播数据包数量。
     - Informative

   - - `rx_crc_errors_phy`
     - 物理端口上因 FCS（Frame Check Sequence，帧校验序列）错误而被丢弃的接收数据包数量。若该计数器高速率增长，请使用下方的 `rx_symbol_error_phy` 与 `rx_corrected_bits_phy` 计数器检查链路质量。
     - Error
   - - `rx_in_range_len_errors_phy`
     - 因物理端口上的长度/类型错误而被丢弃的接收数据包数量。
     - Error

   - - `rx_out_of_range_len_phy`
     - 因物理端口上长度超出允许值而被丢弃的接收数据包数量。若该计数器增长，意味着连接到适配器的对端配置了更大的 MTU。使用相同的 MTU 配置即可解决此问题。
     - Error

   - - `rx_oversize_pkts_phy`
     - 因物理端口上长度超出 MTU 大小而被丢弃的接收数据包数量。若该计数器增长，意味着连接到适配器的对端配置了更大的 MTU。使用相同的 MTU 配置即可解决此问题。
     - Error

   - - `rx_symbol_err_phy`
     - 因物理编码错误（符号错误）而被丢弃的接收数据包数量，发生在物理端口上。
     - Error

   - - `rx_mac_control_phy`
     - 物理端口上接收的 MAC 控制数据包数量。
     - Informative

   - - `tx_mac_control_phy`
     - 物理端口上发送的 MAC 控制数据包数量。
     - Informative

   - - `rx_pause_ctrl_phy`
     - 物理端口上接收的链路层 pause 数据包数量。若该计数器增长，意味着网络发生拥塞，无法吸收来自适配器的流量。
     - Informative

   - - `tx_pause_ctrl_phy`
     - 物理端口上发送的链路层 pause 数据包数量。若该计数器增长，意味着 NIC 发生拥塞，无法吸收来自网络的流量。
     - Informative

   - - `rx_unsupported_op_phy`
     - 物理端口上接收到的、带有不支持操作码的 MAC 控制数据包数量。
     - Error

   - - `rx_discards_phy`
     - 因物理端口上缓冲区不足而被丢弃的接收数据包数量。若该计数器增长，意味着适配器发生拥塞，无法吸收来自网络的流量。
     - Error

   - - `tx_discards_phy`
     - 在发送时被丢弃的数据包数量，即便未检测到错误。丢弃可能由于链路处于 down 状态、队头阻塞（head of line drop）、来自网络的 pause 等原因发生。
     - Error

   - - `tx_errors_phy`
     - 因物理端口上长度超出 MTU 大小而被丢弃的发送数据包数量。
     - Error

   - - `rx_undersize_pkts_phy`
     - 因物理端口上长度短于 64 字节而被丢弃的接收数据包数量。若该计数器增长，意味着连接到适配器的对端配置了非标准 MTU，或有畸形数据包到达。
     - Error

   - - `rx_fragments_phy`
     - 因物理端口上长度短于 64 字节且存在 FCS 错误而被丢弃的接收数据包数量。若该计数器增长，意味着连接到适配器的对端配置了非标准 MTU。
     - Error

   - - `rx_jabbers_phy`
     - 因物理端口上长度长于 64 字节且存在 FCS 错误而被丢弃的接收数据包数量。
     - Error

   - - `rx_64_bytes_phy`
     - 物理端口上接收的大小为 64 字节的数据包数量。
     - Informative
   - - `rx_65_to_127_bytes_phy`
     - 物理端口上接收的大小为 65 到 127 字节的数据包数量。
     - Informative

   - - `rx_128_to_255_bytes_phy`
     - 物理端口上接收的大小为 128 到 255 字节的数据包数量。
     - Informative

   - - `rx_256_to_511_bytes_phy`
     - 物理端口上接收的大小为 256 到 511 字节的数据包数量。
     - Informative

   - - `rx_512_to_1023_bytes_phy`
     - 物理端口上接收的大小为 512 到 1023 字节的数据包数量。
     - Informative

   - - `rx_1024_to_1518_bytes_phy`
     - 物理端口上接收的大小为 1024 到 1518 字节的数据包数量。
     - Informative

   - - `rx_1519_to_2047_bytes_phy`
     - 物理端口上接收的大小为 1519 到 2047 字节的数据包数量。
     - Informative

   - - `rx_2048_to_4095_bytes_phy`
     - 物理端口上接收的大小为 2048 到 4095 字节的数据包数量。
     - Informative

   - - `rx_4096_to_8191_bytes_phy`
     - 物理端口上接收的大小为 4096 到 8191 字节的数据包数量。
     - Informative

   - - `rx_8192_to_10239_bytes_phy`
     - 物理端口上接收的大小为 8192 到 10239 字节的数据包数量。
     - Informative

   - - `link_down_events_phy`
     - 链路运行状态变为 down 的次数。若该计数器增长，可能意味着端口抖动（flapping），可能需要更换线缆/收发器。
     - Error

   - - `total_success_recovery_phy`
     - 端口复位周期内任意类型的总成功恢复事件次数。
     - Error

   - - `rx_out_of_buffer`
     - 接收队列没有为适配器入向流量分配软件缓冲区的次数。
     - Error

   - - `module_bus_stuck`
     - 检测到模块 I\ `2`\C 总线（数据或时钟）短路的次数。可能需要更换线缆/收发器。
     - Error

   - - `module_high_temp`
     - 模块温度过高发生的次数。若问题持续，可能需要检查环境温度或更换线缆/收发器模块。
     - Error

   - - `module_bad_shorted`
     - 模块线缆短路的次数。可能需要更换线缆/收发器模块。
     - Error

   - - `module_unplug`
     - 模块被弹出的次数。
     - Informative

   - - `rx_buffer_passed_thres_phy`
     - 端口接收缓冲区超过 85% 满的事件数量。
     - Informative

   - - `tx_pause_storm_warning_events`
     - 设备长时间发送 pause 的次数。
     - Informative

   - - `tx_pause_storm_error_events`
     - 设备长时间发送 pause、最终超时并禁用 pause 帧发送的次数。在 pause 帧被禁用的期间，可能发生丢包。
     - Error

   - - `rx[i]_buff_alloc_err`
     - 在 ring i 上分配接收数据包（或 SKB）缓冲区失败。
     - Error
   - - `rx_bits_phy`
     - 该计数器提供本可被接收的流量总量信息，可用作衡量 `rx_pcs_symbol_err_phy` 与 `rx_corrected_bits_phy` 中错误流量比例的参考。
     - Informative

   - - `rx_pcs_symbol_err_phy`
     - 该计数器统计未被 FEC 纠正算法纠正、或该接口上 FEC 算法未激活的符号错误数量。若该计数器增长，意味着 NIC 与网络之间的链路存在高 BER（误码率），且有流量丢失，可能需要更换线缆/收发器。错误率为特定时间帧内 `rx_pcs_symbol_err_phy` 数量除以 `rx_bits_phy` 数量。
     - Error

   - - `rx_corrected_bits_phy`
     - 根据活动 FEC（RS/FC）在该端口上被纠正的比特数。若该计数器增长，意味着 NIC 与网络之间的链路存在高 BER。纠正比特率为特定时间帧内 `rx_corrected_bits_phy` 数量除以 `rx_bits_phy` 数量。
     - Error

   - - `rx_err_lane_[l]_phy`
     - 该计数器统计每个通道 l 索引上的物理原始错误数量，统计在 FEC 纠正之前。若该计数器增长，意味着 NIC 与网络之间的链路存在高 BER，且可能有流量丢失，可能需要更换线缆/收发器。请结合 `rx_corrected_bits_phy` 一并检查。
     - Error

   - - `rx_global_pause`
     - 物理端口上接收的 pause 数据包数量。若该计数器增长，意味着网络发生拥塞，无法吸收来自适配器的流量。注意：该计数器仅在启用全局 pause 模式时有效。
     - Informative

   - - `rx_global_pause_duration`
     - 物理端口上接收 pause 的持续时间（微秒）。该计数器表示端口未发送任何流量的时间。若该计数器增长，意味着网络发生拥塞，无法吸收来自适配器的流量。注意：该计数器仅在启用全局 pause 模式时有效。
     - Informative

   - - `tx_global_pause`
     - 物理端口上发送的 pause 数据包数量。若该计数器增长，意味着适配器发生拥塞，无法吸收来自网络的流量。注意：该计数器仅在启用全局 pause 模式时有效。
     - Informative

   - - `tx_global_pause_duration`
     - 物理端口上 pause 发送器的持续时间（微秒）。注意：该计数器仅在启用全局 pause 模式时有效。
     - Informative

   - - `rx_global_pause_transition`
     - 物理端口上从 Xoff 到 Xon 的切换次数。注意：该计数器仅在启用全局 pause 模式时有效。
     - Informative

   - - `rx_if_down_packets`
     - 因接口 down 而被丢弃的接收数据包数量。
     - Informative

### 优先级端口计数器


以下计数器是按 L2 优先级（0-7）统计的物理端口计数器。
**注意：** 计数器名称中的 `p` 表示优先级。

   :widths: 2 3 1

   - - 计数器
     - 描述
     - 类型

   - - `rx_prio[p]_bytes`
     - 物理端口上以优先级 p 接收的字节数。
     - Informative

   - - `rx_prio[p]_packets`
     - 物理端口上以优先级 p 接收的数据包数量。
     - Informative

   - - `tx_prio[p]_bytes`
     - 物理端口上以优先级 p 发送的字节数。
     - Informative

   - - `tx_prio[p]_packets`
     - 物理端口上以优先级 p 发送的数据包数量。
     - Informative

   - - `rx_prio[p]_pause`
     - 物理端口上以优先级 p 接收的 pause 数据包数量。若该计数器增长，意味着网络发生拥塞，无法吸收来自适配器的流量。注意：该计数器仅在优先级 p 上启用了 PFC 时可用。
     - Informative

   - - `rx_prio[p]_pause_duration`
     - 物理端口上以优先级 p 接收 pause 的持续时间（微秒）。该计数器表示端口在该优先级上未发送任何流量的时间。若该计数器增长，意味着网络发生拥塞，无法吸收来自适配器的流量。注意：该计数器仅在优先级 p 上启用了 PFC 时可用。
     - Informative

   - - `rx_prio[p]_pause_transition`
     - 物理端口上以优先级 p 从 Xoff 到 Xon 的切换次数。注意：该计数器仅在优先级 p 上启用了 PFC 时可用。
     - Informative

   - - `tx_prio[p]_pause`
     - 物理端口上以优先级 p 发送的 pause 数据包数量。若该计数器增长，意味着适配器发生拥塞，无法吸收来自网络的流量。注意：该计数器仅在优先级 p 上启用了 PFC 时可用。
     - Informative

   - - `tx_prio[p]_pause_duration`
     - 物理端口上以优先级 p 的 pause 发送器持续时间（微秒）。注意：该计数器仅在优先级 p 上启用了 PFC 时可用。
     - Informative

   - - `rx_prio[p]_buf_discard`
     - 设备因缺少每主机接收缓冲区而丢弃的数据包数量。
     - Informative

   - - `rx_prio[p]_cong_discard`
     - 设备因每主机拥塞而丢弃的数据包数量。
     - Informative

   - - `rx_prio[p]_marked`
     - 设备因每主机拥塞而进行 ECN 标记的数据包数量。
     - Informative

   - - `rx_prio[p]_discards`
     - 设备因缺少接收缓冲区而丢弃的数据包数量。
     - Informative

### 设备计数器


   :widths: 2 3 1

   - - 计数器
     - 描述
     - 类型

   - - `rx_pci_signal_integrity`
     - 统计物理层 PCIe 信号完整性错误，以及因帧错误和 CRC（dlp 与 tlp）而进入 recovery 的次数。若该计数器上升，尝试将适配器卡换到另一个插槽，以排除 PCI 插槽故障。请确认你运行的是最新的可用固件与最新的服务器 BIOS 版本。
     - Error
   - - `tx_pci_signal_integrity`
     - 统计物理层 PCIe 信号完整性错误，以及由对端发起的进入 recovery 的次数（因收到 TS/EIEOS 而进入 recovery）。若该计数器上升，尝试将适配器卡换到另一个插槽，以排除 PCI 插槽故障。请确认你运行的是最新的可用固件与最新的服务器 BIOS 版本。
     - Error

   - - `outbound_pci_buffer_overflow`
     - 因 PCI 缓冲区溢出而被丢弃的数据包数量。若该计数器高速率上升，可能意味着某主机的接收流量速率超过了 PCIe 总线，从而发生拥塞。
     - Informative

   - - `outbound_pci_stalled_rd`
     - 在过去一秒内，NIC 有出站非 posted 读请求但因 posted credit 不足而无法执行的时间占比（范围 0...100）。
     - Informative

   - - `outbound_pci_stalled_wr`
     - 在过去一秒内，NIC 有出站 posted 写请求但因 posted credit 不足而无法执行的时间占比（范围 0...100）。
     - Informative

   - - `outbound_pci_stalled_rd_events`
     - `outbound_pci_stalled_rd` 高于 30% 的秒数。
     - Informative

   - - `outbound_pci_stalled_wr_events`
     - `outbound_pci_stalled_wr` 高于 30% 的秒数。
     - Informative

   - - `dev_out_of_buffer`
     - 设备自有队列没有分配足够缓冲区的次数。
     - Error

   - - `pci_bw_inbound_high`
     - 设备越过入站 PCIe 高带宽阈值的次数。需与 `pci_bw_inbound_low` 比较以判断设备是否处于拥塞状态。
       若 `pci_bw_inbound_high` == `pci_bw_inbound_low`，则设备未拥塞。
       若 `pci_bw_inbound_high` > `pci_bw_inbound_low`，则设备已拥塞。
     - Informative

   - - `pci_bw_inbound_low`
     - 设备越过低入站 PCIe 带宽阈值的次数。需与 `pci_bw_inbound_high` 比较以判断设备是否处于拥塞状态。
       若 `pci_bw_inbound_high` == `pci_bw_inbound_low`，则设备未拥塞。
       若 `pci_bw_inbound_high` > `pci_bw_inbound_low`，则设备已拥塞。
     - Informative

   - - `pci_bw_outbound_high`
     - 设备越过出站 PCIe 高带宽阈值的次数。需与 `pci_bw_outbound_low` 比较以判断设备是否处于拥塞状态。
       若 `pci_bw_outbound_high` == `pci_bw_outbound_low`，则设备未拥塞。
       若 `pci_bw_outbound_high` > `pci_bw_outbound_low`，则设备已拥塞。
     - Informative

   - - `pci_bw_outbound_low`
     - 设备越过低出站 PCIe 带宽阈值的次数。需与 `pci_bw_outbound_high` 比较以判断设备是否处于拥塞状态。
       若 `pci_bw_outbound_high` == `pci_bw_outbound_low`，则设备未拥塞。
       若 `pci_bw_outbound_high` > `pci_bw_outbound_low`，则设备已拥塞。
     - Informative

   - - `pci_bw_stale_event`
     - 设备触发 PCIe 拥塞事件、但查询时发现状态无变化的次数。
     - Informative