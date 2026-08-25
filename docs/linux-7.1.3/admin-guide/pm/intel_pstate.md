
## ``intel_pstate`` CPU 性能缩放驱动


:Copyright: |copy| 2017 Intel Corporation

:Author: Rafael J. Wysocki <rafael.j.wysocki@intel.com>


## 概述


`intel_pstate` Linux 内核[CPU 性能缩放子系<cpufreq>](CPU performance scaling subsystem <cpufreq>)
（`CPUFreq`）的一部分。它是一个面Sandy Bridge 及更世代 Intel 处理器的缩放驱动。不过请注意，其中部分处理器可能不受支持[要理`intel_pstate`，有必要先了`CPUFreq` 的一般工作方式，
因此如果你还没有读过 Documentation/admin-guide/pm/cpufreq.rst，现在正是时候。]

对于 `intel_pstate` 所支持的处理器而言，P-state 概念比单纯的运行频率运行性能点（operating performance point）要宽泛（关于这一点，可参Kristen Accardi LinuxCon Europe 2015 上的演讲 [^1^]_ 获取更多信息）因此，`intel_pstate` 内部使用P-state 表示方式遵循硬件规范
（详情参Intel 软件开发人员手[^2^]_）。然而，`CPUFreq` 核心使用频率
来标CPU 的运行性能点，并且在它向用户空间暴露的接口中也涉及频率所`intel_pstate` 也将其内部的 P-state 表示映射到频率上
（所幸这种映射是无歧义的）。与此同时，`intel_pstate` `CPUFreq`
核心提供一个可用频率表是不切实际的，因为该表可能非常大，因此驱动并这样做。核心的某些功能因此受到限制
由于 `intel_pstate` 所使用的硬P-state 选择接口在逻辑 CPU 级别可用该驱动始终以单个 CPU 为单位工作。因此，如果 `intel_pstate` 处于使用状态，
每个 `CPUFreq` 策略对象都对应一个逻辑 CPU，`CPUFreq` 策略实际上就
等同CPU。特别地，这意味着当相应的 CPU 下线时，它们会变成“非活动”状态，
并在CPU 重新上线时需要被重新初始化
`intel_pstate` 不是模块，因此无法卸载，这意味着向其传递早期配置参数的
唯一方式是通过内核命令行。不过，它的配置在很大程度上可以通过 `sysfs`
进行调整。在某些配置下，甚至可以通过 `sysfs` 将其注销，从而允许加载并注册
另一`CPUFreq` 缩放驱动（见下文 <status_attr>）

## 运行模式


`intel_pstate` 有两种不同的运行模式：主动（active）模式与被动（passive模式。在主动模式下，它使用自身内部性能缩放调控器算法，或者允许硬件自进行性能缩放；而在被动模式下，它响应由实现了特定性能缩放算法的通用
`CPUFreq` 调控器发出的请求。究竟哪种模式生效，取决于所使用的内核命令行
选项以及处理器的能力

### 主动模式


这是 `intel_pstate` 针对支持硬件托管 P-state（HWP）的处理器的默认运行模式如果它工作在此模式下，所`CPUFreq` 策略`sysfs` 中的 `scaling_driver`
策略属性都包含字符"intel_pstate"
在此模式下，驱动绕过`CPUFreq` 的缩放调控器层，并提供自身的缩放算法
用于 P-state 选择。这些算法可以像通用缩放调控器那样应用到 `CPUFreq`
策略上（即通过 `sysfs` 中的 `scaling_governor` 策略属性）[请注意，可以为不同策略选择不同P-state 选择算法，但这并不推荐。]

它们并非通用缩放调控器，但其名称与其中一些调控器的名称相同而且，容易让人混淆的是，它们的工作方式通常与同名通用调控器并不相同例如，`intel_pstate` 提供`powersave` P-state 选择算法并不是通用
`powersave` 调控器的对应物（大致上，它对应于 `schedutil` `ondemand`
调控器）
`intel_pstate` 在主动模式下提供两种 P-state 选择算法：`powersave` `performance`。它们两者的运作方式取决于处理器中是否启用了硬件托管 P-state
（HWP）特性，也可能取决于处理器型号
默认使用哪种 P-state 选择算法取决`CONFIG_CPU_FREQ_DEFAULT_GOV_PERFORMANCE`
内核配置选项。具体来说，如果该选项被设置，则默认使`performance` 算法如果未设置，则默认使用另一种算法

#### 启用 HWP 的主动模

