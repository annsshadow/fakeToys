## Coresight - ARM 上的硬件辅助追踪


   :Author:   Mathieu Poirier <mathieu.poirier@linaro.org>
   :Date:     September 11th, 2014

### 简

Coresight 是一系列技术的统称，用于调试基ARM SoC。它包括 JTAG 和硬件辅助追踪的解决方案。本文档关注后者
在处理拥有许SoC 以及 GPU、DMA 引擎等其他组件的系统时，硬件辅助追踪变得越来越有用。ARM 通过不同的组件开发了一套硬件辅助追踪方案，每个组件在综合（synthesis）时加入设计以满足特定的追踪需求。组件通常分类为源（source）、链路（link）和汇（sink），并（通常）通过 AMBA 总线发现
“源”根据用户配置的追踪场景，生成表示处理器指令路径的压缩流。此后该流通过连接源与一个或多个汇的链路，在 coresight 系统中（经由 ATB 总线）流动。汇作为 coresight 实现的端点，要么将压缩流存储在内存缓冲区中，要么创建到外部世界的接口，使数据可以传输到主机而不必担心板coresight 内存缓冲区被填满```

  *****************************************************************
 **************************** AMBA AXI  ****************************===||
  *****************************************************************    ||
        ^                    ^                            |            ||
        |                    |                            *            **
     0000000    :::::     0000000    :::::    :::::    @@@@@@@    ||||||||||||
     0 CPU 0<-->: C :     0 CPU 0<-->: C :    : C :    @ STM @    || System ||
  |->0000000    : T :  |->0000000    : T :    : T :<--->@@@@@     || Memory ||
  |  #######<-->: I :  |  #######<-->: I :    : I :      @@@<-|   ||||||||||||
  |  # ETM #    :::::  |  # PTM #    :::::    :::::       @   |
  |   #####      ^ ^   |   #####      ^ !      ^ !        .   |   |||||||||
  | |->###       | !   | |->###       | !      | !        .   |   || DAP ||
  | |   #        | !   | |   #        | !      | !        .   |   |||||||||
  | |   .        | !   | |   .        | !      | !        .   |      |  |
  | |   .        | !   | |   .        | !      | !        .   |      |  *
  | |   .        | !   | |   .        | !      | !        .   |      | SWD/
  | |   .        | !   | |   .        | !      | !        .   |      | JTAG
  *****************************************************************<-|
 *************************** AMBA Debug APB ************************
  *****************************************************************
   |    .          !         .          !        !        .    |
   |    .          *         .          *        *        .    |
  *****************************************************************
 ******************** Cross Trigger Matrix (CTM) *******************
  *****************************************************************
   |    .     ^              .                            .    |
   |    *     !              *                            *    |
  *****************************************************************
 ****************** AMBA Advanced Trace Bus (ATB) ******************
  *****************************************************************
   |          !                        ===============         |
   |          *                         ===== F =====<---------|
   |   :::::::::                         ==== U ====
   |-->:: CTI ::<!!                       === N ===
   |   :::::::::  !                        == N ==
   |    ^         *                        == E ==
   |    !  &&&&&&&&&       IIIIIII         == L ==
   |------>&& ETB &&<......II     I        =======
   |    !  &&&&&&&&&       II     I           .
   |    !                    I     I          .
   |    !                    I REP I<..........
   |    !                    I     I
   |    !!>&&&&&&&&&       II     I           *Source: ARM ltd.
   |------>& TPIU  &<......II    I            DAP = Debug Access Port
           &&&&&&&&&       IIIIIII            ETM = Embedded Trace Macrocell
               ;                              PTM = Program Trace Macrocell
               ;                              CTI = Cross Trigger Interface
               *                              ETB = Embedded Trace Buffer
          To trace port                       TPIU= Trace Port Interface Unit
                                              SWD = Serial Wire Debug

```
虽然组件在目标上的配置是通过 APB 总线完成的，但所有追踪数据都ATB 总线上带外（out-of-band）传输。CTM 提供了一种在 CoreSight 组件之间聚合和分发信号的方式
coresight 框架提供了一个中心点来表示、配置和管理平台上的 coresight 设备。第一个实现集中在基本的追踪功能上，支ETM/PTM、funnel、replicator、TMC、TPIU ETB 等组件。未来的工作将支持更复杂IP 块，STM CTI

