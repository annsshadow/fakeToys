## 动DMA 映射指南


:Author: David S. Miller <davem@redhat.com>
:Author: Richard Henderson <rth@cygnus.com>
:Author: Jakub Jelinek <jakub@redhat.com>

本指南面向设备驱动开发者，介绍如何使用 DMA API，并附有伪代码示例。关API 简明说明，请参Documentation/core-api/dma-api.rst
## CPU 地址DMA 地址


DMA API 中涉及几种不同的地址，理解它们的区别非常重要
内核通常使用虚拟地址。kmalloc()、vmalloc() 以及类似接口返回的任何地址都是虚拟地址可以保存`void *` 中
虚拟内存系统（TLB、页表等）将虚拟地址转换CPU 物理地址，物理地址"phys_addr_t" "resource_size_t" 形式存储。内核将寄存器等设备资源当作物理地址
管理。这些就/proc/iomem 中的地址。物理地址对驱动并不直接有用；驱动必须使用
ioremap() 来映射该空间并得到一个虚拟地址
I/O 设备使用第三种地址：“总线地址”（bus address）。如果设备在某个 MMIO 地址上拥寄存器，或者它执行 DMA 来读写系统内存，设备所使用的地址就是总线地址。在某些系统中，
总线地址CPU 物理地址完全相同，但一般情况下并非如此。IOMMU 和主机桥（host bridge可以在物理地址和总线地址之间建立任意映射
从设备的角度看，DMA 使用的是总线地址空间，但可能仅限于该空间的一个子集。例如，即使
一个系统支64 位的主存地址PCI BAR，它也可能使IOMMU，使得设备只需使用 32 DMA 地址
```

               CPU                  CPU                  Bus
             Virtual              Physical             Address
             Address              Address               Space
              Space                Space

            +-------+             +------+             +------+
            |       |             |MMIO  |   Offset    |      |
            |       |  Virtual    |Space |   applied   |      |
          C +-------+ --------> B +------+ ----------> +------+ A
            |       |  mapping    |      |   by host   |      |
  +-----+   |       |             |      |   bridge    |      |   +--------+
  |     |   |       |             +------+             |      |   |        |
  | CPU |   |       |             | RAM  |             |      |   | Device |
  |     |   |       |             |      |             |      |   |        |
  +-----+   +-------+             +------+             +------+   +--------+
            |       |  Virtual    |Buffer|   Mapping   |      |
          X +-------+ --------> Y +------+ <---------- +------+ Z
            |       |  mapping    | RAM  |   by IOMMU
            |       |             |      |
            |       |             |      |
            +-------+             +------+

```
在枚举过程中，内核会了解I/O 设备及其 MMIO 空间，以及将设备连接到系统的主机桥例如，如果一PCI 设备有一BAR，内核会BAR 中读取总线地址（A）并将其转换CPU
物理地址（B）。地址 B 保存在一struct resource 中，通常通过 /proc/iomem 暴露。当
驱动认领一个设备时，它通常使用 ioremap() 将物理地址 B 映射到某个虚拟地址（C）。然它就可以使用例如 ioread32(C) 来访问总线地址 A 处的设备寄存器
如果设备支持 DMA，驱动使kmalloc() 或类似接口建立一块缓冲区，该接口返回一个虚地址（X）。虚拟内存系统将 X 映射到系RAM 中的某个物理地址（Y）。驱动可以使用虚地址 X 来访问该缓冲区，但设备本身不能，因为 DMA 不经CPU 的虚拟内存系统
在某些简单系统中，设备可以直接对物理地址 Y DMA。但在许多其他系统中，有 IOMMU 硬件
DMA 地址转换为物理地址，例如将 Z 转换Y。这正是需DMA API 的部分原因：驱动可以
把一个虚拟地址 X 交给dma_map_single() 这样的接口，由它建立任何必需IOMMU 映射返回 DMA 地址 Z。然后驱动告知设备对 Z DMA，IOMMU 再将其映射到系统 RAM 中地址 Y 处的
缓冲区
为了Linux 能够使用动DMA 映射，它需要驱动提供一些帮助，即必须考虑DMA 地址
只应在实际使用期间被映射，并DMA 传输完成后被取消映射
当然，即使在不存在此类硬件的平台上，下面API 也能工作
注意，DMA API 适用于任何总线，而与底层的微处理器架构无关。你应该使用 DMA API，而不总线特定DMA API，也就是说，使用 dma_map_*() 接口，而不pci_map_*() 接口
```

	#include <linux/dma-mapping.h>

```
出现在你的驱动中，它提供dma_addr_t 的定义。该类型可以保存平台上任何有效的 DMA
地址，凡是要保存DMA 映射函数返回DMA 地址时，都应使用此类型
## 哪些内存可做 DMA