如果处理器支HWP 特性，它会在处理器初始化期间被启用，且此后无法禁用可以通过向内核命令行传`intel_pstate=no_hwp` 参数来避免启用它
如果 HWP 特性已被启用，`intel_pstate` 依赖处理器自行选择 P-state，但可以向处理器的内P-state 选择逻辑给出提示。这些提示的内容取决于应用于
给定策略（或其所对应CPU）的 P-state 选择算法
尽管 P-state 选择由处理器自动完成，`intel_pstate` 在此模式下仍会向
CPU 调度器注册利用率更新回调。不过，这些回调并非用于运行 P-state 选择
算法，而是用于周期性地更新当前 CPU 频率信息，使其可以从 `sysfs` 中的
`scaling_cur_freq` 策略属性获取
HWP + `performance`
.....................

在此配置下，`intel_pstate` 会向处理器的能效偏好（Energy-Performance
Preference，EPP）旋钮（若支持）或其能效偏置（Energy-Performance BiasEPB）旋钮（否则）写0，这意味着处理器的内部 P-state 选择逻辑应完聚焦于性能
这将覆盖来自 `sysfs` 接口EPP/EPB 设置（见下文energy_performance_hints）此外，在此配置下，任何试图通过 `sysfs` EPP/EPB 改为0performance"的值都会被拒绝
另外，在此配置下，处理器内部 P-state 选择逻辑可用P-state 范围始终限制在上界（即驱动被允许使用的最P-state）
HWP + `powersave`
...................

在此配置下，`intel_pstate` 会将处理器的能效偏好（EPP）旋钮（若支持）或其
能效偏置（EPB）旋钮（否则）设置为之前通过 `sysfs` 设定的任何值（或平固件设定的任何默认值）。这通常会导致处理器的内P-state 选择逻辑较少
聚焦于性能
#### 未启HWP 的主动模

此运行模式对于不支持 HWP 特性的处理器，或在命令行中向内核传递了
`intel_pstate=no_hwp` 参数的情况是可选的。如果向内核命令行传递了
`intel_pstate=active` 参数，则在这些情况下使用主动模式。在此模式下`intel_pstate` 可能会拒绝与它所不识别的处理器协同工作。[请注意，
`intel_pstate` 绝不会拒绝与任何启用HWP 特性的处理器协同工作。]

在此模式下，`intel_pstate` CPU 调度器注册利用率更新回调，以运行一P-state 选择算法，即 `powersave` `performance`，具体取决于 `sysfs`
中的 `scaling_governor` 策略设置。那些利用率更新回调也会周期性地更新
将由 `sysfs` 中的 `scaling_cur_freq` 策略属性提供的当前 CPU 频率信息
`performance`
...............

在没HWP 的情况下，此 P-state 选择算法始终相同，与处理器型号和平台配置
无关
每当给定 CPU 的驱动配置被更新（例如通过 `sysfs`）时，它会选择其被允许
使用的、受 `sysfs` 所设限制约束的最P-state
如果设置`CONFIG_CPU_FREQ_DEFAULT_GOV_PERFORMANCE` 内核配置选项这就是默认的 P-state 选择算法
`powersave`
.............

在没HWP 的情况下，此 P-state 选择算法类似于通用 `schedutil` 缩放
调控器所实现的算法，只是它所使用的利用率度量基于来自 CPU 反馈寄存器的
数值。它通常选择与当CPU 利用率成比例P-state
该算法由驱动为给CPU 注册的利用率更新回调CPU 调度器调用它时运行，
但运行间隔不会短于每 10 毫秒一次。与 `performance` 的情况类似，如果新的
P-state 恰好与当P-state 相同，则不会触碰硬件配置
如果未设`CONFIG_CPU_FREQ_DEFAULT_GOV_PERFORMANCE` 内核配置选项这就是默认的 P-state 选择算法

### 被动模式


