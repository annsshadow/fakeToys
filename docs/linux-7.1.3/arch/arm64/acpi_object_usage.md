## ACPI 琛。

下面对各ACPI 表的期望进行了讨论
如果使用了章节号，它指的是定义该对象ACPI 规范中的章节号。如果使用了 "Signature Reserved"，则表签名（表的前四个字节）是该规范唯一识别的部分，而实际表的定义在 UEFI Forum 之外（参见规范的 5.2.6 节）
对于 arm64 上的 ACPI，表还分为以下几类：

       - 必需（Required DSDT, FADT, GTDT, MADT, MCFG, RSDP, SPCR, XSDT

       - 推荐（Recommended BERT, EINJ, ERST, HEST, PCCT, SSDT

       - 可选（Optional AGDI, BGRT, CEDT, CPEP, CSRT, DBG2, DRTM, ECDT, FACS, FPDT,
          HMAT, IBFT, IORT, MCHI, MPAM, MPST, MSCT, NFIT, PMTT, PPTT, RASF, SBST,
          SDEI, SLIT, SPMI, SRAT, STAO, TCPA, TPM2, UEFI, XENV

       - 不支持（Not supported AEST, APMT, BOOT, DBGP, DMAR, ETDT, HPET, IVRS, LPIT,
          MSDM, OEMx, PDTT, PSDT, RAS2, RSDT, SLIC, WAET, WDAT, WDRT, WPBT

====== ========================================================================
Table  用于 ARMv8 Linux 的用====== ========================================================================
AEST   Signature Reserved (signature == "AEST")

       **Arm 错误源表（Arm Error Source Table*

       该表告知操作系统系统中所有符Arm RAS 架构的错误节点
AGDI   Signature Reserved (signature == "AGDI")

       **Arm 通用诊断转储与复位设备接口表（Arm Generic diagnostic Dump and Reset Device Interface Table*

       该表描述一个不可屏蔽事件，由平台固件使用，用于请求操作系统生成诊断转储并复位设备
APMT   Signature Reserved (signature == "APMT")

       **Arm 性能监控表（Arm Performance Monitoring Table*

       该表描述系统中各组件所实现PMU 支持属性
BERT   Section 18.3 (signature == "BERT")

       **启动错误记录表（Boot Error Record Table*

       如果平台提供 RAS 支持则必须提供。建议提供此表
BOOT   Signature Reserved (signature == "BOOT")

       **简BOOT 标志表（simple BOOT flag table*

       仅微软使用的表，将不被支持
BGRT   Section 5.2.22 (signature == "BGRT")

       **启动图形资源表（Boot Graphics Resource Table*

       可选，当前不支持，ARM 服务器没有实际用例
CEDT   Signature Reserved (signature == "CEDT")

       **CXL 早期发现表（CXL Early Discovery Table*

       该表允许操作系统发现任何 CXL 主机桥及其主机桥寄存器
CPEP   Section 5.2.18 (signature == "CPEP")

       **已修正平台错误轮询表（Corrected Platform Error Polling table*

       可选，当前不支持，且在具备 ARM 兼容硬件并适当修改规范之前不建议使用
CSRT   Signature Reserved (signature == "CSRT")

       **核心系统资源表（Core System Resources Table*

       可选，当前不支持
DBG2   Signature Reserved (signature == "DBG2")

       **调试端口2（DeBuG port table 2*

       许可证已变更，应当可用。如果在命令行中替代 earlycon=<device> 使用则为可选
DBGP   Signature Reserved (signature == "DBGP")

       **调试端口表（DeBuG Port table*

       仅微软使用的表，将不被支持
DSDT   Section 5.2.11.1 (signature == "DSDT")

       **差异化系统描述表（Differentiated System Description Table*

       DSDT 是必需的；另见 SSDT
       ACPI 表只包含一DSDT，但可以包含一个或多个可选的 SSDT。每SSDT 只能       ACPI 命名空间添加内容，不能修改或替换 DSDT 中的任何内容
DMAR   Signature Reserved (signature == "DMAR")

       **DMA 重映射表（DMA Remapping table*

       x86 使用的表，将不被支持
DRTM   Signature Reserved (signature == "DRTM")

       **动态度量信任根表（Dynamic Root of Trust for Measurement table*

       可选，当前不支持
ECDT   Section 5.2.16 (signature == "ECDT")

       **嵌入式控制器描述表（Embedded Controller Description Table*

       可选，当前不支持，但仅在硬件精简模式下使GPE_BIT 字段来表IRQ 号时才可能在
       ARM 上使用，因为在硬件精简模式下没有定GPE 块。这需要在 ACPI 规范中作出修改
