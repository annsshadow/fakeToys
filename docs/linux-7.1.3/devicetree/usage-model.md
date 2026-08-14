## Linux 与设备树（Devicetree）


Linux 对设备树数据的使用模型

:Author: Grant Likely <grant.likely@secretlab.ca>

本文描述了 Linux 如何使用设备树。关于设备树数据格式的概述可以在 devicetree.org 的设备树使用页面\ [^1^]_ 上找到。

"开放固件设备树"（Open Firmware Device Tree），或简称设备树（DT），是一种用于描述硬件的数据结构和语言。更具体地说，它是一种硬件的描述，操作系统可以读取它，这样操作系统就无需对机器的细节进行硬编码。

从结构上看，DT 是一棵树，或者说是一个带有命名节点的有向无环图，节点可以拥有任意数量的命名属性，这些属性封装任意数据。还提供了一种机制，可以在自然树结构之外从一个节点创建到另一个节点的任意链接。

从概念上讲，定义了一组通用的使用约定，称为"bindings"（绑定），用于规定数据应如何出现在树中以描述典型的硬件特征，包括数据总线、中断线、GPIO 连接和外设。

尽可能使用现有绑定来描述硬件，以最大化现有支持代码的复用；但由于属性和节点名只是文本字符串，通过定义新节点和新属性来扩展现有绑定或创建新绑定都很简单。不过要注意，在事先不了解已有内容的情况下创建新绑定。目前存在两种互不相容的 i2c 总线绑定，就是因为在创建新绑定时没有先调查 i2c 设备已如何在现有系统中被枚举而产生的。

### 1. 历史

DT 最初由 Open Firmware 创建，作为从 Open Firmware 向客户端程序（例如操作系统）传递数据的通信方法的一部分。操作系统使用设备树在运行时发现硬件的拓扑结构，从而无需硬编码信息（假设所有设备都有可用的驱动）即可支持大多数可用硬件。

由于 Open Firmware 常用于 PowerPC 和 SPARC 平台，Linux 对这些体系结构的支持长期以来一直使用设备树。

2005 年，当 PowerPC Linux 开始进行重大清理并合并 32 位和 64 位支持时，决定要求所有 powerpc 平台都支持 DT，无论它们是否使用 Open Firmware。为此，创建了称为扁平设备树（Flattened Device Tree，FDT）的 DT 表示形式，它可以作为二进制 blob 传递给内核，而无需真正的 Open Firmware 实现。U-Boot、kexec 和其他引导加载程序被修改，以支持传递设备树二进制文件（dtb）并在启动时修改 dtb。DT 也被添加到 PowerPC 引导包装器（`arch/powerpc/boot/*`）中，以便将 dtb 与内核镜像打包在一起，从而支持引导现有的不感知 DT 的固件。

此后一段时间，FDT 基础设施被泛化，可供所有体系结构使用。在撰写本文时，6 个已主线化的体系结构（arm、microblaze、mips、powerpc、sparc 和 x86）以及 1 个尚未主线化的（nios）都具备某种程度的 DT 支持。

### 2. 数据模型

如果你还没有读过设备树使用\ [^1^]_ 页面，那么现在去读吧。没关系的，我会等你……

### 2.1 高层视角

最重要的一点是要理解，DT 只是一种描述硬件的数据结构。它没什么神奇的，也不会神奇地让所有硬件配置问题消失。它真正做的是提供一种语言，用于将硬件配置与 Linux 内核（或就此而言的任何其他操作系统）中的板级和设备驱动支持解耦。使用它使得板级和设备支持变成数据驱动的；根据传入内核的数据而不是根据逐机器的硬编码选择来做出设置决策。

理想情况下，数据驱动的平台设置应能减少代码重复，并使使用单个内核镜像支持各种硬件变得更加容易。

Linux 将 DT 数据用于三个主要目的：

1) 平台识别，
2) 运行时配置，以及
3) 设备填充（device population）。

### 2.2 平台识别

首先也是最重要的，内核将使用 DT 中的数据来识别具体的机器。在理想的世界中，具体的平台对内核来说应该无关紧要，因为所有平台细节都会以一致可靠的方式被设备树完美描述。但硬件并不完美，因此内核必须在早期启动期间识别机器，以便有机会运行机器特定的修复（fixup）。