这是 `intel_pstate` 针对不支持硬件托P-state（HWP）的处理器的默认运行
模式。如果向内核命令行传递了 `intel_pstate=passive` 参数，则无论给定
处理器是否支HWP，都始终使用此模式。[请注意，`intel_pstate=no_hwp`
设置会在未与 `intel_pstate=active` 组合时使驱动以被动模式启动。] 与未
启用 HWP 支持的主动模式类似，在此模式下，如果通过内核命令行阻HWP 启用，`intel_pstate` 可能会拒绝与它所不识别的处理器协同工作
如果驱动工作在此模式下，所`CPUFreq` 策略`sysfs` 中的
`scaling_driver` 策略属性都包含字符"intel_cpufreq"。此时，驱动的行就像一个常规的 `CPUFreq` 缩放驱动。也就是说，它在必要时由通用缩放调控调用来与硬件通信，以改变某个 CPU P-state（特别是，`schedutil` 调控可以直接从调度器上下文调用它）
在此模式下，`intel_pstate` 可以`sysfs` `scaling_available_governors`
策略属性所列出的所有（通用）缩放调控器配合使用（且不使用上P-state 选择
算法）。此时，它负责配置与 CPU 相对应的策略对象，并`CPUFreq` 核心
（以及挂接到策略对象的缩放调控器）提供关于硬件所支持的最大和最小运行频（包括所谓的“turbo”频率范围）的准确信息。换言之，在被动模式下`intel_pstate` `CPUFreq` 核心暴露了整个可P-state 范围。不过，此模下驱动不会向 CPU 调度器注册利用率更新回调，`scaling_cur_freq` 信息来自
`CPUFreq` 核心（即当前缩放调控器为给定策略所选的最后一个频率）


## Turbo P-state 支持


在绝大多数情况下，`intel_pstate` 可用的整P-state 范围可以划分为两子范围，分别对应于在下面称为“turbo 阈值”（turbo threshold）的边界之上之下的不同类型处理器行为
turbo 阈值之上的 P-state 被称为“turbo P-state”，而它们所属的那个 P-state
子范围被称为“turbo 范围”（turbo range）。这些名称与 Turbo Boost 技术相关，
该技术允许多核处理器在功率充足且不会导致处理器封装的热封套（thermal
envelope）被超出的情况下，相机将一个或多个核的 P-state 提升到更高档
具体来说，如果软件将某个 CPU 核的 P-state 设置turbo 范围内（即高turbo 阈值），处理器就被允许接管该核的性能缩放控制，并自行将其置入它所
选择turbo P-state。不过，这一许可在不同处理器世代中的解释有所不同也就是说，Sandy Bridge 世代的处理器绝不会使用任何高于软件为给定核所最后一P-state 的档位，即使它位turbo 范围内；而所有更晚的处理器世则将其视为可以使turbo 范围内任P-state 的许可，即使高于软件所设的
档位。换言之，在这些处理器上，设置 turbo 范围内的任意 P-state 都会使处理器
能够自行将该核置入从最低到所支持的最大档之间的所turbo P-state
turbo P-state 的一个重要特性是它们不可持续。更精确地说，无法保证任CPU
能够无限期地停留在这些状态中的任何一个，因为处理器封装内的功率分配会
随时间改变，或者如果使turbo P-state 的时间过长，可能会超出其设计时所
针对的热封套
反过来，turbo 阈值之下的 P-state 通常是可持续的。事实上，如果软件设置了
其中之一，除非处于热应力或功率限制违例的情形，否则处理器不应将其改为
更低的档位（例如，如果同一封装中另一CPU 同时被设为更高的 P-state仍可能使用更高的 P-state）
某些处理器允许多个核同时处于 turbo P-state，但可以为它们设置的最P-state
通常取决于并发运行的核数量。可以为 3 个核同时设置的最turbo P-state
通常低于2 个核设置的相应最P-state，而后者通常又低于为 1 个核设置最turbo P-state。因此，单核最turbo P-state 就是整体所支持的最大值
所支持的最turbo P-state、turbo 阈值（所支持的最大非 turbo P-state）以所支持的最P-state 都特定于处理器型号，可以通过读取处理器的模型特定
寄存器（MSR）来确定。此外，某些处理器支持可配置 TDP（Thermal Design
Power，热设计功耗）特性，当该特性启用时，turbo 阈值实际上成为一个可平台固件设定的可配置值
ACPI 表中`_PSS` 对象不同，`intel_pstate` 始终将整个可P-state 范围
（包括整turbo 范围）暴露给 `CPUFreq` 核心，并（在被动模式下）暴露通用缩放调控器。这通常会导致在使用 `intel_pstate` 时，相对于基ACPI CPU 性能缩放，turbo P-state 被更频繁地设置（参见下文 <acpi-cpufreq> 了解
更多信息）
此外，由`intel_pstate` 始终知道真实turbo 阈值是什么（即便处理器中
启用了可配置 TDP 特性），其`sysfs` 中的 `no_turbo` 属性（下文
<no_turbo_attr> 描述）在所有情况下都应如预期般工作（即，如果设置为禁用
turbo P-state，它应始终阻`intel_pstate` 使用它们）

## 处理器支