EINJ   Section 18.6 (signature == "EINJ")

       **错误注入表（Error Injection table*

       该表对于测试平台对错误条件的响应非常有用；它允许向系统中注入一个错误，就像它实       发生一样。但是，此表不应随生产系统发布；它应仅在测试期间使用 ACPICA 工具动态加       和执行
ERST   Section 18.5 (signature == "ERST")

       **错误记录序列化表（Error Record Serialization Table*

       在支RAS 的平台上，若其非基于 UEFI，则必须提供此表；若基于 UEFI，则可以提供此表       当没有此表时，将使用 UEFI 运行时服务来在持久化存储中保存和取回硬件错误信息
ETDT   Signature Reserved (signature == "ETDT")

       **事件定时器描述表（Event Timer Description Table*

       已废弃的表，将不被支持
FACS   Section 5.2.10 (signature == "FACS")

       **固件 ACPI 控制结构（Firmware ACPI Control Structure*

       该表不太可能非常有用。如果提供，将不会使用全局锁，因为它不属于硬件精简配置的一部分       并且只有 64 位地址字段会被视为有效
FADT   Section 5.2.9 (signature == "FACP")

       **固定 ACPI 描述表（Fixed ACPI Description Table*
       arm64 是必需的

       HW_REDUCED_ACPI 标志必须设置。当设置 HW_REDUCED_ACPI 时应忽略的所有字段都应为零
       如果提供FACS 表，应使X_FIRMWARE_CTRL 字段，而不FIRMWARE_CTRL
       如果使用 PSCI（如建议），请确保正确填ARM_BOOT_ARCH —设置 PSCI_COMPLIANT 标志       并根据需要设置或清除 PSCI_USE_HVC（见5-37）
       对于同样必需DSDT，应使用 X_DSDT 字段，而不DSDT 字段
FPDT   Section 5.2.23 (signature == "FPDT")

       **固件性能数据表（Firmware Performance Data Table*

       可选，对启动性能分析有用
GTDT   Section 5.2.24 (signature == "GTDT")

       **通用定时器描述表（Generic Timer Description Table*

       arm64 是必需的
HEST   Section 18.3.2 (signature == "HEST")

       **硬件错误源表（Hardware Error Source Table*

       已经定义ARM 特有的错误源；请使用这些，或者使PCI 类型，例如类6（AER 根端口）       7（AER 端点）或 8（AER 桥），或者使用类9（通用硬件错误源）。仅arm64 上使       Trusted Firmware 时才可能进行固件优先的错误处理
       如果平台提供 RAS 支持则必须提供。建议提供此表
HMAT   Section 5.2.28 (signature == "HMAT")

       **异构内存属性表（Heterogeneous Memory Attribute Table*

       该表描述与内存邻近域相关的内存属性，例如内存侧缓存属性以及带宽和延迟细节。操作系       使用这些信息来优化系统内存配置
HPET   Signature Reserved (signature == "HPET")

       **高精度事件定时器表（High Precision Event timer Table*

       x86 使用的表，将不被支持
IBFT   Signature Reserved (signature == "IBFT")

       **iSCSI 启动固件表（iSCSI Boot Firmware Table*

       微软定义的表，支持情况待定
IORT   Signature Reserved (signature == "IORT")

       **输入输出重映射表（Input Output Remapping Table*

       arm64 使用的表，用于描IO 拓扑、SMMU GIC ITS，以及这些不同组件如何连接在一起，
       例如标识哪些组件位于哪些 SMMU/ITS 之后。该表仅在特SBSA 平台上是必需的（例如使用
       GICv3-ITS SMMU 时）；在 SBSA Level 0 平台上它仍是可选的
IVRS   Signature Reserved (signature == "IVRS")

       **I/O 虚拟化报告结构（I/O Virtualization Reporting Structure*

       x86_64（AMD）使用的表，将不被支持
LPIT   Signature Reserved (signature == "LPIT")

       **低功耗空闲表（Low Power Idle Table*

       ACPI 5.1 之前x86 使用的表；从 ACPI 6.0 起，ARM 平台上的处理器描述和电源状态应
       使用 DSDT 并定义处理器容器设备（_HID ACPI0010，第 8.4 节，更具体地8.4.3 8.4.4）
