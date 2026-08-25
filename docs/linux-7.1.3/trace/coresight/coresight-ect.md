
## CoreSight Embedded Cross Trigger (CTI & CTM).


    :Author:   Mike Leach <mike.leach@linaro.org>
    :Date:     November 2019

### 硬件 Description


The CoreSight Cross Trigger 接口 (CTI) 一硬件 设备 takes
各个 输入 输出 硬件 signals known 作为 triggers 来自
设备 interconnects them 通过 the Cross Trigger Matrix (CTM) 其他
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
The CTI 驱动 enables the programming the CTI attach triggers 
channels. 一输入 trigger becomes active, the attached channel 
become active. 任何 输出 trigger attached channel 
become active. The active channel propagated 其他 CTIs 通过 the CTM,
activating connected 输出 triggers 那里, 除非 filtered the CTI
channel gate.

它是 可能 activate 一channel 使用 系统 软件 directly
programming 寄存the CTI.

The CTIs registered the 系统 associated CPUs 其他
CoreSight 设备 the trace 数据 path. 这些 设备 已启the
attached CTIs 已启 默认情况电源 up the CTIs 具有
programmed trigger/channel attachments, 因此 affect the 系统
直到 explicitly programmed.

The 硬件 trigger connections 之间 CTIs 设备 implementation
定义, 除非 the CPU/ETM combination 一v8 architecture, case
the connections 具有 一architecturally 定义 标准 layout.

The 硬件 trigger signals connected non-CoreSight 设备
(e.g. UART), propagated off 芯片 作为 硬件 IO lines.

全部 the CTI 设备 associated 一CTM. 许多 系统 那里 一
单个 effective CTM (one CTM, 多个 CTMs 全部 interconnected), 它是
可能 系统 具有 nets CTIs+CTM interconnected 
一CTM 每个 其他. 这些 系统 一CTM 索引 declared associate
CTI 设备 interconnected 通过 一given CTM.

### Sysfs 文件 directories


The CTI 设备 appear the existing CoreSight 总线 alongside the 其他
```

    >$ ls /sys/bus/coresight/devices
     cti_cpu0  cti_cpu2  cti_sys0  etm0  etm2  funnel0  replicator0  tmc_etr0
     cti_cpu1  cti_cpu3  cti_sys1  etm1  etm3  funnel1  tmc_etf0     tpiu0

```
The `cti_cpu<N>` named CTIs associated 一CPU, 任何 ETM 使用 
核心. The `cti_sys<N>` CTIs 通用 系统 infrastructure CTIs 
associated 其他 CoreSight 设备, 其他 系统 硬件
```

  >$ ls /sys/bus/coresight/devices/etm0/cti_cpu0
  channels  ctmid  enable  nr_trigger_cons mgmt  power powered  regs
  connections subsystem triggers0 triggers1  uevent

```
**Key 文件 items -**
   - `enable`: enables/disables the CTI. 读取 determine 电流 状
     显示 作为 已启(1), `powered` 显示 unpowered (0), 然后
     the 启用 indicates 一请求 已启the 设备 powered.
   - `ctmid` : associated CTM - relevant 系统 具有 多个 CTI+CTM
     clusters 璇，鏄，涓?interconnected.
   - `nr_trigger_cons` : 总计 connections - triggers<N> directories.
   - `powered` : 读取 determine the CTI currently powered.

**Sub-directories:-**
   - `triggers<N>`: 包含 列出 triggers 用于 一各个 连接.
   - `channels`: 包含 the channel API - CTI 主要 programming 接口.
   - `regs`: Gives access 鍒?the raw programmable CTI regs.
   - `mgmt`: the 鏍囧噯 CoreSight 绠＄悊 瀵勫瓨鍣。
   - `connections`: Links connected **CoreSight** 设备. The 数字 
     links 0 `nr_trigger_cons`. Actual 数字 given `nr_links`
     鍦，姝?directory.


#### triggers<N> directories


各个 trigger 连接 information. describes trigger signals 用于
CoreSight 鍜?non-CoreSight connections.

每个 triggers directory 具有 一set 参数 describing the triggers 用于
the 连接.

   - `name` : name 连接
   - `in_signals` : 输入 trigger 信号 indexes 使用 连接.
   - `in_types` : functional types 用于 signals.
   - `out_signals` : 输出 trigger signals 用于 连接.
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
一连接 具有 zero signals 任一the ' 'out' triggers 然后
那些 参数 omitted.

#### Channels API Directory


提供 一easy way attach triggers channels, needing
the 多个 注册 操作 必需 manipulating the
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
何处 the 可<trigger> needed 用于 trigXX_attach | detach
操作.

```

   >$ echo 0 1 > ./cti_sys0/channels/trigout_attach
   >$ echo 0 > ./cti_sys0/channels/chan_set

```
Attaches trigout(1) channel(0), 然后 activates channel(0) generating 一
set 状cti_sys0.trigout(1)


**API 操作**

   - `trigin_attach, trigout_attach`: Attach 一channel 一trigger 信号.
   - `trigin_detach, trigout_detach`: Detach 一channel 来自 一trigger 信号.
   - `chan_set`: Set the channel - the set 状propagated around
     the CTM 其他 connected 设备.
   - `chan_clear`: Clear the channel.
   - `chan_pulse`: Set the channel 用于 一单个 CoreSight clock cycle.
   - `chan_gate_enable`: 写入 操作 sets the CTI gate propagate
     (启用) the channel 其他 设备. 操作 takes 一channel
     数字. CTI gate 已启用于 全部 channels 默认情况电源 up. 读取
     列出 the currently 已启channels the gate.
   - `chan_gate_disable`: 写入 channel 数字 禁用 gate 用于 
     channel.
   - `chan_inuse`: 显示 the 电流 channels attached 任何 信号
   - `chan_free`: 显示 channels attached signals.
   - `chan_xtrigs_sel`: 写入 一channel 数字 select 一channel view,
     读取 显示 the selected channel 数字.
   - `chan_xtrigs_in`: 读取 显示 the 输入 triggers attached 
     the selected view channel.
   - `chan_xtrigs_out`:读取 显示 the 输出 triggers attached 
     the selected view channel.
   - `trig_filter_enable`: Defaults 已启 禁用 允许 potentially
     dangerous 输出 signals set.
   - `trigout_filtered`: Trigger out signals prevented 来自 正在
     set filtering `trig_filter_enable` 已启 One 使用 prevent
     accidental `EDBGREQ` signals stopping 一核心.
   - `chan_xtrigs_reset`: 写入 1 clear 全部 channel / trigger programming.
     Resets 设备 硬件 默认 状


The 示例 下文 attaches 输入 trigger 索引 1 channel 2, 输出
trigger 索引 6 the 相同 channel. 然后 examines the 状the
channel / trigger connections 使用 the appropriate sysfs attributes.

The 设置 mean 任一输入 trigger 1, channel 2 go active 然后
trigger out 6 go active. 我们 然后 启用 the CTI, 使用 the 软件
channel control activate channel 2. 我们 参见 the active channel the
`choutstatus` 注册 the active 信号 the `trigoutstatus`
娉ㄥ唽. Finally clearing the channel removes 姝。

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