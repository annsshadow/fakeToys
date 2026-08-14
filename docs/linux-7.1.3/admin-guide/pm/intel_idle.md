
## ``intel_idle`` CPU 空闲时间管理驱动

:Copyright: |copy| 2020 Intel Corporation

:Author: Rafael J. Wysocki <rafael.j.wysocki@intel.com>


## 概述

`intel_idle` 是 Linux 内核中
[CPU 空闲时间管理子系统 <cpuidle>](CPU idle time management subsystem <cpuidle>)
（`CPUIdle`）的一部分。它是 Nehalem 及后续各代 Intel 处理器的默认 CPU 空闲时间管理驱动，
但对某个具体处理器型号的的支持程度，取决于它是否识别该处理器型号，也可能取决于来自平台固件的信息。
[要理解 `intel_idle`，有必要先了解 `CPUIdle` 的一般工作原理，因此如果你尚未熟悉，现在正是阅读
Documentation/admin-guide/pm/cpuidle.rst 的好时机。]

`intel_idle` 使用 `MWAIT` 指令来通知处理器：执行该指令的逻辑 CPU 处于空闲状态，
因此可以将处理器的一些功能模块置于低功耗状态。该指令接受两个参数（通过目标 CPU 的 `EAX` 和 `ECX`
寄存器传入），其中第一个参数称为**提示（hint）**，处理器可利用它来决定可以执行什么操作
（详见 Intel 软件开发人员手册 [^1^]_）。相应地，`intel_idle` 拒绝在以下处理器上工作：
其 `MWAIT` 指令支持已被禁用（例如经由平台固件配置菜单）或不支持该指令的处理器。

`intel_idle` 不是可加载模块，因此无法卸载，这意味着向它传递早期配置参数的唯一方式是通过内核命令行。


## sysfs 接口

`intel_idle` 驱动在 `/sys/devices/system/cpu/cpuidle/` 下导出以下 `sysfs` 属性：

`intel_c1_demotion`
	为系统中所有 CPU 启用或禁用 C1 降级。该文件仅在支持 C1 降级特性且经过测试的平台上导出。
	取值 0 表示禁用 C1 降级，取值 1 表示启用。写入 0 或 1 可禁用或启用所有 CPU 的 C1 降级。

	C1 降级特性是指平台固件将来自 OS 的深层 C-state 请求（例如 C6 请求）降级为 C1。
	其思路是固件监控 CPU 唤醒速率，若高于平台特定的阈值，便将深层 C-state 请求降级为 C1。
	例如，Linux 请求 C6，但固件发现每秒唤醒次数过多，便将 CPU 保持在 C1。
	当 CPU 在 C1 停留足够长时间后，平台又将其提升回 C6。这可能改善某些工作负载的性能，
	但也可能增加功耗。


## 空闲状态的枚举

每个 `MWAIT` 提示值都会被处理器解释为一项许可：以某种特定方式重新配置自身以节省能耗。
由此产生的（功耗降低的）处理器配置称为 C-states（ACPI 术语中）或空闲状态。
有意义的 `MWAIT` 提示值及与之对应的空闲状态（即处理器的低功耗配置）列表，
取决于处理器型号，也可能取决于平台的配置。

为了创建 `CPUIdle` 子系统所需的可用空闲状态列表（参见
Documentation/admin-guide/pm/cpuidle.rst 中的 idle-states-representation），
`intel_idle` 可以使用两类信息来源：驱动本身包含的针对不同处理器型号的空闲状态静态表，
以及系统的 ACPI 表。如果 `intel_idle` 识别了当前处理器型号，则始终使用前者；
而对于后者，在当前处理器型号需要时（所有被 `intel_idle` 识别的服务器处理器型号均属此情况）
或处理器型号未被识别时使用。[存在一个模块参数可让驱动对任意被其识别的处理器型号都使用 ACPI 表；
参见 `下文 <intel-idle-parameters_>`_。]

