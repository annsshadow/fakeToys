
##  drm/amdgpu - 图形与计算（GC

CPU GPU 之间的关系可以描述为生产消费者问题：CPU 将待 GPU（消费者）
执行的操作填充到缓冲区（生产者）中。缓冲区中请求的操作称为**命令（Command Packets*，可以概括为向图形控制器传输命令信息的一种压缩方式
充当 CPU GPU 之间前端组件的称*命令处理器（CP，Command Processor*该组件负责为**图形与计算（GC，Graphics and Compute*提供更大的灵活性，
因为 CP 使得GPU 流水线的各个方面进行编程成为可能。CP 还通过一种名**环形缓冲区（Ring Buffers*的机制来协调 CPU GPU 之间的通信，其CPU 向缓冲区追加信息，GPU 移除操作。CP 还负责处*间接缓冲区（IBIndirect Buffers*
作为参考，CP 在内部由若干子模块组成（CPC - CP compute、CPG - CP graphics以及 CPF - CP fetcher）。其中一些缩写会出现在寄存器名称中，但这更多实现细节，不会直接影响驱动编程或调试
### 图形（GFX）与计算微控制器


GC 是一个很大的模块，因此它关联了多个固件。其中一些如下：

CP（命令处理器，Command Processor    涵盖 GFX/Compute 流水线前端的硬件模块名称。主要由一组微控制    （PFP、ME、CE、MEC）组成。运行在这些微控制器上的固件提供了与
    GFX/Compute 引擎交互的驱动接口
    MEC（MicroEngine Compute，微引擎计算        这是控制 GFX/compute 引擎上计算队列的微控制器
    MES（MicroEngine Scheduler，微引擎调度器）
        这是用于管理队列的引擎。更多细节请参阅
        MicroEngine Scheduler (MES) <amdgpu-mes>銆。
RLC（RunList Controller，运行列表控制器    这是 GFX/Compute 引擎中的另一个微控制器。它处理 GFX/Compute 引擎    与电源管理相关的功能。该名称是旧硬件的遗留物，当时它最初被加入    与现在该引擎实际所做的事情并没有太大关系
- [mes.rst](mes.rst)
