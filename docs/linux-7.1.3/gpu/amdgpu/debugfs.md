## AMDGPU DebugFS


amdgpu 驱动提供若干 debugfs 文件，用于辅助调试驱动中的问题。这些文件通常位于
/sys/kernel/debug/dri/<num>。

## DebugFS 文件


### amdgpu_benchmark


使用驱动用于 GPU 显存分页的 DMA 引擎运行基准测试。向该文件写入一个数字以运行
测试。结果写入内核日志。VRAM 位于设备内存（dGPU）或预留内存（APU）中，GTT
（Graphics Translation Tables，图形转换表）是 GPU 可访问的系统内存。可用的测试
如下：

- 1：简单测试，VRAM 到 GTT 以及 GTT 到 VRAM
- 2：简单测试，VRAM 到 VRAM
- 3：GTT 到 VRAM，缓冲区大小扫描，2 的幂
- 4：VRAM 到 GTT，缓冲区大小扫描，2 的幂
- 5：VRAM 到 VRAM，缓冲区大小扫描，2 的幂
- 6：GTT 到 VRAM，缓冲区大小扫描，常见显示尺寸
- 7：VRAM 到 GTT，缓冲区大小扫描，常见显示尺寸
- 8：VRAM 到 VRAM，缓冲区大小扫描，常见显示尺寸

### amdgpu_test_ib


读取该文件以对所有内核管理的环（ring）运行简单的 IB（Indirect Buffer，间接
缓冲区）测试。IB 通常是由用户空间应用程序生成的命令缓冲区，被提交给内核以在某个
特定 GPU 引擎上执行。这里只运行内核中包含的简单 IB 测试。这些测试是引擎相关的，
用于验证 IB 提交是否正常工作。

### amdgpu_discovery


提供对 GPU 提供的 IP discovery 二进制的原始访问。读取该文件以访问原始二进制。
这对于验证 IP discovery 表的内容很有用。它是芯片相关的。

### amdgpu_vbios


提供对 GPU 的 ROM 二进制镜像的原始访问。读取该文件以访问原始二进制。这对于验证
video BIOS ROM 的内容很有用。它是板卡相关的。

### amdgpu_evict_gtt


从 GTT 内存池中驱逐所有缓冲区。读取该文件以从该池中驱逐所有缓冲区。

### amdgpu_evict_vram


从 VRAM 内存池中驱逐所有缓冲区。读取该文件以从该池中驱逐所有缓冲区。

### amdgpu_gpu_recover


触发一次 GPU 复位。读取该文件以触发整个 GPU 的复位。当前正在 GPU 上运行的所有
工作都将丢失。

### amdgpu_ring_<name>


提供对内核管理的每个环 <name> 的环缓冲区的读取访问。这些对于调试某个特定环上的
问题很有用。环缓冲区是 CPU 向 GPU 发送命令的方式。CPU 将命令写入缓冲区，然后
请求 GPU 引擎处理它。这是环缓冲区的原始二进制内容。使用 UMR 之类的工具可将环
解码为可读形式。

### amdgpu_mqd_<name>


提供对内核驱动管理的环 <name> 的内核管理 MQD（Memory Queue Descriptor，内存
队列描述符）的读取访问。MQD 定义了环的特性，并用于在环未连接到硬件时存储其状态。
驱动将所请求的环特性和元数据（环本身及相关缓冲区的 GPU 地址）写入 MQD，固件在
环映射到硬件槽位时使用 MQD 来填充硬件。仅在使用了 MQD 的引擎上可用。这提供了
对原始 MQD 二进制的访问。

### amdgpu_error_<name>


提供一个接口，用于在与环 <name> 关联的 dma fence 上设置错误码。指定的错误码会
传播到与该环关联的所有 fence。用它向某个环注入一个 fence 错误。

### amdgpu_pm_info


提供关于 GPU 电源管理特性和状态的可读信息。包括当前的 GFX 时钟、显存时钟、电压、
平均 SoC 功耗、温度、GFX 负载、显存负载、SMU 特性掩码、VCN 电源状态，以及时钟
与电源门控特性。

### amdgpu_firmware_info


列出 GPU 使用的所有固件的版本。只有版本非 0 的条目才是有效的。如果版本为 0，则
该固件对 GPU 无效。

### amdgpu_fence_info


显示每个内核驱动管理的环上，最近发出信号（signalled）与发出（emitted）的 fence
序列号。fence 与向引擎的提交相关联。已发出的 fence 已提交到环，已发出信号的
fence 已由 GPU 发出信号。发出 fence 值较大的环存在仍由拥有该环的引擎处理中的
未完成工作。当发出与发出信号的 fence 值相等时，环处于空闲状态。

### amdgpu_gem_info


列出所有使用 GPU 的 PID 以及它们已分配的 GPU 缓冲区。这里列出缓冲区大小、池
（VRAM、GTT 等）以及缓冲区属性（需要 CPU 访问、CPU 缓存属性等）。

### amdgpu_vm_info


列出所有使用 GPU 的 PID 以及它们已分配的 GPU 缓冲区，以及这些缓冲区相对于该
进程的 GPU 虚拟地址空间（例如已驱逐、空闲、已失效等）的状态。

### amdgpu_sa_info


打印内核驱动中子分配管理器（suballocation manager）的所有子分配（sa）。打印与
每个子分配关联的 GPU 地址、大小和 fence 信息。子分配在内核驱动内部用于各种用途。

### amdgpu_<pool>_mm


打印关于内存池 <pool> 的 TTM 信息。

### amdgpu_vram


提供对 VRAM 的直接访问。被 UMR 之类的工具用于检查 VRAM 中的对象。

### amdgpu_iomem


提供对 GTT 内存的直接访问。被 UMR 之类的工具用于检查 GTT 内存。

### amdgpu_regs_*


提供对 GPU 上各种寄存器区间（aperture）的直接访问。被 UMR 之类的工具用于访问
GPU 寄存器。

### amdgpu_regs2


提供一个供 UMR 用于与 GPU 寄存器交互的 IOCTL 接口。


### amdgpu_sensors


提供一个用于查询 GPU 电源指标（温度、平均功耗等）的接口。被 UMR 之类的工具用于
查询 GPU 电源指标。


### amdgpu_gca_config


提供一个用于查询 GPU 细节（Graphics/Compute Array 配置、PCI 配置、GPU 系列等）的
接口。被 UMR 之类的工具用于查询 GPU 细节。

### amdgpu_wave


用于从硬件查询 GFX/compute wave 信息。被 UMR 之类的工具用于查询 GFX/compute
wave 信息。

### amdgpu_gpr


用于从硬件查询 GFX/compute GPR（General Purpose Register，通用寄存器）信息。被
UMR 之类的工具用于在调试着色器时查询 GPR。

### amdgpu_gprwave


提供一个供 UMR 用于与着色器 wave 交互的 IOCTL 接口。

### amdgpu_fw_attestation


提供一个用于读回固件认证记录的接口。
