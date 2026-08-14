
## CoreSight Embedded Cross Trigger (CTI & CTM).


    :Author:   Mike Leach <mike.leach@linaro.org>
    :Date:     November 2019

### 硬件 Description


The CoreSight Cross Trigger 接口 (CTI) 是 一个 硬件 设备 该 takes
各个 输入 和 输出 硬件 signals known 作为 triggers 到 和 来自
设备 和 interconnects them 通过 the Cross Trigger Matrix (CTM) 到 其他
设备 通过 numbered channels, 为了 propagate 事件 之间 设备.

```

 0000000  in_trigs  :::::::
 0 C   0----------->:     :             +======>(other CTI channel IO)
 0  P  0<-----------:     :             v
 0   U 0  out_trigs :     : Channels  *****      :::::::
 0000000            : CTI :<=========>*CTM*<====>: CTI :---+
 #######  in_trigs  :     : (id 0-3)  *****      :::::::   v
 # ETM #----------->:     :                         ^   #######
 #     #<-----------:     :                         +---# ETR #
 ####### out_trigs  :::::::                             #######

```
The CTI 驱动 enables the programming 的 the CTI 到 attach triggers 到
channels. 当 一个 输入 trigger becomes active, the attached channel 将
become active. 任何 输出 trigger attached 到 该 channel 将 也
become active. The active channel 是 propagated 到 其他 CTIs 通过 the CTM,
activating connected 输出 triggers 那里, 除非 filtered 由 the CTI
channel gate.

它是 也 可能 到 activate 一个 channel 使用 系统 软件 directly
programming 寄存器 在 the CTI.

The CTIs 是 registered 由 the 系统 到 为 associated 与 CPUs 和/或 其他
CoreSight 设备 在 the trace 数据 path. 当 这些 设备 是 已启用 the
attached CTIs 将 也 为 已启用. 默认情况下/在 电源 up the CTIs 具有
无 programmed trigger/channel attachments, 因此 将 不 affect the 系统
直到 explicitly programmed.

The 硬件 trigger connections 之间 CTIs 和 设备 是 implementation
定义, 除非 the CPU/ETM combination 是 一个 v8 architecture, 在 其 case
the connections 具有 一个 architecturally 定义 标准 layout.

The 硬件 trigger signals 可 也 为 connected 到 non-CoreSight 设备
(e.g. UART), 或 为 propagated off 芯片 作为 硬件 IO lines.

全部 the CTI 设备 是 associated 与 一个 CTM. 在 许多 系统 那里 将 为 一个
单个 effective CTM (one CTM, 或 多个 CTMs 全部 interconnected), 但 它是
可能 该 系统 可 具有 nets 的 CTIs+CTM 该 是 不 interconnected 由
一个 CTM 到 每个 其他. 在 这些 系统 一个 CTM 索引 是 declared 到 associate
CTI 设备 该 是 interconnected 通过 一个 given CTM.

### Sysfs 文件 和 directories


The CTI 设备 appear 在 the existing CoreSight 总线 alongside the 其他
```

    >$ ls /sys/bus/coresight/devices
     cti_cpu0  cti_cpu2  cti_sys0  etm0  etm2  funnel0  replicator0  tmc_etr0
     cti_cpu1  cti_cpu3  cti_sys1  etm1  etm3  funnel1  tmc_etf0     tpiu0

```
The `cti_cpu<N>` named CTIs 是 associated 与 一个 CPU, 和 任何 ETM 使用 由
该 核心. The `cti_sys<N>` CTIs 是 通用 系统 infrastructure CTIs 该
可 为 associated 与 其他 CoreSight 设备, 或 其他 系统 硬件
```

  >$ ls /sys/bus/coresight/devices/etm0/cti_cpu0
  channels  ctmid  enable  nr_trigger_cons mgmt  power powered  regs
  connections subsystem triggers0 triggers1  uevent

```
**Key 文件 items 是:-**
   - `enable`: enables/disables the CTI. 读取 到 determine 电流 状态.
     若 此 显示 作为 已启用 (1), 但 `powered` 显示 unpowered (0), 然后
     the 启用 indicates 一个 请求 到 已启用 当 the 设备 是 powered.
   - `ctmid` : associated CTM - 仅 relevant 若 系统 具有 多个 CTI+CTM
     clusters 该 是 不 interconnected.
   - `nr_trigger_cons` : 总计 connections - triggers<N> directories.
   - `powered` : 读取 到 determine 若 the CTI 是 currently powered.

**Sub-directories:-**
   - `triggers<N>`: 包含 列出 的 triggers 用于 一个 各个 连接.
   - `channels`: 包含 the channel API - CTI 主要 programming 接口.
   - `regs`: Gives access 到 the raw programmable CTI regs.
   - `mgmt`: the 标准 CoreSight 管理 寄存器.
   - `connections`: Links 到 connected **CoreSight** 设备. The 数字 的
     links 可 为 0 到 `nr_trigger_cons`. Actual 数字 given 由 `nr_links`
     在 此 directory.


#### triggers<N> directories


各个 trigger 连接 information. 此 describes trigger signals 用于
CoreSight 和 non-CoreSight connections.

每个 triggers directory 具有 一个 set 的 参数 describing the triggers 用于
the 连接.

   - `name` : name 的 连接
   - `in_signals` : 输入 trigger 信号 indexes 使用 在 此 连接.
   - `in_types` : functional types 用于 在 signals.
   - `out_signals` : 输出 trigger signals 用于 此 连接.
   - `out_types` : functional types 用于 out signals.

