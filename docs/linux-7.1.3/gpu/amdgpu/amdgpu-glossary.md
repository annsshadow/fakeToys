AMDGPU 术语



这里你可以找amdgpu 驱动中使用的一些通用缩写。请注意，我们在
'Documentation/gpu/amdgpu/display/dc-glossary.rst' 为显示核心（Display Core）提供了专门的术语表



    active_cu_number
      系统中处于活动状态的 CU 数量。活CU 的数量可能少SE ** SH ** CU，具体取决于板卡配置


    BACO
      总线存活，芯片关闭（Bus Alive, Chip Off


    BOCO
      总线关闭，芯片关闭（Bus Off, Chip Off


    CE
      常量引擎（Constant Engine


    CIK
      Sea Islands（海岛系列）


    CB
      颜色缓冲区（Color Buffer


    CP
      命令处理器（Command Processor


    CPC
      命令处理器（计算）（Command Processor Compute


    CPF
      命令处理器（取指）（Command Processor Fetch


    CPG
      命令处理器（图形）（Command Processor Graphics


    CPLIB
      内容保护库（Content Protection Library


    CS
      命令提交（Command Submission


    CSB
      清除状态间接缓冲区（Clear State Indirect Buffer


    CU
      计算单元（Compute Unit


    DB
      深度缓冲区（Depth Buffer


    DFS
      数字频率合成器（Digital Frequency Synthesizer


    ECP
      增强内容保护（Enhanced Content Protection


    EOP
      管线末端 / 流水线末端（End Of Pipe/Pipeline


    FLR
      功能级复位（Function Level Reset


    GART
      图形地址重映射表（Graphics Address Remapping Table）。这GPU 内核驱动所使用GPUVM 页表的名称。它将系统资源（内存MMIO 空间）重新映射到 GPU 的地址空间，以GPU 能够访问它们。GART 这个名字可追溯到 AGP 时代，当时平台提供了一GPU 可以用来获取用于 DMA 的连续散页视图的 MMU。此MMU 已迁移到 GPU 上，但这个名字保留了下来


    GC
      图形与计算（Graphics and Compute


    GDS
      全局数据共享（Global Data Share


    GE
      几何引擎（Geometry Engine


    GMC
      图形内存控制器（Graphic Memory Controller


    GPR
      通用寄存器（General Purpose Register


    GPUVM
      GPU 虚拟内存（GPU Virtual Memory）。这GPU MMU。GPU 支持多个任意时刻可以同时在飞的虚拟地址空间。它们允GPU VRAM 和系统资源重新映射到 GPU 虚拟地址空间，供 GPU 内核驱动以及使用 GPU 的应用程序使用。它们为使用 GPU 的不同应用提供内存保护


    GTT
      图形转换表（Graphics Translation Tables）。这是一个通过 TTM 管理的内存池，为 GPU 提供对系统资源（内存MMIO 空间）的访问。这些地址可以被映射到“GARTGPUVM 页表供内核驱动使用，或映射到各进程的 GPUVM 页表供应用程序使用


    GWS
      全局波同步（Global Wave Sync


    IH
      中断处理程序（Interrupt Handler


    IV
      中断向量（Interrupt Vector


    HQD
      硬件队列描述符（Hardware Queue Descriptor


    IB
      间接缓冲区（Indirect Buffer


    IMU
      集成管理单元（电源管理支持）（Integrated Management Unit


    IP
      知识产权模块（Intellectual Property blocks


    KCQ
      内核计算队列（Kernel Compute Queue


    KFD
      内核 Fusion 驱动（Kernel Fusion Driver


    KGQ
      内核图形队列（Kernel Graphics Queue


    KIQ
      内核接口队列（Kernel Interface Queue


    MC
      内存控制器（Memory Controller


    MCBP
      命令缓冲区中途抢占（Mid Command Buffer Preemption


    ME
      微引擎（图形）（MicroEngine (Graphics)


    MEC
      微引擎（计算）（MicroEngine Compute


    MES
      微引擎调度器（MicroEngine Scheduler


    MMHUB
      多媒体集线器（Multi-Media HUB


    MQD
      内存队列描述符（Memory Queue Descriptor


    PA
      图元汇编/ 物理地址（Primitive Assembler / Physical Address


    PDE
      页目录项（Page Directory Entry


    PFP
      预取解析器（图形）（Pre-Fetch Parser (Graphics)


    PPLib
      PowerPlay 库——PowerPlay 是电源管理组件


    PRT
      部分驻留纹理（也称为稀疏驻留）（Partially Resident Texture


    PSP
      平台安全处理器（Platform Security Processor


    PTE
      页表项（Page Table Entry


    RB
      渲染后端（Render Backends）。有些人称之ROPs


    RLC
      运行列表控制器（RunList Controller）。这个名字是过去时代的遗留，如今已没有太多含义。它是一组用GFX 块的通用辅助引擎，除其他职责外，还参GFX 电源管理SR-IOV


    SC
      扫描转换器（Scan Converter


    SDMA
      系统 DMA（System DMA


    SE
      着色器引擎（Shader Engine


    SGPR
      标量通用寄存器（Scalar General-Purpose Registers


    SH
      着色器阵列（SHader array


    SI
      Southern Islands（南方群岛系列）


    SMU/SMC
      系统管理单元 / 系统管理控制器（System Management Unit / System Management Controller


    SPI (AMDGPU)
      着色器处理器输入（Shader Processor Input


    SRLC
      保存/恢复列表控制（Save/Restore List Control


    SRLG
      保存/恢复列表 GPM_MEM（Save/Restore List GPM_MEM


    SRLS
      保存/恢复列表 SRM_MEM（Save/Restore List SRM_MEM


    SS
      扩频（Spread Spectrum


    SX
      着色器导出（Shader Export


    TA
      可信应用（Trusted Application


    TC
      纹理缓存（Texture Cache


    TCP (AMDGPU)
      每条管线的纹理缓存（Texture Cache per Pipe）。尽管“Texture”是该缩写的一部分，但 TCP 代表的是通往内存着色器的路径；也就是说，它与纹理无关。这个名字是从较旧设计中遗留下来的，当时着色器阶段有不同的缓存设计；它指的是旧架构中的 L1 缓存


    TMR
      可信内存区域（Trusted Memory Region


    TMZ
      可信内存区（Trusted Memory Zone


    TOC
      目录（Table of Contents


    UMC
      统一内存控制器（Unified Memory Controller


    UMSCH
      用户模式调度器（User Mode Scheduler


    UTC (AMDGPU)
      统一转换缓存（Unified Translation Cache）。UTC 等同TLB。你可能会看到该缩写末尾L 的变体，UTCL 后跟数字；L 表示缓存级别（例UTCL1 UTCL2）


    UVD
      统一视频解码器（Unified Video Decoder


    VCE
      视频压缩引擎（Video Compression Engine


    VCN
      下一代视频编解码器（Video Codec Next


    VGPR
      向量通用寄存器（Vector General-Purpose Registers


    VMID
      虚拟内存 ID（Virtual Memory ID


    VPE
      视频处理引擎（Video Processing Engine


    XCC
      加速器核心复合体（Accelerator Core Complex


    XCP
      加速器核心分区（Accelerator Core Partition

