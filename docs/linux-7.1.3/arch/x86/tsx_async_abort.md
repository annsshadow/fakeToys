
## TSX 异步中止（TAA）缓解

本文档说明针对 Intel 处理器的 TSX 异步中止（TAA）侧信道漏洞的缓解措施，介绍其原理、内核可用的缓解模式（如禁用 TSX、清除 CPU 缓冲区）以及对应的启动参数与配置方法。


### 概述


TSX 异步中止（TSX Async Abort, TAA）是针对某些 Intel 处理器内部缓冲区的侧信道攻击，
类似于微架构数据采样（Microarchitectural Data Sampling, MDS）。在这种情况下，当在
事务同步扩展（Transactional Synchronization Extensions, TSX）事务中存在挂起的异步
中止条件时，某些加载可能会投机地将无效数据传递给依赖操作。这包括没有 fault 或 assist
条件的加载。此类加载可能像 MDS 一样投机地暴露来自相同 uarch 数据结构的陈旧数据，暴露
范围相同，即同线程和跨线程。此问题影响所有当前支持 TSX 的处理器。

### 缓解策略


a) 禁用 TSX —— 缓解措施之一是禁用 TSX。一个新的 MSR IA32_TSX_CTRL 将在未来的以及
当前的处理器通过微码更新后可用，可用于禁用 TSX。此外，它控制 CPUID 中 TSX 特性位
（RTM 和 HLE）的枚举。

b) 清除 CPU 缓冲区 —— 与 MDS 类似，清除 CPU 缓冲区可缓解此漏洞。有关此方法的更多
详细信息，请参阅 Documentation/admin-guide/hw-vuln/mds.rst <mds>。

### 内核内部缓解模式


 =============    ============================================================
 off              缓解已禁用。要么 CPU 不受影响，要么在内核命令行上提供了
                  tsx_async_abort=off。

 tsx disabled     缓解已启用。在支持 TSX 控制的处理器上，TSX 特性在启动时默认禁用。

 verw             缓解已启用。CPU 受影响，且 MD_CLEAR 在 CPUID 中通告。

 ucode needed     缓解已启用。CPU 受影响，但 MD_CLEAR 未在 CPUID 中通告。这主要用于
                  虚拟化场景，其中宿主机有更新的微码，但 hypervisor 未在 CPUID 中暴露
                  MD_CLEAR。这是一种尽力而为的方法，不提供保证。
 =============    ============================================================

如果 CPU 受影响且未提供 "tsx_async_abort" 内核命令行参数，则内核会根据 RTM 和
MD_CLEAR 的 CPUID 位状态选择适当的缓解措施。

下表指示了 tsx=on|off|auto 命令行选项对各种 MSR_IA32_ARCH_CAPABILITIES 位组合下的
TAA 缓解状态、VERW 行为和 TSX 特性的影响。

1. "tsx=off"

=========  =========  ============  ============  ==============  ===================  ======================
MSR_IA32_ARCH_CAPABILITIES bits     Result with cmdline tsx=off
----------------------------------  -------------------------------------------------------------------------
TAA_NO     MDS_NO     TSX_CTRL_MSR  TSX state     VERW can clear  TAA mitigation       TAA mitigation
                                    after bootup  CPU buffers     tsx_async_abort=off  tsx_async_abort=full
=========  =========  ============  ============  ==============  ===================  ======================
    0          0           0         HW default         Yes           Same as MDS           Same as MDS
    0          0           1        Invalid case   Invalid case       Invalid case          Invalid case
    0          1           0         HW default         No         Need ucode update     Need ucode update
    0          1           1          Disabled          Yes           TSX disabled          TSX disabled
    1          X           1          Disabled           X             None needed           None needed
=========  =========  ============  ============  ==============  ===================  ======================

2. "tsx=on"

=========  =========  ============  ============  ==============  ===================  ======================
MSR_IA32_ARCH_CAPABILITIES bits     Result with cmdline tsx=on
----------------------------------  -------------------------------------------------------------------------
TAA_NO     MDS_NO     TSX_CTRL_MSR  TSX state     VERW can clear  TAA mitigation       TAA mitigation
                                    after bootup  CPU buffers     tsx_async_abort=off  tsx_async_abort=full
=========  =========  ============  ============  ==============  ===================  ======================
    0          0           0         HW default        Yes            Same as MDS          Same as MDS
    0          0           1        Invalid case   Invalid case       Invalid case         Invalid case
    0          1           0         HW default        No          Need ucode update     Need ucode update
    0          1           1          Enabled          Yes               None              Same as MDS
    1          X           1          Enabled          X              None needed          None needed
=========  =========  ============  ============  ==============  ===================  ======================

3. "tsx=auto"

=========  =========  ============  ============  ==============  ===================  ======================
MSR_IA32_ARCH_CAPABILITIES bits     Result with cmdline tsx=auto
----------------------------------  -------------------------------------------------------------------------
TAA_NO     MDS_NO     TSX_CTRL_MSR  TSX state     VERW can clear  TAA mitigation       TAA mitigation
                                    after bootup  CPU buffers     tsx_async_abort=off  tsx_async_abort=full
=========  =========  ============  ============  ==============  ===================  ======================
    0          0           0         HW default    Yes                Same as MDS           Same as MDS
    0          0           1        Invalid case  Invalid case        Invalid case          Invalid case
    0          1           0         HW default    No              Need ucode update     Need ucode update
    0          1           1          Disabled      Yes               TSX disabled          TSX disabled
    1          X           1          Enabled       X                 None needed           None needed
=========  =========  ============  ============  ==============  ===================  ======================

在表中，TSX_CTRL_MSR 是 MSR_IA32_ARCH_CAPABILITIES 中的一个新位，指示是否支持
MSR_IA32_TSX_CTRL。

IA32_TSX_CTRL MSR 中有两个控制位：

      Bit 0: 设置时禁用 TSX 的受限事务内存（Restricted Transactional Memory, RTM）
             子特性（将强制所有事务在 XBEGIN 指令上中止）。

      Bit 1: 设置时禁用 RTM 和 HLE 特性的枚举（即它会使 CPUID(EAX=7).EBX{bit4} 和
             CPUID(EAX=7).EBX{bit11} 读为 0）。