```

    >$ ls ./cti_cpu0/triggers0/
    in_signals  in_types  name  out_signals  out_types
    >$ cat ./cti_cpu0/triggers0/name
    cpu0
    >$ cat ./cti_cpu0/triggers0/out_signals
    0-2
    >$ cat ./cti_cpu0/triggers0/out_types
    pe_edbgreq pe_dbgrestart pe_ctiirq
    >$ cat ./cti_cpu0/triggers0/in_signals
    0-1
    >$ cat ./cti_cpu0/triggers0/in_types
    pe_dbgtrigger pe_pmuirq

```
若 一个 连接 具有 zero signals 在 任一个 the '在' 或 'out' triggers 然后
那些 参数 将 为 omitted.

#### Channels API Directory


此 提供 一个 easy way 到 attach triggers 到 channels, 无 needing
the 多个 注册 操作 该 是 必需 若 manipulating the
'regs' sub-directory elements directly.

```

   >$ ls ./cti_sys0/channels/
   chan_clear         chan_inuse      chan_xtrigs_out     trigin_attach
   chan_free          chan_pulse      chan_xtrigs_reset   trigin_detach
   chan_gate_disable  chan_set        chan_xtrigs_sel     trigout_attach
   chan_gate_enable   chan_xtrigs_in  trig_filter_enable  trigout_detach
   trigout_filtered

```
```

  echo <chan> [<trigger>] > /<device_path>/<operation>

```
何处 the 可选 <trigger> 是 仅 needed 用于 trigXX_attach | detach
操作.

```

   >$ echo 0 1 > ./cti_sys0/channels/trigout_attach
   >$ echo 0 > ./cti_sys0/channels/chan_set

```
Attaches trigout(1) 到 channel(0), 然后 activates channel(0) generating 一个
set 状态 在 cti_sys0.trigout(1)


**API 操作**

   - `trigin_attach, trigout_attach`: Attach 一个 channel 到 一个 trigger 信号.
   - `trigin_detach, trigout_detach`: Detach 一个 channel 来自 一个 trigger 信号.
   - `chan_set`: Set the channel - the set 状态 将 为 propagated around
     the CTM 到 其他 connected 设备.
   - `chan_clear`: Clear the channel.
   - `chan_pulse`: Set the channel 用于 一个 单个 CoreSight clock cycle.
   - `chan_gate_enable`: 写入 操作 sets the CTI gate 到 propagate
     (启用) the channel 到 其他 设备. 此 操作 takes 一个 channel
     数字. CTI gate 是 已启用 用于 全部 channels 默认情况下 在 电源 up. 读取
     到 列出 the currently 已启用 channels 在 the gate.
   - `chan_gate_disable`: 写入 channel 数字 到 禁用 gate 用于 该
     channel.
   - `chan_inuse`: 显示 the 电流 channels attached 到 任何 信号
   - `chan_free`: 显示 channels 与 无 attached signals.
   - `chan_xtrigs_sel`: 写入 一个 channel 数字 到 select 一个 channel 到 view,
     读取 到 显示 the selected channel 数字.
   - `chan_xtrigs_in`: 读取 到 显示 the 输入 triggers attached 到
     the selected view channel.
   - `chan_xtrigs_out`:读取 到 显示 the 输出 triggers attached 到
     the selected view channel.
   - `trig_filter_enable`: Defaults 到 已启用, 禁用 到 允许 potentially
     dangerous 输出 signals 到 为 set.
   - `trigout_filtered`: Trigger out signals 该 是 prevented 来自 正在
     set 若 filtering `trig_filter_enable` 是 已启用. One 使用 是 到 prevent
     accidental `EDBGREQ` signals stopping 一个 核心.
   - `chan_xtrigs_reset`: 写入 1 到 clear 全部 channel / trigger programming.
     Resets 设备 硬件 到 默认 状态.


The 示例 下文 attaches 输入 trigger 索引 1 到 channel 2, 和 输出
trigger 索引 6 到 the 相同 channel. 它 然后 examines the 状态 的 the
channel / trigger connections 使用 the appropriate sysfs attributes.

The 设置 mean 该 若 任一个 输入 trigger 1, 或 channel 2 go active 然后
trigger out 6 将 go active. 我们 然后 启用 the CTI, 和 使用 the 软件
channel control 到 activate channel 2. 我们 参见 the active channel 在 the
`choutstatus` 注册 和 the active 信号 在 the `trigoutstatus`
注册. Finally clearing the channel removes 此.

```

   .../cti_sys0/channels# echo 2 1 > trigin_attach
   .../cti_sys0/channels# echo 2 6 > trigout_attach
   .../cti_sys0/channels# cat chan_free
   0-1,3
   .../cti_sys0/channels# cat chan_inuse
   2
   .../cti_sys0/channels# echo 2 > chan_xtrigs_sel
   .../cti_sys0/channels# cat chan_xtrigs_trigin
   1
   .../cti_sys0/channels# cat chan_xtrigs_trigout
   6
   .../cti_sys0/# echo 1 > enable
   .../cti_sys0/channels# echo 2 > chan_set
   .../cti_sys0/channels# cat ../regs/choutstatus
   0x4
   .../cti_sys0/channels# cat ../regs/trigoutstatus
   0x40
   .../cti_sys0/channels# echo 2 > chan_clear
   .../cti_sys0/channels# cat ../regs/trigoutstatus
   0x0
   .../cti_sys0/channels# cat ../regs/choutstatus
   0x0

```