你必须知道的第一件事是，哪些内核内存可以DMA 映射设施一起使用。关于这一点一直有一不成文的规则，本文试图最终把它们写下来
如果你通过页分配器（即 __get_free_page*()）或通用内存分配器（kmalloc() kmem_cache_alloc()）获得内存，那么你就可以使用这些例程返回的地址对该内存进行 DMA
读写
这具体意味着你_不能_使用 vmalloc() 返回的内地址来做 DMA。可以对映射vmalloc() 区域
的_底层_内存DMA，但这需要遍历页表以获得物理地址，然后用 __va() 之类的函数把每个
页再转换回内核地址。[注：待我们集Gerd Knorr 实现此功能的通用代码时再更新此处。]

这条规则还意味着，你既不能使用内核映像地址（data/text/bss 段中的项），也不能使用模映像地址或栈地址来做 DMA。这些都可能被映射到与物理内存其余部分完全不同的地方。即这些类别的内存在物理上可以与 DMA 配合工作，你也需要确I/O 缓冲区是按缓存行对齐的否则，在具有 DMA 不一致（DMA-incoherent）缓存的 CPU 上，你会遇到缓存行共享问题（数据
损坏）。（CPU 可能写一个字，DMA 写同一个缓存行中的另一个字，其中一个可能被覆盖。）

同样，这意味着你不能拿 kmap() 调用的返回值去DMA 读写。这vmalloc() 类似
I/O 和网络缓冲区呢？I/O 和网络子系统会确保它们使用的缓冲区对 DMA 读写是有效的
## __dma_from_device_group_begin/end 注解


如前所述，当一个结构体包含一DMA_FROM_DEVICE / DMA_BIDIRECTIONAL 缓冲区（设备写入
内存）以CPU 写入的字段时，DMA 缓冲区与 CPU 写入字段之间的缓存行共享，会在具DMA 不一致缓存的 CPU 上导致数据损坏
`__dma_from_device_group_begin(GROUP)/__dma_from_device_group_end(GROUP)`
```

	struct my_device {
		spinlock_t lock1;
		__dma_from_device_group_begin();
		char dma_buffer1[16];
		char dma_buffer2[16];
		__dma_from_device_group_end();
		spinlock_t lock2;
	};

```
为了DMA 缓冲区与相邻字段隔离开来，请在第一DMA 缓冲区字段之前使`__dma_from_device_group_begin(GROUP)`，在最后一DMA 缓冲区字段之后使`__dma_from_device_group_end(GROUP)`（使用相同的 GROUP 名称）。这会保护缓冲区的头部和
尾部都不受缓存行共享的影响
GROUP 参数是一个可选的标识符，用于命名 DMA 缓冲区组
```

	struct my_device {
		spinlock_t lock1;
		__dma_from_device_group_begin(buffer1);
		char dma_buffer1[16];
		__dma_from_device_group_end(buffer1);
		spinlock_t lock2;
		__dma_from_device_group_begin(buffer2);
		char dma_buffer2[16];
		__dma_from_device_group_end(buffer2);
	};

```
在缓存一致（cache-coherent）的平台上，这些宏会展开为零长度数组标记。在非一致平台上它们还会确保最小的 DMA 对齐，最大可能达128 字节

        允许（尽管有些脆弱）在组内包含不打算供设备做 DMA 的额外字段（以便让结构体
        紧凑排列）——但前提是，只要组内的任何字段被映射DMA_FROM_DEVICE         DMA_BIDIRECTIONAL，CPU 就不得写入这些字段
## DMA 寻址能力