为了处理给定的处理器，`intel_pstate` 需要了解关于它的若干不同信息，包括
 - 所支持的最P-state
 - 所支持的最大非 turbo P-state <turbo>
 - 是否支持 turbo P-state
 - 所支持的单核最turbo P-state <turbo>（如果支turbo P-state）
 - 将驱动内部的 P-state 表示转换为频率、以及反向转换的缩放公式
一般而言，获取这些信息的方式特定于处理器型号或系列。尽管通常可以从处理器
自身（使用模型特定寄存器）获取全部信息，但也有些情况下需要查阅硬件手才能获取
因此，`intel_pstate` 中有一份受支持处理器的列表，如果检测到的处理器不在
该列表中（除非它支持 HWP 特性），驱动初始化将会失败。[获取上述所有信息的
接口对所有支HWP 特性的处理器都是相同的，这正是 `intel_pstate` 能与
它们全部协同工作的原因。]


## 混合处理器的支持


`intel_pstate` 所支持的某些处理器包含两种或更多类型的 CPU 核，它们在最turbo P-state、性能与功耗特性、缓存大小以及可能的其他属性上有所不同。它通常被称为混合（hybrid）处理器。为了支持它们，`intel_pstate` 需要启HWP并且它假设系统中所CPU HWP 性能单位是相同的，因此给定的 HWP 性能等级
无论对应哪种核（CPU）类型，都大致代表相同的物理性能
### SMT 的混合处理器


在至少在一个核上启用了 SMT（Simultaneous Multithreading，同步多线程Intel 处理器语境中也被称为超线程（HyperThreading，HT））的系统上`intel_pstate` CPU 分配基于性能的优先级。具体来说，给定 CPU 的优先级
反映了其最高的 HWP 性能等级，这会使 CPU 调度器通常偏好性能更高CPU，从在其CPU 完全满载时才使用性能较低CPU。SMT 兄弟线程（即共享同一物理核的
逻辑 CPU）被赋予相同的优先级。调度器可以从低优先级核拉取任务并将其放置到
任意兄弟线程上。由于调度器在物理核之间分散任务，任务只会在所有物理核繁忙之后才被放置到物理核SMT 兄弟线程上
这种方法在绝大多数情况下能最大化性能，但不幸的是，它也会在一些重要场（如视频播放）下导致过度的能耗，这通常并不可取。由于启SMT 时没有其可行选择（因SMT 兄弟线程的有效容量和利用率难以确定），不SMT 的混处理器可以用更节能的方式来处理

### 容量感知调度支持


CPU 调度器中的容量感知调度（Capacity-Aware Scheduling，CAS）支持在默认情况`intel_pstate` 在不SMT 的混合处理器上启用。CAS 通常会使调度器将任务
放置在某CPU 上，只要CPU 上有足够的空闲容量；而如果给定任务的利用对其过高，该任务就需要去往别处
由于 CAS 考虑CPU 容量，它不需要对 CPU 划分优先级，并允许任务在性能较高
与较低的 CPU 之间更对称地分布。一旦被放置在具有足够容量容纳它CPU 上，
任务就可以继续在那里运行，而无论其CPU 是否完全满载，因此平均而言 CAS
降低了性能较高 CPU 的利用率，从而由于性能较高CPU 通常比性能较低CPU
能效更差，使能耗变得更加均衡
为了使用 CAS，调度器需要知道系统中每个 CPU 的容量，并且需要能够计CPU 缩放不变（scale-invariant）利用率，因`intel_pstate` 向它提供必要的信息
首先，每CPU 的容量由其最高的 HWP 性能等级乘以 1024、再除以系统中性能最CPU 的最HWP 性能等级之比来表示，这之所以可行是因为所CPU HWP
性能单位是相同的。其次，调度器为始终以相同单位表CPU 利用率而进行的
频率不变性（frequency-invariance）计算被调整为考虑 CPU 容量。所有这些都会在
`intel_pstate` 已向 `CPUFreq` 核心注册、并且它已确定自己运行在不带 SMT
的混合处理器上时发生
### 能量感知调度支持


