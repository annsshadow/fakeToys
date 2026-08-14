## 核心驱动基础设施


## GPU 硬件结构


每个 ASIC 都是一组硬件块的集合。我们称它们为 "IP"（知识产权块，Intellectual Property
blocks）。每个 IP 封装了特定的功能。IP 有版本号，也可以混搭使用。例如，你可能有两个
不同的 ASIC，它们都有 System DMA（SDMA）5.x 的 IP。驱动按 IP 来组织。有用于初始化和
操作每个 IP 的驱动组件。也有一大堆较小的 IP，几乎不需要或完全不需要驱动的参与。那些
最终被归入 soc 文件中的公共部分。soc 文件（例如 vi.c、soc15.c、nv.c）包含的是 SoC 本身
而非特定 IP 的代码。例如，像 GPU 复位和寄存器访问函数这类东西就是依赖 SoC 的。

APU 包含的不仅仅是 CPU 和 GPU，它还包含所有的平台部件（音频、usb、gpio 等）。此外，
很多组件在 CPU、平台和 GPU 之间共享（例如 SMU、PSP 等）。特定组件（CPU、GPU 等）通常
有它们与这些公共组件交互的接口。对于像 S0i3 这样的情况，需要在所有组件之间进行大量的
协调，但这可能稍微超出了本节的范畴。

关于 GPU，我们有以下主要 IP：

GMC（Graphics Memory Controller，图形内存控制器）
    在 vega 之前的较旧芯片上，这是一个专用的 IP，但在 vega 及更新的芯片上变得有些
    去中心化了。它们现在为特定的 IP 或 IP 组设有专用的内存集线器（hub）。不过我们在
    驱动中仍将其视为单一组件，因为编程模型仍然非常相似。这就是 GPU 上不同的 IP 获取
    内存（VRAM 或系统内存）的方式。它还提供每进程 GPU 虚拟地址空间的支持。

IH（Interrupt Handler，中断处理程序）
    这是 GPU 上的中断控制器。所有 IP 都将其中断馈入这个 IP，它将其聚合为一组环形
    缓冲区，驱动可以解析这些缓冲区来处理来自不同 IP 的中断。

PSP（Platform Security Processor，平台安全处理器）
    它处理 SoC 的安全策略，执行可信应用，并验证和加载其他块的固件。

SMU（System Management Unit，系统管理单元）
    这是电源管理微控制器。它管理整个 SoC。驱动与它交互以控制时钟、电压、电源轨等
    电源管理特性。

DCN（Display Controller Next，下一代显示控制器）
    这是显示控制器。它处理显示硬件。在 Display Core <amdgpu-display-core> 中有更详细的
    描述。

SDMA（System DMA，系统 DMA）
    这是一个多用途 DMA 引擎。内核驱动将它用于各种事情，包括分页和 GPU 页表更新。它也
    被暴露给用户空间，供用户模式驱动（OpenGL、Vulkan 等）使用。

GC（Graphics and Compute，图形与计算）
    这是图形和计算引擎，即包含 3D 流水线和着色器块的那个块。这是迄今为止 GPU 上最大的
    块。3D 流水线有大量的子块。除此之外，它还包含 CP 微控制器（ME、PFP、CE、MEC）和
    RLC 微控制器。它被暴露给用户空间供用户模式驱动（OpenGL、Vulkan、OpenCL 等）使用。
    更多细节见 :ref:`Graphics (GFX) and Compute <amdgpu-gc>`。

VCN（Video Core Next，下一代视频核心）
    这是多媒体引擎。它处理视频和图像的编码与解码。它被暴露给用户空间供用户模式驱动
    （VA-API、OpenMAX 等）使用。

需要注意的是，这些块之间可以互相交互。下图展示了一些组件及其相互连接：


在图中，与内存相关的块用绿色显示。注意特定的 IP 有一个绿色方块，代表一个名为 'hub'
的小型硬件块，它负责与内存接口。所有的内存集线器都连接在 UMC 中，而 UMC 又连接到内存
块。注意，vega 之前的设备有一个专用的图形内存控制器（GMC）块，在新架构中被 UMC 和 hub
取代。在驱动代码中，你可以通过查找 hub 后缀来识别这个组件，例如：gfxhub、dchub、mmhub、
vmhub 等。要记住，组件与内存块的交互可能因架构而异。例如，在 Navi 及更新的架构上，GC
和 SDMA 都连接到 GCHUB；在 Navi 之前，SDMA 经过 MMHUB；VCN、JPEG 和 VPE 经过 MMHUB；DCN
经过 DCHUB。

