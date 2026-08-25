
## 时钟与定时器


### arm64

arm64 上，Hyper-V ARMv8 架构的系统计数器（system counter）和定时器（timer进行了虚拟化。客户机 VM 通过标准arm_arch_timer.c 驱动将这一虚拟化的硬件用作
Linux clocksource clockevents，就像在裸机上一样。在 Hyper-V 上的客户VM
中，针对架构系统计数器的 Linux vDSO 支持是可用的。虽Hyper-V 还提供了一个合系统时钟和四个合per-CPU 定时器（TLFS 中所述），但arm64 上的 Hyper-V 客户中，Linux 内核并未使用它们。不过，较旧版本arm64 Hyper-V 只部分虚拟化 ARMv8
架构定时器，导致该定时器不会VM 中生成中断。由于这一限制，在这些较旧Hyper-V 版本上运行当前的 Linux 内核版本，需要一个树外（out-of-tree）补丁，以改Hyper-V 合成时钟/定时器
### x86/x64

x86/x64 上，Hyper-V 向客户机 VM 提供TLFS 中所述的合成系统时钟和四个合per-CPU 定时器。Hyper-V 还通过 RDTSC 及相关指令提供对虚拟TSC 的访问。这TSC
指令不会陷入（trap）到 hypervisor，因此在 VM 中提供出色的性能。Hyper-V 执行 TSC
校准，并通过一个合MSR TSC 频率提供给客户机 VM。Linux 中的 Hyper-V 初始代码读取MSR 以获取频率，因此它会跳过 TSC 校准并设tsc_reliable。Hyper-V 提供
了虚拟化PIT（仅Hyper-V 第一VM）、local APIC timer RTC。Hyper-V 不在
客户VM 中提供虚拟化HPET
Hyper-V 合成系统时钟可以通过一个合MSR 读取，但这种访问会陷入到 hypervisor。作更快的替代方案，客户机可以配置一个在客户机与 hypervisor 之间共享的内存页。Hyper-V
在该内存页中填入一64 位的 scale 值和 offset 值。要读取合成时钟的值，客户机读TSC，然后按Hyper-V TLFS 中的描述应用 scale offset。得到的结果以恒定的 10 MHz
频率前进。在实时迁移到具有不TSC 频率的主机的情况下，Hyper-V 会调整共享页中的
scale offset 值，以维10 MHz 的频率
Windows Server 2022 Hyper-V 开始，Hyper-V 使用TSC 频率缩放的硬件支持，实现 VM TSC 频率可能不同Hyper-V 主机之间的实时迁移。当 Linux 客户机检测到Hyper-V 功能可用时，它倾向于使Linux 标准的基TSC clocksource。否则，它会
使用通过共享页实现的 Hyper-V 合成系统时钟clocksource（标识为
"hyperv_clocksource_tsc_page"）
Hyper-V 合成系统时钟可通过 vDSO 提供给用户空间，gettimeofday() 及相关的系统调用
可以完全在用户空间中执行。vDSO 通过将带scale offset 值的共享页映射到用户空间
来实现。用户空间代码执行相同的算法：读TSC 并应scale offset 来得到恒定的
10 MHz 时钟
Linux clockevents 基于 Hyper-V 合成定时0（stimer0）。虽Hyper-V 为每CPU
提供 4 个合成定时器，但 Linux 只使用定时器 0。在较旧版本Hyper-V 中，来自 stimer0
的中断会产生一VMBus 控制消息，由 vmbus_isr() 进行解复用，Documentation/virt/hyperv/vmbus.rst 文档中所述。在较新版本Hyper-V 中，stimer0
中断可以映射到一个架构中断，这被称为“Direct Mode”（直接模式）。Linux 在可用时倾向
于使Direct Mode。由x86/x64 不支per-CPU 中断，Direct Mode 会在所CPU 静态分配一x86 中断向量（HYPERV_STIMER0_VECTOR），并显式编码以调用 stimer0 中断
处理程序。因此，来自 stimer0 的中断记录在 /proc/interrupts 的“HVS”行中，而不与某Linux IRQ 关联。基于虚拟化 PIT local APIC timer clockevents 也能工作Hyper-V stimer0 是首选
Hyper-V 合成系统时钟和定时器的驱动位drivers/clocksource/hyperv_timer.c