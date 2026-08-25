
## ``amd-pstate`` CPU 性能缩放驱动


:Copyright: |copy| 2021 Advanced Micro Devices, Inc.

:Author: Huang Rui <ray.huang@amd.com>


## 引言


`amd-pstate` AMD CPU 性能缩放驱动，它在现AMD APU CPU 系列
Linux 内核引入了一种新CPU 频率控制机制。新机制基于协作处理器性能控制
（Collaborative Processor Performance Control，CPPC），它提供比传统 ACPI
硬件 P-States 更细粒度的频率管理。当AMD CPU/APU 平台使用 ACPI P-states
驱动，仅能在 3 P-state 之间切换来管CPU 频率和时钟。CPPC 取代ACPI
P-states 控制，并Linux 内核提供了一个灵活、低延迟的接口，用于直接与硬
通信性能提示

`amd-pstate` 利用 Linux 内核的调控器（governor），例如 `schedutil`
`ondemand` 等，来管理由 CPPC 硬件功能提供的性能提示，后者在内部遵循硬件
规范（详情参AMD64 架构程序员手册第 2 卷：系统编程 [^1^]_）。目前，
`amd-pstate` 已在一Zen2 Zen3 处理器上，根据内核调控器支持基本的频
控制功能，在我们在硬件和 SBIOS 上验证之后，未来会实现更AMD 特有的功能

## AMD CPPC 概述


协作处理器性能控制（Collaborative Processor Performance Control，CPPC）接
枚举一个连续的、抽象的、无单位（unit-less）的性能值，其刻度并不绑定到特定
性能状/ 频率。这是一ACPI 标准 [^2^]_，软件可以据此将应用性能目标和提
作为相对目标指定给基础设施的限制。AMD 处理器提供低延迟的寄存器模型（MSR），
而不是用 AML 代码解释器来进行性能调整。`amd-pstate` 会用回调初始化一
`struct cpufreq_driver` 实例 `amd_pstate_driver`

```

 Highest Perf ------>+-----------------------+                         +-----------------------+
                     |                       |                         |                       |
                     |                       |                         |                       |
                     |                       |          Max Perf  ---->|                       |
                     |                       |                         |                       |
                     |                       |                         |                       |
 Nominal Perf ------>+-----------------------+                         +-----------------------+
                     |                       |                         |                       |
                     |                       |                         |                       |
                     |                       |                         |                       |
                     |                       |                         |                       |
                     |                       |                         |                       |
                     |                       |                         |                       |
                     |                       |      Desired Perf  ---->|                       |
                     |                       |                         |                       |
                     |                       |                         |                       |
                     |                       |                         |                       |
                     |                       |                         |                       |
                     |                       |                         |                       |
                     |                       |                         |                       |
                     |                       |                         |                       |
                     |                       |                         |                       |
  Lowest non-        |                       |                         |                       |
  linear perf ------>+-----------------------+                         +-----------------------+
                     |                       |                         |                       |
                     |                       |          Min perf  ---->|                       |
                     |                       |                         |                       |
  Lowest perf ------>+-----------------------+                         +-----------------------+
                     |                       |                         |                       |
                     |                       |                         |                       |
                     |                       |                         |                       |
          0   ------>+-----------------------+                         +-----------------------+

                                     AMD P-States Performance Scale


```
### AMD CPPC 性能能力


Highest Performance (RO)
.........................

这是单个处理器在理想条件下可能达到的绝对最大性能。该性能水平可能无法长时
持续，并且可能仅当其他平台组件处于特定状态时才可实现；例如，它可能要求其
处理器处于空闲状态。这相当于处理器支持的最高频率

Nominal (Guaranteed) Performance (RO)
......................................

这是处理器在理想运行条件下的最大持续性能水平。在没有外部约束（功耗、温度等
的情况下，这是处理器预期能够持续维持的性能水平。所有核 / 处理器都应能够同
维持其标称性能状态

Lowest non-linear Performance (RO)
...................................

这是实现非线性节能的最低性能水平，例如由于电压和频率缩放的综合效应。高于此
阈值时，较低的性能水平通常应比较高性能水平更节能。该寄存器有效地`amd-pstate`
传达了最高效的性能水平

Lowest Performance (RO)
........................

这是处理器的绝对最低性能水平。选择低于最低非线性性能水平的性能可能导致效率
损失，但应会降低处理器的瞬时功耗

### AMD CPPC 性能控制


`amd-pstate` 通过这些寄存器传递性能目标。该寄存器驱动期望性能目标的行为

Minimum requested performance (RW)
...................................

`amd-pstate` 指定允许的最小性能水平

Maximum requested performance (RW)
...................................

`amd-pstate` 指定硬件预期提供的最大性能的限制

Desired performance target (RW)
...................................

`amd-pstate` CPPC 性能刻度中以一个相对数字指定期望目标。这可以表示为标
性能（基础设施最大值）的百分比。在标称持续性能水平以下，期望性能表示受硬
约束的处理器平均性能水平。在标称性能水平以上，处理器必须至少提供所请求的标
性能，并在当前运行条件允许时进一步提高

Energy Performance Preference (EPP) (RW)
.........................................

该属性向硬件提供一个提示，表示软件希望偏向性能x0）还是能效（0xff）


