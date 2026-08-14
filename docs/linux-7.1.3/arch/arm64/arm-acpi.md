## Arm 系统上的 ACPI


ACPI 可用于遵循 BSA（Arm Base System Architecture，Arm 基础系统架构）[^0^] 与 BBR（Arm Base Boot Requirements，Arm 基础启动要求）[^1^] 规范设计的 Armv8 和 Armv9 系统。BSA 与 BBR 均为公开可获取的文档。除符合 BSA 外，Arm 服务器还需遵循 SBSA（Server Base System Architecture，服务器基础系统架构）[^2^] 中定义的一组规则。

Arm 内核实现了 ACPI 5.1 或更高版本的精简硬件模型（reduced hardware model）。该规范及其引用的所有外部文档的链接均由 UEFI Forum 管理。规范可从 http://www.uefi.org/specifications 获取，规范引用的文档可通过 http://www.uefi.org/acpi 找到。

如果某个 Arm 系统不满足 BSA 与 BBR 的要求，或者无法用所需 ACPI 规范中定义的机制来描述，那么 ACPI 可能并不适合该硬件。

尽管上述文档规定了构建行业标准的 Arm 系统的要求，它们同样适用于不止一个操作系统。本文档的目的仅在于描述在 Arm 系统上 ACPI 与 Linux 之间的交互——也就是说，Linux 对 ACPI 有何期望，以及 ACPI 对 Linux 有何期望。


### 为何在 Arm 上使用 ACPI？

在考察 ACPI 与 Linux 之间接口的诸多细节之前，先理解为何要使用 ACPI 是有益的。毕竟，Linux 中早已存在多种用于描述不可枚举（non-enumerable）硬件的技术。本节我们概括了 Grant Likely 的一篇博客文章 [^3^]，其中概述了在 Arm 系统上使用 ACPI 的缘由。老实说，我们几乎直接摘录了其中的大部分总结文字。

在 Arm 上使用 ACPI 的简要理由如下：

- ACPI 的字节码（AML）允许平台对硬件行为进行编码，而 DT 明确不支持这一点。对硬件厂商而言，能够对行为编码是支持新硬件上操作系统发布的关键工具。

- ACPI 的 OSPM 定义了一种电源管理模型，将平台被允许执行的操作约束到特定的模型中，同时仍为硬件设计保留灵活性。

- 在企业服务器环境中，ACPI 已经建立了一套绑定（例如用于 RAS），目前已在生产系统中使用。DT 没有。这类绑定或许将来可以在 DT 中定义，但那样做意味着 Arm 与 x86 最终将不得不在固件和内核中都使用完全不同的代码路径。

- 选择单一接口来描述平台与操作系统之间的抽象是很重要的。若硬件厂商希望支持多个操作系统，他们将不必同时实现 DT 和 ACPI。而且，就单一接口达成一致、而不是各自分裂成每个操作系统一套接口，整体上会带来更好的互操作性。

- 新的 ACPI 治理流程运作良好，如今 Linux 与硬件厂商及其他操作系统厂商坐在同一张谈判桌上。事实上，已没有任何理由认为 ACPI 只属于 Windows，或认为 Linux 在这一领域从任何意义上都低于 Microsoft。ACPI 治理权移交给 UEFI Forum 极大地开放了规范的开发流程，目前对 ACPI 所做的大量修改正是由 Linux 推动的。

使用 ACPI 的关键在于其支持模型。对于服务器总体而言，硬件行为的责任不能仅由内核承担，而必须在平台与内核之间分担，以便能够有序地随时间演进。ACPI 使操作系统无需理解硬件的所有细微细节，从而不必针对每个设备逐一进行移植。它让硬件厂商能够承担电源管理行为的责任，而无需依赖其无法控制的操作系统发布周期。

ACPI 之所以重要，还因为硬件与操作系统厂商已经摸索出了支撑通用计算生态的机制。相关基础设施已就位，绑定已就位，流程也已就位。在处理垂直整合的设备时，DT 恰好完成了 Linux 所需的工作，但并没有良好的流程来支撑服务器厂商的需求。Linux 或许最终能用 DT 做到，但那样做实际上只是重复一个已经可行的东西。ACPI 已经实现了硬件厂商所需的功能，Microsoft 不会在 DT 上合作，而硬件厂商最终仍将不得不提供两套完全独立的固件接口——一套给 Linux，一套给 Windows。