### 缩写与分

缩写
PTM:
    程序追踪宏单元（Program Trace MacrocellETM:
    嵌入式追踪宏单元（Embedded Trace MacrocellSTM:
    系统追踪宏单元（System trace MacrocellETB:
    嵌入式追踪缓冲区（Embedded Trace BufferITM:
    检测追踪宏单元（Instrumentation Trace MacrocellTPIU:
     追踪端口接口单元（Trace Port Interface UnitTMC-ETR:
        追踪内存控制器，配置为嵌入式追踪路由器（Embedded Trace RouterTMC-ETF:
        追踪内存控制器，配置为嵌入式追踪 FIFO（Embedded Trace FIFOCTI:
    交叉触发接口（Cross Trigger Interface
分类
源（Source
   ETMv3.x ETMv4, PTMv1.0, PTMv1.1, STM, STM500, ITM
链路（Link
   Funnel, replicator（智能或非智能）, TMC-ETR
汇（Sinks
   ETBv1.0, ETB1.1, TPIU, TMC-ETF
其他（Misc
   CTI


### 设备树绑

详见 `Documentation/devicetree/bindings/arm/arm,coresight-*.yaml`
截至撰写本文时，ITM、STM CTI 的驱动尚未提供，但预计会随着方案成熟而加入

### 框架与实

coresight 框架提供了一个中心点来表示、配置和管理平台上的 coresight 设备。任何符coresight 规范的设备，只要使用正确API，就可以向框架注册：


注册函数接受一`struct coresight_desc *desc` 并将设备注册到核心框架。注销函数接受注册时获得的 `struct coresight_device *csdev` 引用
如果注册过程一切顺利，新设备将
```

    root:~# ls /sys/bus/coresight/devices/
    replicator  20030000.tpiu    2201c000.ptm  2203c000.etm  2203e000.etm
    20010000.etb         20040000.funnel  2201d000.ptm  2203d000.etm
    root:~#

```
```

    struct coresight_desc {
            enum coresight_dev_type type;
            struct coresight_dev_subtype subtype;
            const struct coresight_ops *ops;
            struct coresight_platform_data *pdata;
            struct device *dev;
            const struct attribute_group **groups;
    };


```
“coresight_dev_type标识设备是什么（即源、链路还是汇），“coresight_dev_subtype会进一步刻画该类型
`struct coresight_ops` 是必填的，它告诉框架如何执行与组件相关的基本操作，每个组件都有不同的需求集。为此提供了 `struct coresight_ops_sink`、`struct coresight_ops_link` `struct coresight_ops_source`
下一个字`struct coresight_platform_data *pdata` 通过调用 `of_get_coresight_platform_data()` 获取，作为驱_probe 例程的一部分，并```

    static int etm_probe(struct amba_device *adev, const struct amba_id *id)
    {
     ...
     ...
     drvdata->dev = &adev->dev;
     ...
    }

```
特定类别的设备（源、链路或汇）具有可对其执行的通用操作（见 `struct coresight_ops`）。`**groups` 是与仅该组件特有的操作相关的 sysfs 条目列表。“实现定义”（Implementation defined）的定制预期通过使用这些条目来访问和控制
### 设备命名方案


