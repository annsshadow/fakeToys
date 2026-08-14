
## CPU 热插拔与 ACPI


在 arm64 领域，CPU 热插拔通常用来描述内核使用 PSCI 将 CPU 上线/下线。本文档
讲述 ACPI 固件允许那些在引导时不可用的 CPU 在之后被加入系统。

`possible` 和 `present` 指的是 CPU 从 Linux 视角看到的两种状态。


### 物理系统上的 CPU 热插拔——引导时不存在的 CPU


物理系统需要把一个 `possible` 但非 `present` 的 CPU 标记为 `present`。
一个例子是双路（dual socket）机器，其中一个插槽上的处理器包可以在系统运行时
被更换。

这不被支持。

在 arm64 领域，CPU 不是一个单独的设备，而是系统的一个切片（slice）。没有任何
系统支持在系统运行时物理地添加（或移除）CPU，而且 ACPI 也不足以描述它们。

例如，新 CPU 会带来新的缓存，但平台的缓存拓扑是在一张静态表 PPTT 中描述的。
缓存如何在 CPU 之间共享是不可发现的，必须由固件来描述。

例如，每个 CPU 的 GIC 再分发器（redistributor）必须由驱动在引导时访问，以发现
系统级支持的特性。ACPI 的 MADT GICC 结构可以描述与一个禁用 CPU 关联的再分发器，
但不能描述该再分发器是否可访问，只能说明它是否“常开（always on）”。

arm64 的 ACPI 表假设所描述的都是 `present`。


### 虚拟系统上的 CPU 热插拔——引导时未启用的 CPU


虚拟系统的优势在于系统将拥有的所有属性都可以在引导时描述。因为这类设备是
模拟的，所以不存在电源域的考量。

虚拟系统上的 CPU 热插拔受支持。它与物理 CPU 热插拔不同，因为所有资源都被描述
为 `present`，但 CPU 可能被固件标记为禁用。只有 CPU 的上线/下线行为受固件影响。
例如，一台虚拟机以单个 CPU 引导，当某个云编排器部署工作负载后再添加额外的 CPU。

对于虚拟机，VMM（例如 Qemu）扮演固件的角色。

虚拟热插拔被实现为一种影响哪些 CPU 可以上线的固件策略。固件可以通过 PSCI 的
返回码来强制执行其策略，例如 `DENIED`。

ACPI 表必须描述虚拟机的所有资源。固件希望禁用（无论是从引导时还是之后）的 CPU
不应在 MADT GICC 结构中被 `enabled`，而应置上 `online capable` 位，以表明它们
之后可以被启用。引导 CPU 必须被标记为 `enabled`。“常开”的 GICR 结构必须用来
描述再分发器。

被描述为 `online capable` 但非 `enabled` 的 CPU，可以通过 DSDT 的 Processor
对象的 _STA 方法设置为 enabled。在虚拟系统上，_STA 方法必须总是把 CPU 报告为
`present`。固件策略的改变可以通过 device-check 或 eject-request 通知操作系统。

在静态表中被描述为 `enabled` 的 CPU，不应由固件动态修改其 _STA。像 kexec 这样
的软重启特性会从这些静态表重新读取系统的静态属性，如果它们不再描述运行中的
系统就可能发生故障。Linux 会在引导后期通过 _STA 方法重新发现系统的动态属性。