MADT   Section 5.2.12 (signature == "APIC")

       **APIC 描述表（Multiple APIC Description Table*

       arm64 是必需的。只应使GIC 中断控制器结构（类型 0xA - 0xF）
MCFG   Signature Reserved (signature == "MCFG")

       **内存映射配置空间（Memory-mapped ConFiGuration space*

       如果平台支持 PCI/PCIe，则需MCFG 表
MCHI   Signature Reserved (signature == "MCHI")

       **管理控制器主机接口表（Management Controller Host Interface table*

       可选，当前不支持
MPAM   Signature Reserved (signature == "MPAM")

       **内存分区与监控表（Memory Partitioning And Monitoring table*

       该表允许操作系统发现各子系统实现MPAM 控制
MPST   Section 5.2.21 (signature == "MPST")

       **内存电源状态表（Memory Power State Table*

       可选，当前不支持
MSCT   Section 5.2.19 (signature == "MSCT")

       **最大系统特性表（Maximum System Characteristic Table*

       可选，当前不支持
MSDM   Signature Reserved (signature == "MSDM")

       **微软数据管理表（Microsoft Data Management table*

       仅微软使用的表，将不被支持
NFIT   Section 5.2.25 (signature == "NFIT")

       **NVDIMM 固件接口表（NVDIMM Firmware Interface Table*

       可选，当前不支持
OEMx   Signature of "OEMx" only

       **OEM 特定表（OEM Specific Tables*

       所有以 "OEM" 签名的表都保留给 OEM 使用。由于这些表并非用于通用目的，而是限于非常
       特定的最终用户，因此不建议使用，并且 arm64 的内核也不支持它们