出现“coresight总线上的设备被命名为与其父设备（即出现在 AMBA 总线或平台总线上的真实设备）相同的名称。因此名称基Linux Open Firmware 层的命名约定，即先跟随时钟后面跟着设备
```

    root:~# ls /sys/bus/coresight/devices/
     20010000.etf  20040000.funnel      20100000.stm     22040000.etm
     22140000.etm  230c0000.funnel      23240000.etm     20030000.tpiu
     20070000.etr  20120000.replicator  220c0000.funnel
     23040000.etm  23140000.etm         23340000.etm

```
然而，随着 ACPI 支持的引入，真实设备的名称有些晦涩且不直观。因此，引入了一种新的命名方案，根据设备类型使用更通用的名称。该
```

  1) 绑定CPU 的设备，根据 CPU 的逻辑编号命名
     e.g, ETM bound to CPU0 is named "etm0"

  2) 所有其他设备遵循一种模式，"<device_type_prefix>N"，其中：

	<device_type_prefix> 	- 特定于设备类型的前缀
	N			- 根据探测顺序分配的序号
	e.g, tmc_etf0, tmc_etr0, funnel0, funnel1

```
```

    root:~# ls /sys/bus/coresight/devices/
     etm0     etm1     etm2         etm3  etm4      etm5      funnel0
     funnel1  funnel2  replicator0  stm0  tmc_etf0  tmc_etr0  tpiu0

```
下面的一些示例可能引用旧的命名方案，一些引用新的方案，以确认你在系统上看到的并非异常。必须使用系统上指定位置出现时的“名称”
### 拓扑表示