## 关键调控器支


`amd-pstate` 可以`sysfs` `scaling_available_governors` 策略属性列出的
所有（通用）缩放调控器一起使用。然后，它负责配置与 CPU 对应的策略对象，并向
`CPUFreq` 核心（以及附加到策略对象的缩放调控器）提供硬件支持的最大和最小运
频率的准确信息。用户可以查看来`CPUFreq` 核心`scaling_cur_freq` 信息

`amd-pstate` 主要支持 `schedutil` `ondemand` 用于动态频率控制。它是将
处理器配置通过 `amd-pstate` 微调到带 CPU CFS 调度器的 `schedutil`。`amd-pstate`
注册 adjust_perf 回调，以实现类似CPPC 的性能更新行为。它`sugov_start`
初始化，然后填充 CPU update_util_data 指针，将 `sugov_update_single_perf`
赋值为 CPU 调度器中的利用率更新回调函数。CPU 调度器将调用 `cpufreq_update_util`
并根据该利用率更新所属的 `struct sugov_cpu` 分配目标性能。然后，`amd-pstate`
根据 CPU 调度器分配的值更新期望性能


## 处理器支


如果检测到的处理器ACPI SBIOS 里不存在 `_CPC` 条目，`amd-pstate` 的初始化
将失败。它使用 `acpi_cpc_valid` 来检`_CPC` 是否存在。所有基Zen 的处理器
都支持传统的 ACPI 硬件 P-States 功能，因此当 `amd-pstate` 初始化失败时
内核会回退去初始化 `acpi-cpufreq` 驱动

`amd-pstate` 有两种硬件实现：一种是 `Full MSR Support <perf_cap_>`_，另一种是
`Shared Memory Support <perf_cap_>`_。它可以使用 `X86_FEATURE_CPPC` 特性标
来指示不同的类型。（详情参见 AMD Family 19h Model 51h Revision A1 处理器编
参考手册（PPR）[^3^]_。）`amd-pstate` 会为不同的硬件实现注册不同的 `static_call`
实例

目前，一Zen2 Zen3 处理器支`amd-pstate`。未来，它将在越来越多的 AMD
处理器上得到支持

### 完整 MSR 支持（Full MSR Support


一些新Zen3 处理器（Cezanne）在 `X86_FEATURE_CPPC` CPU 特性标志被设置时，
直接提供 MSR 寄存器。`amd-pstate` 可以处理 MSR 寄存器，`CPUFreq` 中实现快
切换（fast switch）功能，从而降低中断上下文中频率控制的延迟。带`pstate_xxx`
前缀的函数表示对 MSR 寄存器的操作

### 共享内存支持（Shared Memory Support


如果未设`X86_FEATURE_CPPC` CPU 特性标志，则处理器支持共享内存方案。在这种
情况下，`amd-pstate` 使用 `cppc_acpi` 辅助方法来实现定义在 `static_call` 上的
回调函数。带`cppc_xxx` 前缀的函数表示对共享内存方案ACPI CPPC 辅助方法
操作


AMD P-States ACPI 硬件 P-States 始终可以在同一个处理器上得到支持。但 AMD
P-States 具有更高的优先级，如果它通过 `MSR_AMD_CPPC_ENABLE` `cppc_set_enable`
被启用，它将响应来自 AMD P-States 的请求


## 用户空间接口（``sysfs``）—每策略控


`amd-pstate` `sysfs` 中暴露了几个全局属性（文件）来在系统级别控制其功能
它们位于

```

 root@hr-test1:/home/ray# ls /sys/devices/system/cpu/cpufreq/policy0/*amd*
 /sys/devices/system/cpu/cpufreq/policy0/amd_pstate_highest_perf
 /sys/devices/system/cpu/cpufreq/policy0/amd_pstate_hw_prefcore
 /sys/devices/system/cpu/cpufreq/policy0/amd_pstate_lowest_nonlinear_freq
 /sys/devices/system/cpu/cpufreq/policy0/amd_pstate_max_freq
 /sys/devices/system/cpu/cpufreq/policy0/amd_pstate_floor_freq
 /sys/devices/system/cpu/cpufreq/policy0/amd_pstate_floor_count
 /sys/devices/system/cpu/cpufreq/policy0/amd_pstate_prefcore_ranking


```
`amd_pstate_highest_perf / amd_pstate_max_freq`

驱动允许设置的最CPPC 性能CPU 频率，以最大支持的 CPPC 性能水平
（在 `AMD CPPC Performance Capability <perf_cap_>`_ 中的最高性能）的百分比表示
在某ASIC 中，最高的 CPPC 性能并不`_CPC` 表中，因此我们需要把它暴露到
sysfs。如boost 未激活但仍受支持，该最大频率将大于 `cpuinfo` 中的那个
该属性是只读的

`amd_pstate_lowest_nonlinear_freq`

驱动允许设置的最低非线CPPC CPU 频率，以最大支持的 CPPC 性能水平的百分比表示
（请参见 `AMD CPPC Performance Capability <perf_cap_>`_ 中的最低非线性性能。）
该属性是只读的

`amd_pstate_hw_prefcore`

平台是否支持首选核（preferred core）特性并且已启用。该属性是只读的。这个文
只在支持首选核特性的平台上可见

