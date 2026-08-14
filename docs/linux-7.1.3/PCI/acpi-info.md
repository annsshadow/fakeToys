
## PCI 主机桥的 ACPI 考虑事项


一般规则是，除非操作系统有其他途径可以发现，否则 ACPI 命名空间应当描述
操作系统可能用到的一切内容 [1, 2]。

例如，枚举 PCI 主机桥并没有标准的硬件机制，因此 ACPI 命名空间必须描述
每个主机桥、访问其下方 PCI 配置空间的方法、主机桥转发给 PCI 的地址空间
窗口（使用 _CRS），以及传统 INTx 中断的路由（使用 _PRT）。

位于主机桥下方的 PCI 设备，通常不需要通过 ACPI 来描述。操作系统可以通过
标准的 PCI 枚举机制发现它们，使用配置访问来发现并识别设备，以及读取并
确定它们的 BAR 大小。不过，如果 ACPI 为 PCI 设备提供了电源管理或热插拔
功能，或者该设备由平台中断控制器连接了 INTx 中断、并且需要 _PRT 来描述
这些连接，那么 ACPI 也可以描述 PCI 设备。

ACPI 资源描述是通过 ACPI 命名空间中设备的 _CRS 对象完成的 [^2^]。_CRS 类似
于一种通用化的 PCI BAR：操作系统可以读取 _CRS 并弄清楚正在被消耗的是何种
资源，即便它没有该设备的驱动程序 [^3^]。这一点很重要，因为它意味着即使在一
个存在操作系统不认识的新设备的系统上，旧的操作系统也能正确工作。新设备
可能什么都不做，但操作系统至少可以确保没有资源与它们发生冲突。

像 MCFG、HPET、ECDT 等静态表，并**不是**用于预留地址空间的机制。静态表
用于操作系统在能够解析 ACPI 命名空间之前、在启动早期就需要知道的事情。
如果定义了一个新表，旧的操作系统即便忽略该表，也需要能正确运行。_CRS 使
这一点成为可能，因为它是通用的、且能被旧操作系统理解；而静态表则做不到。

如果操作系统需要管理一个通过 ACPI 描述的非可发现设备，那么该设备会有一个
特定的 _HID/_CID，告诉操作系统应当绑定哪个驱动程序，而 _CRS 告诉操作系统
和驱动程序设备寄存器在哪里。

PCI 主机桥是 PNP0A03 或 PNP0A08 设备。它们的 _CRS 应当描述它们消耗的所有
地址空间。这包括它们向下转发给 PCI 总线的所有窗口，以及主机桥自身那些
不被转发给 PCI 的寄存器。主机桥寄存器包括诸如确定桥下方总线范围的主/从
总线寄存器、描述窗口区间的窗口寄存器等。这些都是设备相关、非架构化的
内容，因此 PNP0A03/PNP0A08 驱动程序管理它们的唯一途径是通过包含设备相关
细节的 _PRS/_CRS/_SRS。主机桥寄存器还包括 ECAM 空间，因为它由主机桥所
消耗。

ACPI 定义了一个 Consumer/Producer 位来区分桥寄存器（“Consumer”）与桥窗口
（“Producer”）[4, 5]，但早期的 BIOS 没有正确使用该位。结果是，当前的 ACPI
规范仅在扩展地址空间描述符中定义了 Consumer/Producer；在较旧的 QWord/DWord/
Word 地址空间描述符中应当忽略该位。因此，操作系统不得不假设所有的 QWord/
DWord/Word 描述符都是窗口。

在加入扩展地址空间描述符之前，Consumer/Producer 的缺失意味着无法在 PNP0A03/
PNP0A08 设备自身中描述桥寄存器。变通办法是在 PNP0C02 兜底设备 [^6^] 中描述
桥寄存器（包括 ECAM 空间）。除 ECAM 外，桥寄存器空间本身就是设备相关的，
因此通用的 PNP0A03/PNP0A08 驱动程序（pci_root.c）无需知道它。

新的架构应当能够在 PNP0A03 设备中使用 “Consumer” 扩展地址空间描述符来描述
桥寄存器（包括 ECAM），尽管对 [^6^] 的严格解释可能禁止这样做。旧的 x86 和
ia64 内核假设所有地址空间描述符（包括 “Consumer” 扩展地址空间描述符）都是
窗口，因此在那些架构上以这种方式描述桥寄存器并不安全。

PNP0C02 “主板”设备基本上是一个兜底设备。除了“不要把这类资源用于其他任何
用途”之外，对它们没有别的编程模型。因此 PNP0C02 的 _CRS 应当声明任何满足
以下条件的地址空间：(1) 未被 ACPI 命名空间中任何其他设备对象的 _CRS 所声明，
且 (2) 不应由操作系统分配给别的东西。

