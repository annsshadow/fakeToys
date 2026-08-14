## 遗留指令（Legacy instructions）


Linux 内核的 arm64 移植提供了支持仿真（emulation）架构中已废弃或已淘汰指令的基础设施。该基础设施代码使用未定义指令钩子来支持仿真。在可用的情况下，它还允许在硬件中开启指令的执行。

仿真模式可以通过写入 sysctl 节点（/proc/sys/abi）来控制。下面解释不同的执行行为以及对应的 sysctl 节点值——

- Undef
    值：0

  产生未定义指令中止（abort）。这是架构中已淘汰指令（如 SWP）的默认值。

- Emulate
    值：1

  使用软件仿真。为了辅助软件迁移，在此模式下会跟踪被仿真指令的使用情况，并发出速率受限的警告。这是已废弃指令（如 CP15 屏障）的默认值。

- Hardware Execution
    值：2

  尽管被标记为已废弃，某些实现可能支持开启/关闭用于执行这些指令的硬件支持。使用硬件执行通常能提供更好的性能，但会失去收集已废弃指令使用运行时统计信息的能力。

默认模式取决于指令在架构中的状态。已废弃指令应默认仿真，而已淘汰指令默认必须为未定义（undefined）。

注意：在所有情况下指令仿真可能都无法实现。更多信息请参阅各指令的说明。

### 受支持的遗留指令


- SWP{B}

:Node: /proc/sys/abi/swp
:Status: Obsolete
:Default: Undef (0)

- CP15 Barriers

:Node: /proc/sys/abi/cp15_barrier
:Status: Deprecated
:Default: Emulate (1)

- SETEND

:Node: /proc/sys/abi/setend
:Status: Deprecated
:Default: Emulate (1)*

  注意：要启用此特性，系统上所有 cpu 在 EL0 都必须支持混合字节序（mixed endian）。如果在启用此特性后热插拔（hotplug）进一个不支持混合字节序的新 CPU，应用程序中可能会出现意外结果。