如果在内核配置期间设置了 `CONFIG_ENERGY_MODEL`，且 `intel_pstate` 运行不带 SMT 的混合处理器上，则除了启CAS 之外，它还会为处理器注册一个能模型（Energy Model）。如`schedutil` 被用`CPUFreq` 调控器，这就允许CPU 调度器中启用能量感知调度（Energy-Aware Scheduling，EAS）支持，而这要求
`intel_pstate` 以被动模<passive_mode> 运行
`intel_pstate` 注册的能量模型是人为构造的（即，它基于抽象的开销值，且不包含
任何真实的功率数值），因此相对简单，可避免调度器中进行不必要的计算。其系统中的每个 CPU 都有一个性能域，这些性能域的开销值被选择为：在性能较低
（小）的 CPU 上运行任务看起来总是比在性能较高（大）的 CPU 上运行该任务更便宜然而，对于同类型的两个 CPU，开销差异取决于它们当前的利用率，当前利用率较高的
CPU 通常看起来是给定任务的更昂贵目标。这有助于在同类CPU 之间平衡负载
由于 EAS 工作CAS 之上，高利用率任务总是被迁移到具有足够容量容纳它们CPU；但得益EAS，低利用率任务倾向于被放置在调度器看来开销较低CPU 上实际上，这会导致只要性能较低且负载较轻的 CPU 有足够空闲容量运行给定任务，优先选用它们，这通常带来能耗的降低
`intel_pstate` 创建的能量模型可以通过查看 `debugfs` 中的 `energy_model`
目录来查看（通常挂载`/sys/kernel/debug/`）

## ``sysfs`` 中的用户空间接口



### 全局属

`intel_pstate` `sysfs` 中暴露了若干全局属性（文件），以在系统级别控制
其功能。它们位`/sys/devices/system/cpu/intel_pstate/` 目录中，并影响所CPU
如果向内核命令行传递了 `intel_pstate=per_cpu_perf_limits` 参数，其中部属性不会存在
`max_perf_pct`
	驱动被允许设置的最P-state，以最大支持性能等级（最高的
	支持 :ref:`turbo P-state <turbo>`）的百分比表示
	如果在内核命令行中存`intel_pstate=per_cpu_perf_limits`
	参数，则不会暴露此属性
`min_perf_pct`
	驱动被允许设置的最P-state，以最大支持性能等级（最高的
	支持 :ref:`turbo P-state <turbo>`）的百分比表示
	如果在内核命令行中存`intel_pstate=per_cpu_perf_limits`
	参数，则不会暴露此属性
`num_pstates`
	处理器所支持P-state 数量（介0 255 之间，含两端），
	包括 turbo 与非 turbo P-state（见 turbo）
	仅当系统所CPU 暴露的值都相同时，此属性才存在
	此属性的值不受下<no_turbo_attr> 描述`no_turbo` 设置影响
	此属性是只读的
`turbo_pct`
	turbo 范围 <turbo> 大小相对于所支持 P-state 总大小的比例，以
	百分比表示
	仅当系统所CPU 暴露的值都相同时，此属性才存在
	此属性是只读的

`no_turbo`
	如果设置（等1），驱动不允许设置任turbo P-state（见
	turbo）。如果未设置（等0，即默认值），驱动可以设turbo
	P-state。[请注意，`intel_pstate` 不支持通用`boost` 属	（部分其它缩放驱动支持），该属性由本属性替代。]

	此属性不影响提供`CPUFreq` 核心并通过策略接口暴露的最大支	频率值，但它影响每个策略P-state 限制的最大可能值（详见下文
	policy_attributes_interpretation）
`hwp_dynamic_boost`
	此属性仅`intel_pstate` 在处理器中以启用 HWP 特性的主动模式
	<active_mode_hwp> 工作时存在。如果设置（等于 1），它会导致每当
	之前等待 I/O 的任务被选中在某个给定逻辑 CPU 上运行时，最P-state
	限制被短时间动态提高（此机制的目的是改善性能）
	此设置对最P-state 限制被直接设为最高非 turbo P-state 或更高档	逻辑 CPU 没有影响

`status`
	驱动的运行模式："active"passive" "off"
	"active"
		驱动处于工作状态且处于 :ref:`主动模式 <active_mode>`
	"passive"
		驱动处于工作状态且处于 :ref:`被动模式 <passive_mode>`
	"off"
		驱动不工作（未作为缩放驱动向 `CPUFreq` 核心注册）
	可以写入此属性以改变驱动的运行模式或将其注销。写入的字符串必须是
	其可能取值之一，如果成功，写入将导致驱动切换到该字符串所代表	运行模式——或"off" 的情况下被注销。[实际上，从主动模式切换到
	被动模式或反之，会导致驱动被注销并以一组不同的回调重新注册，因此它
	的所有设置（全局的以及每个策略的）都会被重置为默认值，可能取决	目标运行模式。]