`amd_pstate_prefcore_ranking`

该核的性能排名。这个数字没有任何单位，但在读取时数值越大越被优先。它会根
平台条件在运行时变化。该属性是只读的。这个文件只在支持首选核特性的平台上可见

`amd_pstate_floor_freq`

与每CPU 关联的地板频率（floor frequency）。用户空间可以向该文件写
`cpuinfo_min_freq` `scaling_max_freq` 之间的任意值。当系统处于功耗或温度
约束下时，平台固件会尝试先将 CPU 频率限制`amd_pstate_floor_freq` 中指定的
值，然后再进一步限制。这允许用户空间为不同的 CPU 指定不同的地板频率。为了获
最佳结果，同一核的线程应具有相同的地板频率值。这个文件只在支CPPC 性能优先
（Performance Priority）特性的平台上可见


`amd_pstate_floor_count`

平台支持的不同的地板性能（Floor Performance）级别的数量。例如，如果该值为 2
那么从命``cat
/sys/devices/system/cpu/cpufreq/policy*/amd_pstate_floor_freq |
sort -n | uniq`` 获得的唯一值的数量，对`amd_pstate_floor_freq` 中描述的行为
生效而言，最多应为该数字。零值表示平台支持无限多的地板性能级别。这个文件只
支持 CPPC 性能优先级特性的平台上可见

**注意**：当 `amd_pstate_floor_count` 非零时，如果在功率或温度约束下对 CPU 进行
限制的频点，在系统中所CPU `amd_pstate_floor_freq` 唯一值数量超
`amd_pstate_floor_count` 时，是未定义的

`energy_performance_available_preferences`

可用于本系统 `energy_performance_preference` 的所有受支持EPP 偏好的列表
这些配置文件代表了提供给底层固件的不同提示，关于用户所期望的能效与性能权衡
`default` 表示 epp 值由平台固件设置。`custom` 表示也可以写0-255 的整数值
该属性是只读的

`energy_performance_preference`

当前的能效性能偏好可以从该属性读取，用户可以根据能效或性能需求更改当前偏好
该属性中提供了粗粒度的命名配置文
`energy_performance_available_preferences`銆。
用户也可以写0 255 之间的单个整数值
当启用了动EPP 时，即使平台固件已启EPP 特性，energy_performance_preference
的写入也会被阻止。较低的 epp 值会将偏向转向改进性能，而较高的 epp 值会将偏
转向节能。确切的影响会因平台而异
如果最后一次写入的是有效整数，则未来读取时将返回一个数字
如果最后一次写入的是有效字符串，则未来读取时将返回一个字符串
该属性是可读写的

`boost`
`boost` sysfs 属性提供对 CPU 核性能 boost 的控制，允许用户管理 CPU 的最
频率限制。该属性可用于在单CPU 上启用或禁用 boost 特性

boost 特性启用时，CPU 可以动态地将频率提升到基础频率之上，为要求苛刻
工作负载提供增强的性能。另一方面，禁boost 特性会CPU 限制在基础频率
运行，在某些场景下为了优先考虑能效或管理温度，这可能是可取的

要操`boost` 属性，用户可以使用 sysfs 路径
`/sys/devices/system/cpu/cpuX/cpufreq/boost`（其`X` 表示 CPU 编号），
相应CPU 写入`0` 来禁boost，或写入 `1` 来启boost

其他性能和频率值可以从
`/sys/devices/system/cpu/cpuX/acpi_cppc/` 读回，参cppc_sysfs

## 动态能效性能配置文件


amd-pstate 驱动支持根据机器是运行在交流（AC）还是直流（DC）电源上，动态地
选择能效性能配置文件

此行为是否默认启用取决于内核命令行选项 `amd_dynamic_epp` 是否被设置。该行为
也可以在运行时通过 sysfs 文件
`/sys/devices/system/cpu/amd_pstate/dynamic_epp` 被覆盖

当设置为启用时，驱动会在机器运行在电池或交流电源上时选择不同的能效性能配置文件
驱动还会向平台配置文件处理程序（platform profile handler）注册，以接收用
期望的电源状态通知并做出反应。当设置为禁用时，驱动不会根据电源来源改变能
性能配置文件，也不会对用户的期望电源状态做出反应

`dynamic_epp` 启用时，尝试手动写入 `energy_performance_preference` sysfs
文件将会失败

## ``amd-pstate`` ``acpi-cpufreq`` 对比


`acpi-cpufreq` 支持的大多数 AMD 平台上，平台固件提供ACPI 表用CPU
性能缩放，但AMD 处理器上仅提3 P-state。然而，在现AMD APU CPU
系列上，硬件根据 ACPI 协议提供协作处理器性能控制，并针对 AMD 平台进行了定制
也就是说，是细粒度且连续的频率范围，而不是传统的硬件 P-states。`amd-pstate`
是支持未来大多数 AMD 平台上新AMD P-States 机制的内核模块。AMD P-States
机制AMD 处理器上性能和能效更高的频率管理方法


## ``amd-pstate`` 驱动运行模式


`amd_pstate` CPPC 3 种运行模式：自主（active）模式、非自主（passive）模
和引导自主（guided）模式。可以通过不同的内核参数选择 active/passive/guided 模式