默认情况下，内核假设你的设备能够进行 32 位的 DMA 寻址。对于支64 位的设备，需提高这个值；对于有限制的设备，需要降低这个值
关于 PCI 的特别说明：PCI-X 规范规定 PCI-X 设备必须支持对所有事务的 64 位寻址（DAC）并且至少有一个平台（SGI SN2）要求在 IO 总线处于 PCI-X 模式时，使用 64 位一致性分配才正常工作
为了正确运行，你必须设置 DMA 掩码（mask）来告知内核你的设备DMA 寻址能力
```

	int dma_set_mask_and_coherent(struct device *dev, u64 mask);

```
该函数会同时为流式（streaming）和一致性（coherent）API 设置掩码。如果你有一些特需求，可以改用下面两个独立的调用：

	流式映射的设置通过一个对如下函数的调用来完成
```

		int dma_set_mask(struct device *dev, u64 mask);

	一致性分配的设置通过调用 dma_set_coherent_mask() 来完:

		int dma_set_coherent_mask(struct device *dev, u64 mask);

```
这里，dev 是指向你的设备的 device 结构体的指针，mask 是一个位掩码，描述你的设备支地址的哪些位。通常，你的设备的 device 结构体嵌入在它的总线特定device 结构体中例如pdev->dev 是指PCI 设备device 结构体的指针（pdev 是指向你的设备的 PCI
device 结构体的指针）
这些调用通常返回零，表示在给定的地址掩码下，你的设备可以在该机器上正常执DMA；但
如果掩码太小以至于该系统无法支持，它们也可能返回错误。如果返回非零，说明你的设备在该
平台上无法正确执DMA，尝试这样做将导致未定义的行为。除dma_set_mask 系列函数
返回成功，否则你不得在该设备上使DMA
这意味着在失败的情况下，你有两个选择
1) 如果可能，使用某种非 DMA 模式进行数据传输2) 忽略该设备，不要初始化它
建议你的驱动在设DMA 掩码失败时打印一条内KERN_WARNING 消息。这样，如果你的驱动用户报告性能很差或者设备甚至未被检测到，你可以向他们要内核消息来查明确切的原因
```

	if (dma_set_mask_and_coherent(dev, DMA_BIT_MASK(24))) {
		dev_warn(dev, "mydev: No suitable DMA available\n");
		goto ignore_this_device;
	}

```
```

	dma_set_mask_and_coherent(dev, DMA_BIT_MASK(64))

```
当为 DMA_BIT_MASK(64) 时，dma_set_mask_and_coherent() 永远不会返回失败。典```

	/* 错误的代*/
	if (dma_set_mask_and_coherent(dev, DMA_BIT_MASK(64)))
		dma_set_mask_and_coherent(dev, DMA_BIT_MASK(32))

```
当大32 位时，dma_set_mask_and_coherent() 永远不会返回失败```

	/* 推荐的代*/
	if (support_64bit)
		dma_set_mask_and_coherent(dev, DMA_BIT_MASK(64));
	else
		dma_set_mask_and_coherent(dev, DMA_BIT_MASK(32));

```
如果设备仅对一致性分配中的描述符支持 32 位寻址，但对流式映射支持完整的 64 ```

	if (dma_set_mask(dev, DMA_BIT_MASK(64))) {
		dev_warn(dev, "mydev: No suitable DMA available\n");
		goto ignore_this_device;
	}

```
一致性掩码总是能够设置为与流式掩码相同或更小的掩码。但对于设备驱动仅使用一致性分这种罕见情况，就必须检dma_set_coherent_mask() 的返回值
最后，如果你的设备只能驱动24 ```

	if (dma_set_mask(dev, DMA_BIT_MASK(24))) {
		dev_warn(dev, "mydev: 24-bit DMA addressing not available\n");
		goto ignore_this_device;
	}

```
dma_set_mask() dma_set_mask_and_coherent() 成功并返回零时，内核会保存你提供这个掩码。之后在你进DMA 映射时，内核会使用这些信息
目前我们了解到一种值得一提的情况，值得在本文档中说明。如果你的设备支持多个功（例如一块声卡提供播放和录音功能），并且各个不同功能具有_不同的_ DMA 寻址限制，你
可能希望探测每个掩码，只提供该机器能够处理的功能。重要的是，dma_set_mask() 的最一次调用应当是针对最具体的掩码
```

	#define PLAYBACK_ADDRESS_BITS	DMA_BIT_MASK(32)
	#define RECORD_ADDRESS_BITS	DMA_BIT_MASK(24)

	struct my_sound_card *card;
	struct device *dev;

	...
	if (!dma_set_mask(dev, PLAYBACK_ADDRESS_BITS)) {
		card->playback_enabled = 1;
	} else {
		card->playback_enabled = 0;
		dev_warn(dev, "%s: Playback disabled due to DMA limitations\n",
		       card->name);
	}
	if (!dma_set_mask(dev, RECORD_ADDRESS_BITS)) {
		card->record_enabled = 1;
	} else {
		card->record_enabled = 0;
		dev_warn(dev, "%s: Record disabled due to DMA limitations\n",
		       card->name);
	}

```
这里以声卡为例，是因为这PCI 设备似乎充斥着带有 PCI 前端ISA 芯片，因而保留了
ISA 16MB DMA 寻址限制
## DMA 映射的类