`energy_efficiency`
	此属性仅存在CPU 型号匹配 Kaby Lake Coffee Lake 桌面 CPU 型号	平台上。默认情况下，如果启用了 HWP，这CPU 型号上的能效优化是禁用的	启用能效优化可能会限制最大运行频率（无论是否启用 HWP 特性）。启HWP
	时，优化仅在 turbo 频率范围内进行；未启用时，优化在整个可用频率范围	进行。将此属性设"1" 启用能效优化，设"0" 则禁用它们

### 策略属性的解释


`intel_pstate` 作为当前缩放驱动时，`Documentation/admin-guide/pm/cpufreq.rst`
中描述的部分 `CPUFreq` 策略属性的解释是特殊的，并且通常取决于驱动的运行
模式 <operation_modes>
首先，`cpuinfo_max_freq`、`cpuinfo_min_freq` `scaling_cur_freq` 属性的是通过将处理器特定的乘子应用到 `intel_pstate` 所使用的内P-state 表示
而得到的。此外，`scaling_max_freq` `scaling_min_freq` 属性的值被限制对应于驱动被允许设置的最P-state 的频率
如果设置了全局属<no_turbo_attr> `no_turbo`，驱动不允许使用 turbo
P-state，因`scaling_max_freq` `scaling_min_freq` 的最大值被限制最大的turbo P-state 频率。相应地，设`no_turbo` 会使 `scaling_max_freq`
`scaling_min_freq` 下降到该值（如果它们之前高于该值）。不过，在取消设`no_turbo` 之后，除非这些属性在 `no_turbo` 被设置之后曾被写入过，否`scaling_max_freq` `scaling_min_freq` 的旧值会被恢复
如果未设`no_turbo`，`scaling_max_freq` `scaling_min_freq` 的最大可能对应于所支持的最turbo P-state，这也正是两种情况下 `cpuinfo_max_freq` 的值
接下来，如果 `intel_pstate` 工作于主动模<active_mode>，以下策略属具有特殊含义
`scaling_available_governors`
	`intel_pstate` 提供P-state 选择算法列表
`scaling_governor`
	`intel_pstate` 当前用于给定策略P-state 选择算法
`scaling_cur_freq`
	给定策略所代表CPU 的平P-state 频率，对应于 CPU 调度器对	CPU 最后一次两次调用驱动利用率更新回调之间的时间间隔
如果处理器中启用HWP 特性，还存在一个策略属性：

`base_frequency`
	显示 CPU 的基础频率。任何高于此频率的频率都将位turbo 频率范围内
这些属性在被动模式 <passive_mode> 下的含义与其它缩放驱动相同
此外，`intel_pstate` `scaling_driver` 属性的值取决于驱动的运行模式具体来说，它要么"intel_pstate"（主动模<active_mode> 下），要么是
"intel_cpufreq"（被动模<passive_mode> 下）

### P-State 限制的协

`intel_pstate` 允许通过两种方式设置 P-state 限制：借助 `max_perf_pct` `min_perf_pct` :ref:`全局属<global_attributes>`，或通过 `scaling_max_freq`
`scaling_min_freq` `CPUFreq` 策略属性。无论驱动当前的运行模式如何，这限制之间的协调基于以下规则：

 1. 所CPU 都受全局限制影响（即，没有任何一CPU 可以被要求运行得比全局
    最大值更快，也没有任何一个可以被要求运行得比全局最小值更慢）
 2. 每个单独CPU 受其自身的每策略限制影响（即，它不能被要求运行得比自    每策略最大值更快，也不能被要求运行得比自身每策略最小值更慢）。有效性能
    取决于平台是否支持每P-state、是否启用了超线程，以及来自其它 CPU     当前性能请求。当平台不支持每P-state 时，如果其它 CPU 此刻请求更高    性能，有效性能可能高于在某 CPU 上设置的策略限制。即使支持每P-state    当启用超线程时，如果兄弟 CPU 请求更高的性能，其它兄弟线程也会获得高    其策略限制的性能
 3. 全局限制与每策略限制可以独立设置
在启HWP 特性的主动模式 <active_mode_hwp> 下，每当限制发生变化，所的有效值就会被写入硬件寄存器，以请求其内部P-state 选择逻辑始终在这限制内设P-state。否则，这些限制会被缩放调控器（被动模式 <passive_mode>
下）以及驱动在每次为CPU 设置新的 P-state 之前纳入考量
此外，如果向内核传递了 `intel_pstate=per_cpu_perf_limits` 命令行参数，
`max_perf_pct` `min_perf_pct` 将完全不被暴露，设置限制的唯一方式是使策略属性

### 能效与性能提示