如果打算使用 ACPI 表来构建可用空闲状态列表，`intel_idle` 会先在系统 CPU 对应的某个 ACPI 对象下
查找 `_CST` 对象（关于 `_CST` 及其返回包的描述，请参阅 ACPI 规范 [^2^]_）。
由于 `CPUIdle` 子系统期望驱动提供的空闲状态列表适用于它所处理的所有 CPU，
而 `intel_idle` 又是作为系统中所有 CPU 的 `CPUIdle` 驱动注册的，
因此驱动会查找第一个返回至少一个有效空闲状态描述、且其返回包中的所有空闲状态均为 FFH
（Functional Fixed Hardware，功能固定硬件）类型的 `_CST` 对象，这意味着应使用 `MWAIT`
指令来告知处理器可以进入其中的某个状态。随后，该 `_CST` 的返回包被假定适用于系统中所有其它 CPU，
从中提取的空闲状态描述被存入一个来自 ACPI 表的初步空闲状态列表。
[如果 `intel_idle` 被配置为忽略 ACPI 表，则跳过此步骤；参见 `下文 <intel-idle-parameters_>`_。]

接下来，可用空闲状态列表中的第一个（索引 0）条目被初始化为代表“轮询空闲状态”
（一种伪空闲状态，目标 CPU 持续取指并执行指令），随后的（真实）空闲状态条目按如下方式填充。

如果 `intel_idle` 识别了当前处理器型号，驱动中存在一份针对它的（静态）空闲状态描述表。
此时，“内部”表是空闲状态信息的主要来源，其中的信息会被复制到最终的可用空闲状态列表中。
如果枚举空闲状态不需要使用 ACPI 表（取决于处理器型号），则列出的所有空闲状态默认启用
（因此 `CPUIdle` 的调速器在选择 CPU 空闲状态时都会考虑它们）。否则，如果来自 ACPI 表的
初步空闲状态列表中没有匹配条目，部分列出的空闲状态可能默认不启用。这种情况下，用户空间之后仍可借助
`sysfs` 中的 `disable` 空闲状态属性（按每个 CPU 分别）启用它们（参见
Documentation/admin-guide/pm/cpuidle.rst 中的 idle-states-representation）。
这基本意味着：若平台固件（通过 ACPI 表）未曾暴露，驱动“已知”的空闲状态可能默认不启用。

如果 `intel_idle` 未识别给定的处理器型号，但它支持 `MWAIT`，则使用来自 ACPI 表的
初步空闲状态列表来构建最终列表，该列表将在驱动注册时提供给 `CPUIdle` 核心。
对于该列表中的每个空闲状态，其描述、`MWAIT` 提示和退出延迟会被复制到最终空闲状态列表中对应的条目。
它所代表的空闲状态的名称（由 `sysfs` 中的 `name` 空闲状态属性返回）为“CX_ACPI”，
其中 X 为该空闲状态在最终列表中的索引（注意 X 的最小值为 1，因为 0 保留给“轮询”状态），
其目标驻留时间基于退出延迟值。具体而言，对于 C1 类型的空闲状态，退出延迟值同时被用作目标驻留时间
（以与 `intel_idle` 识别的各类处理器型号的大多数“内部”空闲状态表兼容），
而对于其它空闲状态类型（C2 和 C3），目标驻留时间为退出延迟的 3 倍
（同样是因为它反映了 `intel_idle` 识别的处理器型号在大多数情况下的目标驻留/退出延迟之比）。
在这种情况下，最终列表中的所有空闲状态默认启用。


## 初始化

`intel_idle` 的初始化首先检查内核命令行选项是否禁止使用 `MWAIT` 指令。若是，则立即返回错误码。

下一步是检查驱动是否知道该处理器型号，这决定了空闲状态枚举方法（参见
`上文 <intel-idle-enumeration-of-states_>`_），以及处理器是否支持 `MWAIT`
（若不支持则初始化失败）。然后，通过 `CPUID` 枚举处理器中的 `MWAIT` 支持，
若支持程度不符合预期（例如返回的 `MWAIT` 子状态总数为 0），则驱动初始化失败。

接下来，如果驱动未被配置为忽略 ACPI 表（参见 `下文 <intel-idle-parameters_>`_），
则从 ACPI 表中提取平台固件提供的空闲状态信息。