### 内核兼容性

采用 ACPI 的主要动机之一是标准化，并借此为 Linux 内核提供向后兼容性。在服务器市场，软硬件常被长期使用。ACPI 让内核与固件就一个一致的抽象达成一致，该抽象即使硬件或软件发生变化也能长期维护。只要该抽象仍受支持，系统就可以在不必须更换内核的情况下进行更新。

当一个 Linux 驱动或子系统首次基于 ACPI 实现时，它必然需要特定版本的 ACPI 规范——即其基线版本。ACPI 固件必须继续工作，即便可能不是最优的，也要能配合最早开始支持该基线 ACPI 版本的内核版本。可能需要额外的驱动，但添加新功能（例如 CPU 电源管理）不应破坏旧内核版本。此外，ACPI 固件还必须能配合最新版本的内核工作。


### 与 Device Tree 的关系

在 Arm 的驱动与子系统中，ACPI 支持与 DT 支持在编译时绝不应当互斥。

在启动时，内核只会根据引导加载程序（包括内核 bootargs）传入的参数使用一种描述方法。

无论使用 DT 还是 ACPI，内核必须始终能够用这两种方案中的任意一种启动（在编译时同时启用两种方案的内核中）。


### 使用 ACPI 表启动

在 Arm 上向内核传递 ACPI 表的唯一定义方式是通过 UEFI 系统配置表。明确地说，这意味着 ACPI 仅在使用 UEFI 启动的平台上受支持。

当 Arm 系统启动时，它可能有 DT 信息、ACPI 表，或者在极少数情况下两者都有。如果不使用任何命令行参数，内核将尝试使用 DT 进行设备枚举；如果没有 DT，内核将尝试使用 ACPI 表，但仅当它们存在时。如果两者都不可用，内核将无法启动。如果在命令行使用 acpi=force，内核将首先尝试使用 ACPI 表，但如果没有 ACPI 表存在则回退到 DT。其基本理念是：除非确实别无选择，内核不会启动失败。

通过在内核命令行传入 acpi=off 可以禁用 ACPI 表的处理；这是默认行为。

为了让内核加载并使用 ACPI 表，UEFI 实现必须设置 ACPI_20_TABLE_GUID 指向 RSDP 表（带有 ACPI 签名 "RSD PTR " 的表）。如果该指针不正确且使用了 acpi=force，内核将禁用 ACPI 并尝试改用 DT 启动；实际上，内核此时已判定 ACPI 表不存在。

如果指向 RSDP 表的指针正确，ACPI 核心将使用 UEFI 提供的地址将该表映射到内核中。

随后，ACPI 核心会利用 RSDP 表中的地址找到 XSDT（eXtended System Description Table，扩展系统描述表），进而定位并映射所有其他 ACPI 表。XSDT 反过来提供了系统固件提供的所有其他 ACPI 表的地址；ACPI 核心随后会遍历该表并映射其中列出的表。

ACPI 核心会忽略任何提供的 RSDT（Root System Description Table，根系统描述表）。RSDT 已被弃用，在 arm64 上会被忽略，因为它们只支持 32 位地址。

此外，ACPI 核心只会使用 FADT（Fixed ACPI Description Table，固定 ACPI 描述表）中的 64 位地址字段。FADT 中的任何 32 位地址字段在 arm64 上都会被忽略。

硬件精简模式（hardware reduced mode，参见 ACPI 6.1 规范第 4.1 节）将由 ACPI 核心在 arm64 上强制启用。这样做可以让 ACPI 核心运行更简单的代码，因为它不再需要为其他架构的旧硬件提供支持。任何不用于硬件精简模式的字段都必须设置为零。

