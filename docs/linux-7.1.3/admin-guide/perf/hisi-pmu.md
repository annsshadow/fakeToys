## 海思（HiSilicon）SoC 非核（uncore）性能监控单元（PMU

海SoC 芯片包含各种独立的系统设PMU，例L3 缓存（L3C）、Hydra Home Agent（HHA）和 DDRC。这PMU 是独立的，并具有收集统计和性能信息的硬件逻辑
海SoC 封装了多CPU I/O 裸片（die）。每CPU 簇（CCL）由 4 个共享一L3 缓存CPU 核组成；每个 CPU 裸片称为超级 CPU 簇（SCCL），6 CCL 组成。每SCCL 分别有两HHA - 1）和四个 DDRC - 3）
### 海SoC 非核 PMU 驱动


每个设备 PMU 都有用于事件计数、控制和中断的独立寄存器，PMU 驱动应注册像 L3C、HHA DDRC perf PMU 驱动。可用的事件和配置选项应位```
```
/sys/bus/event_source/devices/hisi_sccl{X}_<l3c{Y}/hha{Y}/ddrc{Y}>

"perf list" 命令应从 sysfs 列出可用事件
每个 L3C、HHA DDRC 都作为独立的 PMU 注册perf。PMU 名称在事件列表中显示hisi_sccl<sccl-id>_module<index-id>。其"sccl-id" SCCL 的标识符index-id" 是模块的索引
例如：hisi_sccl3_l3c0/rd_hit_cpipe SCCL ID #3 L3C 索引 #0 READ_HIT_CPIPE 事件
例如：hisi_sccl1_hha0/rx_operations SCCL ID #1 HHA 索引 #0 RX_OPERATIONS 事件
驱动还提供一"cpumask" sysfs 属性，显示用于计数非核 PMU 事件CPU ID。还提供 "associated_cpus" sysfs 属性以显示与此 PMU 关联CPUcpumask" 指示打开事件CPU，通常作为perf 这样的用户空间工具的提示它只包含来自 "associated_cpus" 的一个关CPU
```
  $# perf list
  hisi_sccl3_l3c0/rd_hit_cpipe/ [kernel PMU event]
  ------------------------------------------
  hisi_sccl3_l3c0/wr_hit_cpipe/ [kernel PMU event]
  ------------------------------------------
  hisi_sccl1_l3c0/rd_hit_cpipe/ [kernel PMU event]
  ------------------------------------------
  hisi_sccl1_l3c0/wr_hit_cpipe/ [kernel PMU event]
  ------------------------------------------

  $# perf stat -a -e hisi_sccl3_l3c0/rd_hit_cpipe/ sleep 5
  $# perf stat -a -e hisi_sccl3_l3c0/config=0x02/ sleep 5
```

对于标识符为 0x30 的海思非PMU v2，其拓扑PMU v1 相同，但硬件上增加了一些新功能
1. L3C PMU 支持按簇内的线程进行过滤，可通过
```
  $# perf stat -a -e hisi_sccl3_l3c0/config=0x02,tt_core=0x3/ sleep 5
```
这只会计数该簇中线程 0 1 的操作
用户不应使用 tt_core_deprecated 来指定核/线程过滤。该选项仅为了向后兼容而提供，且只支持 8 位，可能无法覆盖共享 L3C 的所有核/线程
2. Tracetag 允许用户通过 perf 中的 tt_req 参数选择只计数读、写或原子操作。默认计数所有操作。tt_req 3 位，3'b100 表示读操作，3'b101 表示写操作，3'b110 表示原子存储操作，且
```
  $# perf stat -a -e hisi_sccl3_l3c0/config=0x02,tt_req=0x4/ sleep 5
```
这只会计数该簇中的读操作
3. Datasrc 允许用户检查数据来自何处。它5 位。一些重要编码如下：

