## 使用 ISA 和 LPC 设备进行 DMA


:Author: Pierre Ossman <drzeus@drzeus.cx>

本文档描述如何使用旧的 ISA DMA 控制器进行 DMA 传输。尽管 ISA 如今或多或少已经消亡，
但 LPC 总线使用相同的 DMA 系统，因此它还会存在相当长的一段时间。

### 头文件与依赖


```

	#include <linux/dma-mapping.h>
	#include <asm/dma.h>

```
第一个是用于把虚拟地址转换为总线地址的通用 DMA API（详见 Documentation/core-api/dma-api.rst）。

第二个包含特定于 ISA DMA 传输的例程。由于它不是在所有平台上都存在，请确保你的 Kconfig
依赖于 ISA_DMA_API（而非 ISA），这样就不会有人在不支持的平台上去构建你的驱动。

### 缓冲区分配


ISA DMA 控制器对它能访问的内存有非常严格的要求，因此在分配缓冲区时必须格外小心。

（你通常需要为 DMA 传输分配一个特殊的缓冲区，而不是直接从你的普通数据结构进行传输。）

可进行 DMA 的地址空间是 _物理_ 内存最低的 16 MB。此外，传输块不能跨越页边界（根据所用
通道不同，页大小为 64 或 128 KiB）。

为了分配一块满足所有这些要求的内存，你向 kmalloc 传入 GFP_DMA 标志。

遗憾的是可用于 ISA DMA 的内存十分稀缺，因此除非你在启动时就分配内存，否则最好同时传入
__GFP_RETRY_MAYFAIL 和 __GFP_NOWARN，让分配器更努力地尝试。

（这种稀缺性也意味着你应当尽早分配缓冲区，并且在驱动卸载之前不要释放它。）

### 地址转换


要将虚拟地址转换为总线地址，请使用普通的 DMA API。_不要_ 使用 isa_virt_to_bus()，即使
它做的是同一件事。原因是函数 isa_virt_to_bus() 会要求 Kconfig 依赖于 ISA，而不只是
真正所需的 ISA_DMA_API。请记住，尽管 DMA 控制器起源于 ISA，但它也被用在其他地方。

注意：x86_64 在 ISA 方面的 DMA API 曾经有问题，但后来已经修复。如果你的架构有问题，请
修复 DMA API，而不是回退到 ISA 函数。

### 通道


一个普通的 ISA DMA 控制器有 8 个通道。较低的四个用于 8 位传输，较高的四个用于 16 位传输。

（实际上 DMA 控制器是两个独立的控制器，其中通道 4 用于让第二个控制器（0-3）获得 DMA
访问。这意味着四个 16 位通道中只有三个可用。）

你分配它们的方式与所有基本资源类似：

extern int request_dma(unsigned int dmanr, const char * device_id);
extern void free_dma(unsigned int dmanr);

使用 16 位还是 8 位传输的能力_不_由你作为驱动作者决定，而是取决于硬件支持什么。请查阅
你的规格说明或测试不同的通道。

### 传输数据


现在是好东西，实际的 DMA 传输。:)

在使用任何 ISA DMA 例程之前，你需要使用 claim_dma_lock() 获取 DMA 锁。原因是某些 DMA
操作不是原子的，因此同一时间只能有一个驱动去摆弄这些寄存器。

你第一次使用 DMA 控制器时应当调用 clear_dma_ff()。这会清除 DMA 控制器中用于非原子操作
的内部寄存器。只要你（以及其它所有人）都使用锁函数，就只需重置一次。

接下来，使用 set_dma_mode() 告诉控制器你打算进行哪个方向的传输。目前你有 DMA_MODE_READ
和 DMA_MODE_WRITE 两个选项。

设置传输应开始的地址（对于 16 位传输需要 16 位对齐）以及要传输的字节数。注意那是_字节_。
DMA 例程会完成所有到 DMA 控制器所能理解的数值所需的转换。

最后一步是使能 DMA 通道并释放 DMA 锁。

DMA 传输完成（或超时）后，你应当再次禁用该通道。你还应当检查 get_dma_residue() 以确保
所有数据都已传输。

```

	int flags, residue;

	flags = claim_dma_lock();

	clear_dma_ff();

	set_dma_mode(channel, DMA_MODE_WRITE);
	set_dma_addr(channel, phys_addr);
	set_dma_count(channel, num_bytes);

	dma_enable(channel);

	release_dma_lock(flags);

	while (!device_done());

	flags = claim_dma_lock();

	dma_disable(channel);

	residue = dma_get_residue(channel);
	if (residue != 0)
		printk(KERN_ERR "driver: Incomplete DMA transfer!"
			" %d bytes left!\n", residue);

	release_dma_lock(flags);

```
### 挂起/恢复


驱动有责任确保在 DMA 传输进行期间机器不会被挂起。此外，当系统挂起时所有 DMA 设置都会
丢失，因此如果你的驱动依赖 DMA 控制器处于某种状态，那么你必须在恢复时恢复这些寄存器。