在大多数情况下，机器标识是无关的，内核反而会基于机器的核心 CPU 或 SoC 选择设置代码。例如在 ARM 上，`arch/arm/kernel/setup.c` 中的 setup_arch() 会调用 `arch/arm/kernel/devtree.c` 中的 setup_machine_fdt()，后者在 machine_desc 表中搜索，并选出与设备树数据最匹配的 machine_desc。它通过查看根设备树节点中的 'compatible' 属性，并将其与 struct machine_desc 中的 dt_compat 列表（如果好奇的话，定义在 `arch/arm/include/asm/mach/arch.h` 中）进行比较来确定最佳匹配。

'compatible' 属性包含一个排序的字符串列表，以机器的确切名称开头，后跟一组它兼容的板子，按兼容性从高到低排序。例如，TI BeagleBoard 及其

```
	compatible = "ti,omap3-beagleboard", "ti,omap3450", "ti,omap3";
	compatible = "ti,omap3-beagleboard-xm", "ti,omap3450", "ti,omap3";
```

其中 "ti,omap3-beagleboard-xm" 指定了确切的型号，它还声明它与 OMAP 3450 SoC 以及 omap3 SoC 系列兼容。你会注意到该列表是从最具体（确切的板子）到最不具体（SoC 系列）排序的。

敏锐的读者可能会指出，Beagle xM 也可以声明与原始 Beagle 板兼容。然而，应当警惕在板级这样做，因为即使在同一产品线上，从一个板子到另一个板子通常也有很大变化，而且很难确切界定当一个板子声明与另一个板子兼容时到底意味着什么。对于顶层，最好谨慎行事，不要声明一个板子与另一个板子兼容。明显的例外是当一个板子是另一个板子的载板时，例如连接到载板上的 CPU 模块。

关于 compatible 值的另一个说明。任何用于 compatible 属性的字符串都必须记录它指示了什么。请在 Documentation/devicetree/bindings 中为 compatible 字符串添加文档。

同样在 ARM 上，对于每个 machine_desc，内核会查看 dt_compat 列表中的任何条目是否出现在 compatible 属性中。如果出现，那么该 machine_desc 就是驱动该机器的候选。在搜索完整个 machine_descs 表之后，setup_machine_fdt() 根据每个 machine_desc 所匹配的 compatible 属性中的条目，返回"最兼容"的 machine_desc。如果找不到匹配的 machine_desc，则返回 NULL。

此方案背后的理由是观察到，在大多数情况下，如果所有板子都使用相同的 SoC 或同一 SoC 系列，单个 machine_desc 就可以支持大量板子。然而，不可避免地会有一些例外，其中某个特定的板子需要特殊的设置代码，在通用情况下无用。特殊情况可以通过在通用设置代码中显式检查有问题的板子来处理，但如果超过区区几个这样的情况，这样做很快就会变得丑陋和/或难以维护。

相反，compatible 列表允许一个通用的 machine_desc 通过指定 dt_compat 列表中"兼容性较低"的值，来为一组广泛的通用板子提供支持。在上面的例子中，通用板子支持可以声明与 "ti,omap3" 或 "ti,omap3450" 兼容。如果在原始 beagleboard 上发现了需要在早期启动期间进行特殊规避代码的 bug，那么可以添加一个新的 machine_desc，它实现这些规避代码，并且只匹配 "ti,omap3-beagleboard"。

PowerPC 使用略有不同的方案，它调用每个 machine_desc 的 .probe() 钩子，使用第一个返回 TRUE 的那个。然而，这种方法没有考虑 compatible 列表的优先级，对于新的体系结构支持大概应该避免使用。

### 2.3 运行时配置

在大多数情况下，DT 将是固件向内核传递数据的唯一方法，因此它也被用于传递运行时和配置数据，例如内核参数字符串和 initrd 镜像的位置。

大部分数据包含在 /chosen 节点中，在引导时

```
	chosen {
		bootargs = "console=ttyS0,115200 loglevel=8";
		initrd-start = <0xc8000000>;
		initrd-end = <0xc8200000>;
	};
```

bootargs 属性包含内核参数，initrd-* 属性定义了 initrd blob 的地址和大小。注意 initrd-end 是 initrd 镜像之后的第一个地址，因此它不匹配 struct resource 的通常语义。chosen 节点还可以可选地包含任意数量的附加属性，用于平台特定的配置数据。

