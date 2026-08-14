

##  AMD NPU

:Copyright: |copy| 2024 Advanced Micro Devices, Inc.
:Author: Sonal Santan <sonal.santan@amd.com>

## 概述

AMD NPU（神经网络处理单元，Neural Processing Unit）是集成在 AMD 客户端 APU 中的多用户 AI 推理加速器。NPU 能够高效执行 CNN、LLM 等机器学习（Machine Learning）应用。NPU 基于 `AMD XDNA Architecture`_ 架构，由 **amdxdna** 驱动管理。

## 硬件描述

AMD NPU 由以下硬件组件构成：

### AMD XDNA 阵列

AMD XDNA 阵列由采用 `AMD AI Engine Technology`_ 技术构建的计算（compute）与存储（memory）tile 的二维阵列组成。每一列有 4 行计算 tile 和 1 行存储 tile。每个计算 tile 包含一个带有自身专用程序和数据存储器的 VLIW 处理器。存储 tile 充当 L2 存储器。该二维阵列可以在列边界处进行划分，从而创建一个空间上相互隔离的分区（partition），并可以将其绑定到一个工作负载上下文。

每一列还拥有专用的 DMA 引擎，用于在主机 DDR 与存储 tile 之间搬移数据。

AMD Phoenix 和 AMD Hawk Point 客户端 NPU 采用 4x5 拓扑，即 4 行计算 tile 排列为 5 列。AMD Strix Point 客户端 APU 采用 4x8 拓扑，即 4 行计算 tile 排列为 8 列。

### 共享 L2 存储器

单独一行的存储 tile 构成了一块由软件管理的片上 L2 存储器池。DMA 引擎用于在主机 DDR 与存储 tile 之间搬移数据。AMD Phoenix 和 AMD Hawk Point NPU 共有 2560 KB 的 L2 存储器。AMD Strix Point NPU 共有 4096 KB 的 L2 存储器。

### 微控制器

一个微控制器运行 NPU 固件（Firmware），负责命令处理、XDNA 阵列分区设置、XDNA 阵列配置、工作负载上下文管理以及工作负载编排（orchestration）。

NPU 固件使用一个被隔离的、无特权的上下文（称为 ERT）的专用实例来服务每个工作负载上下文。ERT 也用于执行与工作负载上下文相关联、由用户提供的 `ctrlcode`。

NPU 固件使用一个单一的、被隔离的有特权上下文（称为 MERT）来服务来自 amdxdna 驱动的管理命令。

### 邮箱（Mailboxes）

微控制器与 amdxdna 驱动使用一个有特权的通道来执行管理任务，例如建立上下文、遥测（telemetry）、查询、错误处理、建立用户通道等。如前所述，有特权通道的请求由 MERT 服务。该有特权通道绑定到单一的邮箱。

微控制器与 amdxdna 驱动为每个工作负载上下文使用一个专用的用户通道。用户通道主要用于向 NPU 提交工作。如前所述，用户通道的请求由一个 ERT 实例服务。每个用户通道都绑定到其自身专用的邮箱。

### PCIe EP

NPU 对于 x86 主机 CPU 而言是一个带有多个 BAR 和若干 MSI-X 中断向量的 PCIe 设备。NPU 使用一个专用的、高带宽的 SoC 级互连（fabric）来读写主机内存。每个 ERT 实例都拥有其自身专用的 MSI-X 中断。MERT 获得单一的 MSI-X 中断实例。

PCIe BAR 的数量因具体设备而异。根据其功能，PCIe BAR 一般可分为以下几类。

- PSP BAR：暴露 AMD PSP（平台安全处理器，Platform Security Processor）功能
- SMU BAR：暴露 AMD SMU（系统管理单元，System Management Unit）功能
- SRAM BAR：暴露用于邮箱的环形缓冲区
- Mailbox BAR：暴露邮箱控制寄存器（head、tail 以及 ISR 等寄存器）
- Public Register BAR：暴露公共寄存器

在特定设备上，上述 BAR 类型可能会被合并到单个物理 PCIe BAR 中。或者某个模块可能需要两个物理 PCIe BAR 才能完全正常工作。例如：

- 在 AMD Phoenix 设备上，PSP、SMU、Public Register BAR 位于 PCIe BAR 索引 0。
- 在 AMD Strix Point 设备上，Mailbox 与 Public Register BAR 位于 PCIe BAR 索引 0。PSP 的部分寄存器位于 PCIe BAR 索引 0（Public Register BAR）和 PCIe BAR 索引 4（PSP BAR）。

### 进程隔离硬件

如前所述，XDNA 阵列可以动态划分为相互隔离的空间分区，每个分区可以有一列或多列。空间分区由微控制器通过对列隔离寄存器进行编程来建立。每个空间分区都关联一个同样由微控制器编程的 PASID。因此，NPU 中的多个空间分区可以进行由 PASID 保护的并发主机访问。

NPU 固件本身使用由微控制器 MMU 强制的隔离上下文来服务用户和有特权通道请求。

## 空间与时间混合调度

AMD XDNA 架构支持二维阵列的空间与时间（时间片共享）混合调度。这意味着空间分区可以被动态地建立和拆除，以适应各种工作负载。一个**空间**分区可以被**独占**绑定到单个工作负载上下文，而另一个分区可以被**临时**绑定到多个工作负载上下文。微控制器会更新临时共享分区的 PASID，以匹配在任一时刻被绑定到该分区的上下文。

### 资源求解器（Resource Solver）

amdxdna 驱动的资源求解器（Resource Solver）组件管理二维阵列在各工作负载之间的分配。每个工作负载在其元数据中描述了运行 NPU 二进制所需的列数。资源求解器组件利用工作负载传入的提示及其自身的启发式规则，来决定用于列的空间与时间共享的二维阵列（重新）分区策略以及工作负载的映射。固件（FW）强制执行由资源求解器做出的上下文到列（或若干列）的资源绑定决策。

