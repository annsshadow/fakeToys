
## Arm 机密计算架构（Confidential Compute Architecture

支持 Realm Management Extension（RME）的 Arm 系统包含硬件，允许以某种方式运行 VM 客户机，从而保客户机的代码与数据免hypervisor 的侵害。它将旧的“两个世界”模型（Normal Secure World）扩展为
四个世界：Normal、Secure、Root Realm。Linux 也可以作为客户机运行Realm 世界中运行的监视器（monitor之下
运行Realm 世界中的监视器被称为 Realm Management Monitor（RMM），实现Realm Management Monitor
规范[^1^]。该监视器有点像 hypervisor（例如它运行EL2，并管理运行Realm 世界中的客户机的 stage 2
页表等），但大部分控制权由运行在 Normal World 中的 hypervisor 掌握。Normal World hypervisor 使用
RMM 规范定义Realm Management Interface（RMI）来请求 RMM 执行操作（例如映射内存或执行一vCPU）
RMM 为客户机定义了一个环境，其中地址空间（IPA）被一分为二。下半部分是受保护的——映射到此半部分的任何内都无法被 Normal World 看到，且 RMM 限制 Normal World 对此内存可执行的操作（例如，未经客户机配合，Normal World
无法替换此区域内的页）。上半部分是共享的，Normal World 可以自由地更改此区域内的页，并能够在此区域中模拟
MMIO 设备
运行Realm 中的客户机也可以通过 Realm Services Interface（RSI）与 RMM 通信，以请求更改其环境或对其
环境进行证明（attestation）。特别地，它可以请求将受保护地址空间的某些区域在“RAM”与“EMPTY”之间转换（任一方向）这允Realm 客户机交出内存以归还Normal World，或Normal World 请求新内存。如果没Realm 客户机的显式请求RMM 会阻Normal World 进行这些更改
### Linux 作为 Realm 客户

要在 Realm 中将 Linux 作为客户机运行，以下内容必须VMM 或在 Linux 之前运行Realm 中的 `boot loader` 提供
 - 描述Linux 的所有受保护 RAM（通过 DT ACPI）在移交Linux 之前必须标记RIPAS RAM
 - MMIO 设备必须未被保护（例如由 Normal World 模拟）或标记RIPAS DEV
 - Normal World 模拟并在启动早期（特别是 earlycon）使用的 MMIO 设备必须指定IPA 的上半部分。对earlycon   这可以通过在命令行上指定地址来完成，例如 IPA 大小33 位、被模拟 UART 的基地址0x1000000   `earlycon=uart,mmio,0x101000000`

 - Linux 将使用反弹缓冲区与未受保护的设备通信。它会将一些受保护内存转换RIPAS EMPTY，并期望能够在相同的 IPA 地址   但最高有IPA 位置位的情况下访问未受保护的页。预期是 VMM 会从受保护映射中移除物理页，并将这些页作为未受保护的页提供
### 参

[^1^] https://developer.arm.com/documentation/den0137/