如果处理器中启用了硬件托P-state（HWP），则每`sysfs` 中的 `CPUFreq`
策略目录下都会出现额外的属性，旨在允许用户空间通过`intel_pstate` 聚焦性能或能效、或介于两者之间的某处，来调整处理器的内部 P-state 选择逻辑它们是：

`energy_performance_preference`
	给定策略（或其代表的 CPU）的能效与性能提示的当前值
	可以通过写入此属性来更改该提示
`energy_performance_available_preferences`
	可以写入 `energy_performance_preference` 属性的字符串列表
	它们代表不同的能效与性能提示，应当是不言自明的，例外`default`
	代表由平台固件设定的任何提示值
写入 `energy_performance_preference` 属性的字符串会在内部被转换为写入处理器
能效偏好（EPP）旋钮（若支持）或其能效偏置（EPB）旋钮的整数值。如EPP 特存在，也可以写入 0 255 之间的正整数值。如EPP 特性不存在，则不支持向
此属性写入整数值。在这种情况下，用户可以使用
"/sys/devices/system/cpu/cpu*/power/energy_perf_bias" 接口
[请注意，任务可能会被调度器的负载均衡算法从一CPU 迁移到另一CPU，如为这CPU 设置了不同的能效与性能提示，可能会导致不良后果。为避免此类问题最好为所CPU 设置相同的能效与性能提示，或者将每个可能对它们敏感的任务
固定到特定的 CPU。]


## ``intel_pstate`` ``acpi-cpufreq`` 的对

`intel_pstate` 支持的大多数系统上，由平台固件提供的 ACPI 表包`_PSS` 对象，返回可用于 CPU 性能缩放的信息（关于 `_PSS` 对象及其返回信息
的格式，请参ACPI 规范 [^3^]_）
`acpi-cpufreq` 缩放驱动使用 ACPI `_PSS` 对象返回的信息。在 `intel_pstate`
支持的系统中，`acpi-cpufreq` 驱动使用相同的硬CPU 性能缩放接口，但它能
使用P-state 集合`_PSS` 输出限制
在这些系统上，每`_PSS` 对象返回一个由相应 CPU 支持P-state 列表，这
基本上是 `intel_pstate` 在同一系统上可以使用的 P-state 范围的子集，只有
一个例外：整个 turbo 范围 <turbo> 在其中由一个条目（最顶端的那个）表示按照惯例，`_PSS` 为该条目返回的频率比它列出的最高非 turbo P-state 的频1 MHz，但为其返回的相P-state 表示（遵循硬件规范）匹配所支持的最turbo P-state（或者是特殊255，基本上意味着“能跑多高就跑多高”）
`_PSS` 返回P-state 列表`acpi-cpufreq` 提供`CPUFreq` 核心和缩调控器的可用频率表所反映，它报告的最小和最大支持频率也来自该列表。特别是鉴于上述 turbo 范围的特殊表示，这意味着 `acpi-cpufreq` 报告的最大支持频`_PSS` 列出的最高支持非 turbo P-state 的频率高 1 MHz，这当然会影响缩调控器做出的决策，但 `powersave` `performance` 除外
例如，如果某个调控器试图选择正比于估CPU 负载的频率，并将 100% 负载映射最大支持频率（可能乘以某个常数），那么当使`acpi-cpufreq` 作为缩放驱动时，
它将倾向于选择低于 turbo 阈值的 P-state，因为在这种情况turbo 范围仅对应于
它所能使用的频率频段的一小部分（1 MHz 对比 1 GHz 或更高）。结果，它只会在
最高负载时进入 turbo 范围，而其它可能受益于 turbo 频率50% 以上负载将被
赋予turbo P-state
与此相关的另一个问题可能出现在支持允许平台固件设置 turbo 阈值的可配TDP
特<turbo> 的系统上。具体来说，如果这没有与 `_PSS` 返回P-state 列表
正确协调，那些列表中就可能出现多个对应于 turbo P-state 的条目，并且可能难以
避开 turbo 范围（如果这是可取或必要的）。通常，为了整体避免使turbo
P-state，`acpi-cpufreq` 简单地避免使用 `_PSS` 列出的最顶端状态，但当返回列表中存在其turbo P-state 时，这并不足够
除上述之外，`acpi-cpufreq` 的工作方式类似于被动模式 <passive_mode> 下的
`intel_pstate`，只是它能设置的 P-state 数量被限制为 ACPI `_PSS` 对象所列出那些

## ``intel_pstate`` 的内核命令行选项


可以使用若干内核命令行选项来向 `intel_pstate` 传递早期配置参数，以强制其
表现出特定行为。它们都必须`intel_pstate=` 前缀开头
`disable`
	即使处理器受 `intel_pstate` 支持，也不要将其注册为缩放驱动