AMD Phoenix 和 AMD Hawk Point 客户端 NPU 可以支持 6 个并发的工作负载上下文。AMD Strix Point 可以支持 16 个并发的工作负载上下文。

## 应用程序二进制文件

一个 NPU 应用工作负载由 NPU 编译器生成的两个独立二进制文件组成。

1. AMD XDNA 阵列叠加（overlay），用于配置一个 NPU 空间分区。该 overlay 包含用于设置流开关（stream switch）配置以及面向计算 tile 的 ELF。该 overlay 由与之关联的 ERT 实例加载到绑定给该工作负载的空间分区上。更多细节请参考
   `Versal Adaptive SoC AIE-ML Architecture Manual (AM020)`_。

2. `ctrlcode`，用于编排加载在空间分区上的 overlay。`ctrlcode` 由在微控制器上以保护模式运行的、处于该工作负载上下文中的 ERT 执行。`ctrlcode` 由一系列名为 `XAie_TxnOpcode` 的操作码（opcode）构成。更多细节请参考
   `AI Engine Run Time`_。

## 特殊主机缓冲区

### 每上下文指令缓冲区

每个工作负载上下文都使用一个驻留在主机上的 64 MB 缓冲区，它被内存映射到为服务该工作负载而创建的 ERT 实例中。该工作负载所使用的 `ctrlcode` 会被复制到这块特殊内存中。该缓冲区与所有其他由该工作负载使用的输入/输出缓冲区一样，受 PASID 保护。指令缓冲区也被映射到该工作负载的用户空间。

### 全局有特权缓冲区

此外，驱动还分配一个单一的缓冲区用于维护任务，例如记录来自 MERT 的错误。该全局缓冲区使用全局 IOMMU 域，并且只能由 MERT 访问。

## 高层使用流程

以下是在 AMD NPU 上运行一个工作负载的步骤：

1. 将工作负载编译为一个 overlay 和一个 `ctrlcode` 二进制文件。
2. 用户空间在驱动中打开一个上下文，并提供该 overlay。
3. 驱动与资源求解器协商，为该工作负载分配一组列。
4. 驱动随后请求 MERT 在设备上用所需的列创建一个上下文。
5. MERT 随后创建一个 ERT 实例。MERT 还将指令缓冲区映射到 ERT 内存中。
6. 用户空间随后将 `ctrlcode` 复制到指令缓冲区中。
7. 用户空间随后创建一个带有指向输入、输出以及指令缓冲区指针的命令缓冲区；然后它将命令缓冲区提交给驱动，并进入睡眠以等待完成。
8. 驱动通过邮箱将命令发送给 ERT。
9. ERT **执行**指令缓冲区中的 `ctrlcode`。
10. `ctrlcode` 的执行会启动 AMD XDNA 阵列运行期间往返于主机 DDR 的 DMA。
11. 当 ERT 到达 `ctrlcode` 末尾时，它会触发一个 MSI-X 来向驱动发送完成信号，驱动随后唤醒等待中的工作负载。

## 启动流程

amdxdna 驱动使用 PSP 来安全地加载经过签名的 NPU 固件（FW），并启动 NPU 微控制器的引导。amdxdna 驱动随后在 BAR 0 上某个特殊位置等待 alive 信号。NPU 在 SoC 挂起（suspend）期间被关闭，并在恢复（resume）后重新打开，此时 NPU 固件被重新加载，并再次执行握手。

## 用户空间组件

### 编译器

Peano 是一个基于 LLVM 的、开源的、面向 AMD XDNA 阵列计算 tile 的单核编译器。Peano 位于：
https://github.com/Xilinx/llvm-aie

IRON 是一个开源的、面向基于 AMD XDNA 阵列的 NPU 的阵列编译器，它在底层使用 Peano。IRON 位于：
https://github.com/Xilinx/mlir-aie

### 用户态驱动（UMD）

开源的 XRT 运行时栈与 amdxdna 内核驱动对接。XRT 位于：
https://github.com/Xilinx/XRT

开源的、面向 NPU 的 XRT shim 位于：
https://github.com/amd/xdna-driver

## DMA 操作

DMA 操作指令被编码在 `ctrlcode` 中，形式为 `XAIE_IO_BLOCKWRITE` 操作码。当 ERT 执行 `XAIE_IO_BLOCKWRITE` 时，会在主机 DDR 与 L2 存储器之间产生 DMA 操作。

## 错误处理

当 MERT 在 AMD XDNA 阵列中检测到错误时，它会暂停该工作负载上下文的执行，并通过有特权通道向驱动发送一条异步消息。驱动随后向 MERT 发送一个缓冲区指针，以捕获绑定到出错工作负载上下文的分区的寄存器状态。驱动随后通过读取该缓冲区指针的内容来解码错误。

## 遥测

MERT 可以报告各种遥测信息，例如：

- L1 中断计数
- DMA 计数
- 深度睡眠（Deep Sleep）计数
- 等

## 参考资料

- `AMD XDNA Architecture <https://www.amd.com/en/technologies/xdna.html>`_
- `AMD AI Engine Technology <https://www.xilinx.com/products/technology/ai-engine.html>`_
- `Peano <https://github.com/Xilinx/llvm-aie>`_
- `Versal Adaptive SoC AIE-ML Architecture Manual (AM020) <https://docs.amd.com/r/en-US/am020-versal-aie-ml>`_
- `AI Engine Run Time <https://github.com/Xilinx/aie-rt/tree/release/main_aig>`_