有两种类型的 DMA 映射
- 一致性（Coherent）DMA 映射，通常在驱动初始化时映射、在结束时取消映射，硬件应当
  保证设备CPU 可以并行访问数据，并且无需任何显式的软件刷新即可看到对方所做的更新
  可以把“一致性”理解为“同步”
  当前的默认行为是DMA 空间的低 32 位中返回一致性内存。但是，为了将来的兼容性，即使
  这个默认值对你的驱动来说没问题，你也应该设置一致性掩码
  适合使用一致性映射的好例子有
 - 网卡 DMA 环形描述符 - SCSI 适配器邮箱命令数据结构 - 从主存中执行的设备固件微码
  这些例子的共同不变式是：任何 CPU 对内存的存储都立即对设备可见，反之亦然。一致  映射保证了这一点
```

	     Coherent DMA memory does not preclude the usage of
	     proper memory barriers.  The CPU may reorder stores to
	     coherent memory just as it may normal memory.  Example:
	     if it is important for the device to see the first word
	     of a descriptor updated before the second, you must do
	     something like::

		desc->word0 = address;
		wmb();
		desc->word1 = DESC_VALID;

             in order to get correct behavior on all platforms.

	     Also, on some platforms your driver may need to flush CPU write
	     buffers in much the same way as it needs to flush write buffers
	     found in PCI bridges (such as by reading a register's value
	     after writing it).

```
- 流式（Streaming）DMA 映射，通常映射用于一DMA 传输，传输后立即取消映射（除非你
  在下面使dma_sync_*），硬件可以针对顺序访问进行优化
  可以把“流式”理解为“异步”或“在一致性域之外”
  适合使用流式映射的好例子有：

 - 设备发接收的网络缓冲区 - SCSI 设备写入/读取的文件系统缓冲区
  这类映射的使用接口在设计时就考虑到了实现可以做硬件允许的任何性能优化。为此，使用
  这类映射时，你必须明确说明你希望发生什么
两种 DMA 映射都没有来自底层总线的对齐限制，尽管某些设备可能有这样的限制。此外，缓存不是 DMA 一致的系统中，当底层缓冲区不与其他数据共享缓存行时，工作情况会更好
## 使用一致DMA 映射