- 在自主模式下，平台忽略期望性能水平的请求，只考虑设置到最小值、最大值和
  能效性能偏好寄存器中的值
- 在非自主模式下，平台通过期望性能寄存器（Desired Performance Register）直接从
  OS 获取期望性能水平
- 在引导自主模式下，平台根据当前工作负载，并在 OS 通过最小和最大性能寄存器设定的
  限制范围内，自主地设置运行性能水平

### 主动模式（Active Mode


`amd_pstate=active`

这是`amd_pstate_epp` 驱动实现的底层固件控制模式，通过在命令行向内核传
`amd_pstate=active` 来启用。在此模式下，`amd_pstate_epp` 驱动向硬件提供一
提示，表示软件想要偏向性能x0）还是能效（0xff）到 CPPC 固件。然CPPC 电源
算法将根据电源供应和温度、核电压以及其他一些硬件条件计算运行时工作负载并调
实时核频率

### 被动模式（Passive Mode


`amd_pstate=passive`

如果在命令行向内核传递了 `amd_pstate=passive`，则会启用此模式。在此模式下
`amd_pstate` 驱动软件CPPC 性能刻度中以一个相对数字指定期望的 QoS 目标
这可以表示为标称性能（基础设施最大值）的百分比。在标称持续性能水平以下
期望性能表示受性能降低容差（Performance Reduction Tolerance）寄存器约束
处理器平均性能水平。在标称性能水平以上，处理器必须至少提供所请求的标称性能
并在当前运行条件允许时进一步提高

### 引导模式（Guided Mode


`amd_pstate=guided`

如果在内核命令行选项中传递了 `amd_pstate=guided`，则会激活此模式。在此模式下
驱动请求最小和最大性能水平，平台在该范围内自主选择一个适合当前工作负载
性能水平

## ``amd-pstate`` 首选核（Preferred Core


核频率受制于半导体中的工艺差异。并非所有核都能在遵守基础设施限制的情况下达到
最大频率。因此，AMD 重新定义了部件最大频率的概念。这意味着一部分核可以达
最大频率。为了给给定场景找到最佳的进程调度策略，OS 需要通过 CPPC 接口的最
性能能力寄存器获知平台告知的核排序

`amd-pstate` 首选核使调度器优先调度在能够以更低电压达到更高频率的核上。首选核
排名可以根据工作负载、平台条件、温度和老化而动态变化

优先级度量将`amd-pstate` 驱动初始化。`amd-pstate` 驱动还将确定平台是否支持
`amd-pstate` 首选核

`amd-pstate` 驱动将在系统启动时提供一个初始的核排序。平台使CPPC 接口将核
排名传达给操作系统和调度器，以确OS 优先选择具有最高性能的核来调度进程。当
`amd-pstate` 驱动收到最高性能变化的消息时，它将更新核排名并设cpu 的优先级

## ``amd-pstate`` 首选核开


### 内核参数


`amd-pstate` peferred core`` 有两种状态：启用和禁用
可以通过不同的内核参数选择启用 / 禁用状态
默认启用 `amd-pstate` 首选核

`amd_prefcore=disable`

对于支持 `amd-pstate` 首选核的系统，核排名将始终由平台通告。但 OS 可以通过
内核参数 `amd_prefcore=disable` 选择忽略它

`amd_dynamic_epp`

AMD pstate 处于自动模式时，动EPP 将控制内核是否自主更EPP 模式。默
为禁用。可以通过内核参数 `amd_dynamic_epp=enable` 启用

## 用户空间接口（``sysfs``）—通用


### 全局属


`amd-pstate` `sysfs` 中暴露了几个全局属性（文件）来在系统级别控制其功能
它们位于 `/sys/devices/system/cpu/amd_pstate/` 目录，并影响所CPU

`status`
	驱动的运行模式："active"passive"guided" "disable"

	"active"
		驱动处于可用状态，并处`active mode`

	"passive"
		驱动处于可用状态，并处`passive mode`

	"guided"
		驱动处于可用状态，并处`guided mode`

	"disable"
		驱动已注销，当前不可用

        可以写入该属性以更改驱动的运行模式或注销它。写入的字符串必须是其可能
        之一，如果成功，向该 sysfs 文件写入这些值之一将导致驱动切换到该字符串
        所代表的运行模式——或"disable" 情况下被注销

`prefcore`
	驱动的首选核状态："enabled" "disabled"

	"enabled"
		启用 `amd-pstate` 首选核

	"disabled"
		禁用 `amd-pstate` 首选核


        该属性是只读的，用于检查由内核参数设置的首选核状态

## ``cpupower`` 工具``amd-pstate`` 的支


`amd-pstate` `cpupower` 工具支持，该工具可用于转储频率信息。目前正在开发中
以支持越来越多的

