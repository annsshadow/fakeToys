## 利用 OHCI-1394 火线（FireWire）控制器提供的物理 DMA 进行调试


### 简介（Introduction）


基本上，当今使用的所有火线控制器都符合 OHCI-1394 规范，该规范将控制器定义为一个 PCI 总线主设备，它使用 DMA 将数据传输从 CPU 上卸载，并拥有一个“物理响应单元（Physical Response Unit）”，该单元在应用由 OHCI-1394 驱动定义的过滤器之后，通过 PCI 总线主 DMA 执行特定请求。

一旦正确配置，远程机器就可以发送这些请求，要求 OHCI-1394 控制器对物理系统内存执行读和写请求，并且对于读请求，将物理内存读取的结果发送回请求方。

由此，可以通过读取诸如 printk 缓冲区或进程表之类的缓冲区等有趣的内存位置来调试问题。

也可以通过火线获取完整的系统内存转储，数据传输速率可达 10MB/s 或更高。

对于大多数火线控制器，内存访问被限制在物理地址空间的低 4 GB。在内存主要位于该限制之上的机器上这可能成为问题，但在 x86、x86-64 和 PowerPC 等更常见的硬件上很少成为问题。

已知至少 LSI FW643e 和 FW643e2 控制器支持访问 4 GB 以上的物理地址，但 Linux 目前尚未启用此功能。

配合 OHCI-1394 控制器的早期初始化用于调试，该设施被证明对于检查 printk 缓冲区中冗长的调试日志最为有用，以调试 ACPI 等区域中系统无法启动的早期引导问题，而其他调试手段（串口）要么不可用（笔记本），要么对于大量调试信息（如 ACPI）而言太慢。

### 驱动（Drivers）


drivers/firewire 中的 firewire-ohci 驱动默认使用经过过滤的物理 DMA，这更安全但不适合远程调试。向该驱动传递 remote_dma=1 参数以获得未经过滤的物理 DMA。

由于 firewire-ohci 驱动依赖于 PCI 枚举的完成，因此已为 x86 实现了一个运行得相当早的初始化例程。该例程在 console_init() 能够被调用之前很久就运行，即在 printk 缓冲区出现在控制台之前。

要激活它，请启用 CONFIG_PROVIDE_OHCI1394_DMA_INIT（Kernel hacking 菜单：Remote debugging over FireWire early on boot），并在引导时向重新编译的内核传递参数 "ohci1394_dma=early"。

### 工具（Tools）


firescope - 最初由 Benjamin Herrenschmidt 开发，Andi Kleen 将其从 PowerPC 移植到 x86 和 x86_64 并添加了功能，firescope 现在可用于查看远程机器的 printk 缓冲区，甚至支持实时更新。

Bernhard Kaindl 增强了 firescope，以支持从 32 位 firescope 访问 64 位机器，反之亦然：
- http://v3.sk/~lkundrak/firescope/

并且他实现了快速系统转储（alpha 版本 - 请阅读 README.txt）：
- http://halobates.de/firewire/firedump-0.1.tar.bz2

还有一个用于火线的 gdb 代理，允许使用 gdb 访问可从 gdb 在 vmlinux 中找到的符号所引用的数据：
- http://halobates.de/firewire/fireproxy-0.33.tar.bz2

此 gdb 代理的最新版本（fireproxy-0.34）可以通过一个基于内存的通信模块（kgdbom）与 kgdb 通信（尚不稳定）。

### 开始使用（Getting Started）


OHCI-1394 规范规定，OHCI-1394 控制器必须在每次总线复位时禁用所有物理 DMA。

这意味着，如果你想在系统处于中断被禁用、且不对 OHCI-1394 控制器进行总线复位轮询的状态下调试某个问题，你必须在系统进入这种状态__之前__建立任何火线电缆连接并完全初始化所有火线硬件。

使用 firescope 配合早期 OHCI 初始化的分步说明：

1) 验证你的硬件受支持：

   加载 firewire-ohci 模块并检查你的内核日志。
```
     firewire_ohci 0000:15:00.1: added OHCI v1.0 device as card 2, 4 IR + 4 IT
     ... contexts, quirks 0x11
```
   加载驱动时。如果你没有受支持的控制器，许多完全符合 OHCI-1394 规范的 PCI、CardBus 甚至某些 Express 卡都可用。如果它不需要 Windows 操作系统的驱动，那它很可能就是。只有专门的商店才有不符合规范的卡，它们基于 TI PCILynx 芯片并需要 Windows 操作系统的驱动。

   上述内核日志消息包含字符串 "physUB"，如果该控制器实现了可写的物理上界（Physical Upper Bound）寄存器。这是 4 GB 以上物理 DMA 所必需的（但 Linux 尚未使用）。

2) 建立可用的火线电缆连接：

   任何火线电缆，只要提供电气和机械上稳定的连接并具有匹配的接头（有小型 4 针和大型 6 针火线端口）即可。

```
     firewire_core 0000:15:00.1: created device fw1: GUID 00061b0020105917, S400
```
   当电缆插入并连接两台机器时，两台机器的内核日志中都会出现。

3) 使用 firescope 测试物理 DMA：

   在调试主机上，确保 /dev/fw* 可访问，
```
	$ firescope
	Port 0 (/dev/fw1) opened, 2 nodes detected

	FireScope
	---------
	Target : <unspecified>
	Gen    : 1
	[Ctrl-T] choose target
	[Ctrl-H] this menu
	[Ctrl-Q] quit

    ------> 现在按 Ctrl-T，输出应类似如下：

	2 nodes available, local node is: 0
	 0: ffc0, uuid: 00000000 00000000 [LOCAL]
	 1: ffc1, uuid: 00279000 ba4bb801

   除了 [LOCAL] 节点外，它必须无错误地显示另一个节点。

```
4) 为配合早期 OHCI-1394 初始化进行调试做准备：

   4.1) 在调试目标上编译并安装内核

   编译要调试的内核，并启用 CONFIG_PROVIDE_OHCI1394_DMA_INIT（Kernel hacking：Provide code for enabling DMA over FireWire early on boot），然后将其安装到要调试的机器（调试目标）上。

   4.2) 将受调试内核的 System.map 传输到调试主机

   将受调试内核的 System.map 复制到调试主机（即通过火线电缆连接到受调试机器的主机）。

5) 获取 printk 缓冲区内容：

   在火线电缆已连接、调试主机上已加载 OHCI-1394 驱动的情况下，重新启动受调试机器，引导启用了 CONFIG_PROVIDE_OHCI1394_DMA_INIT 的内核，并使用选项 ohci1394_dma=early。

```
	firescope -A System.map-of-debug-target-kernel
```
   注意：-A 会自动连接到第一个非本地节点。它仅在仅通过火线连接两台机器时才可靠工作。

   连接到调试目标后，按 Ctrl-D 查看完整的 printk 缓冲区，或按 Ctrl-U 进入自动更新模式，获取受调试目标上记录的最近内核消息的实时视图。

   调用 "firescope -h" 可获取有关 firescope 选项的更多信息。

### 备注（Notes）


文档和规范：http://halobates.de/firewire/

FireWire 是 Apple Inc. 的商标 - 更多信息请参阅：
https://en.wikipedia.org/wiki/FireWire