要分配并映射较大的（大约 PAGE_SIZE 大小）一致DMA 区域```

	dma_addr_t dma_handle;

	cpu_addr = dma_alloc_coherent(dev, size, &dma_handle, gfp);

```
其中 device `struct device *`。这可以在中断上下文中以 GFP_ATOMIC 标志调用
size 是你想分配的区域长度，以字节为单位
该例程会为那个区域分RAM，所以它类似__get_free_pages()（但接受 size 而不是页
order）。如果你的驱动需要小于一页的区域，你可能更倾向于使用下面描述的 dma_pool 接口
一致DMA 映射接口默认返回一32 位可寻址DMA 地址。即使设备（通过 DMA 掩码）表它可以寻址32 位，一致性分配也只会在通过 dma_set_coherent_mask() 显式更改了一致DMA 掩码的情况下，才会为 DMA 返回 > 32 位的地址。dma_pool 接口也是如此
dma_alloc_coherent() 返回两个值：你可以从 CPU 用来访问它的虚拟地址，以及你传给
网卡dma_handle
CPU 虚拟地址DMA 地址都保证对齐到大于等于请求大小的最PAGE_SIZE order。存在这不变式（例如）是为了保证：如果你分配的块小于等于 64 千字节，你收到的缓冲区的范围不会
跨越 64K 边界
```

	dma_free_coherent(dev, size, cpu_addr, dma_handle);

```
其中 dev、size 与上面调用中的相同，cpu_addr dma_handle dma_alloc_coherent() 返回
给你的值。该函数不能在中断上下文中调用
如果你的驱动需要大量较小的内存区域，你可以编写自定义代码来细分 dma_alloc_coherent()
返回的页，或者使dma_pool API 来做这件事。dma_pool 类似kmem_cache，但它使dma_alloc_coherent()，而不__get_free_pages()。此外，它理解常见的硬件对齐约束，例队列头需要对齐到 N 字节边界
```

	struct dma_pool *pool;

	pool = dma_pool_create(name, dev, size, align, boundary);

```
“name”用于诊断（类似 kmem_cache 的名称）；dev size 同上。该类型数据的设备的硬件
对齐要求是“align”（以字节表示，且必须是 2 的幂）。如果你的设备没有跨越边界的限制boundary 0；传 4096 表示从这个池中分配的内存不得跨越 4K 字节边界（但在那种情况下或许最好直接使dma_alloc_coherent()）
```

	cpu_addr = dma_pool_alloc(pool, flags, &dma_handle);

```
如果允许阻塞（不in_interrupt 中，也没有持SMP 锁），flags GFP_KERNEL，否则为
GFP_ATOMIC。与 dma_alloc_coherent() 一样，这也返回两个值：cpu_addr dma_handle
```

	dma_pool_free(pool, cpu_addr, dma_handle);

```
其中 pool 是你传给 dma_pool_alloc() 的值，cpu_addr dma_handle dma_pool_alloc()
返回的值。该函数可以在中断上下文中调用
```

	dma_pool_destroy(pool);

```
在销毁池之前，请确保你已经对从该池分配的所有内存调用了 dma_pool_free()。该函数不能中断上下文中调用
## DMA 方向


本文档后续部分描述的接口接受一DMA 方向参数，它是一个整数，取值为
```

 DMA_BIDIRECTIONAL
 DMA_TO_DEVICE
 DMA_FROM_DEVICE
 DMA_NONE

```
如果你知道方向，就应当提供确切的 DMA 方向
DMA_TO_DEVICE 表示“从主存到设备DMA_FROM_DEVICE 表示“从设备到主存它是 DMA 传输过程中数据移动的方向
你被_强烈_鼓励尽可能精确地指定它
如果你绝对无法知DMA 传输的方向，请指DMA_BIDIRECTIONAL。它表示 DMA 可以向任一方向
进行。平台保证你可以合法地指定它，并且它会正常工作，但这可能是以性能为代价的
DMA_NONE 这个值用于调试。你可以在确切方向确定之前将它保存在一个数据结构中，这有助捕获你的方向跟踪逻辑未能正确设置的情况
精确指定这个值（除了潜在的平台特定优化之外）的另一个好处是便于调试。某些平台实际上
有一个写权限布尔值，DMA 映射可以被标记上它，就像用户程序地址空间中的页保护一样。当
DMA 控制器硬件检测到违反了该权限设置时，这类平台可以并且确实会在内核日志中报告错误
只有流式映射才指定方向，一致性映射隐式地将方向属性设DMA_BIDIRECTIONAL
SCSI 子系统会在你的驱动正在处理的 SCSI 命令'sc_data_direction' 成员中告诉你要使的方向
对于网络驱动，这是一件相当简单的事情。对于发送数据包，使DMA_TO_DEVICE 方向说明来映取消映射它们。对于接收数据包，则正好相反，使DMA_FROM_DEVICE 方向说明符来
映射/取消映射它们
## 使用流式 DMA 映射


