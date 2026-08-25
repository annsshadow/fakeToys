
## Falcon (FAst Logic Controller)

以下各节描述 Falcon 核心及其上运行的微码（ucode）。这些描述基Ampere GPU 或更早的设计；不过它们大体上也适用于未来的设计，但一切均可能变动。此处提供的概述主要旨在帮助理解 nova-core 驱动Falcon 的交互
NVIDIA GPU 内嵌了称Falcon 核心的小型类 RISC 微控制器，负责处理安全固件任务、初始化和电源管理。现NVIDIA GPU 可能拥有多个这样Falcon 实例（例GSP（GPU 系统处理器）SEC2（安全引擎）），并且也可能集成一RISC-V 核心。该核心既能运行 RISC-V 代码，也能运Falcon 代码
运行Falcon 核心上的代码也称'ucode'（微码），后续章节将沿用此称呼
Falcon 拥有独立的指令与数据存储器（IMEM/DMEM），并提供小DMA 引擎（经FBIF—帧缓冲接，Frame Buffer Interface）从系统内存加载代码。nova-core 驱动必须复位并配Falcon，通过 DMA 加载其固件，并启动其 CPU
## Falcon 安全级别

Falcon 可以运行在非安全（NS）、轻安全（LS）或重安全（HS）模式中
### 重安全（HS），也称特权3（PL3
HS 微码是最受信任的代码，几乎可以访问芯片上的一切。HS 二进制文件内包含一个在启动时验证的签名。该签名验证由硬件自身完成，从而建立信任根（root of trust）。例如，FWSEC-FRTS 命令（见 fwsec.rst）在 HS 模式下的 GSP 上运行。FRTS 涉及建立并向 WPR（写保护区域，Write Protect Region）加载内容，必须HS 微码完成，主CPU LS 微码都无法完成
### 轻安全（LS PL2）与非安全（NS PL0
这些模式的安全性低HS。与 HS 类似，LS NS 微码二进制文件通常也包含一个签名。要Falcon 加载 LS NS 模式的固件，需要另一Falcon 运行HS 模式下，这也建立了信任根。例如，Ampere GPU 上，CPU SEC2 Falcon 上以 HS 模式运行 "Booter" 微码，随后它对运行时GSP 二进制文件（GSP-RM）进行认证，并以 LS 模式GSP Falcon 上运行它。类似的例子是：Ampere 复位后，FWSEC GSP 上运行，随后devinit 引擎LS 模式加载PMU 上
### 信任根的建立

要建立信任根，运行在 Falcon 上的代码必须是不可变的，并硬连线到只读存储器（ROM）中。这符合业界固件验证的规范。这段代码称为引ROM（Boot ROM，BROM）。CPU 上的 nova-core 驱动通过多个"BROM" 为前缀Falcon 寄存器与 Falcon Boot ROM 通信（见 regs.rs）
nova-core 驱动VBIOS 读取必要的微码后，会BROM DMA 寄存器进行编程，以触Falcon HS 微码从系统内存加载到 Falcon IMEM/DMEM 中。HS 微码加载完成后，会由 Falcon Boot ROM 验证
一旦经过验证的 HS 代码Falcon 上运行，它就可以验证并将其他 LS/NS 微码二进制文件加载到其他 Falcon 上并启动它们。签名验证的过程HS 相同；只是这种情况下，计算签名的不是硬件（BROM），而是 HS 微码
因此，信任根的建立过程如下：
     Hardware (Boot ROM running on the Falcon) -> HS ucode -> LS/NS ucode.

例如，在 Ampere GPU 上，启动验证流程为：
     Hardware (Boot ROM running on the SEC2) ->
          HS ucode (Booter running on the SEC2) ->
               LS ucode (GSP-RM running on the GSP)

     虽然 CPU 可以HS 微码加载Falcon 微控制器上，并让它由硬件验证后运行，CPU 本身通常不会去加LS NS 微码并运行它。LS NS 微码的加载主要由 HS 微码完成。例如，Ampere GPU 上，Booter 微码SEC2 上以 HS 模式运行并将 GSP-RM 二进制文件加载到 GSP 上之后，它在运行时还需要运"SEC2-RTOS" 微码。这就带来了一个问题：没有任何组件能把 SEC2-RTOS 微码加载SEC2 上。CPU 无法加载 LS 代码，GSP-RM 又必须在 LS 模式下运行。为克服这一点，GSP 被临时设置为运行 HS 微码（该微码本身CPU 经由 nova-core 驱动使用一"GSP 提供的定序器" 加载），由它再以 LS 模式SEC2-RTOS 微码加载SEC2 上。随GSP 恢复运行它自身的 GSP-RM LS 微码
## Falcon 存储器子系统DMA 引擎

Falcon 拥有独立的指令与数据存储器（IMEM/DMEM），并包含一个称FBDMA（帧缓冲 DMA，Framebuffer DMA）的小型 DMA 引擎，它经由 FBIF（帧缓冲接口，Framebuffer Interface）在 Falcon 内部IMEM/DMEM 存储器与外部内存之间执行 DMA 传输
DMA 传输可以Falcon 的存储器发往系统内存和帧缓冲内存（VRAM）
要通过 FBDMA 执行 DMA，需要对 FBIF 进行配置，以决定内存如何被访问（也称aperture 类型）。在 nova-core 驱动中，这由 `FalconFbifTarget` 枚举决定
Falcon 中的 IO-PMP 块（输入输出物理内存保护，Input/Output Physical Memory Protection）单元控制着 FBDMA 对外部内存的访问
```

               External Memory (Framebuffer / System DRAM)
                              ^  |
                              |  |
                              |  v
     +-----------------------------------------------------+
     |                           |                         |
     |   +---------------+       |                         |
     |   |     FBIF      |-------+                         |  FALCON
     |   | (FrameBuffer  |   Memory Interface              |  PROCESSOR
     |   |  InterFace)   |                                 |
     |   |  Apertures    |                                 |
     |   |  Configures   |                                 |
     |   |  mem access   |                                 |
     |   +-------^-------+                                 |
     |           |                                         |
     |           | FBDMA uses configured FBIF apertures    |
     |           | to access External Memory
     |           |
     |   +-------v--------+      +---------------+
     |   |    FBDMA       |  cfg |     RISC      |
     |   | (FrameBuffer   |<---->|     CORE      |----->. Direct Core Access
     |   |  DMA Engine)   |      |               |      |
     |   | - Master dev.  |      | (can run both |      |
     |   +-------^--------+      | Falcon and    |      |
     |           |        cfg--->| RISC-V code)  |      |
     |           |        /      |               |      |
     |           |        |      +---------------+      |    +------------+
     |           |        |                             |    |   BROM     |
     |           |        |                             <--->| (Boot ROM) |
     |           |       /                              |    +------------+
     |           |      v                               |
     |   +---------------+                              |
     |   |    IO-PMP     | Controls access by FBDMA     |
     |   | (IO Physical  | and other IO Masters         |
     |   | Memory Protect)                              |
     |   +-------^-------+                              |
     |           |                                      |
     |           | Protected Access Path for FBDMA      |
     |           v                                      |
     |   +---------------------------------------+      |
     |   |       Memory                          |      |
     |   |   +---------------+  +------------+   |      |
     |   |   |    IMEM       |  |    DMEM    |   |<-----+
     |   |   | (Instruction  |  |   (Data    |   |
     |   |   |  Memory)      |  |   Memory)  |   |
     |   |   +---------------+  +------------+   |
     |   +---------------------------------------+
     +-----------------------------------------------------+

```