`active`
	以主动模<active_mode> 注册 `intel_pstate` 作为起始模式
`passive`
	以被动模<passive_mode> 注册 `intel_pstate` 作为起始模式
`force`
	`intel_pstate` 注册为缩放驱动，以取`acpi-cpufreq`，即使后者在
	给定系统上更受青睐
	这可能会阻止某些依赖 ACPI P-state 信息可用性的平台特性（如热控制	功率封顶）按预期工作，因此应谨慎使用
	此选项对不`intel_pstate` 支持的处理器、以及使`pcc-cpufreq`
	缩放驱动代替 `acpi-cpufreq` 的平台不起作用
`no_hwp`
	即使处理器支持硬件托P-state（HWP）特性，也不要启用它
`hwp_only`
	仅当处理器支持硬件托P-state（HWP）特性时，才`intel_pstate`
	注册为缩放驱动
`support_acpi_ppc`
	ACPI `_PPC` 性能限制纳入考量
	如果 FADT（Fixed ACPI Description Table，固ACPI 描述表）中的
	首选电源管理配置文件被设为 "Enterprise Server" "Performance
	Server"，则默认就纳ACPI `_PPC` 限制，此选项没有效果
`per_cpu_perf_limits`
	使用每逻辑 CPU P-state 限制（详pstate_limits_coordination）
`no_cas`
	不要启用容量感知调度 <CAS>，该特性在不带 SMT 的混合系统上默认启用
## 诊断与调

### 跟踪事件


有两个可用于 `intel_pstate` 诊断的静态跟踪事件。其中一个是 `CPUFreq`
一般使用的 `cpu_frequency` 跟踪事件，另一个则`intel_pstate` 特有`pstate_sample` 跟踪事件。仅`intel_pstate` 工作于主动模<active_mode>
时，这两个事件才会被它触发
可以使用以下 shell 命令序列来启用它们并查看输出
```

 # cd /sys/kernel/tracing/
 # echo 1 > events/power/pstate_sample/enable
 # echo 1 > events/power/cpu_frequency/enable
 # cat trace
 gnome-terminal--4510  [001] ..s.  1177.680733: pstate_sample: core_busy=107 scaled=94 from=26 to=26 mperf=1143818 aperf=1230607 tsc=29838618 freq=2474476
 cat-5235  [002] ..s.  1177.681723: cpu_frequency: state=2900000 cpu_id=2

```
如果 `intel_pstate` 工作于被动模<passive_mode>，则 `cpu_frequency`
跟踪事件将由 `schedutil` 缩放调控器（针对其挂接的策略）或 `CPUFreq` 核心
（针对使用其它缩放调控器的策略）触发
### ``ftrace``


`ftrace` 接口可用`intel_pstate` 的底层诊断。例如，要检查设P-state
的函数被调用的频率，可以`ftrace` 过滤器设```

 # cd /sys/kernel/tracing/
 # cat available_filter_functions | grep -i pstate
 intel_pstate_set_pstate
 intel_pstate_cpu_init
 ...
 # echo intel_pstate_set_pstate > set_ftrace_filter
 # echo function > current_tracer
 # cat trace | head -15
 # tracer: function
 #
 # entries-in-buffer/entries-written: 80/80   #P:4
 #
 #                              _-----=> irqs-off
 #                             / _----=> need-resched
 #                            | / _---=> hardirq/softirq
 #                            || / _--=> preempt-depth
 #                            ||| /     delay
 #           TASK-PID   CPU#  ||||    TIMESTAMP  FUNCTION
 #              | |       |   ||||       |         |
             Xorg-3129  [000] ..s.  2537.644844: intel_pstate_set_pstate <-intel_pstate_timer_func
  gnome-terminal--4510  [002] ..s.  2537.649844: intel_pstate_set_pstate <-intel_pstate_timer_func
      gnome-shell-3409  [001] ..s.  2537.650850: intel_pstate_set_pstate <-intel_pstate_timer_func
           <idle>-0     [000] ..s.  2537.654843: intel_pstate_set_pstate <-intel_pstate_timer_func


```
## 参考资

       https://events.static.linuxfound.org/sites/events/files/slides/LinuxConEurope_2015.pdf

       https://www.intel.com/content/www/us/en/architecture-and-technology/64-ia-32-architectures-software-developer-system-programming-manual-325384.html

       https://uefi.org/sites/default/files/resources/ACPI_6_3_final_Jan30.pdf