流式 DMA 映射例程可以在中断上下文中调用。每个映取消映射都有两个版本，一个映
取消映射单个内存区域，另一个映取消映射一scatterlist
```

	struct device *dev = &my_dev->dev;
	dma_addr_t dma_handle;
	void *addr = buffer->ptr;
	size_t size = buffer->len;

	dma_handle = dma_map_single(dev, addr, size, direction);
	if (dma_mapping_error(dev, dma_handle)) {
		/*
		 * reduce current DMA mapping usage,
		 * delay and try again later or
		 * reset driver.
		 */
		goto map_error_handling;
	}

```
```

	dma_unmap_single(dev, dma_handle, size, direction);

```
你应该调dma_mapping_error()，因dma_map_single() 可能会失败并返回错误。这样做可以
确保映射代码在所DMA 实现上都能正确工作，而不依赖于底层实现的细节。不经错误检查就
使用返回的地址，可能导致从内核崩溃到静默数据损坏等各种故障。这同样适用dma_map_page()
你应该在 DMA 活动结束时（例如，从告诉DMA 传输已完成的中断中）调用 dma_unmap_single()
像这样对单个映射使用 CPU 指针有一个缺点：你无法以这种方式引用 HIGHMEM 内存。因此，存在
一对类似于 dma_{map,unmap}_single() 的映取消映射接口。这些接口处理的是页/偏移对，
而不CPU 指针```

	struct device *dev = &my_dev->dev;
	dma_addr_t dma_handle;
	struct page *page = buffer->page;
	unsigned long offset = buffer->offset;
	size_t size = buffer->len;

	dma_handle = dma_map_page(dev, page, offset, size, direction);
	if (dma_mapping_error(dev, dma_handle)) {
		/*
		 * reduce current DMA mapping usage,
		 * delay and try again later or
		 * reset driver.
		 */
		goto map_error_handling;
	}

	...

	dma_unmap_page(dev, dma_handle, size, direction);

```
这里，“offset”表示给定页内的字节偏移
你应该调dma_mapping_error()，因为如前在 dma_map_single() 讨论中所概述的，dma_map_page()
可能会失败并返回错误
你应该在 DMA 活动结束时（例如，从告诉DMA 传输已完成的中断中）调用 dma_unmap_page()
```

	int i, count = dma_map_sg(dev, sglist, nents, direction);
	struct scatterlist *sg;

	for_each_sg(sglist, sg, count, i) {
		hw_address[i] = sg_dma_address(sg);
		hw_len[i] = sg_dma_len(sg);
	}

```
其中 nents sglist 中的条目数
实现可以自由地将几个连续sglist 条目合并为一个（例如，如DMA 映射PAGE_SIZE 粒度进行，那么任意连续的 sglist 条目都可以合并为一个，只要第一个在页边界结束、第二个
在页边界开始——事实上，对于不能做分散/聚集（scatter-gather）或分散/聚集条目数量非常
有限的网卡来说，这是一个巨大的优势），并返回它映射到的实际 sg 条目数。失败时返回 0
然后你应该循count 次（注意：这可能少于 nents 次），并在你原先访问 sg->address sg->length 的地方使sg_dma_address() sg_dma_len() 宏，如上所示
```

	dma_unmap_sg(dev, sglist, nents, direction);

```
再次强调，请确保 DMA 活动已经结束

	传给 dma_unmap_sg 调用'nents' 参数必须是你传给 dma_map_sg 调用	_同一个_，它_不应_dma_map_sg 调用_返回_'count' 值
每个 dma_map_{single,sg}() 调用都应该有对应dma_unmap_{single,sg}() 调用，因DMA
地址空间是一个共享资源，如果你耗尽了所DMA 地址，可能会使机器无法使用
如果你需要多次使用同一个流DMA 区域，并且在 DMA 传输之间会触碰数据，那么该缓冲区
需要被正确同步，以CPU 和设备都能看到最新且正确DMA 缓冲区副本
所以，首先，只dma_map_{single,sg}() 映射它，然后在每DMA 之后
```

	dma_sync_single_for_cpu(dev, dma_handle, size, direction);

```
```

	dma_sync_sg_for_cpu(dev, sglist, nents, direction);

```
视情况使用
然后，如果你想让设备再次访问 DMA 区域，在 CPU 完成对数据的访问之后，并且在真正
```

	dma_sync_single_for_device(dev, dma_handle, size, direction);

```
```

	dma_sync_sg_for_device(dev, sglist, nents, direction);

```
视情况使用

	      dma_sync_sg_for_cpu() 鍜?dma_sync_sg_for_device() 鐨?'nents'
	      参数必须与传dma_map_sg() 的相同。它_不是_ dma_map_sg()
	      返回count