- 5'b00001：来自本裸片L3C- 5'b01000：来自跨裸片L3C- 5'b01001：来自另一个插槽（socket）的 L3C- 5'b01110：来自本DDR- 5'b01111：来自跨裸片DDR- 5'b10000：来自跨插槽DDR
等等，它主要有助于发现数据源距离 CPU 核最近。如果在多芯片中使用 datasrc_cfg，则 datasrc_skt 应为
```
  $# perf stat -a -e hisi_sccl3_l3c0/config=0xb9,datasrc_cfg=0xE/,
  hisi_sccl3_l3c0/config=0xb9,datasrc_cfg=0xF/ sleep 5
```

4. 一些海SoC 封装了多CPU I/O 裸片。每CPU 裸片包含若干计算簇（CCL）。I/O 裸片称为超级 I/O 簇（SICL），包含多个 I/O 簇（ICL）。SoC 中的每个 CCL/ICL 都有一个唯一 ID。每ID 11 位，包含 6 位的 SCCL-ID 5 位的 CCL/ICL-ID。对I/O 裸片，ICL-ID 后跟
- 5'b00000：I/O_MGMT_ICL- 5'b00001：Network_ICL- 5'b00011：HAC_ICL- 5'b10000：PCIe_ICL
5. uring_channel：UC PMU 事件 0x47~0x59 支持tx request uring 通道进行过滤。它2 位。一些重要编码如下：

- 2'b11：计数发送到 uring_ext（MATA）通道的事件；
- 2'b01：与 2'b11 相同- 2'b10：计数发送到 uring（非 MATA）通道的事件；
- 2'b00：默认值，计数发送到 uring uring_ext 两个通道的事件；

6. ch：NoC PMU 支持使用此选项过滤特定事务通道的事件计数。当前支持的通道如下
- 3'b010：请求通道（Request channel- 3'b100：侦听通道（Snoop channel- 3'b110：响应通道（Response channel- 3'b111：数据通道（Data channel
7. tt_en：如果设置了此选项，NoC PMU 仅支持计数设置了 tracetag 的事务。有tracetag 的更多信息请参见2 条列表
对于标识符为 0x40 的海思非PMU v3，一些非PMU 被进一步划分为若干部分以获得更细粒度的追踪，每个部分有自己的专PMU，所有这PMU 一起覆盖特定非核设备上的事件监控任务。此PMU sysfs 中以如下名称格式描述```
```
/sys/bus/event_source/devices/hisi_sccl{X}_<l3c{Y}_{Z}/ddrc{Y}_{Z}/noc{Y}_{Z}>

Z 是子 ID（sub-id），表示硬件设备某个部分PMU
大多数具有不同子 ID PMU 用法相同。特别地，L3C PMU 提供 `ext` 选项以允许探L3C PMU 更细粒度的统计。L3C PMU 驱动在向硬件下发 perf 命令时将其用作终止提示：

- ext=0：默认，可与事件名一起使用- ext=1 ext=2：必须与事件码一起使用，不支持事件名
```
  $# perf stat -a -e hisi_sccl0_l3c1_0/rd_spipe/ sleep 5
```
```
  $# perf stat -a -e hisi_sccl0_l3c1_0/event=0x1,ext=1/ sleep 5
```
如上，`hisi_sccl0_l3c1_0` 定位到超CPU 0、L3 缓存 1 pipe0
第一条命令定位到 L3C 的第一部分，因为默认隐`ext=0`。第二条命令L3C 的另一部分上以事件 `0x1` 进行计数
用户可以通过设置 srcid_cmd & srcid_msk 来配ID 以计数来自特CCL/ICL 的数据，并通过设置 tgtid_cmd & tgtid_msk 来计数发往特定 CCL/ICL 的数据。srcid_msk/tgtid_msk 中置位的位表PMU 在匹srcid_cmd/tgtid_cmd 时不会检查该位
如果所有这些选项都被禁用，它可以按默认值工作，即不区分过滤条件ID 信息，并返回 PMU 计数器中的总计数器值
当前驱动不支持采样。因此不支持 "perf record"。同样，由于事件都是非核的，也不支持附加到任务（task）
注意：如果需要，请联系维护者获SoC PMU 设备支持的完整事件列表及其信息