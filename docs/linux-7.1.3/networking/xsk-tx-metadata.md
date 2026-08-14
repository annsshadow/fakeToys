
## AF_XDP TX 元数据


本文档描述在使用 [af_xdp](af_xdp) 发送包时如何启用卸载（offload）。关于如何在接收侧
访问类似的元数据，请参阅 [xdp-rx-metadata](xdp-rx-metadata)。

## 总体设计


元数据的头部空间（headroom）通过 `struct xdp_umem_reg` 中的 `tx_metadata_len`
和 `XDP_UMEM_TX_METADATA_LEN` 标志预留。因此，对于共享同一 umem 的每个套接字，
元数据长度是相同的。元数据布局是一个固定的 UAPI，请参阅
`include/uapi/linux/if_xdp.h` 中的 `union xsk_tx_metadata`。因此，一般来说，上面
的 `tx_metadata_len` 字段应包含 `sizeof(union xsk_tx_metadata)`。

注意，在最初的实现中并不要求 `XDP_UMEM_TX_METADATA_LEN` 标志。应用程序可以先尝试
带该标志创建一个 umem，如果失败，再做一次不带标志的尝试。

头部空间以及元数据本身应位于 umem 帧中 `xdp_desc->addr` 的正前方。在一个帧内，
元数据
```

           tx_metadata_len
     /                         \
    +-----------------+---------+----------------------------+
    | xsk_tx_metadata | padding |          payload           |
    +-----------------+---------+----------------------------+
                                ^
                                |
                          xdp_desc->addr

```
一个 AF_XDP 应用程序可以请求大于 ``sizeof(struct xsk_tx_metadata)`` 的头部空间。内核
会忽略 padding（并仍使用 `xdp_desc->addr - tx_metadata_len` 来定位 `xsk_tx_metadata`）。
对于那些不应携带任何元数据（即没有 `XDP_TX_METADATA` 选项）的帧，元数据区域同样被
内核忽略。

flags 字段用于启用特定的卸载：

- `XDP_TXMD_FLAGS_TIMESTAMP`：请求设备将发送时间戳放入 `union xsk_tx_metadata` 的
  `tx_timestamp` 字段。
- `XDP_TXMD_FLAGS_CHECKSUM`：请求设备计算 L4 校验和。`csum_start` 指定校验和应开始
  的字节偏移，`csum_offset` 指定设备应存放计算所得校验和的字节偏移。
- `XDP_TXMD_FLAGS_LAUNCH_TIME`：请求设备在称为 launch time（发射时间）的预定时间
  调度发送该包。launch time 的值由 `union xsk_tx_metadata` 的 `launch_time` 字段
  指示。

除了上述标志外，为了触发卸载，第一个包的 `struct xdp_desc` 描述符应在 `options`
字段中设置 `XDP_TX_METADATA` 位。还要注意，在多缓冲区（multi-buffer）包中，只有
第一个分片应携带元数据。

## 软件 TX 校验和


出于开发和测试目的，可以向 `XDP_UMEM_REG` UMEM 注册调用传入 `XDP_UMEM_TX_SW_CSUM`
标志。在这种情况下，当运行在 `XDK_COPY` 模式时，TX 校验和在 CPU 上计算。不要在生产
环境中启用此选项，因为它会对性能产生负面影响。

## Launch Time


所请求的 launch time 的值应基于设备的 PTP 硬件时钟（PHC）以确保准确性。与 ETF
排队规则（它组织包并延迟其发送）不同，AF_XDP 走的是不同的数据路径。相反，AF_XDP
会立即将包交给设备驱动，而不重新排列其顺序或在发送前暂留它们。由于驱动保持 FIFO
行为且不进行包重排序，带有 launch time 请求的包会阻塞同一 Tx 队列中的其他包，直到
它被发送。因此，建议为计划在未来发送的流量分配单独的队列。

在 launch time 卸载特性被禁用的情况下，设备驱动应忽略 launch time 请求。为了正确
解释和有意义地操作，launch time 绝不应被设置为大于未来最远可编程时间（horizon，
地平线）的值。不同的设备对 launch time 卸载特性有不同的硬件限制。

### stmmac 驱动


对于 stmmac，TSO 与 launch time（TBS）特性对于每个独立的 Tx 队列是互斥的。默认
情况下，驱动将 Tx 队列 0 配置为支持 TSO，其余 Tx 队列配置为支持 TBS。launch time
硬件卸载特性可以通过使用 tc-etf 命令调用驱动的 ndo_setup_tc() 回调来启用或禁用。

编程到 Enhanced Normal Transmit Descriptors 中的 launch time 值是一个 32 位值，
其中最高 8 位表示以秒为单位的时间，剩余 24 位表示以 256 ns 为增量的时间。编程的
launch time 与 PTP 时间（bits[39:8]）进行比较，并在 256 秒后回绕。因此，对于
dwmac4 和 dwxlgmac2，launch time 的 horizon 是未来 128 秒。

### igc 驱动


对于 igc，全部四个 Tx 队列都支持 launch time 特性。launch time 硬件卸载特性可以
通过使用 tc-etf 命令调用驱动的 ndo_setup_tc() 回调来启用或禁用。当进入 TSN 模式时，
igc 驱动会复位设备，并创建一个默认 Qbv 调度，周期为 1 秒，所有 Tx 队列始终开放。

编程到 Advanced Transmit Context Descriptor 中的 launch time 值是相对于队列 Qbv
发送窗口起始时间的相对偏移。描述符的 Frst 标志可被设置，以将该包调度到下一个 Qbv
周期。因此，对于 i225 和 i226，launch time 的 horizon 是队列下一个 Qbv 发送窗口
周期的结束时间。例如，当 Qbv 周期设为 1 秒时，launch time 的 horizon 范围从 1 秒
到 2 秒，具体取决于 Qbv 周期当前运行到何处。

## 查询设备能力


每个设备通过其 netlink netdev family 导出其卸载能力。请参阅
`Documentation/netlink/specs/netdev.yaml` 中的 `xsk-flags` 特性位掩码。

- `tx-timestamp`：设备支持 `XDP_TXMD_FLAGS_TIMESTAMP`
- `tx-checksum`：设备支持 `XDP_TXMD_FLAGS_CHECKSUM`
- `tx-launch-time-fifo`：设备支持 `XDP_TXMD_FLAGS_LAUNCH_TIME`

关于如何查询此信息，请参阅 `tools/net/ynl/samples/netdev.c`。

## 示例


关于处理 TX 元数据的示例程序，请参阅
`tools/testing/selftests/bpf/xdp_hw_metadata.c`。另请参阅
https://github.com/fomichev/xskgen 获取一个更精简的示例。