每个 CoreSight 组件都有一`connections` 目录，其中包含指向其CoreSight 组件的链接。这允许用户探索追踪拓扑，对于较大的系统，可以确定给定源最合适的汇。连接信息还可用于确定哪CTI 设备连接到给定组件。该目录包含一`nr_links` 属性，详细说明目录中的链接数量
对于一ETM 源，本例中为 Juno 平台上的 `etm0`，一个典```

  linaro-developer:~# ls - l /sys/bus/coresight/devices/etm0/connections
  <file details>  cti_cpu0 -> ../../../23020000.cti/cti_cpu0
  <file details>  nr_links
  <file details>  out:0 -> ../../../230c0000.funnel/funnel2

```
```

  linaro-developer:~# ls -l /sys/bus/coresight/devices/funnel2/connections
  <file details> in:0 -> ../../../23040000.etm/etm0
  <file details> in:1 -> ../../../23140000.etm/etm3
  <file details> in:2 -> ../../../23240000.etm/etm4
  <file details> in:3 -> ../../../23340000.etm/etm5
  <file details> nr_links
  <file details> out:0 -> ../../../20040000.funnel/funnel0

```
```

  linaro-developer:~# ls -l /sys/bus/coresight/devices/funnel0/connections
  <file details> in:0 -> ../../../220c0000.funnel/funnel1
  <file details> in:1 -> ../../../230c0000.funnel/funnel2
  <file details> nr_links
  <file details> out:0 -> ../../../20010000.etf/tmc_etf0

```
找到第一个汇 `tmc_etf0`。这可用于收集数```

  linaro-developer:~# ls -l /sys/bus/coresight/devices/tmc_etf0/connections
  <file details> cti_sys0 -> ../../../20020000.cti/cti_sys0
  <file details> in:0 -> ../../../20040000.funnel/funnel0
  <file details> nr_links
  <file details> out:0 -> ../../../20150000.funnel/funnel4

```
```

  linaro-developer:~# ls -l /sys/bus/coresight/devices/funnel4/connections
  <file details> in:0 -> ../../../20010000.etf/tmc_etf0
  <file details> in:1 -> ../../../20140000.etf/tmc_etf1
  <file details> nr_links
  <file details> out:0 -> ../../../20120000.replicator/replicator0

```
```

  linaro-developer:~# ls -l /sys/bus/coresight/devices/replicator0/connections
  <file details> in:0 -> ../../../20150000.funnel/funnel4
  <file details> nr_links
  <file details> out:0 -> ../../../20030000.tpiu/tpiu0
  <file details> out:1 -> ../../../20070000.etr/tmc_etr0

```
```

  linaro-developer:~# ls -l /sys/bus/coresight/devices/tmc_etr0/connections
  <file details> cti_sys0 -> ../../../20020000.cti/cti_sys0
  <file details> in:0 -> ../../../20120000.replicator/replicator0
  <file details> nr_links

```
如下所述，使用 sysfs 时，只需使能一个汇和一个源即可成功追踪。框架会按需正确使能所有中间链路
注意：`cti_sys0` 出现在上面两个连接列表中。CTI 可以连接到多个设备，并通过 CTM 以星型拓扑排列。详(Documentation/trace/coresight/coresight-ect.rst) [#fourth]_```

  linaro-developer:~# ls -l /sys/bus/coresight/devices/cti_sys0/connections
  <file details> nr_links
  <file details> stm0 -> ../../../20100000.stm/stm0
  <file details> tmc_etf0 -> ../../../20010000.etf/tmc_etf0
  <file details> tmc_etr0 -> ../../../20070000.etr/tmc_etr0
  <file details> tpiu0 -> ../../../20030000.tpiu/tpiu0


```
### 如何使用追踪器模

使用 Coresight 框架有两种方式：

1. 使用 perf 命令行工具2. 使用 sysFS 接口直接Coresight 设备交互
优先使用前者，因为使用 sysFS 接口需要对 Coresight 硬件有深入理解。以下各节提供两种方法的详细信息
#### 使用 sysFS 接口


在开始追踪收集之前，需要确定一coresight 汇。在任何给定时刻可以启用的汇（以及源）数量没有限制。作为通用操作，所有属于该汇的设备
```

    root:/sys/bus/coresight/devices# ls
    replicator  20030000.tpiu    2201c000.ptm  2203c000.etm  2203e000.etm
    20010000.etb         20040000.funnel  2201d000.ptm  2203d000.etm
    root:/sys/bus/coresight/devices# ls 20010000.etb
    enable_sink  status  trigger_cntr
    root:/sys/bus/coresight/devices# echo 1 > 20010000.etb/enable_sink
    root:/sys/bus/coresight/devices# cat 20010000.etb/enable_sink
    1
    root:/sys/bus/coresight/devices#

```
在启动时，当前的 etm3x 驱动会将第一个地址比较器配置为 “_stext“_etext”，本质上追踪落在该范围内的任何指令。因此“使能”一个源将立```

    root:/sys/bus/coresight/devices# echo 1 > 2201c000.ptm/enable_source
    root:/sys/bus/coresight/devices# cat 2201c000.ptm/enable_source
    1
    root:/sys/bus/coresight/devices# cat 20010000.etb/status
    Depth:          0x2000
    Status:         0x1
    RAM read ptr:   0x0
    RAM wrt ptr:    0x19d3   <----- The write pointer is moving
    Trigger cnt:    0x0
    Control:        0x1
    Flush status:   0x0
    Flush ctrl:     0x2001
    root:/sys/bus/coresight/devices#

```
```

    root:/sys/bus/coresight/devices# echo 0 > 2201c000.ptm/enable_source
    root:/sys/bus/coresight/devices#

```
```

    root:/sys/bus/coresight/devices# dd if=/dev/20010000.etb \
    of=~/cstrace.bin
    64+0 records in
    64+0 records out
    32768 bytes (33 kB) copied, 0.00125258 s, 26.2 MB/s
    root:/sys/bus/coresight/devices#

```
文件 cstrace.bin 可以使用 “ptm2human”、DS-5 Trace32 解压缩
以下是一DS-5 输出，展示了一个实验性循环将变量递增到某个值的过程。该示例很简单，却让人一coresight 所提供的丰富可能性```

    Info                                    Tracing enabled
    Instruction     106378866       0x8026B53C      E52DE004        false   PUSH     {lr}
    Instruction     0       0x8026B540      E24DD00C        false   SUB      sp,sp,#0xc
    Instruction     0       0x8026B544      E3A03000        false   MOV      r3,#0
    Instruction     0       0x8026B548      E58D3004        false   STR      r3,[sp,#4]
    Instruction     0       0x8026B54C      E59D3004        false   LDR      r3,[sp,#4]
    Instruction     0       0x8026B550      E3530004        false   CMP      r3,#4
    Instruction     0       0x8026B554      E2833001        false   ADD      r3,r3,#1
    Instruction     0       0x8026B558      E58D3004        false   STR      r3,[sp,#4]
    Instruction     0       0x8026B55C      DAFFFFFA        true    BLE      {pc}-0x10 ; 0x8026b54c
    Timestamp                                       Timestamp: 17106715833
    Instruction     319     0x8026B54C      E59D3004        false   LDR      r3,[sp,#4]
    Instruction     0       0x8026B550      E3530004        false   CMP      r3,#4
    Instruction     0       0x8026B554      E2833001        false   ADD      r3,r3,#1
    Instruction     0       0x8026B558      E58D3004        false   STR      r3,[sp,#4]
    Instruction     0       0x8026B55C      DAFFFFFA        true    BLE      {pc}-0x10 ; 0x8026b54c
    Instruction     9       0x8026B54C      E59D3004        false   LDR      r3,[sp,#4]
    Instruction     0       0x8026B550      E3530004        false   CMP      r3,#4
    Instruction     0       0x8026B554      E2833001        false   ADD      r3,r3,#1
    Instruction     0       0x8026B558      E58D3004        false   STR      r3,[sp,#4]
    Instruction     0       0x8026B55C      DAFFFFFA        true    BLE      {pc}-0x10 ; 0x8026b54c
    Instruction     7       0x8026B54C      E59D3004        false   LDR      r3,[sp,#4]
    Instruction     0       0x8026B550      E3530004        false   CMP      r3,#4
    Instruction     0       0x8026B554      E2833001        false   ADD      r3,r3,#1
    Instruction     0       0x8026B558      E58D3004        false   STR      r3,[sp,#4]
    Instruction     0       0x8026B55C      DAFFFFFA        true    BLE      {pc}-0x10 ; 0x8026b54c
    Instruction     7       0x8026B54C      E59D3004        false   LDR      r3,[sp,#4]
    Instruction     0       0x8026B550      E3530004        false   CMP      r3,#4
    Instruction     0       0x8026B554      E2833001        false   ADD      r3,r3,#1
    Instruction     0       0x8026B558      E58D3004        false   STR      r3,[sp,#4]
    Instruction     0       0x8026B55C      DAFFFFFA        true    BLE      {pc}-0x10 ; 0x8026b54c
    Instruction     10      0x8026B54C      E59D3004        false   LDR      r3,[sp,#4]
    Instruction     0       0x8026B550      E3530004        false   CMP      r3,#4
    Instruction     0       0x8026B554      E2833001        false   ADD      r3,r3,#1
    Instruction     0       0x8026B558      E58D3004        false   STR      r3,[sp,#4]
    Instruction     0       0x8026B55C      DAFFFFFA        true    BLE      {pc}-0x10 ; 0x8026b54c
    Instruction     6       0x8026B560      EE1D3F30        false   MRC      p15,#0x0,r3,c13,c0,#1
    Instruction     0       0x8026B564      E1A0100D        false   MOV      r1,sp
    Instruction     0       0x8026B568      E3C12D7F        false   BIC      r2,r1,#0x1fc0
    Instruction     0       0x8026B56C      E3C2203F        false   BIC      r2,r2,#0x3f
    Instruction     0       0x8026B570      E59D1004        false   LDR      r1,[sp,#4]
    Instruction     0       0x8026B574      E59F0010        false   LDR      r0,[pc,#16] ; [0x8026B58C] = 0x80550368
    Instruction     0       0x8026B578      E592200C        false   LDR      r2,[r2,#0xc]
    Instruction     0       0x8026B57C      E59221D0        false   LDR      r2,[r2,#0x1d0]
    Instruction     0       0x8026B580      EB07A4CF        true    BL       {pc}+0x1e9344 ; 0x804548c4
    Info                                    Tracing enabled
    Instruction     13570831        0x8026B584      E28DD00C        false   ADD      sp,sp,#0xc
    Instruction     0       0x8026B588      E8BD8000        true    LDM      sp!,{pc}
    Timestamp                                       Timestamp: 17107041535

```
#### 使用 perf 框架


Coresight 追踪器使Perf 框架的性能监控单元（PMU）抽象来表示。因perf 框架负责根据感兴趣进程的调度时机来控制追踪何时被启用。当在系统中配置好时，Coresight PMU 会在 perf 命令行工具查询时列出
	linaro@linaro-nano:~$ ./perf list pmu

		List of pre-defined events (to be used in -e):

		cs_etm//                                    [Kernel PMU event]

无论系统中可用的追踪器数量多少（通常等于处理器核心数量），“cs_etmPMU 只会列出一次
Coresight PMU 的工作方式与其他任何 PMU 相同，即 PMU 的名称与配置选项一起在斜杠 内提供（`Config option formats`_）
### Perf 框架的高级用

#### 汇的选择


会为Perf 配合使用自动选择一个合适的汇，但由于通常会有多个汇，要使用的汇的名称可以作为一个以 ‘@为前缀的特殊配置选项来指定
可用的汇sysFS 下列出，位于
```

	root@localhost:/sys/bus/event_source/devices/cs_etm/sinks# ls
	tmc_etf0  tmc_etr0  tpiu0

	root@linaro-nano:~# perf record -e cs_etm/@tmc_etr0/u --per-thread program

```
关于上述及其他如何使Coresight perf 工具的示例，更多信息可在 openCSD gitHub 仓库“HOWTO.md文件中找[#third]_
#### 使用 perf 工具进行 AutoFDO 分析


perf 可用于记录和分析程序的追踪
可以使用cs_etm 事件‘perf record记录执行```

    perf record -e cs_etm//u --per-thread

```
‘perf report‘perf script命令可用于分析执行，从指令追踪中合成指令和分支事件。‘perf inject可用于用合成的事件替换追踪数据-itrace 选项控制合成事件的类型和频率（见 perf 文档）
注意目前仅支64 位程—需要更多工作来支持 32 Arm 程序的指令解码
#### 追踪 PID


内核可以构建为将 PID 值写PE ContextID 寄存器。对于运行在 EL1 的内核，PID 存储CONTEXTIDR_EL1 中。PE 可以实现 Arm 虚拟化主机扩展（VHE），内核可运行在 EL2 作为虚拟化主机；此时，PID 值存储在 CONTEXTIDR_EL2 中
perf 提供 PMU 格式来编ETM，将这些值插入追踪数据；PMU 格式定义如下
  “contextid1”：EL1 内核EL2 内核上都可用。当内核运行EL1 时，“contextid1启用 PID 追踪；当内核运行EL2 时，这启用对客户机应用程PID 的追踪
  “contextid2”：仅在内核运行EL2 时可用。选中时，启用 EL2 内核上的 PID 追踪
  “contextid”：将作为启PID 追踪选项的别名。即，在 EL1 内核contextid == contextid1，在 EL2 内核contextid == contextid2
perf 总是在相关的 EL 上启PID 追踪，这是通过自动启用 “contextid配置实现—但对EL2，可以使“contextid1“contextid2配置进行特定调整，例如，如果用户想同时追踪主机和客户机的 PID，可以同时设“contextid1“contextid2这两个配置：

  perf record -e cs_etm/contextid1,contextid2/u -- vm


#### 为反馈导向优化（Feedback Directed Optimization）生成覆盖率文件：AutoFDO


‘perf inject接受 --itrace 选项，此时追踪数据被移除并替换为合成的事件。例```

	perf inject --itrace --strip -i perf.data -o perf.data.new

```
以下是使ARM ETM 进行 autoFDO 的示例。它需autofdo (https://github.com/google/autofdo) gcc 5 版本。bubble sort 示例来自 AutoFDO 教程 (https://gcc.gnu.org/wiki/AutoFDO/Tutorial)```

	$ gcc-5 -O3 sort.c -o sort
	$ taskset -c 2 ./sort
	Bubble sorting array of 30000 elements
	5910 ms

	$ perf record -e cs_etm//u --per-thread taskset -c 2 ./sort
	Bubble sorting array of 30000 elements
	12543 ms
	[ perf record: Woken up 35 times to write data ]
	[ perf record: Captured and wrote 69.640 MB perf.data ]

	$ perf inject -i perf.data -o inj.data --itrace=il64 --strip
	$ create_gcov --binary=./sort --profile=inj.data --gcov=sort.gcov -gcov_version=1
	$ gcc-5 -O3 -fauto-profile=sort.gcov sort.c -o sort_autofdo
	$ taskset -c 2 ./sort_autofdo
	Bubble sorting array of 30000 elements
	5806 ms

```
#### 配置选项格式


以下字符串可以在 perf 命令行上提供// 之间，以启用各种选项。它们也列在文件/sys/bus/event_source/devices/cs_etm/format/ 
   :header-rows: 1

   - - 选项（Option     - 描述（Description   - - branch_broadcast
     - 系统级设置的会话本地版本：ETM_MODE_BB <coresight-branch-broadcast>
   - - contextid
     - 瑙?`Tracing PID`_
   - - contextid1
     - 瑙?`Tracing PID`_
   - - contextid2
     - 瑙?`Tracing PID`_
   - - configid
     - 用于自定义配置的选择。这是一个实现细节，不直接使用，trace/coresight/coresight-config:Using Configurations in perf
   - - preset
     - 自定义配置中参数的覆盖，trace/coresight/coresight-config:Using Configurations in perf
   - - sinkid
     - 用于选择汇的字符串的哈希版本，使@ 表示法时自动设置。这是内部实现细节，不直接使用，`Using perf
       framework`_銆?   - - cycacc
     - 系统级设置的会话本地版本ref:`ETMv4_MODE_CYCACC
       <coresight-cycle-accurate>`
   - - retstack
     - 系统级设置的会话本地版本ref:`ETM_MODE_RETURNSTACK
       <coresight-return-stack>`
   - - timestamp
     - 控制时间戳的生成和间隔
       0 = 关闭 = 最小间.. 15 = 最大间隔
       1 - 14 使用一个每周期递减的计数器，在递减到零时生成时间戳。计数器的重载值为 2 ^ (interval
       - 1)。如果值为 1，则重载值为 1；如果值为 11，则重载值为 1024，依此类推
       设置最大间隔（15）将禁用计数器生成的时间戳，释放计数器资源，只保留生SYNC 包时发出的时间戳。同步间隔由 TRCSYNCPR.PERIOD 控制，默认每 4096 字节的追踪生成一个
   - - cc_threshold
     - 周期计数阈值。如果这里未提供值或提供的值为 0，则使用默认值（0x100）。如果提供的值小于最小周期阈值（TRCIDR3.CCITMIN 指示），则改用最小值
### 如何使用 STM 模块


使用 System Trace Macrocell 模块与使用追踪器相同 —唯一的区别是客户端驱动追踪捕获，而不是代码中的程序流
与任何其CoreSight 组件一样，关于 STM 追踪器的具体信息可以
```

    root@genericarmv8:~# ls /sys/bus/coresight/devices/stm0
    enable_source   hwevent_select  port_enable     subsystem       uevent
    hwevent_enable  mgmt            port_select     traceid
    root@genericarmv8:~#

```
与任何其他源一样，需要先确定汇并使能 STM，然```

    root@genericarmv8:~# echo 1 > /sys/bus/coresight/devices/tmc_etf0/enable_sink
    root@genericarmv8:~# echo 1 > /sys/bus/coresight/devices/stm0/enable_source

```
此后用户空间应用程序可以通过 devfs 请求并使用通道
```

    root@genericarmv8:~# ls -l /dev/stm0
    crw-------    1 root     root       10,  61 Jan  3 18:11 /dev/stm0
    root@genericarmv8:~#

```
关于如何使用通用 STM API 的详细信息可在此处找到：
- Documentation/trace/stm.rst [#second]_銆?
### CTI CTM 模块


CTI（Cross Trigger Interface，交叉触发接口）在单CTI 与组件之间提供一组触发信号，并可以通过 CTM（Cross Trigger Matrix，交叉触发矩阵）上的通道在所CTI 之间传播这些信号
提供了一份单独的文档文件来解释这些设备的使用(Documentation/trace/coresight/coresight-ect.rst) [#fourth]_
### CoreSight 系统配置


CoreSight 组件可以是具有许多编程选项的复杂设备。此外，组件可以被编程为在整个系统中相互交互
提供CoreSight 系统配置管理器，以便能从 perf sysfs 中轻松选择和使用这些复杂的编程配置
更多信息请参阅单独的文档(Documentation/trace/coresight/coresight-config.rst) [#fifth]_