在早期启动期间，体系结构设置代码会在分页建立之前，多次调用 of_scan_flat_dt()，并配合不同的辅助回调来解析设备树数据。of_scan_flat_dt() 代码扫描设备树，并使用这些辅助函数提取早期启动所需的信息。通常 early_init_dt_scan_chosen() 辅助函数用于解析 chosen 节点（包括内核参数），early_init_dt_scan_root() 用于初始化 DT 地址空间模型，early_init_dt_scan_memory() 用于确定可用 RAM 的大小和位置。

在 ARM 上，函数 setup_machine_fdt() 负责在选择了支持该板的正确的 machine_desc 之后，对设备树进行早期扫描。

### 2.4 设备填充（Device population）

在识别出板子、并解析了早期配置数据之后，内核初始化就可以以正常方式进行。在这个过程中某个时刻，会调用 unflatten_device_tree() 将数据转换为更高效的运行时表示。这也是调用机器特定设置钩子的时机，例如 ARM 上的 machine_desc .init_early()、.init_irq() 和 .init_machine() 钩子。本节其余部分使用 ARM 实现的例子，但所有体系结构在使用 DT 时都会做几乎相同的事情。

从名称可以猜到，.init_early() 用于任何需要在启动过程早期执行的机器特定设置，.init_irq() 用于设置中断处理。使用 DT 并不会实质性地改变这两个函数的行为。如果提供了 DT，那么 .init_early() 和 .init_irq() 都能够调用任何 DT 查询函数（include/linux/of**.h 中的 of_**）来获取有关平台的额外数据。

在 DT 上下文中最有趣的钩子是 .init_machine()，它主要负责用有关平台的数据填充 Linux 设备模型。历史上，在嵌入式平台上这是通过定义一组静态时钟结构、platform_devices 和其他数据（在板级支持 .c 文件中），并在 .init_machine() 中一次性注册来实现的。当使用 DT 时，不再为每个平台硬编码静态设备，而是可以通过解析 DT 获取设备列表，并动态分配设备结构。

最简单的情况是 .init_machine() 只负责注册一堆 platform_devices。platform_device 是 Linux 用于内存或 I/O 映射设备（无法被硬件探测到）以及"复合"或"虚拟"设备（稍后详述）的一个概念。虽然 DT 没有"平台设备"术语，但平台设备大致对应于树根处的设备节点以及简单内存映射总线节点的子节点。

现在正好可以给出一个例子。以下是

```
  /{
	compatible = "nvidia,harmony", "nvidia,tegra20";
	#address-cells = <1>;
	#size-cells = <1>;
	interrupt-parent = <&intc>;

	chosen { };
	aliases { };

	memory {
		device_type = "memory";
		reg = <0x00000000 0x40000000>;
	};

	soc {
		compatible = "nvidia,tegra20-soc", "simple-bus";
		#address-cells = <1>;
		#size-cells = <1>;
		ranges;

		intc: interrupt-controller@50041000 {
			compatible = "nvidia,tegra20-gic";
			interrupt-controller;
			#interrupt-cells = <1>;
			reg = <0x50041000 0x1000>, < 0x50040100 0x0100 >;
		};

		serial@70006300 {
			compatible = "nvidia,tegra20-uart";
			reg = <0x70006300 0x100>;
			interrupts = <122>;
		};

		i2s1: i2s@70002800 {
			compatible = "nvidia,tegra20-i2s";
			reg = <0x70002800 0x100>;
			interrupts = <77>;
			codec = <&wm8903>;
		};

		i2c@7000c000 {
			compatible = "nvidia,tegra20-i2c";
			#address-cells = <1>;
			#size-cells = <0>;
			reg = <0x7000c000 0x100>;
			interrupts = <70>;

			wm8903: codec@1a {
				compatible = "wlf,wm8903";
				reg = <0x1a>;
				interrupts = <347>;
			};
		};
	};

	sound {
		compatible = "nvidia,harmony-sound";
		i2s-controller = <&i2s1>;
		i2s-codec = <&wm8903>;
	};
  };

```
在 .init_machine() 时，Tegra 板级支持代码需要查看此 DT，并决定为哪些节点创建 platform_devices。然而，查看这棵树，并不能立刻看出每个节点代表什么类型的设备，甚至无法看出某个节点是否代表设备。/chosen、/aliases 和 /memory 节点是信息性节点，不描述设备（尽管 memory 可以说是一个设备）。/soc 节点的子节点是内存映射设备，但 codec@1a 是一个 i2c 设备，而 sound 节点代表的不是设备，而是其他设备如何连接在一起以创建音频子系统。我知道每个设备是什么，因为我熟悉板子设计，但内核怎么知道该如何处理每个节点呢？

