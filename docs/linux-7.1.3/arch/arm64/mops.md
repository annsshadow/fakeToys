
## 内存复制/设置指令（MOPS）


一个 MOPS 内存复制/设置操作由三条连续的 CPY** 或 SET** 指令组成：一个前导（prologue）、主体（main）和尾声（epilogue）（例如：CPYP、CPYM、CPYE）。

主体或尾声指令可能出于多种原因而产生 MOPS 异常，例如当任务被迁移到具有不同 MOPS 实现的 CPU，或者当指令的对齐和大小要求未满足时。随后软件异常处理程序应重置寄存器并从 prologue 指令重新启动执行。通常这由内核处理。

更多细节请参阅 Arm 架构参考手册 DDI 0487K.a（Arm ARM）中的 “D1.3.5.7 Memory Copy and Memory Set exceptions”。


### 虚拟机监视器（Hypervisor）要求


运行 Linux 客户机的虚拟机监视器必须处理来自客户机内核的所有 MOPS 异常，因为 Linux 可能无法在所有时候处理该异常。例如，当虚拟机监视器将 vCPU 迁移到具有不同 MOPS 实现的另一个物理 CPU 时，可能会触发 MOPS 异常。

为此，虚拟机监视器必须：

  - 将 HCRX_EL2.MCE2 设置为 1，以便异常被陷入虚拟机监视器。

  - 拥有一个实现了 Arm ARM 规则 CNTMJ 和 MWFQH 算法的异常处理程序。

  - 在异常处理程序中将客户机的 PSTATE.SS 设置为 0，以处理当前指令可能的单步执行。

    注意：需要清除 PSTATE.SS，以便在下一条指令（prologue 指令）上产生单步异常。否则 prologue 会被静默地单步跳过，而单步异常会在主体指令上产生。注意，如果客户机指令没有被单步执行，那么清除 PSTATE.SS 没有效果。