为了让 ACPI 核心正确运行，进而提供内核配置设备所需的信息，它需要找到以下表（所有章节号均指 ACPI 6.5 规范）：

    - RSDP（Root System Description Pointer，根系统描述指针），第 5.2.5 节

    - XSDT（eXtended System Description Table，扩展系统描述表），第 5.2.8 节

    - FADT（Fixed ACPI Description Table，固定 ACPI 描述表），第 5.2.9 节

    - DSDT（Differentiated System Description Table，差异化系统描述表），第
       5.2.11.1 节

    - MADT（Multiple APIC Description Table，多 APIC 描述表），第 5.2.12 节

    - GTDT（Generic Timer Description Table，通用定时器描述表），第 5.2.24 节

    - PPTT（Processor Properties Topology Table，处理器属性拓扑表），第 5.2.30 节

    - DBG2（DeBuG port table 2，调试端口表 2），第 5.2.6 节，具体为表 5-6。

    - APMT（Arm Performance Monitoring unit Table，Arm 性能监控单元表），第 5.2.6 节，具体为表 5-6。

    - AGDI（Arm Generic diagnostic Dump and Reset Device Interface Table，Arm 通用诊断转储与复位设备接口表），第 5.2.6 节，具体为表 5-6。

    - 如果支持 PCI，则 MCFG（Memory mapped ConFiGuration Table，内存映射配置表），第 5.2.6 节，具体为表 5-6。

    - 如果支持在不带 console=<device> 内核参数的情况下启动，则 SPCR（Serial Port Console Redirection table，串口控制台重定向表），第 5.2.6 节，具体为表 5-6。

    - 如果为描述 I/O 拓扑、SMMUs 和 GIC ITSs 所必需，则 IORT（Input Output Remapping Table，输入输出重映射表），第 5.2.6 节，具体为表 5-6。

    - 如果支持 NUMA，则还需要以下表：

       - SRAT（System Resource Affinity Table，系统资源亲和性表），第 5.2.16 节

       - SLIT（System Locality distance Information Table，系统局部距离信息表），第 5.2.17 节

    - 如果支持 NUMA，且系统包含异构内存，则 HMAT（Heterogeneous Memory Attribute Table，异构内存属性表），第 5.2.28 节。

    - 如果需要 ACPI Platform Error Interfaces，则以下条件性地需要以下表：

       - BERT（Boot Error Record Table，启动错误记录表），第 18.3.1 节

       - EINJ（Error INJection table，错误注入表），第 18.6.1 节

       - ERST（Error Record Serialization Table，错误记录序列化表），第 18.5 节

       - HEST（Hardware Error Source Table，硬件错误源表），第 18.3.2 节

       - SDEI（Software Delegated Exception Interface table，软件委托异常接口表），第 5.2.6 节，具体为表 5-6

       - AEST（Arm Error Source Table，Arm 错误源表），第 5.2.6 节，具体为表 5-6

       - RAS2（ACPI RAS2 feature table，ACPI RAS2 特性表），第 5.2.21 节

    - 如果系统包含使用 PCC 通道的控制器，则 PCCT（Platform Communications Channel Table，平台通信通道表），第 14.1 节

    - 如果系统包含用于捕获板级系统状态、并通过 PCC 与主机通信的控制器，则 PDTT（Platform Debug Trigger Table，平台调试触发表），第 5.2.29 节。

    - 如果支持 NVDIMM，则 NFIT（NVDIMM Firmware Interface Table，NVDIMM 固件接口表），第 5.2.26 节

    - 如果存在视频帧缓冲，则 BGRT（Boot Graphics Resource Table，启动图形资源表），第 5.2.23 节

    - 如果实现了 IPMI，则 SPMI（Server Platform Management Interface，服务器平台管理接口），第 5.2.6 节，具体为表 5-6。

    - 如果系统包含 CXL Host Bridge，则 CEDT（CXL Early Discovery Table，CXL 早期发现表），第 5.2.6 节，具体为表 5-6。

    - 如果系统支持 MPAM，则 MPAM（Memory Partitioning And Monitoring table，内存分区与监视表），第 5.2.6 节，具体为表 5-6。

    - 如果系统缺少持久化存储，则 IBFT（ISCSI Boot Firmware Table，iSCSI 启动固件表），第 5.2.6 节，具体为表 5-6。


如果上述表未全部存在，内核可能能够也可能无法正常启动，因为它可能无法配置所有可用设备。此表清单并非旨在包罗万象；在某些环境中，可能需要其他表（例如第 18 节中的任何 APEI 表）来支持特定功能。


### ACPI 检测

驱动应通过检查 ACPI_HANDLE 是否为空值，或检查 .of_node，或检查设备结构体中的其他信息，来确定其 probe() 类型。这一点在“驱动建议”一节中有更详细的说明。