```

 root@hr-test1:/home/ray# cpupower frequency-info
 analyzing CPU 0:
   driver: amd-pstate
   CPUs which run at the same hardware frequency: 0
   CPUs which need to have their frequency coordinated by software: 0
   maximum transition latency: 131 us
   hardware limits: 400 MHz - 4.68 GHz
   available cpufreq governors: ondemand conservative powersave userspace performance schedutil
   current policy: frequency should be within 400 MHz and 4.68 GHz.
                   The governor "schedutil" may decide which speed to use
                   within this range.
   current CPU frequency: Unable to call hardware
   current CPU frequency: 4.02 GHz (asserted by call to kernel)
   boost state support:
     Supported: yes
     Active: yes
     AMD PSTATE Highest Performance: 166. Maximum Frequency: 4.68 GHz.
     AMD PSTATE Nominal Performance: 117. Nominal Frequency: 3.30 GHz.
     AMD PSTATE Lowest Non-linear Performance: 39. Lowest Non-linear Frequency: 1.10 GHz.
     AMD PSTATE Lowest Performance: 15. Lowest Frequency: 400 MHz.


```
## 诊断与调


### 跟踪事件（Trace Events


有两个静态跟踪事件可用于 `amd-pstate` 的诊断。其中一个是 `cpu_frequency` 跟踪
事件，通常`CPUFreq` 使用；另一个是 `amd_pstate_perf` 跟踪事件，特定于
`amd-pstate`。可以使用以shell 命令序列来启用它们并查看其输出（如果内核

```

 root@hr-test1:/home/ray# cd /sys/kernel/tracing/
 root@hr-test1:/home/ray# echo 1 > events/amd_cpu/enable
 root@hr-test1:/home/ray# cat trace
 # tracer: nop
 #
 # entries-in-buffer/entries-written: 47827/42233061   #P:2
 #
 #                                _-----=> irqs-off
 #                               / _----=> need-resched
 #                              | / _---=> hardirq/softirq
 #                              || / _--=> preempt-depth
 #                              ||| /     delay
 #           TASK-PID     CPU#  ||||   TIMESTAMP  FUNCTION
 #              | |         |   ||||      |         |
          <idle>-0       [015] dN...  4995.979886: amd_pstate_perf: amd_min_perf=85 amd_des_perf=85 amd_max_perf=166 cpu_id=15 changed=false fast_switch=true
          <idle>-0       [007] d.h..  4995.979893: amd_pstate_perf: amd_min_perf=85 amd_des_perf=85 amd_max_perf=166 cpu_id=7 changed=false fast_switch=true
             cat-2161    [000] d....  4995.980841: amd_pstate_perf: amd_min_perf=85 amd_des_perf=85 amd_max_perf=166 cpu_id=0 changed=false fast_switch=true
            sshd-2125    [004] d.s..  4995.980968: amd_pstate_perf: amd_min_perf=85 amd_des_perf=85 amd_max_perf=166 cpu_id=4 changed=false fast_switch=true
          <idle>-0       [007] d.s..  4995.980968: amd_pstate_perf: amd_min_perf=85 amd_des_perf=85 amd_max_perf=166 cpu_id=7 changed=false fast_switch=true
          <idle>-0       [003] d.s..  4995.980971: amd_pstate_perf: amd_min_perf=85 amd_des_perf=85 amd_max_perf=166 cpu_id=3 changed=false fast_switch=true
          <idle>-0       [011] d.s..  4995.980996: amd_pstate_perf: amd_min_perf=85 amd_des_perf=85 amd_max_perf=166 cpu_id=11 changed=false fast_switch=true

```
`cpu_frequency` 跟踪事件会由 `schedutil` 缩放调控器（对于它所附加的策略）
`CPUFreq` 核心（对于使用其他缩放调控器的策略）触发


### 跟踪工具（Tracer Tool


`amd_pstate_tracer.py` 可以记录和解`amd-pstate` 跟踪日志，然后生成性能图
该工具可用于调试和调`amd-pstate` 驱动的性能。该跟踪工具需要导intel pstate
跟踪器

跟踪工具位于 `linux/tools/power/x86/amd_pstate_tracer`。它有两种使用方式。如
跟踪文件可用，则直接解析该文

```

 ./amd_pstate_trace.py [-c cpus] -t <trace_file> -n <test_name>


```
```

 sudo ./amd_pstate_trace.py [-c cpus] -n <test_name> -i <interval> [-m kbytes]


```
测试结果可以`results/test_name` 中找到。以下是示例

```

 common_cpu  common_secs  common_usecs  min_perf  des_perf  max_perf  freq    mperf   apef    tsc       load   duration_ms  sample_num  elapsed_time  common_comm
 CPU_005     712          116384        39        49        166       0.7565  9645075 2214891 38431470  25.1   11.646       469         2.496         kworker/5:0-40
 CPU_006     712          116408        39        49        166       0.6769  8950227 1839034 37192089  24.06  11.272       470         2.496         kworker/6:0-1264


```
### amd-pstate 的单元测


`amd-pstate-ut` 是一个用于测`amd-pstate` 驱动的测试模块

 - 它可以帮助所有用户验证他们的处理器支持（SBIOS/固件或硬件）

 - 内核可以有一个基本的功能测试，以避免在更新期间发生内核回归

 - 我们可以引入更多的功能或性能测试来对齐结果，这将有利于功耗和性能规模的优化

