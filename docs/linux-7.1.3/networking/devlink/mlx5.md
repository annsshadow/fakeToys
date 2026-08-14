
## mlx5 devlink 支持


本文档描述了 `mlx5` 设备驱动实现的 devlink 特性。

## 参数


   - - 名称
     - 模式
     - 校验
     - 说明
   - - `enable_roce`
     - driverinit
     - 布尔值
     - 若设备支持禁用 RoCE，则 RoCE 启用状态控制设备对 RoCE 能力的支持。
       否则，控制发生在驱动栈中。当在驱动层面禁用 RoCE 时，仅支持原始
       ethernet QP。
   - - `io_eq_size`
     - driverinit
     - 取值范围在 64 到 4096 之间。
     -
   - - `event_eq_size`
     - driverinit
     - 取值范围在 64 到 4096 之间。
     -
   - - `max_macs`
     - driverinit
     - 取值范围在 1 到 2^31 之间。仅支持 2 的幂的值。
     -
   - - `enable_sriov`
     - permanent
     - 布尔值
     - 若设备支持，则分别独立地应用于每个物理功能（PF）。否则，对称地
       应用于所有 PF。
   - - `total_vfs`
     - permanent
     - 取值范围在 1 到设备相关的最大值之间。
     - 若设备支持，则分别独立地应用于每个物理功能（PF）。否则，对称地
       应用于所有 PF。

注意：诸如 `enable_sriov` 和 `total_vfs` 这类 permanent 参数需要 FW reset 才能生效


   # 设置参数
   devlink dev param set pci/0000:01:00.0 name enable_sriov value true cmode permanent
   devlink dev param set pci/0000:01:00.0 name total_vfs value 8 cmode permanent

   # Fw reset
   devlink dev reload pci/0000:01:00.0 action fw_activate

   # 对于 PCI 相关配置，例如 sriov 需要 PCI reset/rescan：
   echo 1 >/sys/bus/pci/devices/0000:01:00.0/remove
   echo 1 >/sys/bus/pci/rescan
   grep ^ /sys/bus/pci/devices/0000:01:00.0/sriov_*

   - - `num_doorbells`
     - driverinit
     - 该参数控制 netdev 使用的通道 doorbell 数量。在所有情况下，都会额外
       分配并使用一个 doorbell 用于非通道通信（例如用于 PTP、HWS 等）。支持的
       取值为：

       - 0：不使用通道特定的 doorbell，所有事情都使用全局 doorbell。
       - [1, max_num_channels]：将这些 netdev 通道均摊到这些 doorbell 上。