在非驱动代码中，如果需要在运行时检测 ACPI 是否存在，则检查 acpi_disabled 的值。如果未设置 CONFIG_ACPI，acpi_disabled 将始终为 1。


### 设备枚举

ACPI 中的设备描述应使用标准且被认可的 ACPI 接口。这些描述所包含的信息可能少于通常通过 Device Tree 为同一设备提供的信息。这也是 ACPI 之所以有用的原因之一——驱动会考虑到它可能拥有关于该设备的较少详细信息，转而使用合理的默认值。如果在驱动中妥善处理，硬件可以随着时间的推移而变更和改进，而驱动完全无需改变。

时钟就是一个极好的例子。在 DT 中，时钟需要被显式指定，驱动也需要将其考虑在内。在 ACPI 中，假设是 UEFI 会将设备留在合理的默认状态，包括任何时钟设置。如果由于某种原因驱动需要改变某个时钟值，这可以通过某个 ACPI method 来完成；驱动只需调用该方法，而无需关心该方法为了改变时钟需要做什么。如此一来，硬件的变更就可以随着时间推移通过改变 ACPI method 的行为来实现，而不是改变驱动。

在 DT 中，驱动为设置上述时钟所需的参数被称为“bindings（绑定）”；在 ACPI 中，这些被称为“Device Properties（设备属性）”，并通过 _DSD 对象提供给驱动。

ACPI 表用一种称为 ASL（ACPI Source Language，ACPI 源语言，规范第 19 节）的形式化语言来描述。这意味着总有多种方式来描述同一事物——包括设备属性。例如，设备属性可以使用如下形式的 ASL 构造：Name(KEY0, "value0")。随后，ACPI 设备驱动会通过求值 KEY0 对象来获取该属性的值。然而，以这种方式使用 Name() 存在多个问题：(1) 与 DT 不同，ACPI 将名称（"KEY0"）限制为四个字符；(2) 没有业界范围的注册表来维护名称列表，难以复用；(3) 同样也没有针对属性值（"value0"）定义的注册表，同样使复用变得困难；(4) 当新硬件出现时，如何保持向后兼容性？_DSD 方法正是为解决此类问题而创建的；Linux 驱动应始终对设备属性使用 _DSD 方法，而不使用其他任何方法。

_DSM 对象（ACPI 第 9.14.1 节）也可用于向驱动传递设备属性。Linux 驱动应仅在 _DSD 无法表示所需数据、且无法为 _DSD 对象创建新 UUID 时才期望使用它。注意，对 _DSM 使用的规范甚至少于对 _DSD 的规范。正因如此，依赖 _DSM 对象内容的驱动在将来会更难维护；在撰写本文时，_DSM 的使用正是相当多固件问题的成因，因此不建议使用。

驱动应仅在 _DSD 对象中查找设备属性；_DSD 对象在 ACPI 规范第 6.2.5 节中描述，但该节仅描述了如何定义通过 _DSD 返回的对象的结构，以及特定数据结构如何由特定 UUID 定义。Linux 应只使用 _DSD Device Properties UUID [^4^]：

   - UUID: daffd814-6eba-4d8c-8a91-bc9bbf4aa301

常见的设备属性可以通过向 [^4^] 创建 pull request 来注册，以便它们能在所有支持 ACPI 的操作系统中使用。未向 UEFI Forum 注册的设备属性也可以使用，但不能作为 "uefi-" 公共属性。

在创建新的设备属性之前，请先确认它们此前未被定义，也未被注册到 Linux 内核文档中作为 DT 绑定，或注册到 UEFI Forum 作为设备属性。虽然我们并不想简单地把所有的 DT 绑定都搬进 ACPI 设备属性，但我们可以从之前定义的内容中学习。

如果必须定义新的设备属性，或者将某个绑定的定义综合整理使其在任意固件中都能使用是有意义的，那么 DT 绑定和 ACPI 设备属性针对设备驱动都有各自的审查流程。两者都要使用。当驱动本身提交到 Linux 邮件列表审查时，所需的设备属性定义必须同时提交。一个支持 ACPI 并使用设备属性的驱动，若无其定义，将不被视为完整。一旦设备属性被 Linux 社区接受，就必须向 UEFI Forum [^4^] 注册，后者会再次审查其在注册表中的一致性。这可能需要反复迭代。不过，UEFI Forum 始终是设备属性定义的权威站点。