在最后一DMA 传输之后，调dma_unmap_{single,sg}() 之一。如果你从第一dma_map_**()
调用dma_unmap_**() 都没有触碰数据，那么你根本不需要调dma_sync_*() 例程
下面是一个伪代码，展示了一个你需要使用同步的情况
```

	my_card_setup_receive_buffer(struct my_card *cp, char *buffer, int len)
	{
		dma_addr_t mapping;

		mapping = dma_map_single(cp->dev, buffer, len, DMA_FROM_DEVICE);
		if (dma_mapping_error(cp->dev, mapping)) {
			/*
			 * reduce current DMA mapping usage,
			 * delay and try again later or
			 * reset driver.
			 */
			goto map_error_handling;
		}

		cp->rx_buf = buffer;
		cp->rx_len = len;
		cp->rx_dma = mapping;

		give_rx_buf_to_card(cp);
	}

	...

	my_card_interrupt_handler(int irq, void *devid, struct pt_regs *regs)
	{
		struct my_card *cp = devid;

		...
		if (read_card_status(cp) == RX_BUF_TRANSFERRED) {
			struct my_card_header *hp;

			/* Examine the header to see if we wish
			 * to accept the data.  But synchronize
			 * the DMA transfer with the CPU first
			 * so that we see updated contents.
			 */
			dma_sync_single_for_cpu(&cp->dev, cp->rx_dma,
						cp->rx_len,
						DMA_FROM_DEVICE);

			/* Now it is safe to examine the buffer. */
			hp = (struct my_card_header *) cp->rx_buf;
			if (header_is_ok(hp)) {
				dma_unmap_single(&cp->dev, cp->rx_dma, cp->rx_len,
						 DMA_FROM_DEVICE);
				pass_to_upper_layers(cp->rx_buf);
				make_and_setup_new_rx_buf(cp);
			} else {
				/* CPU should not write to
				 * DMA_FROM_DEVICE-mapped area,
				 * so dma_sync_single_for_device() is
				 * not needed here. It would be required
				 * for DMA_BIDIRECTIONAL mapping if
				 * the memory was modified.
				 */
				give_rx_buf_to_card(cp);
			}
		}
	}

```
## 错误处理


在某些架构上 DMA 地址空间是有限的，分配失败可以通过以下方式判定
- 检dma_alloc_coherent() 是否返回 NULL，或 dma_map_sg 是否返回 0

- 检dma_map_single() dma_map_page() 返回dma_addr_t
```

	dma_addr_t dma_handle;

	dma_handle = dma_map_single(dev, addr, size, direction);
	if (dma_mapping_error(dev, dma_handle)) {
		/*
		 * reduce current DMA mapping usage,
		 * delay and try again later or
		 * reset driver.
		 */
		goto map_error_handling;
	}

```
- 当在多页映射尝试中途发生映射错误时，取消映射已经映射的页。这些例子同样适用  dma_map_page()
```

	dma_addr_t dma_handle1;
	dma_addr_t dma_handle2;

	dma_handle1 = dma_map_single(dev, addr, size, direction);
	if (dma_mapping_error(dev, dma_handle1)) {
		/*
		 * reduce current DMA mapping usage,
		 * delay and try again later or
		 * reset driver.
		 */
		goto map_error_handling1;
	}
	dma_handle2 = dma_map_single(dev, addr, size, direction);
	if (dma_mapping_error(dev, dma_handle2)) {
		/*
		 * reduce current DMA mapping usage,
		 * delay and try again later or
		 * reset driver.
		 */
		goto map_error_handling2;
	}

	...

	map_error_handling2:
		dma_unmap_single(dma_handle1);
	map_error_handling1:

```
```

	/*
	 * if buffers are allocated in a loop, unmap all mapped buffers when
	 * mapping error is detected in the middle
	 */

	dma_addr_t dma_addr;
	dma_addr_t array[DMA_BUFFERS];
	int save_index = 0;

	for (i = 0; i < DMA_BUFFERS; i++) {

		...

		dma_addr = dma_map_single(dev, addr, size, direction);
		if (dma_mapping_error(dev, dma_addr)) {
			/*
			 * reduce current DMA mapping usage,
			 * delay and try again later or
			 * reset driver.
			 */
			goto map_error_handling;
		}
		array[i].dma_addr = dma_addr;
		save_index++;
	}

	...

	map_error_handling:

	for (i = 0; i < save_index; i++) {

		...

		dma_unmap_single(array[i].dma_addr);
	}

```
网络驱动DMA 映射在发送钩子（ndo_start_xmit）中失败时，必须调用 dev_kfree_skb() 释放套接字缓冲区并返NETDEV_TX_OK。这意味着套接字缓冲区在失败情况下被直接丢弃
SCSI 驱动queuecommand 钩子DMA 映射失败时，必须返回 SCSI_MLQUEUE_HOST_BUSY。这意味着
SCSI 子系统稍后会再次将该命令交给驱动
## 优化取消映射状态空间占