`mlx5` 驱动还实现了以下驱动特定的参数。

   :widths: 5 5 5 85

   - - 名称
     - 类型
     - 模式
     - 描述
   - - `flow_steering_mode`
     - string
     - runtime
     - 控制驱动的流导向（flow steering）模式

       - `dmfs` 设备管理的流导向。在 DMFS 模式下，HW steering 实体通过固件
         创建和管理。
       - `smfs` 软件管理的流导向。在 SMFS 模式下，HW steering 实体由驱动创建
         和管理，无需固件介入。
       - `hmfs` 硬件管理的流导向。在 HMFS 模式下，驱动使用带有一种特殊的新型
         WQE（Work Queue Element）的 Work Queue 直接将 steering 规则配置到 HW。

       与默认的 DMFS 模式相比，SMFS 模式更快，并提供更好的规则插入速率。
   - - `fdb_large_groups`
     - u32
     - driverinit
     - 控制 FDB 表中大组（大小 > 1）的数量。

       - 默认值为 15，取值范围在 1 到 1024 之间。
   - - `esw_multiport`
     - 布尔值
     - runtime
     - 控制 MultiPort E-Switch 共享 fdb 模式。

       一种实验性模式，使用单个 E-Switch，NIC 上的所有 vport 和物理端口都
       连接到它。

       例如，将创建在 PF0 上的 VF 的流量发送到原本与 PF1 的 uplink 关联的
       uplink。

       注意：未来的设备，ConnectX-8 及之后，最终会将其作为默认值，以允许在
       单个 E-switch 环境中所有 NIC 端口之间转发，而双 E-switch 模式很可能会被
       弃用。

       默认值：禁用
   - - `esw_port_metadata`
     - 布尔值
     - runtime
     - 在适用的情况下，禁用 eswitch 元数据可根据用例和包大小将包速率提升高达
       20%。

       Eswitch 端口元数据状态控制是否用元数据在内部标记包。多端口 RoCE、
       representor 之间的故障转移以及堆叠设备必须启用元数据标记。默认情况下，
       在受支持的 E-switch 设备上元数据是启用的。元数据仅适用于 switchdev 模式的
       E-switch，当用户不会使用以下任何用例时，可以禁用它：
       1. HCA 处于双/多端口 RoCE 模式。
       2. VF/SF representor bonding（通常用于实时迁移）。
       3. 堆叠设备。

       当元数据被禁用时，如果用户尝试启用上述用例，它们将无法初始化。

       注意：设置此参数不会立即生效。设置必须在 legacy 模式下进行，eswitch 端口
       元数据在启用 switchdev 模式后生效。
   - - `hairpin_num_queues`
     - u32
     - driverinit
     - 我们称涉及转发的 TC NIC 规则为“hairpin”。Hairpin 队列是 mlx5 针对此类
       包的硬件转发的特定硬件实现。

       控制 hairpin 队列的数量。
   - - `hairpin_queue_size`
     - u32
     - driverinit
     - 控制 hairpin 队列的大小（以包为单位）。
   - - `pcie_cong_inbound_high`
     - u16
     - driverinit
     - PCIe 拥塞事件的高阈值配置。当设备侧入站 PCIe 流量在足够长的时间内（至少
       200ms）超过配置的高阈值时，固件将发送一个事件。

       参见 pci_bw_inbound_high ethtool 统计。

       单位为 0.01 %。可接受的取值范围在 [0, 10000]。
       pcie_cong_inbound_low < pcie_cong_inbound_high。
       默认值：9000（对应 90%）。
   - - `pcie_cong_inbound_low`
     - u16
     - driverinit
     - PCIe 拥塞事件的低阈值配置。当设备侧入站 PCIe 流量在先前已处于拥塞状态后
       降到配置的低阈值以下时，固件将发送一个事件。

       参见 pci_bw_inbound_low ethtool 统计。

       单位为 0.01 %。可接受的取值范围在 [0, 10000]。
       pcie_cong_inbound_low < pcie_cong_inbound_high。
       默认值：7500。
   - - `pcie_cong_outbound_high`
     - u16
     - driverinit
     - PCIe 拥塞事件的高阈值配置。当设备侧出站 PCIe 流量在足够长的时间内（至少
       200ms）超过配置的高阈值时，固件将发送一个事件。

       参见 pci_bw_outbound_high ethtool 统计。

       单位为 0.01 %。可接受的取值范围在 [0, 10000]。
       pcie_cong_outbound_low < pcie_cong_outbound_high。
       默认值：9000（对应 90%）。
   - - `pcie_cong_outbound_low`
     - u16
     - driverinit
     - PCIe 拥塞事件的低阈值配置。当设备侧出站 PCIe 流量在先前已处于拥塞状态后
       降到配置的低阈值以下时，固件将发送一个事件。

       参见 pci_bw_outbound_low ethtool 统计。

       单位为 0.01 %。可接受的取值范围在 [0, 10000]。
       pcie_cong_outbound_low < pcie_cong_outbound_high。
       默认值：7500。

   - - `cqe_compress_type`
     - string
     - permanent
     - 配置 NIC 应使用哪种机制/算法，该算法会根据 PCIe 总线状况和其他内部 NIC
       因素，影响压缩 CQE 的速率（激进程度）。此模式影响所有启用压缩的队列。
       - `balanced`：合并较少的 CQE，得到中等的压缩比，但在带宽节省和性能之间
         保持平衡。
       - `aggressive`：将更多 CQE 合并为单个条目，实现更高的压缩率并最大化性能，
         尤其是在高流量负载下。

   - - `swp_l4_csum_mode`
     - string
     - permanent
     - 配置设备在使用软件解析器（SWP）提示来定位头部时如何计算 L4 校验和。

       - `default`：使用设备的默认校验和计算模式。驱动在初始化期间会发现
         使用的是 full_csum 还是 l4_only。不允许从用户空间显式设置此值，但某些
         固件版本在读取参数时可能返回此值。
       - `full_csum`：计算包含伪头的完整校验和。
       - `l4_only`：仅计算 L4 校验和，排除伪头。

`mlx5` 驱动支持通过 `DEVLINK_CMD_RELOAD` 重新加载

## 信息版本


`mlx5` 驱动报告以下版本

   :widths: 5 5 90

   - - 名称
     - 类型
     - 描述
   - - `fw.psid`
     - fixed
     - 用于表示设备的板卡 id。
   - - `fw.version`
     - stored, running
     - 三位数字 major.minor.subminor 固件版本号。

## 健康报告器


### tx 报告器

tx 报告器负责报告和恢复以下三种错误场景：

- tx 超时
    在内核检测到 tx 超时时报告。
    通过搜索丢失的中断来恢复。
- tx 错误完成
    在 tx 完成出错时报告。
    通过刷新 tx 队列并复位它来恢复。
- tx PTP 端口时间戳 CQ 异常
    报告端口 ts CQ 上从未投递的 CQE 过多。
    通过刷新并重建所有 PTP 通道来恢复。

tx 报告器还支持按需诊断回调，通过它提供其发送队列状态的实时信息。