向 UEFI Forum 发出通知，表明有意注册一个此前未使用的设备属性名称，以此作为为将来使用保留该名称的手段，这种做法或许是有意义的。其他操作系统厂商也会提交注册请求，这可能有助于让流程更加顺畅。

一旦注册与审查完成，内核会提供一个接口，以与 DT 或 ACPI 是否正在使用无关的方式来查找设备属性。应当使用此 API [^5^]；它可以消除驱动探测函数中一些代码路径的重复，并抑制 DT 绑定与 ACPI 设备属性之间的分化。


### 可编程电源控制资源

可编程电源控制资源包括诸如电压/电流提供方（regulators，稳压器）和时钟源等资源。

在使用 ACPI 时，内核的 clock 与 regulator 框架预期完全不会被使用。

内核假设这些资源的电源控制由 Power Resource Objects（电源资源对象，ACPI 第 7.1 节）表示。随后 ACPI 核心会正确处理在需要时对资源的启用与禁用。为了使之工作，ACPI 假设每个设备都定义了 D-states，并且可以通过可选的 ACPI 方法 _PS0、_PS1、_PS2 和 _PS3 来控制；在 ACPI 中，_PS0 是使设备完全开启所调用的方法，而 _PS3 是使设备完全关闭所调用的方法。

使用这些 Power Resources 有两种选择。它们可以：

   - 在 _PSx 方法中管理，该方法在进入电源状态 Dx 时被调用。

   - 作为独立的电源资源声明，并拥有各自的 _ON 和 _OFF 方法。随后它们通过 _PRx 关联回特定设备的 D-states，_PRx 指定设备在 Dx 状态下需要保持开启的电源资源。内核随后跟踪使用某个电源资源的设备数量，并按需调用 _ON/_OFF。

内核 ACPI 代码还会假设 _PSx 方法遵循此类方法的正常 ACPI 规则：

   - 如果实现了 _PS0 或 _PS3 中的任意一个，那么另一个方法也必须被实现。

   - 如果设备在开启时需要用到或设置某个电源资源，ASL 应当确保在 _PS0 方法中分配/启用它。

   - 在 _PS0 方法中分配或启用的资源，应在 _PS3 方法中被禁用或释放。

   - 固件在将控制权交给内核之前，会将资源保持在合理的状态。

当然，_PSx 方法中的这类代码会非常平台相关。但是，这可以让驱动将操作设备的接口抽象出来，避免必须从 ACPI 表中读取特殊的非标准值。此外，将这些资源的使用抽象化，可以让硬件随时间改变而无需更新驱动。


### 时钟

ACPI 假设时钟在控制权移交给内核之前，已由固件（在此即 UEFI）初始化为某个可用的值。这对于诸如 UART 或 SoC 驱动的 LCD 显示屏等设备具有影响。

内核启动时，假设时钟已被设置为合理的可用值。如果由于某种原因需要改变频率——例如为了电源管理而节流——设备驱动应预期该过程被抽象到某个可被调用的 ACPI method 中（关于期望使用的标准方法的进一步建议，请参阅 ACPI 规范）。唯一的例外是 CPU 时钟，其中 CPPC 提供了比 ACPI 方法丰富得多的接口。如果时钟未被设置，Linux 没有直接的方式来控制它们。

如果某个 SoC 厂商希望提供对系统时钟的细粒度控制，他们可以通过提供可被 Linux 驱动调用的 ACPI method 来实现。然而，这并不被推荐，Linux 驱动不应使用此类方法，即使它们被提供了。此类方法目前在 ACPI 规范中尚未标准化，使用它们可能把内核绑定到某个非常特定的 SoC，或者把 SoC 绑定到某个非常特定的内核版本，而这二者都是我们试图避免的。


### 驱动建议

在为驱动添加 ACPI 支持时，不要移除任何 DT 处理代码。同一个设备可能用在许多不同的系统上。

请尽量将驱动构建为数据驱动（data-driven）的形式。也就是说，基于默认值以及驱动 probe 函数必须发现的其他内容，建立一个包含每个设备内部状态的 struct。然后让驱动的其余部分根据该 struct 的内容运作。这样做应能使 ACPI 与 DT 功能之间的大部分差异保持在 probe 函数局部，而不是散布于整个驱动。对于