PCCT   Section 14.1 (signature == "PCCT)

       **平台通信通道表（Platform Communications Channel Table*

       推荐arm64 上使用；当使CPPC 控制平台处理器的性能和功耗时，建议使PCC
PDTT   Section 5.2.29 (signature == "PDTT")

       **平台调试触发表（Platform Debug Trigger Table*

       该表描述用于收集非架构特性调试日志的 PCC 通道

PMTT   Section 5.2.21.12 (signature == "PMTT")

       **平台内存拓扑表（Platform Memory Topology Table*

       可选，当前不支持
PPTT   Section 5.2.30 (signature == "PPTT")

       **处理器属性拓扑表（Processor Properties Topology Table*

       该表提供处理器和缓存拓扑
PSDT   Section 5.2.11.3 (signature == "PSDT")

       **持久系统描述表（Persistent System Description Table*

       已废弃的表，将不被支持
RAS2   Section 5.2.21 (signature == "RAS2")

       **RAS 特2 表（RAS Features 2 table*

       该表为平台实现的 RAS 能力提供接口
RASF   Section 5.2.20 (signature == "RASF")

       **RAS 特性表（RAS Feature table*

       可选，当前不支持
RSDP   Section 5.2.5 (signature == "RSD PTR")

       **根系统描述指针（Root System Description PoinTeR*

       arm64 是必需的
RSDT   Section 5.2.7 (signature == "RSDT")

       **根系统描述表（Root System Description Table*

       由于该表只能提供 32 位地址，它arm64 上已被废弃，将不会被使用。如果提供，它将被忽略
SBST   Section 5.2.14 (signature == "SBST")

       **智能电池子系统表（Smart Battery Subsystem Table*

       可选，当前不支持
SDEI   Signature Reserved (signature == "SDEI")

       **软件委托异常接口表（Software Delegated Exception Interface table*

       该表通告 SDEI 接口的存在
SLIC   Signature Reserved (signature == "SLIC")

       **软件许可表（Software LIcensing table*

       仅微软使用的表，将不被支持
SLIT   Section 5.2.17 (signature == "SLIT")

       **系统局部性距离信息表（System Locality distance Information Table*

       一般来说可选，但对 NUMA 系统是必需的
SPCR   Signature Reserved (signature == "SPCR")

       **串口控制台重定向表（Serial Port Console Redirection table*

       arm64 是必需的
SPMI   Signature Reserved (signature == "SPMI")

       **服务器平台管理接口表（Server Platform Management Interface table*

       可选，当前不支持
SRAT   Section 5.2.16 (signature == "SRAT")

       **系统资源亲和性表（System Resource Affinity Table*

       可选，但如果使用，则只读取 GICC Affinity 结构。为了支arm64 NUMA，此表是必需的
SSDT   Section 5.2.11.2 (signature == "SSDT")

       **辅助系统描述表（Secondary System Description Table*

       这些表是 DSDT 的延续；建议将它们用于可以添加到运行中的系统的设备，但也可以起到       设备描述拆分为更易管理的片段的作用
       SSDT 只能ACPI 命名空间添加内容。它不能修改或替换命名空间中已有的设备描述
       不过这些表是可选的。ACPI 表应只包含一DSDT，但可以包含多个 SSDT
STAO   Signature Reserved (signature == "STAO")

       **_STA 覆盖表（_STA Override table*

       可选，但仅在虚拟化环境中为了向客户机操作系统隐藏设备时才需要
TCPA   Signature Reserved (signature == "TCPA")

       **可信计算平台联盟表（Trusted Computing Platform Alliance table*

       可选，当前不支持，并且可能需要修改才能与 arm64 完全互操作
TPM2   Signature Reserved (signature == "TPM2")

       **可信平台模块 2 表（Trusted Platform Module 2 table*

       可选，当前不支持，并且可能需要修改才能与 arm64 完全互操作
UEFI   Signature Reserved (signature == "UEFI")

       **UEFI ACPI 数据表（UEFI ACPI data table*

       可选，当前不支持。目前对 arm64 没有已知用例
WAET   Signature Reserved (signature == "WAET")

       **Windows ACPI 模拟设备表（Windows ACPI Emulated devices Table*

       仅微软使用的表，将不被支持
WDAT   Signature Reserved (signature == "WDAT")

       **看门狗动作表（Watch Dog Action Table*

       仅微软使用的表，将不被支持
WDRT   Signature Reserved (signature == "WDRT")

       **看门狗资源表（Watch Dog Resource Table*

       仅微软使用的表，将不被支持
WPBT   Signature Reserved (signature == "WPBT")

       **Windows 平台二进制表（Windows Platform Binary Table*

       仅微软使用的表，将不被支持
XENV   Signature Reserved (signature == "XENV")

       **Xen 项目表（Xen project table*

       可选，目前Xen 使用
XSDT   Section 5.2.8 (signature == "XSDT")

       **扩展系统描述表（eXtended System Description Table*

       arm64 是必需的====== ========================================================================

### ACPI 对象

下面列出了可能使用的各个 ACPI 对象的期望；任何未在下面明确提及的对象都应根据特定平台或特定
子系统（例如电源管理PCI）的需要来使用
===== ================ ========================================================
Name   Section         Usage for ARMv8 Linux
===== ================ ========================================================
_CCA   6.2.17         该方法必须为 arm64 上所有总线主设备定—不会对这些设                      是否缓存一致做任何假设。_CCA 值由这些设备的所有后代继承，因此无需
                      重复定义。在 arm64 上若没有 _CCA，内核不知道该如何为该设备设DMA
                      NB：该方法提供默认的缓存一致性属性；不过，SMMU 的存在可以对此进                      修改。例如，某个主设备默认可能是非一致的，但通过适当SMMU 配置
                      可以变为一致的（参IORT 规范17，ARM 文档 DEN 0049B）
_CID   6.1.2          按需使用，另_HID
_CLS   6.1.3          按需使用，另_HID
_CPC   8.4.7.1        按需使用，特定于电源管理。在 arm64 上推荐使CPPC
_CRS   6.2.2          arm64 上是必需的
_CSD   8.4.2.2        按需使用，仅_CST 配合使用
_CST   8.4.2.1        推荐使用低功耗空闲状态（8.4.4）而非 C-states
_DDN   6.1.4          该字段可用于设备名称。但是，它本意用DOS 设备名称（例COM1），
                      因此跨操作系统使用时要小心
_DSD   6.2.5          使用时应谨慎。如果使用此对象，请尽量在设备属UUID 已定义的约束
                      范围内使用它。只有在极少数情况下才需要创建新_DSD UUID
                      无论哪种情况，都应提_DSD 定义以及任何驱动补丁以供讨论，尤其是                      使用设备属性时。没有相应的 _DSD 描述，驱动将被视为不完整。一旦获                      内核维护者批准，UUID 或设备属性还必须UEFI Forum 注册；由于会                      多个操作系统注册条目，这可能引起一些反复
_DSM   9.1.1          不要使用此方法。它未标准化，返回值文档不全，并且目前是频繁的错误来源
\_GL   5.7.1          该对象不应在硬件精简模式下使用，因此不应arm64 上使用
_GLK   6.5.7          该对象需要定义一个全局锁；由于 arm64 运行在硬件精简模式下，没有
                      全局锁。因此，不要arm64 上使用此对象
\_GPE  5.3.1          此命名空间仅用于 x86。不要在 arm64 上使用它
_HID   6.1.5          这是在设备探测中使用的主要对象，不过也可以使_CID _CLS
_INI   6.5.1          非必需，但UEFI 将设备留在驱动开始探测前可能不期望的状态时，它                      设置设备可能有用
_LPI   8.4.4.3        推荐arm64 上与处理器定义（_HID ACPI0010）一起使用。另_RDI
_MLS   6.1.7          强烈推荐用于国际化
_OFF   7.2.2          建议为任何可以打开或关闭的设备定义此方法
_ON    7.2.3          建议为任何可以打开或关闭的设备定义此方法
\_OS   5.7.3          默认情况下此方法返回 "Linux"（这Linux 上宏 ACPI_OS_NAME                       值）。命令行参数 acpi_os=<string> 可用于将其设置为其他值
_OSC   6.2.11         此方法可以是 ACPI 中的全局方法（即 \_SB._OSC），也可以与特定设备关联
                      （例\_SB.DEV0._OSC），或两者兼具。当用作全局方法时，只允许使ACPI
                      规范中发布的能力。当用作设备特定方法时，必须使用为使_DSD 所描述                      过程来创_OSC 定义；不允许进程外使_OSC。也就是说，将设备特定的
                      _OSC 用法描述作为内核驱动提交的一部分提交，获得内核社区批准，然后                      UEFI Forum 注册
\_OSI  5.7.2          ARM64 上已废弃。就 ACPI 固件而言，_OSI 不应用于确定正在使用何种
		      系统或提供何种功能。应使用 _OSC 方法代替
_PDC   8.4.1          已废弃，不要arm64 上使用
\_PIC  5.8.1          不应使用此方法。在 arm64 上，唯一可用的中断模型是 GIC
\_PR   5.3.1          此命名空间仅用于传统系统上的 x86。不要在 arm64 上使用它
_PRT   6.2.13         作为所PCI 根设备定义的一部分是必需的
_PRx   7.3.8-11       按需使用；特定于电源管理。如果定义了 _PR0，则也必须定_PR3
_PSx   7.3.2-5        按需使用；特定于电源管理。如果定义了 _PS0，则也必须定_PS3。如                      时钟或调节器需要调整以与功耗一致，请在这些方法中更改它们
_RDI   8.4.4.4        推荐arm64 上与处理器定义（_HID ACPI0010）一起使用。这只应		      _LPI 配合使用
\_REV  5.7.4          始终返回所支持的最ACPI 版本
\_SB   5.3.1          arm64 上是必需的；所有设备都必须在此命名空间中定义
_SLI   6.2.15         当使SLIT 表时建议使用
_STA   6.3.7,         建议为任何可以打开或关闭的设备定义此方法。另STAO 表，它提供在
       7.2.4          虚拟化环境中隐藏设备的覆盖
_SRS   6.2.16         按需使用；另_PRS
_STR   6.1.10         推荐用于向最终用户传达设备名称；这优于使_DDN
_SUB   6.1.9          按需使用；优先使_HID _CID
_SUN   6.1.11         按需使用，但建议使用
_SWS   7.4.3          按需使用；特定于电源管理；在 arm64 上使用可能需要规范变更
_UID   6.1.12         推荐用于区分同一类的设备；尽可能定义它===== ================ ========================================================




### ACPI 事件模型

不要使用 GPE 块设备；这些arm64 使用的硬件精简配置中不受支持。由于在 ARM 平台上未定义用于
使用GPE 块，ACPI 事件必须以不同方式发出信号
有两种选择：GPIO 信号中断（第 5.6.5 节）和中断信号事件（5.6.9 节）。中断信号事件是 ACPI 6.1
规范中的新特性。在给定的平台上可以使用其中一种或两种；使用哪种可能取决于特定 SoC 的限制。如可能，建议使用中断信号事件

### ACPI 处理器控
ACPI 规范8 节在 6.0 版本中发生了重大变化。处理器现在应定义为带有 _HID ACPI0007 Device
对象；不要使ASL 中已废弃Processor 语句。所有多处理器系统还应使用处理器容器设备
（见8.4.3.1 节，_HID ACPI0010）定义处理器的层次结构；不要使用处理器聚合器设备（第 8.5 节）
来描述处理器拓扑。规范第 8.4 节描述了这些对象定义的语义以及它们如何相互关联
最重要的是，所定义的处理器层次结构还定义了平台可用的低功耗空闲状态，以及确定哪些处理器可打开或关闭及其控制条件的规则。没有这些信息，处理器将运行UEFI 将其留在的任意电源状态中
还要注意，所定义的处理器 Device 对象MADT GIC 的条目应当同步。Device 对象_UID 必须
对应MADT 中使用的处理ID
建议arm64 上使CPPC.4.5）作为处理器性能控制的主要模型。C-states P-states 可能将来的某个时候变得可用，但目前大多数设计工作似乎倾向CPPC
此外，ARMv8 SoC 必须提供功能完整PSCI 实现；这将是 ACPI 支持的用于控CPU 电源状态的唯一
机制。使ACPI parking 协议启动辅助 CPU 是可能的，但不推荐，因为 ARM 服务器仅支持 PSCI

### ACPI 系统地址映射接口

ACPI 规范15 节中，提到了几种作为向内核传递内存资源信息的可能机制的方法。对arm64我们将只支持使用 UEFI 通过 ACPI 启动，因UEFI GetMemoryMap() 启动服务将是唯一使用的机制

### ACPI 平台错误接口（APEI
上面已描述所支持APEI 表
APEI ARMv8 上需要等同于 SCI NMI 的机制。SCI 用于通知 OSPM 已经发生但可以纠正的错误，系可以继续正确运行，即使可能有所降级。NMI 用于指示无法纠正的致命错误，需要立即处理
由于没有直接等同x86 SCI NMI 的机制，arm64 的处理方式略有不同。SCI 作为高优先级中断处理鉴于报告的是已纠正（或可纠正）的错误，这已经足够。NMI 被模拟为可能的最高优先级中断。这意味着
必须保持一定谨慎，因为可能存在更高特权级的中断，甚至存在与模拟 NMI 同优先级的中断。在 Linux 中，
不应出现这种情况，但应意识到它可能发生

### ARM64 上不支持ACPI 对象

虽然这在将来可能改变，但有几类对象可以定义，但目前对 ARM 服务器没有普遍意义。其中一些对象有 x86
对应物，并且可能ARM 服务器上确实有道理。但是，目前要么没有可用的硬件，要么甚至可能还没有非
ARM 的实现。因此，目前不支持它们
以下类别的对象不受支持：

       - 9.2 节：环境光传感器设备

       - 9.3 节：电池设备

       - 9.4 节：盖（例如笔记本盖
       - 绗?9.8.2 鑺傦細IDE 鎺у埗鍣。
       - 9.9 节：软盘控制
       - 9.10 节：GPE 块设
       - 9.15 节：PC/AT RTC/CMOS 设备

       - 9.16 节：用户存在检测设
       - 9.17 节：I/O APIC 设备；所GIC 都必须可通过 MADT 枚举

       - 9.18 节：时间和闹钟设备（9.15
       - 10 节：电源和功率计设备

       - 11 节：热管
       - 12 节：嵌入式控制器接口

       - 13 节：SMBus 接口


这也意味着以下对象不受支持
====   =========================== ====   ==========
Name   Section                     Name   Section
====   =========================== ====   ==========
_ALC   9.3.4                       _FDM   9.10.3
_ALI   9.3.2                       _FIX   6.2.7
_ALP   9.3.6                       _GAI   10.4.5
_ALR   9.3.5                       _GHL   10.4.7
_ALT   9.3.3                       _GTM   9.9.2.1.1
_BCT   10.2.2.10                   _LID   9.5.1
_BDN   6.5.3                       _PAI   10.4.4
_BIF   10.2.2.1                    _PCL   10.3.2
_BIX   10.2.2.1                    _PIF   10.3.3
_BLT   9.2.3                       _PMC   10.4.1
_BMA   10.2.2.4                    _PMD   10.4.8
_BMC   10.2.2.12                   _PMM   10.4.3
_BMD   10.2.2.11                   _PRL   10.3.4
_BMS   10.2.2.5                    _PSR   10.3.1
_BST   10.2.2.6                    _PTP   10.4.2
_BTH   10.2.2.7                    _SBS   10.1.3
_BTM   10.2.2.9                    _SHL   10.4.6
_BTP   10.2.2.8                    _STM   9.9.2.1.1
_DCK   6.5.2                       _UPD   9.16.1
_EC    12.12                       _UPP   9.16.2
_FDE   9.10.1                      _WPC   10.5.2
_FDI   9.10.2                      _WPP   10.5.3
====   =========================== ====   ==========
