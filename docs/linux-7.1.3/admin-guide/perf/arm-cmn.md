## Arm 一致性网格网络（Coherent Mesh Network）PMU


CMN-600 是一种可配置的网格互连（mesh interconnect），由矩形的交叉点（crosspoint，XP）网格组成，每个交叉点最多支持两个设备端口，各类 AMBA CHI 代理（agent）连接到这些端口上。

CMN 在其调试与跟踪（debug and trace）功能中实现了一种分布式 PMU 设计。这包含位于每个 XP 的本地监视器（DTM），它对来自所连接设备节点和/或 XP 本身的至多 4 个事件信号进行计数。这些本地计数器的溢出被累加到由主控制器（DTC）实现的至多 8 个全局计数器中，DTC 提供整体的 PMU 控制，并在全局计数器溢出时产生中断。

### PMU 事件


PMU 驱动为整个互连注册一个单一的 PMU 设备，参见 /sys/bus/event_source/devices/arm_cmn_0。多芯片系统可能通过外部 CCIX 链路将多个 CMN 连接在一起——在这种情况下，每个网格完全独立地统计自身的事件，并且额外的 PMU 设备将被命名为 arm_cmn_{1..n}。

大多数事件以一种直接基于 TRM 定义的格式指定——"type" 选择相应的节点类型，"eventid" 选择事件编号。某些事件需要一个额外的占用 ID（occupancy ID），由 "occupid" 指定。

- 由于 RN-D 节点与 RN-I 节点没有任何不同的事件，它们被视为相同的类型（0xa），并且通用的事件模板被命名为 "rnid_*"。

- 周期计数器被视为属于 DTC 节点（"type" == 0x3，"eventid" 被忽略）的合成事件。

- XP 事件还在 "eventid" 字段中编码了端口与通道，以匹配 pmu_event_sel 寄存器底层的 pmu_event0_id 编码。事件模板以前缀命名，以涵盖所有排列组合。

默认情况下，每个事件提供给定类型所有节点的聚合计数。要针对特定节点，"bynodeid" 必须设置为 1，且 "nodeid" 设置为从 CMN 配置导出的适当值（如 TRM 的“Node ID Mapping”章节所定义）。

### 监视点（Watchpoints）


PMU 还可以统计监视点（watchpoint）事件，以监控特定的 flit 流量。监视点被视为一种合成事件类型，并且与 PMU 事件一样，可以是全局的，也可以使用特定 XP 的 "nodeid" 值进行定向。

由于监视点的方向在其他情况下隐含在底层寄存器选择中，因此为 flit 上传与下载分别提供了独立的事件。

flit 的匹配值与掩码通过 config1 与 config2 传入（分别为 "val" 与 "mask"）。"wp_dev_sel"、"wp_chn_sel"、"wp_grp" 与 "wp_exclusive" 按照 TRM 中对 dtm_wp_config0 的定义指定。

当某个监视点需要匹配 REQ 或 SNP 通道上两个匹配组中的字段时，它可以被指定为两个事件——每组一个——并带有相同的非零 "combine" 值。这样一对组合事件的计数将被归属于主匹配。

"combine" 值为 0 的监视点事件被视为相互独立，将分别计数。