随后，为所有 CPU 分配 `CPUIdle` 设备对象，并按 `上文 <intel-idle-enumeration-of-states_>`_
所述创建可用空闲状态列表。

最后，`intel_idle` 借助 cpuidle_register_driver() 注册为系统中所有 CPU 的 `CPUIdle` 驱动，
并通过 cpuhp_setup_state() 注册一个用于配置各个 CPU 的 CPU 上线回调，
（除其它事项外）这会使得该回调例程对当时系统中存在的所有 CPU 被调用
（每个 CPU 执行自己的回调例程实例）。该例程为运行它的 CPU 注册一个 `CPUIdle` 设备
（使 `CPUIdle` 子系统能够操作该 CPU），并可选择性地执行给定的处理器型号可能需要的某些 CPU 特定初始化动作。


## 内核命令行选项与模块参数

**x86** 架构支持代码识别三个与 CPU 空闲时间管理相关的内核命令行选项：
`idle=poll`、`idle=halt` 和 `idle=nomwait`。如果内核命令行中出现其中任意一个，
则不允许使用 `MWAIT` 指令，因此 `intel_idle` 的初始化将会失败。

除此之外，`intel_idle` 自身识别五个可通过内核命令行设置的模块参数
（它们无法通过 sysfs 更新，因此这是改变其取值的唯一方式）。

`max_cstate` 参数值是驱动注册时提供给 `CPUIdle` 核心的空闲状态列表中的最大空闲状态索引。
它同时也是 `intel_idle` 能够使用的常规（非轮询）空闲状态的最大数量，因此找到该数量的可用空闲状态后，
空闲状态枚举即终止（如果 `max_cstate` 更大本可能被使用的其它空闲状态将完全不被考虑）。
设置 `max_cstate` 可以阻止 `intel_idle` 将某些因某种原因被视为“过深”的空闲状态暴露给 `CPUIdle` 核心，
但做法是在系统关闭并重新启动之前使它们实际上不可见，这未必总是可取的。实际上，只有在系统启动期间
无法启用相关空闲状态时才真正需要这样做，因为在系统运行状态下，可以使用 CPU 电源管理服务质量（PM QoS）
特性来阻止 `CPUIdle` 触及这些空闲状态，即使它们已被枚举（参见
Documentation/admin-guide/pm/cpuidle.rst 中的 cpu-pm-qos）。
将 `max_cstate` 设为 0 会导致 `intel_idle` 初始化失败。

`no_acpi`、`use_acpi` 和 `no_native` 模块参数在已配置 ACPI 支持的内核下被 `intel_idle` 识别。
若未配置 ACPI，这些标志对功能没有影响。

`no_acpi` - 完全不使用 ACPI。仅可用原生模式，无 ACPI 模式。

`use_acpi` - 在 ACPI 模式下为空操作，驱动将在原生模式下查阅 ACPI 表以获知 C-state 的开关状态。

`no_native` - 仅以 ACPI 模式工作，无原生模式可用（忽略所有自定义表）。

`states_off` 模块参数的取值（默认 0）以位掩码形式表示默认禁用的空闲状态列表。

具体而言，`states_off` 值中被置位的位的位置，即为默认禁用的空闲状态索引
（如 `sysfs` 中相应空闲状态目录的名称 `state0`、`state1` ... `state<i>` ... 所反映，
其中 `<i>` 是给定空闲状态的索引；参见
Documentation/admin-guide/pm/cpuidle.rst 中的 idle-states-representation）。

例如，若 `states_off` 等于 3，驱动将默认禁用空闲状态 0 和 1；若等于 8，则默认禁用空闲状态 3，
依此类推（超出最大空闲状态索引的位位置将被忽略）。

以这种方式禁用的空闲状态可从用户空间通过 `sysfs` 启用。

`ibrs_off` 模块参数是一个布尔标志（默认为 false）。若置位，它用于控制 CPU 进入空闲状态时
是否应关闭 IBRS（Indirect Branch Restricted Speculation，间接分支受限推测）。
该标志不影响使用 Enhanced IBRS 的 CPU，后者可保持开启且性能影响很小。