1. 测试用例描述

    1). 基本测试

        用于 `amd-pstate` 驱动的前置条件和基本功能

        +---------+--------------------------------+------------------------------------------------------------------------------------+
        | Index   | Functions                      | Description                                                                        |
        +=========+================================+====================================================================================+
        | 1       | amd_pstate_ut_acpi_cpc_valid   || Check whether the _CPC object is present in SBIOS.                                |
        |         |                                ||                                                                                   |
        |         |                                || The detail refer to `Processor Support <processor_support_>`_.                    |
        +---------+--------------------------------+------------------------------------------------------------------------------------+
        | 2       | amd_pstate_ut_check_enabled    || Check whether AMD P-State is enabled.                                             |
        |         |                                ||                                                                                   |
        |         |                                || AMD P-States and ACPI hardware P-States always can be supported in one processor. |
        |         |                                | But AMD P-States has the higher priority and if it is enabled with                 |
        |         |                                | `MSR_AMD_CPPC_ENABLE` or `cppc_set_enable`, it will respond to the      |
        |         |                                | request from AMD P-States.                                                         |
        +---------+--------------------------------+------------------------------------------------------------------------------------+
        | 3       | amd_pstate_ut_check_perf       || Check if the each performance values are reasonable.                              |
        |         |                                || highest_perf >= nominal_perf > lowest_nonlinear_perf > lowest_perf > 0.           |
        +---------+--------------------------------+------------------------------------------------------------------------------------+
        | 4       | amd_pstate_ut_check_freq       || Check if the each frequency values and max freq when set support boost mode       |
        |         |                                | are reasonable.                                                                    |
        |         |                                || max_freq >= nominal_freq > lowest_nonlinear_freq > min_freq > 0                   |
        |         |                                || If boost is not active but supported, this maximum frequency will be larger than  |
        |         |                                | the one in `cpuinfo`.                                                            |
        +---------+--------------------------------+------------------------------------------------------------------------------------+

    2). Tbench 测试

        在指定调控器下运tbench 基准测试时，测试并监cpu 的变化
        这些变化包括期望性能、频率、负载、性能、能耗等
        指定的调控器ondemand schedutil
        Tbench 也可以在 `acpi-cpufreq` 内核驱动上进行测试以作比较

    3). Gitsource 测试

        在指定调控器下运gitsource 基准测试时，测试并监cpu 的变化
        这些变化包括期望性能、频率、负载、时间、能耗等
        指定的调控器ondemand schedutil
        Gitsource 也可以在 `acpi-cpufreq` 内核驱动上进行测试以作比较

#. 如何执行测试

   我们使用 kselftest 框架中的测试模块来实现它
   我们创建 `amd-pstate-ut` 模块并将其绑定到 kselftest。（详情参见 Linux 内核
   自测[^4^]_）

    1). 构建

        - 打开 `CONFIG_X86_AMD_PSTATE` 配置选项
        - `CONFIG_X86_AMD_PSTATE_UT` 配置选项设置M
        - 构建工程