诀窍在于内核从树的根部开始，寻找具有 'compatible' 属性的节点。首先，通常假设任何带有 'compatible' 属性的节点都代表某种设备；其次，可以假设树根部的任何节点要么直接连接到处理器总线，要么是无法以其他方式描述的杂项系统设备。对于这些节点中的每一个，Linux 分配并注册一个 platform_device，它反过来可能被绑定到 platform_driver。

为什么对这些节点使用 platform_device 是一个安全的假设？嗯，对于 Linux 为设备建模的方式，几乎所有制类型都假设其设备是某个总线控制器的子设备。例如，每个 i2c_client 是 i2c_master 的子设备。每个 spi_device 是 SPI 总线的子设备。USB、PCI、MDIO 等也是如此。在 DT 中也发现了相同的层次结构，其中 I2C 设备节点只作为 I2C 总线节点的子节点出现。SPI、MDIO、USB 等也同理。唯一不需要特定类型父设备的设备是 platform_devices（以及 amba_devices，稍后详述），它们会愉快地存在于 Linux /sys/devices 树的底部。因此，如果一个 DT 节点位于树的根部，那么它很可能最好注册为 platform_device。

Linux 板级支持代码调用 of_platform_populate(NULL, NULL, NULL, NULL) 来启动对树根部设备的发现。参数全为 NULL，是因为从树根部开始时，无需提供起始节点（第一个 NULL）、父 struct device（最后一个 NULL），并且我们尚未使用匹配表。对于一个只需要注册设备的板子，.init_machine() 除了 of_platform_populate() 调用之外可以完全是空的。

在 Tegra 的例子中，这涵盖了 /soc 和 /sound 节点，但 SoC 节点的子节点呢？它们不也应该注册为平台设备吗？对于 Linux 的 DT 支持，通用行为是子设备由父设备的驱动在驱动的 .probe() 时注册。因此，i2c 总线设备驱动会为每个子节点注册一个 i2c_client，SPI 总线驱动会注册它的 spi_device 子节点，其他总类型也类似。根据该模型，可以编写一个绑定到 SoC 节点并简单地为其每个子节点注册 platform_devices 的驱动。板级支持代码会分配并注册一个 SoC 设备，一个（理论上的）SoC 设备驱动可以绑定到该 SoC 设备，并在其 .probe() 钩子中为 /soc/interrupt-controller、/soc/serial、/soc/i2s 和 /soc/i2c 注册 platform_devices。很简单，对吧？

实际上，将某些 platform_devices 的子节点注册为更多的 platform_devices 是一种常见模式，设备树支持代码反映了这一点，并使上述例子变得更简单。of_platform_populate() 的第二个参数是一个 of_device_id 表，任何匹配该表中某一项的节点也会让其子节点被注册。在 Tegra 的例子中，代码

```
  static void __init harmony_init_machine(void)
  {
	/* ... */
	of_platform_populate(NULL, of_default_bus_match_table, NULL, NULL);
  }

```
"simple-bus" 在设备树规范中被定义为表示简单内存映射总线的属性，因此 of_platform_populate() 代码可以写成直接假设 simple-bus 兼容节点总是会被遍历。但是，我们将其作为参数传入，以便板级支持代码始终可以覆盖默认行为。

[需要添加关于添加 i2c/spi/等子设备的讨论]

### 附录 A：AMBA 设备


ARM Primecell 是连接到 ARM AMBA 总线的某类设备，包含一些对硬件探测和电源管理的支持。在 Linux 中，struct amba_device 和 amba_bus_type 用于表示 Primecell 设备。然而，棘手之处在于，并非 AMBA 总线上的所有设备都是 Primecell，而对 Linux 来说，amba_device 和 platform_device 实例通常是同一总线段的兄弟节点。

使用 DT 时，这给 of_platform_populate() 带来了问题，因为它必须决定是将每个节点注册为 platform_device 还是 amba_device。遗憾的是这稍微复杂化了设备的创建模型，但解决方案其实并不算太侵入。如果一个节点与 "arm,primecell" 兼容，那么 of_platform_populate() 会将其注册为 amba_device 而不是 platform_device。