```
  static int device_probe_dt(struct platform_device *pdev)
  {
         /* DT specific functionality */
         ...
  }

  static int device_probe_acpi(struct platform_device *pdev)
  {
         /* ACPI specific functionality */
         ...
  }

  static int device_probe(struct platform_device *pdev)
  {
         ...
         struct device_node node = pdev->dev.of_node;
         ...

         if (node)
                 ret = device_probe_dt(pdev);
         else if (ACPI_HANDLE(&pdev->dev))
                 ret = device_probe_acpi(pdev);
         else
                 /* other initialization */
                 ...
         /* Continue with any generic probe operations */
         ...
  }
```

请将 MODULE_DEVICE_TABLE 条目保留在驱动一起，以清楚地表明驱动针对 DT 和

```
  static struct of_device_id virtio_mmio_match[] = {
          { .compatible = "virtio,mmio", },
          { }
  };
  MODULE_DEVICE_TABLE(of, virtio_mmio_match);

  static const struct acpi_device_id virtio_mmio_acpi_match[] = {
          { "LNRO0005", },
          { }
  };
  MODULE_DEVICE_TABLE(acpi, virtio_mmio_acpi_match);


```

### ASWG

ACPI 规范会定期变更。例如，在 2014 年期间，发布了 5.1 版本，并基本完成了 6.0 版本，其中大部分变更由 Arm 特定的需求推动。提案中的变更在 ASWG（ACPI Specification Working Group，ACPI 规范工作组）中展示和讨论，该工作组是 UEFI Forum 的一部分。当前版本的 ACPI 规范是 2022 年 8 月发布的 6.5。

该小组对所有 UEFI 成员开放参与。有关小组成员的详细信息，请参阅 http://www.uefi.org/workinggroup。

Arm ACPI 内核代码的意图是尽可能严格遵循 ACPI 规范，并且只实现符合 UEFI ASWG 已发布标准的功能。实际上，总会有厂商提供糟糕的 ACPI 表或以某种方式违反标准。如果这是因错误所致，可能必要的变通与修补（quirks and fix-ups）将会被采用，但如有可能会加以避免。如果 ACPI 缺少某些特性以至于无法在某个平台上使用，应向 ASWG 提交 ECRs（Engineering Change Requests，工程变更请求），并走正常的审批流程；对于那些不是 UEFI 成员的人，Linux 社区的许多其他成员是成员，并且很可能愿意协助提交 ECRs。


### Linux 代码

内置于 Linux 源码中、特定于 Arm 上 Linux 的个别条目列于下文中：

ACPI_OS_NAME
                       This macro defines the string to be returned when
                       an ACPI method invokes the _OS method.  On Arm
                       systems, this macro will be "Linux" by default.
                       The command line parameter acpi_os=<string>
                       can be used to set it to some other value.  The
                       default value for other architectures is "Microsoft
                       Windows NT", for example.


### ACPI 对象

有关 ACPI 表与对象的详细期望，列于文件 Documentation/arch/arm64/acpi_object_usage.rst 中。


### 参考资料

[^0^] https://developer.arm.com/documentation/den0094/latest
    document Arm-DEN-0094: "Arm Base System Architecture", version 1.0C, dated 6 Oct 2022

[^1^] https://developer.arm.com/documentation/den0044/latest
    Document Arm-DEN-0044: "Arm Base Boot Requirements", version 2.0G, dated 15 Apr 2022

[^2^] https://developer.arm.com/documentation/den0029/latest
    Document Arm-DEN-0029: "Arm Server Base System Architecture", version 7.1, dated 06 Oct 2022

[^3^] http://www.secretlab.ca/archives/151,
    10 Jan 2015, Copyright (c) 2015,
    Linaro Ltd., written by Grant Likely.

[^4^] _DSD（Device Specific Data，设备特定数据）实现指南
    https://github.com/UEFI/DSD-Guide/blob/main/dsd-guide.pdf

[^5^] 统一设备属性接口的 Linux 内核代码可在 include/linux/property.h 和 drivers/base/property.c 中找到。


### 作者

- Al Stone <al.stone@linaro.org>
- Graeme Gregory <graeme.gregory@linaro.org>
- Hanjun Guo <hanjun.guo@linaro.org>

- Grant Likely <grant.likely@linaro.org>，负责“为何在 Arm 上使用 ACPI？”一节