PCIe 规范要求使用增强配置访问方法（ECAM），除非存在标准的固件接口用于
配置访问，例如 ia64 的 SAL 接口 [^7^]。主机桥消耗 ECAM 内存地址空间，并将
内存访问转换为 PCI 配置访问。规范定义了 ECAM 地址空间的布局和功能；只有
地址空间的基地址是设备相关的。ACPI 操作系统从静态 MCFG 表或 PNP0A03 设备
中的 _CBA 方法得知基地址。

MCFG 表必须描述非热插拔主机桥的 ECAM 空间 [^8^]。由于 MCFG 是静态表、无法
通过热插拔更新，因此 PNP0A03 设备中的 _CBA 方法描述热插拔主机桥的 ECAM 空间
[^9^]。注意，对于 MCFG 和 _CBA，基地址始终对应总线 0，即便桥下方的总线范围
（通过 _CRS 报告）不是从 0 开始。


[^1^] ACPI 6.2，第 6.1 节：
    For any device that is on a non-enumerable type of bus (for example, an
    ISA bus), OSPM enumerates the devices' identifier(s) and the ACPI
    system firmware must supply an _HID object ... for each device to
    enable OSPM to do that.

[^2^] ACPI 6.2，第 3.7 节：
    The OS enumerates motherboard devices simply by reading through the
    ACPI Namespace looking for devices with hardware IDs.

    Each device enumerated by ACPI includes ACPI-defined objects in the
    ACPI Namespace that report the hardware resources the device could
    occupy [_PRS], an object that reports the resources that are currently
    used by the device [_CRS], and objects for configuring those resources
    [_SRS].  The information is used by the Plug and Play OS (OSPM) to
    configure the devices.

[^3^] ACPI 6.2，第 6.2 节：
    OSPM uses device configuration objects to configure hardware resources
    for devices enumerated via ACPI.  Device configuration objects provide
    information about current and possible resource requirements, the
    relationship between shared resources, and methods for configuring
    hardware resources.

    When OSPM enumerates a device, it calls _PRS to determine the resource
    requirements of the device.  It may also call _CRS to find the current
    resource settings for the device.  Using this information, the Plug and
    Play system determines what resources the device should consume and
    sets those resources by calling the device’s _SRS control method.

    In ACPI, devices can consume resources (for example, legacy keyboards),
    provide resources (for example, a proprietary PCI bridge), or do both.
    Unless otherwise specified, resources for a device are assumed to be
    taken from the nearest matching resource above the device in the device
    hierarchy.

[^4^] ACPI 6.2，第 6.4.3.5.1, 2, 3, 4 节：
    QWord/DWord/Word Address Space Descriptor (.1, .2, .3)
      General Flags: Bit [^0^] Ignored

    Extended Address Space Descriptor (.4)
      General Flags: Bit [^0^] Consumer/Producer:

        - 1 – This device consumes this resource
        - 0 – This device produces and consumes this resource

[^5^] ACPI 6.2，第 19.6.43 节：
    ResourceUsage specifies whether the Memory range is consumed by
    this device (ResourceConsumer) or passed on to child devices
    (ResourceProducer).  If nothing is specified, then
    ResourceConsumer is assumed.

[^6^] PCI Firmware 3.2，第 4.1.2 节：
    If the operating system does not natively comprehend reserving the
    MMCFG region, the MMCFG region must be reserved by firmware.  The
    address range reported in the MCFG table or by _CBA method (see Section
    4.1.3) must be reserved by declaring a motherboard resource.  For most
    systems, the motherboard resource would appear at the root of the ACPI
    namespace (under \_SB) in a node with a _HID of EISAID (PNP0C02), and
    the resources in this case should not be claimed in the root PCI bus’s
    _CRS.  The resources can optionally be returned in Int15 E820 or
    EFIGetMemoryMap as reserved memory but must always be reported through
    ACPI as a motherboard resource.

[^7^] PCI Express 4.0，第 7.2.2 节：
    For systems that are PC-compatible, or that do not implement a
    processor-architecture-specific firmware interface standard that allows
    access to the Configuration Space, the ECAM is required as defined in
    this section.

[^8^] PCI Firmware 3.2，第 4.1.2 节：
    The MCFG table is an ACPI table that is used to communicate the base
    addresses corresponding to the non-hot removable PCI Segment Groups
    range within a PCI Segment Group available to the operating system at
    boot. This is required for the PC-compatible systems.

    The MCFG table is only used to communicate the base addresses
    corresponding to the PCI Segment Groups available to the system at
    boot.

[^9^] PCI Firmware 3.2，第 4.1.3 节：
    The _CBA (Memory mapped Configuration Base Address) control method is
    an optional ACPI object that returns the 64-bit memory mapped
    configuration base address for the hot plug capable host bridge. The
    base address returned by _CBA is processor-relative address. The _CBA
    control method evaluates to an Integer.

    This control method appears under a host bridge object. When the _CBA
    method appears under an active host bridge object, the operating system
    evaluates this structure to identify the memory mapped configuration
    base address corresponding to the PCI Segment Group for the bus number
    range specified in _CRS method. An ACPI name space object that contains
    the _CBA method must also contain a corresponding _SEG method.
