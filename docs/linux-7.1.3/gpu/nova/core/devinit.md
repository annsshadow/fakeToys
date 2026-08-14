
## 设备初始化（devinit）

devinit 过程十分复杂且可能会发生变化。本文档以 Ampere GPU 系列为例，提供一个高层次的概览。其目标是提供该过程的概念性概述，以帮助理解相应的内核代码。

设备初始化（devinit）是在 GPU 复位后发生的一组关键的寄存器读/写操作序列。devinit 序列对于在 GPU 硬件可用之前对其进行正确配置至关重要。

devinit 引擎是一个解释器程序，通常运行在 GPU 的 PMU（电源管理单元，Power Management Unit）微控制器上。该解释器执行一段初始化的“脚本”命令。devinit 引擎本身是 VBIOS ROM 的一部分，与 FWSEC（固件安全，Firmware Security）镜像位于同一 ROM 映像中（参见 fwsec.rst 和 vbios.rst），并且它在 nova-core 驱动加载之前就已运行。在 Ampere GPU 上，devinit 微码与 FWSEC 微码是分离的。它由 FWSEC 启动，后者以“重度安全（heavy-secure）”模式运行在 GSP 上，而 devinit 则以“轻度安全（light-secure）”模式运行在 PMU 上。

### devinit 的关键功能

devinit 执行几项关键任务：

1. 配置 VRAM 显存控制器时序
2. 电源时序控制
3. 时钟与 PLL（锁相环，Phase-Locked Loop）配置
4. 热管理

### 低层固件初始化流程

复位后，GPU 上的若干微控制器（如 PMU、SEC2、GSP 等）会运行 GPU 固件（gfw）代码，以设置 GPU 及其核心参数。在初始化过程完成之前，GPU 的大部分被认为不可用。

这些低层 GPU 固件组件通常：

1. 位于 VBIOS ROM 的同一 ROM 分区中（参见 vbios.rst 和 fwsec.rst）。
2. 在不同的微控制器上依次执行：

  - devinit 引擎通常（但不一定）运行在 PMU 上。
  - 在 Ampere GPU 上，FWSEC 通常运行在 GSP（GPU 系统处理器，GPU System Processor）上，处于重度安全模式。

在驱动可以继续后续初始化之前，它必须等待一个表示核心初始化已完成的信号（称为 GFW_BOOT）。该信号由运行在 GSP 上、处于重度安全模式的 FWSEC 置位。

### 运行时考量

需要注意的是，devinit 序列不仅在初次启动时、在运行时的挂起/恢复操作中也需要运行，因为它对电源管理至关重要。

### 安全与访问控制

初始化过程涉及谨慎的特权管理。例如，在访问某些完成状态寄存器之前，驱动必须检查特权级掩码。某些寄存器只有在安全固件（FWSEC）降低特权级以允许 CPU（LS/低安全，low-secure）访问后才可访问。例如，在接收 GFW_BOOT 信号时就是这种情况。