用户命令示例：

```

    $ devlink health diagnose pci/0000:82:00.0 reporter tx

```
   此命令仅在接口处于 up 状态时才有有效输出，否则命令输出为空。

- 显示指示的 tx 错误数量、成功结束的恢复流程数量，
```

    $ devlink health show pci/0000:82:00.0 reporter tx

```
### rx 报告器

rx 报告器负责报告和恢复以下两种错误场景：

- rx 队列初始化（填充）超时
    环形缓冲区初始化时对 rx 队列描述符的填充是通过触发一个 irq 在 napi 上下文中
    完成的。如果未能获得最少数量的描述符，就会发生超时，并且可以通过轮询 EQ
    （Event Queue）来恢复描述符。
- rx 带错误的完成（在中断上下文由 HW 报告）
    在 rx 完成出错时报告。
    通过刷新相关队列并复位它来恢复（如果需要）。

rx 报告器还支持按需诊断回调，通过它提供其接收队列状态的实时信息。

```

    $ devlink health diagnose pci/0000:82:00.0 reporter rx

```
   此命令仅在接口处于 up 状态时才有有效输出。否则，命令输出为空。

- 显示指示的 rx 错误数量、成功结束的恢复流程数量，
```

    $ devlink health show pci/0000:82:00.0 reporter rx

```
### fw 报告器

fw 报告器实现了 `diagnose` 和 `dump` 回调。它通过触发 fw core dump 并将其存入
dump 缓冲区，来跟踪 fw 错误（例如 fw syndrome）的症状。用户可以随时触发 fw 报告器的
诊断命令，以检查当前 fw 状态。

用户命令示例：

```

    $ devlink health diagnose pci/0000:82:00.0 reporter fw

```
```

    $ devlink health dump show pci/0000:82:00.0 reporter fw

```
   此命令只能运行在拥有 fw tracer 所有权的 PF 上，在其他 PF 或任何 VF 上运行都会
   返回“Operation not permitted”。

### fw fatal 报告器

fw fatal 报告器实现了 `dump` 和 `recover` 回调。它通过 CR-space dump 和恢复流程
来跟踪致命错误指示。CR-space dump 使用 vsc 接口，即使在 FW 命令接口不可用的情况下
（大多数 FW 致命错误都是这种情况）也有效。recover 函数运行恢复流程，在需要时重新加载
驱动并触发 fw reset。在固件错误时，健康缓冲区会被 dump 到 dmesg。日志级别源自错误的
严重程度（在健康缓冲区中给出）。

用户命令示例：

```

    $ devlink health recover pci/0000:82:00.0 reporter fw_fatal

```
```

    $ devlink health dump show pci/0000:82:00.1 reporter fw_fatal

```
   此命令只能运行在 PF 上。

### vnic 报告器

vnic 报告器仅实现了 `diagnose` 回调。它负责从 fw 查询 vnic 诊断计数器并实时显示
它们。

vnic 计数器的描述：

- total_error_queues
        由于异步错误或出错命令而处于错误状态的队列数量。
- send_queue_priority_update_flow
        QP/SQ 优先级/SL 更新事件的数量。
- cq_overrun
        CQ 由于溢出而进入错误状态的次数。
- async_eq_overrun
        映射到异步事件的 EQ 被溢出的次数。
- comp_eq_overrun
        映射到完成事件的 EQ 被溢出的次数。
- quota_exceeded_command
        由于超过配额而发出并失败的命令数量。
- invalid_command
        由于配额之外的任何其他原因而发出并失败的命令数量。
- nic_receive_steering_discard
        完成 RX 流 steering 但由于流表不匹配而被丢弃的包数量。
- generated_pkt_steering_fail
	由 VNIC 生成并经历意外 steering 失败的包数量（在 steering 流的任意位置）。
- handled_pkt_steering_fail
	由 VNIC 处理并经历意外 steering 失败的包数量（在 VNIC 拥有的 steering 流的
	任意位置，包括 eswitch 所有者的 FDB）。
- icm_consumption
        vnic 消耗的互连主机内存（ICM）量，粒度为 4KB。ICM 是 SW 在 HCA 请求时
        分配的主机内存，用于存储控制 HCA 操作的数据结构。
- bar_uar_access
        对 PCIe BAR 上 UAR 的 WRITE 或 READ 访问操作次数。
- odp_local_triggered_page_fault
        由 ODP 局部触发的缺页数量。
- odp_remote_triggered_page_fault
        由 ODP 远程触发的缺页数量。

用户命令示例：

```

        $ devlink health diagnose pci/0000:82:00.1 reporter vnic

```
- 诊断 representor vnic 计数器（通过提供 devlink 端口来执
```

        $ devlink health diagnose pci/0000:82:00.1/65537 reporter vnic

```
   此命令可以运行在所有接口上，例如 PF/VF 和 representor 端口。