在许多平台上，dma_unmap_{single,page}() 其实就是一个空操作（nop）。因此，记录映射地址长度是在浪费空间。下面的设施提供了办法，而不是用 ifdef 之类的东西把你的驱动填满“绕开”这个问题（那样会违背可移植 API 的整个初衷）
实际上，我们不来逐个描述这些宏，而是转换一些示例代码
1) 在保存状态的结构体中使用 DEFINE_DMA_UNMAP_{ADDR,LEN}```

	struct ring_state {
		struct sk_buff *skb;
		dma_addr_t mapping;
		__u32 len;
	};

   after::

	struct ring_state {
		struct sk_buff *skb;
		DEFINE_DMA_UNMAP_ADDR(mapping);
		DEFINE_DMA_UNMAP_LEN(len);
	};

```
2) 使用 dma_unmap_{addr,len}_set() 来设置这些值```

	ringp->mapping = FOO;
	ringp->len = BAR;

   after::

	dma_unmap_addr_set(ringp, mapping, FOO);
	dma_unmap_len_set(ringp, len, BAR);

```
3) 使用 dma_unmap_{addr,len}() 来访问这些值```

	dma_unmap_single(dev, ringp->mapping, ringp->len,
			 DMA_FROM_DEVICE);

   after::

	dma_unmap_single(dev,
			 dma_unmap_addr(ringp, mapping),
			 dma_unmap_len(ringp, len),
			 DMA_FROM_DEVICE);

```
这应该是不言自明的。我们将 ADDR LEN 分开处理，因为实现有可能只需要地址就能执行
取消映射操作
## 平台相关问题


如果你只是为 Linux 编写驱动，而不维护内核的某个架构移植，你可以安全地跳到“结束语”
1) struct scatterlist 的要求
   如果架构支持 IOMMU（包括软IOMMU），你需要启CONFIG_NEED_SG_DMA_LENGTH
2) ARCH_DMA_MINALIGN

   架构必须确保 kmalloc 分配出的缓冲区是 DMA 安全的。驱动和子系统都依赖它。如果一   架构不是完全 DMA 一致的（即硬件不能确保 CPU 缓存中的数据与主存中的数据相同）   ARCH_DMA_MINALIGN 必须被设置，以便内存分配器确kmalloc 分配的缓冲区不会与其   缓冲区共享缓存行。参arch/arm/include/asm/cache.h 作为例子
   注意，ARCH_DMA_MINALIGN 是关DMA 内存对齐约束的。你不需要担心架构的数据对齐约束
   （例如关64 位对象的对齐约束）
## 结束

如果没有众多个人的反馈和建议，本文档以及 API 本身不会是现在这个样子我们想特别提及以下人士（排名不分先后）：
```

	Russell King <rmk@arm.linux.org.uk>
	Leo Dagum <dagum@barrel.engr.sgi.com>
	Ralf Baechle <ralf@oss.sgi.com>
	Grant Grundler <grundler@cup.hp.com>
	Jay Estabrook <Jay.Estabrook@compaq.com>
	Thomas Sailer <sailer@ife.ee.ethz.ch>
	Andrea Arcangeli <andrea@suse.de>
	Jens Axboe <jens.axboe@oracle.com>
	David Mosberger-Tang <davidm@hpl.hp.com>

```