对于某些 CPU，IBRS 会默认被选为 Spectre v2 和 Retbleed 安全漏洞的缓解措施。
在空闲时保持 IBRS 模式开启可能会对其兄弟 CPU 造成性能影响。CPU 进入深层空闲状态时，IBRS 模式默认会关闭，
但在某些较浅的空闲状态中不会。设置 `ibrs_off` 模块参数将强制在任何可用空闲状态下都关闭 IBRS 模式。
这可能在以空闲 CPU 略微更高的唤醒延迟为代价的情况下，改善兄弟 CPU 的性能。

`table` 参数允许定制空闲状态的延迟和目标驻留时间。其语法为逗号分隔的
`name:latency:residency` 条目列表，其中 `name` 为空闲状态名称，`latency` 为退出延迟（微秒），
`residency` 为目标驻留时间（微秒）。无需指定所有空闲状态，只需指定要定制的那些。
例如，`C1:1:3,C6:50:100` 将 C1 和 C6 的退出延迟和目标驻留时间分别设为 1/3 和 50/100 微秒。
其余空闲状态保留其默认值。驱动会校验较深的空闲状态具有比更浅的空闲状态更高的延迟和目标驻留时间；
此外，目标驻留时间不能小于退出延迟。若不满足任一条件，驱动将忽略整个 `table` 参数。


## 核心级与封装级的空闲状态

通常，在支持 `MWAIT` 指令的处理器中，存在（至少）两级空闲状态（或 C-states）。
一级称为“核心 C-states”，覆盖处理器中的各个核心；另一级称为“封装 C-states”，覆盖整个处理器封装，
并可能还涉及系统的其它组件（GPU、内存控制器、I/O 集线器等）。

部分 `MWAIT` 提示值只允许处理器使用核心 C-states（最重要的是，对应于 `C1` 空闲状态的
`MWAIT` 提示值即属此情况），但大多数提示值则赋予它一项许可：将目标核心
（即包含执行带该提示值的 `MWAIT` 的逻辑 CPU 的核心）置入特定的核心 C-state，然后（若可能）在更深层进入特定的封装 C-state。
例如，代表 `C3` 空闲状态的 `MWAIT` 提示值允许处理器将目标核心置于称为“核心 `C3`”（或 `CC3`）的
低功耗状态，这发生在该核心中所有逻辑 CPU（SMT 兄弟线程）都执行了带 `C3` 提示值
（或带代表更深空闲状态的提示值）的 `MWAIT` 时；此外（在大多数情况下）它赋予处理器一项许可：
将整个封装（可能包括某些非 CPU 组件，如 GPU 或内存控制器）置于称为“封装 `C3`”（或 `PC3`）的
低功耗状态，这发生在所有核心都已进入 `CC3` 状态且（可能）满足某些附加条件时
（例如，若 GPU 被 `PC3` 覆盖，则可能要求它处于某个 GPU 特定的低功耗状态，`PC3` 方可达）。

一般而言，若满足进入相应封装 C-states 的条件，则没有简单的方法让处理器仅使用核心 C-states，
因此执行带非仅核心级（如 `C1`）提示值的 `MWAIT` 的逻辑 CPU 必须始终假定：这可能会导致处理器
进入一个封装 C-state。[正因如此，`intel_idle` 空闲状态“内部”表中大多数 `MWAIT` 提示值对应的
退出延迟和目标驻留值反映了封装 C-states 的属性。] 若完全不希望使用封装 C-states，
必须使用 PM QoS <cpu-pm-qos> 或 `上文 <intel-idle-parameters_>`_ 所述的 `intel_idle` 的
`max_cstate` 模块参数，将允许的空闲状态范围限制为仅具有核心级 `MWAIT` 提示值（如 `C1`）的那些状态。


## 参考文献

       https://www.intel.com/content/www/us/en/architecture-and-technology/64-ia-32-architectures-software-developer-vol-2b-manual.html

       https://uefi.org/specifications