对某些内存元素有一些保护，PSP 在这一领域起着至关重要的作用。当特定的固件被加载到内存中
时，PSP 会采取措施确保其具有有效的签名。它还将固件映像存储在名为可信内存区（TMR）的
受保护内存区域中，这样操作系统或驱动就无法在运行时损坏它们。PSP 的另一个用途是支持可信
应用（TA），这些基本上是在可信处理器上运行、处理可信操作（例如 HDCP）的小型应用。PSP
还用于经由可信内存区（TMZ）对内存进行加密以实现内容保护。

另一个关键的 IP 是 SMU。它负责复位分发，以及 SoC 上所有 IP 的时钟、温度和电源管理。
SMU 还有助于平衡性能和功耗。

## GFX、Compute 和 SDMA 的整体行为


   指 GFX、Compute 和 SDMA。

GFX、Compute 和 SDMA 共享类似的操作形式，可以对其进行抽象以助于理解这些块的行为。参见
下图，它展示了这些块的公共组件：


在图的中央部分，你可以看到两个硬件元素，一个叫 **Pipes**，另一个叫 **Queues**；需要
重点指出的是，Queues 必须与一个 Pipe 关联，反之亦然。每个特定的硬件 IP 可能有不同数量的
Pipe，进而有不同数量的 Queue；例如，GFX 11 的 GFX 前端每个 Pipe 有两个 Pipe 和两个
Queue。

Pipe 是处理 Queue 中可用指令的硬件；换句话说，它是一个执行插入到 Queue 中的操作的线程。
Pipe 的一个关键特性是它们一次只能执行一个 Queue；无论硬件在 Pipe 中有多少个 Queue，每个
Pipe 只运行一个 Queue。

Pipe 具有在硬件层面切换队列的机制。不过，它们只使用被视为已映射（mapped）的 Queue。Pipe
可以基于以下任何输入在队列之间切换：

1. 命令流（Command Stream）；
2. 逐包（Packet by Packet）；
3. 其他硬件请求变更（例如 MES）。

Pipe 内的 Queue 由硬件队列描述符（HQD）定义。与 HQD 概念相关联的，我们有内存队列描述符
（MQD），它负责在内存中存储每个可用 Queue 的状态信息。Queue 的状态包含诸如队列本身的 GPU
虚拟地址、保存区（save areas）、doorbell 等信息。MQD 还存储 HQD 寄存器，这对激活或
停用某个 Queue 至关重要。调度固件（例如 MES）负责从 MQD 加载 HQD，以及反向操作。

队列切换过程也可以在固件请求抢占（preemption）或取消映射（unmapping）某个 Queue 时发生。
固件会等待 HQD_ACTIVE 位变为低电平，然后将状态保存到 MQD 中。要使另一个 Queue 变为活动，
固件将 MQD 状态复制到 HQD 寄存器并加载任何额外的状态。最后，它将 HQD_ACTIVE 位置为高电平
以指示该队列处于活动状态。Pipe 随后将从活动 Queue 执行工作。

## 驱动结构


一般来说，驱动有一个特定 SoC 上所有 IP 的列表，对于 init/fini/suspend/resume 这类事情，
大致只是遍历该列表并逐个处理每个 IP。

一些有用的构造：

KIQ（Kernel Interface Queue，内核接口队列）
    这是由内核驱动用来管理 GFX/compute 引擎上其他 gfx 和 compute 队列的控制队列。你
    可以用它来映射/取消映射额外的队列等。在 GFX 11 及更新的硬件上，它被 MES 取代。

IB（Indirect Buffer，间接缓冲区）
    特定引擎的命令缓冲区。与其将命令直接写入队列，不如将命令写入一块内存，然后将该
    内存的指针放入队列。硬件随后会跟随该指针执行内存中的命令，然后返回到环形缓冲区中
    其余的命令。

## 内存域


   :doc: memory domains

## 缓冲区对象


   :doc: amdgpu_object

   :internal:

## PRIME 缓冲区共享


   :doc: PRIME Buffer Sharing

   :internal:

## MMU 通知器


   :doc: MMU Notifier

   :internal:

## AMDGPU 虚拟内存


   :doc: GPUVM

   :internal:

## 中断处理


   :doc: Interrupt Handling

   :internal:

## IP 块


   :doc: IP Blocks

   :identifiers: amd_ip_block_type amd_ip_funcs DC_FEATURE_MASK DC_DEBUG_MASK