```

            $ cd linux
            $ make -C tools/testing/selftests

        + make perf ::

            $ cd tools/perf/
            $ make


    2). Installation & Steps ::

        $ make -C tools/testing/selftests install INSTALL_PATH=~/kselftest
        $ cp tools/perf/perf /usr/bin/perf
        $ sudo ./kselftest/run_kselftest.sh -c amd-pstate

    3). Specified test case ::

        $ cd ~/kselftest/amd-pstate
        $ sudo ./run.sh -t basic
        $ sudo ./run.sh -t tbench
        $ sudo ./run.sh -t tbench -m acpi-cpufreq
        $ sudo ./run.sh -t gitsource
        $ sudo ./run.sh -t gitsource -m acpi-cpufreq
        $ ./run.sh --help
        ./run.sh: illegal option -- -
        Usage: ./run.sh [OPTION...]
                [-h <help>]
                [-o <output-file-for-dump>]
                [-c <all: All testing,
                     basic: Basic testing,
                     tbench: Tbench testing,
                     gitsource: Gitsource testing.>]
                [-t <tbench time limit>]
                [-p <tbench process number>]
                [-l <loop times for tbench>]
                [-i <amd tracer interval>]
                [-m <comparative test: acpi-cpufreq>]


    4). Results

        + basic

         When you finish test, you will get the following log info ::

          $ dmesg | grep "amd_pstate_ut" | tee log.txt
          [12977.570663] amd_pstate_ut: 1    amd_pstate_ut_acpi_cpc_valid  success!
          [12977.570673] amd_pstate_ut: 2    amd_pstate_ut_check_enabled   success!
          [12977.571207] amd_pstate_ut: 3    amd_pstate_ut_check_perf      success!
          [12977.571212] amd_pstate_ut: 4    amd_pstate_ut_check_freq      success!

        + tbench

         When you finish test, you will get selftest.tbench.csv and png images.
         The selftest.tbench.csv file contains the raw data and the drop of the comparative test.
         The png images shows the performance, energy and performan per watt of each test.
         Open selftest.tbench.csv :

         +-------------------------------------------------+--------------+----------+---------+----------+-------------+---------+----------------------+
         + Governor                                        | Round        | Des-perf | Freq    | Load     | Performance | Energy  | Performance Per Watt |
         +-------------------------------------------------+--------------+----------+---------+----------+-------------+---------+----------------------+
         + Unit                                            |              |          | GHz     |          | MB/s        | J       | MB/J                 |
         +=================================================+==============+==========+=========+==========+=============+=========+======================+
         + amd-pstate-ondemand                             | 1            |          |         |          | 2504.05     | 1563.67 | 158.5378             |
         +-------------------------------------------------+--------------+----------+---------+----------+-------------+---------+----------------------+
         + amd-pstate-ondemand                             | 2            |          |         |          | 2243.64     | 1430.32 | 155.2941             |
         +-------------------------------------------------+--------------+----------+---------+----------+-------------+---------+----------------------+
         + amd-pstate-ondemand                             | 3            |          |         |          | 2183.88     | 1401.32 | 154.2860             |
         +-------------------------------------------------+--------------+----------+---------+----------+-------------+---------+----------------------+
         + amd-pstate-ondemand                             | Average      |          |         |          | 2310.52     | 1465.1  | 156.1268             |
         +-------------------------------------------------+--------------+----------+---------+----------+-------------+---------+----------------------+
         + amd-pstate-schedutil                            | 1            | 165.329  | 1.62257 | 99.798   | 2136.54     | 1395.26 | 151.5971             |
         +-------------------------------------------------+--------------+----------+---------+----------+-------------+---------+----------------------+
         + amd-pstate-schedutil                            | 2            | 166      | 1.49761 | 99.9993  | 2100.56     | 1380.5  | 150.6377             |
         +-------------------------------------------------+--------------+----------+---------+----------+-------------+---------+----------------------+
         + amd-pstate-schedutil                            | 3            | 166      | 1.47806 | 99.9993  | 2084.12     | 1375.76 | 149.9737             |
         +-------------------------------------------------+--------------+----------+---------+----------+-------------+---------+----------------------+
         + amd-pstate-schedutil                            | Average      | 165.776  | 1.53275 | 99.9322  | 2107.07     | 1383.84 | 150.7399             |
         +-------------------------------------------------+--------------+----------+---------+----------+-------------+---------+----------------------+
         + acpi-cpufreq-ondemand                           | 1            |          |         |          | 2529.9      | 1564.4  | 160.0997             |
         +-------------------------------------------------+--------------+----------+---------+----------+-------------+---------+----------------------+
         + acpi-cpufreq-ondemand                           | 2            |          |         |          | 2249.76     | 1432.97 | 155.4297             |
         +-------------------------------------------------+--------------+----------+---------+----------+-------------+---------+----------------------+
         + acpi-cpufreq-ondemand                           | 3            |          |         |          | 2181.46     | 1406.88 | 153.5060             |
         +-------------------------------------------------+--------------+----------+---------+----------+-------------+---------+----------------------+
         + acpi-cpufreq-ondemand                           | Average      |          |         |          | 2320.37     | 1468.08 | 156.4741             |
         +-------------------------------------------------+--------------+----------+---------+----------+-------------+---------+----------------------+
         + acpi-cpufreq-schedutil                          | 1            |          |         |          | 2137.64     | 1385.24 | 152.7723             |
         +-------------------------------------------------+--------------+----------+---------+----------+-------------+---------+----------------------+
         + acpi-cpufreq-schedutil                          | 2            |          |         |          | 2107.05     | 1372.23 | 152.0138             |
         +-------------------------------------------------+--------------+----------+---------+----------+-------------+---------+----------------------+
         + acpi-cpufreq-schedutil                          | 3            |          |         |          | 2085.86     | 1365.35 | 151.2433             |
         +-------------------------------------------------+--------------+----------+---------+----------+-------------+---------+----------------------+
         + acpi-cpufreq-schedutil                          | Average      |          |         |          | 2110.18     | 1374.27 | 152.0136             |
         +-------------------------------------------------+--------------+----------+---------+----------+-------------+---------+----------------------+
         + acpi-cpufreq-ondemand VS acpi-cpufreq-schedutil | Comprison(%) |          |         |          | -9.0584     | -6.3899 | -2.8506              |
         +-------------------------------------------------+--------------+----------+---------+----------+-------------+---------+----------------------+
         + amd-pstate-ondemand VS amd-pstate-schedutil     | Comprison(%) |          |         |          | 8.8053      | -5.5463 | -3.4503              |
         +-------------------------------------------------+--------------+----------+---------+----------+-------------+---------+----------------------+
         + acpi-cpufreq-ondemand VS amd-pstate-ondemand    | Comprison(%) |          |         |          | -0.4245     | -0.2029 | -0.2219              |
         +-------------------------------------------------+--------------+----------+---------+----------+-------------+---------+----------------------+
         + acpi-cpufreq-schedutil VS amd-pstate-schedutil  | Comprison(%) |          |         |          | -0.1473     | 0.6963  | -0.8378              |
         +-------------------------------------------------+--------------+----------+---------+----------+-------------+---------+----------------------+

        + gitsource

         When you finish test, you will get selftest.gitsource.csv and png images.
         The selftest.gitsource.csv file contains the raw data and the drop of the comparative test.
         The png images shows the performance, energy and performan per watt of each test.
         Open selftest.gitsource.csv :

         +-------------------------------------------------+--------------+----------+----------+----------+-------------+---------+----------------------+
         + Governor                                        | Round        | Des-perf | Freq     | Load     | Time        | Energy  | Performance Per Watt |
         +-------------------------------------------------+--------------+----------+----------+----------+-------------+---------+----------------------+
         + Unit                                            |              |          | GHz      |          | s           | J       | 1/J                  |
         +=================================================+==============+==========+==========+==========+=============+=========+======================+
         + amd-pstate-ondemand                             | 1            | 50.119   | 2.10509  | 23.3076  | 475.69      | 865.78  | 0.001155027          |
         +-------------------------------------------------+--------------+----------+----------+----------+-------------+---------+----------------------+
         + amd-pstate-ondemand                             | 2            | 94.8006  | 1.98771  | 56.6533  | 467.1       | 839.67  | 0.001190944          |
         +-------------------------------------------------+--------------+----------+----------+----------+-------------+---------+----------------------+
         + amd-pstate-ondemand                             | 3            | 76.6091  | 2.53251  | 43.7791  | 467.69      | 855.85  | 0.001168429          |
         +-------------------------------------------------+--------------+----------+----------+----------+-------------+---------+----------------------+
         + amd-pstate-ondemand                             | Average      | 73.8429  | 2.20844  | 41.2467  | 470.16      | 853.767 | 0.001171279          |
         +-------------------------------------------------+--------------+----------+----------+----------+-------------+---------+----------------------+
         + amd-pstate-schedutil                            | 1            | 165.919  | 1.62319  | 98.3868  | 464.17      | 866.8   | 0.001153668          |
         +-------------------------------------------------+--------------+----------+----------+----------+-------------+---------+----------------------+
         + amd-pstate-schedutil                            | 2            | 165.97   | 1.31309  | 99.5712  | 480.15      | 880.4   | 0.001135847          |
         +-------------------------------------------------+--------------+----------+----------+----------+-------------+---------+----------------------+
         + amd-pstate-schedutil                            | 3            | 165.973  | 1.28448  | 99.9252  | 481.79      | 867.02  | 0.001153375          |
         +-------------------------------------------------+--------------+----------+----------+----------+-------------+---------+----------------------+
         + amd-pstate-schedutil                            | Average      | 165.954  | 1.40692  | 99.2944  | 475.37      | 871.407 | 0.001147569          |
         +-------------------------------------------------+--------------+----------+----------+----------+-------------+---------+----------------------+
         + acpi-cpufreq-ondemand                           | 1            |          |          |          | 2379.62     | 742.96  | 0.001345967          |
         +-------------------------------------------------+--------------+----------+----------+----------+-------------+---------+----------------------+
         + acpi-cpufreq-ondemand                           | 2            |          |          |          | 441.74      | 817.49  | 0.001223256          |
         +-------------------------------------------------+--------------+----------+----------+----------+-------------+---------+----------------------+
         + acpi-cpufreq-ondemand                           | 3            |          |          |          | 455.48      | 820.01  | 0.001219497          |
         +-------------------------------------------------+--------------+----------+----------+----------+-------------+---------+----------------------+
         + acpi-cpufreq-ondemand                           | Average      |          |          |          | 425.613     | 793.487 | 0.001260260          |
         +-------------------------------------------------+--------------+----------+----------+----------+-------------+---------+----------------------+
         + acpi-cpufreq-schedutil                          | 1            |          |          |          | 459.69      | 838.54  | 0.001192548          |
         +-------------------------------------------------+--------------+----------+----------+----------+-------------+---------+----------------------+
         + acpi-cpufreq-schedutil                          | 2            |          |          |          | 466.55      | 830.89  | 0.001203528          |
         +-------------------------------------------------+--------------+----------+----------+----------+-------------+---------+----------------------+
         + acpi-cpufreq-schedutil                          | 3            |          |          |          | 470.38      | 837.32  | 0.001194286          |
         +-------------------------------------------------+--------------+----------+----------+----------+-------------+---------+----------------------+
         + acpi-cpufreq-schedutil                          | Average      |          |          |          | 465.54      | 835.583 | 0.001196769          |
         +-------------------------------------------------+--------------+----------+----------+----------+-------------+---------+----------------------+
         + acpi-cpufreq-ondemand VS acpi-cpufreq-schedutil | Comprison(%) |          |          |          | 9.3810      | 5.3051  | -5.0379              |
         +-------------------------------------------------+--------------+----------+----------+----------+-------------+---------+----------------------+
         + amd-pstate-ondemand VS amd-pstate-schedutil     | Comprison(%) | 124.7392 | -36.2934 | 140.7329 | 1.1081      | 2.0661  | -2.0242              |
         +-------------------------------------------------+--------------+----------+----------+----------+-------------+---------+----------------------+
         + acpi-cpufreq-ondemand VS amd-pstate-ondemand    | Comprison(%) |          |          |          | 10.4665     | 7.5968  | -7.0605              |
         +-------------------------------------------------+--------------+----------+----------+----------+-------------+---------+----------------------+
         + acpi-cpufreq-schedutil VS amd-pstate-schedutil  | Comprison(%) |          |          |          | 2.1115      | 4.2873  | -4.1110              |
         +-------------------------------------------------+--------------+----------+----------+----------+-------------+---------+----------------------+

```
## Reference


       https://docs.amd.com/v/u/en-US/24593_3.44_APM_Vol2

       https://uefi.org/sites/default/files/resources/ACPI_Spec_6_4_Jan22.pdf

       https://docs.amd.com/v/u/en-US/56569-A1-PUB_3.03

       https://www.kernel.org/doc/html/latest/dev-tools/kselftest.html
