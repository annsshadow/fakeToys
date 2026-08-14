..

## 与总线无关的设备访问


:Author: Matthew Wilcox
:Author: Alan Cox

## 简介


Linux 提供了一套 API，它抽象了对所有总线和设备进行 I/O 的操作，使得设备驱动
可以独立于总线类型编写。

## 内存映射 I/O


### 获取对设备的访问


支持最广泛的 I/O 形式是内存映射 I/O。也就是说，CPU 地址空间的一部分不被解释为
对内存的访问，而是被解释为对设备的访问。有些架构将设备定义在一个固定地址上，
但大多数架构都有某种发现设备的方法。PCI 总线遍历就是这种方案的一个好例子。
本文档不涵盖如何获得这样一个地址，而是假设你已经拥有了一个。物理地址的类型
为 unsigned long。

不应直接使用该地址。相反，为了获得一个适合传给下文描述的各种访问函数的地址，
你应该调用 ioremap()。会返回一个适合访问设备的地址。

当你使用完设备后（例如在你的模块退出例程中），调用 iounmap() 以便将地址空间
归还给内核。大多数架构在每次调用 ioremap() 时都会分配新的地址空间，除非你调用
iounmap()，否则它们可能会耗尽。

### 访问设备


驱动使用最多的接口部分是对设备上内存映射的寄存器进行读写。Linux 提供读写 8 位、
16 位、32 位和 64 位数据的接口。由于一个历史意外，它们被命名为字节（byte）、
字（word）、长（long）和四字（quad）访问。读写访问均受支持；目前没有预取支持。

这些函数的名字是 readb()、readw()、readl()、readq()、readb_relaxed()、
readw_relaxed()、readl_relaxed()、readq_relaxed()、writeb()、writew()、writel()
和 writeq()。

有些设备（例如帧缓冲）希望一次使用大于 8 字节的传输。对于这些设备，提供了
memcpy_toio()、memcpy_fromio() 和 memset_io() 函数。不要在 I/O 地址上使用
memset 或 memcpy；它们不保证按顺序复制数据。

读写函数被定义为有序的。也就是说，编译器不允许重排 I/O 序列。当排序可以被编译器
优化时，你可以使用 __readb() 及其同类来表示宽松排序（relaxed ordering）。使用
时务必小心。

虽然基本函数彼此之间被定义为同步且彼此有序，但设备所在的总线本身可能具有异步性。
特别是许多作者都吃过 PCI 总线写操作是异步提交（posted）的亏。驱动作者必须对该
同一设备发出一次读操作，以确保写操作在作者关心的特定情况下已经发生。这种属性无法
在 API 中对驱动编写者隐藏。在某些情况下，用于冲刷设备的读操作可能预期会失败（例如
当卡正在复位时）。在这种情况下，应当从配置空间读取，如果卡没有响应，配置空间保证
会软失败（soft-fail）。

以下是当驱动希望确保写操作的效果在以下情形前可见时，冲刷对设备写操作的一个例子：

```

    static inline void
    qla1280_disable_intrs(struct scsi_qla_host *ha)
    {
        struct device_reg *reg;

        reg = ha->iobase;
        /* disable risc and host interrupts */
        WRT_REG_WORD(&reg->ictrl, 0);
        /*
         * The following read will ensure that the above write
         * has been received by the device before we return from this
         * function.
         */
        RD_REG_WORD(&reg->ictrl);
        ha->flags.ints_enabled = 0;
    }

```
PCI 排序规则还保证 PIO 读响应在任何来自该总线的未完成 DMA 写之后到达，因为对于某些
设备，readb() 调用的结果可能向驱动发出 DMA 事务已完成的信号。然而在许多情况下，
驱动可能希望表明下一次 readb() 调用与此前设备执行的任何 DMA 写无关。在这些情况下
驱动可以使用 readb_relaxed()，尽管只有某些平台会遵循宽松语义。在支持的平台上使用
宽松读函数会带来显著的性能收益。qla2xxx 驱动提供了如何使用 readX_relaxed() 的示例。
在许多情况下，驱动的大部分 readX() 调用都可以安全地转换为 readX_relaxed() 调用，
因为只有少数调用会表示或依赖于 DMA 完成。

## 端口空间访问


### 端口空间解析


另一种常见支持的 I/O 形式是端口空间。它是一段独立于普通内存地址空间的地址范围。
对这些地址的访问通常不如对内存映射地址的访问快，并且它的地址空间也可能更小。

与内存映射 I/O 不同，访问端口空间无需任何准备工作。

### 访问端口空间


对该空间的访问通过一组函数提供，这些函数允许 8 位、16 位和 32 位访问；也称为字节、
字和长。这些函数是 inb()、inw()、inl()、outb()、outw() 和 outl()。

这些函数还提供一些变体。有些设备要求对其端口的访问被放慢。该功能通过在函数末尾
附加一个 `_p` 来提供。也有与 memcpy 等价的函数。ins() 和 outs() 函数将字节、
字或长复制到给定的端口。

## __iomem 指针标记


MMIO 地址的数据类型是一个带 `__iomem` 限定符的指针，例如 `void __iomem *reg`。
在大多数架构上它是一个指向虚拟内存地址的常规指针，可以偏移或解引用，但在可移植
代码中，它只能从显式操作 `__iomem` 标记的函数传入和传出，特别是 ioremap() 以及
readl()/writel() 函数。可以使用 'sparse' 语义代码检查器来验证这一点是否正确完成。

虽然在大多数架构上，ioremap() 会为一个指向物理 MMIO 地址的未缓存虚拟地址创建页表
项，但某些架构需要特殊的 MMIO 指令，而 `__iomem` 指针只是编码物理地址或一个可由
readl()/writel() 解释的、可偏移的 cookie。

## I/O 访问函数之间的差异


readq()、readl()、readw()、readb()、writeq()、writel()、writew()、writeb()

  这些是最通用的访问函数，提供了针对其他 MMIO 访问和 DMA 访问的序列化，以及对
  访问小端 PCI 设备和片上外设的固定字节序。可移植的设备驱动通常应将这些用于任何
  对 `__iomem` 指针的访问。

  注意，提交写操作相对于自旋锁并非严格有序，参见
  Documentation/driver-api/io_ordering.rst。

readq_relaxed()、readl_relaxed()、readw_relaxed()、readb_relaxed()、
writeq_relaxed()、writel_relaxed()、writew_relaxed()、writeb_relaxed()

  在需要对 DMA 序列化需要昂贵屏障的架构上，这些 MMIO 访问函数的“宽松”版本只彼此
  之间序列化，但包含代价更低的屏障操作。设备驱动可能在性能特别敏感的快速路径中使用
  它们，并附带注释解释为什么在特定位置没有额外屏障也是安全的。

  有关非宽松与宽松版本精确排序保证的更详细讨论，请参阅 memory-barriers.txt。

ioread64()、ioread32()、ioread16()、ioread8()、
iowrite64()、iowrite32()、iowrite16()、iowrite8()

  这些是正常的 readl()/writel() 函数的替代，行为几乎完全相同，但它们也可以操作由
  pci_iomap() 或 ioport_map() 映射 PCI I/O 空间所返回的 `__iomem` 标记。在需要
  特殊 I/O 端口访问指令的架构上，这会为 lib/iomap.c 中实现的一个间接函数调用增加
  少量开销，而在其他架构上这些只是别名。

ioread64be()、ioread32be()、ioread16be()
iowrite64be()、iowrite32be()、iowrite16be()

  它们的行为方式与 ioread32()/iowrite32() 系列相同，但字节顺序相反，用于访问具有
  大端 MMIO 寄存器的设备。可以在大端或小端寄存器上运行的设备驱动可能必须实现一个
  自定义的包装函数，根据找到的设备选择其中之一。

  注意：在某些架构上，正常的 readl()/writel() 函数传统上假设设备与 CPU 字节序相同，
  而在运行大端内核时在 PCI 总线上使用硬件字节反转。以这种方式使用 readl()/writel()
  的驱动通常不可移植，但往往局限于特定的 SoC。

hi_lo_readq()、lo_hi_readq()、hi_lo_readq_relaxed()、lo_hi_readq_relaxed()、
ioread64_lo_hi()、ioread64_hi_lo()、ioread64be_lo_hi()、ioread64be_hi_lo()、
hi_lo_writeq()、lo_hi_writeq()、hi_lo_writeq_relaxed()、lo_hi_writeq_relaxed()、
iowrite64_lo_hi()、iowrite64_hi_lo()、iowrite64be_lo_hi()、iowrite64be_hi_lo()

  某些设备驱动有 64 位寄存器，在 32 位架构上无法原子访问，但允许两次连续的 32 位
  访问。由于哪一半必须先访问取决于特定设备，针对每个 64 位访问函数的组合（采用
  低/高或高/低字顺序）都提供了一个辅助函数。设备驱动必须包含 <linux/io-64-nonatomic-lo-hi.h>
  或 <linux/io-64-nonatomic-hi-lo.h> 之一来获取函数定义，并附带在未原生提供 64 位
  访问的架构上将正常的 readq()/writeq() 重定向到它们的辅助函数。

__raw_readq()、__raw_readl()、__raw_readw()、__raw_readb()、
__raw_writeq()、__raw_writel()、__raw_writew()、__raw_writeb()

  这些是不带屏障或字节序变更、具有架构特定行为的底层 MMIO 访问函数。访问通常在
  原子性意义上是原子的，即一个四字节的 __raw_readl() 不会被拆分成单独的字节加载，
  但多个连续访问可以在总线上合并。在可移植代码中，仅用于访问设备总线后的内存而非
  MMIO 寄存器才是安全的，因为相对于其他 MMIO 访问甚至自旋锁都没有排序保证。字节序
  通常与普通内存相同，因此与其他函数不同，这些函数可用于在内核内存和设备内存之间
  复制数据。

inl()、inw()、inb()、outl()、outw()、outb()

  PCI I/O 端口资源传统上需要单独的辅助函数，因为它们在 x86 架构上使用特殊指令实现。
  在大多数其他架构上，这些在内部被映射到 readl()/writel() 风格的访问函数，通常指向
  虚拟内存中的一个固定区域。代替 `__iomem` 指针，地址是一个用于标识端口号的 32 位
  整数标记。PCI 要求 I/O 端口访问是非提交的（non-posted），即 outb() 必须在后续代码
  执行前完成，而普通的 writeb() 可能仍在进行中。在正确实现这一点的架构上，I/O 端口
  访问因此相对于自旋锁是有序的。然而许多非 x86 的 PCI 主机桥实现和 CPU 架构未能
  在 PCI 上实现非提交 I/O 空间，因此它们最终可能在这些硬件上被提交（posted）。

  在某些架构上，I/O 端口号空间与 `__iomem` 指针有一对一映射，但这不推荐，设备驱动
  不应依赖这一点以保证可移植性。类似地，PCI 基址寄存器中描述的 I/O 端口号可能不
  对应于设备驱动所看到的端口号。可移植的驱动需要读取内核提供的资源对应的端口号。

  没有直接的 64 位 I/O 端口访问函数，但可以改用 pci_iomap() 与 ioread64/iowrite64
  的组合。

inl_p()、inw_p()、inb_p()、outl_p()、outw_p()、outb_p()

  在需要特定时序的 ISA 设备上，I/O 访问函数的 _p 版本会增加一个小的延迟。在没有
  ISA 总线的架构上，这些是正常的 inb/outb 辅助函数的别名。

readsq, readsl, readsw, readsb
writesq, writesl, writesw, writesb
ioread64_rep, ioread32_rep, ioread16_rep, ioread8_rep
iowrite64_rep, iowrite32_rep, iowrite16_rep, iowrite8_rep
insl, insw, insb, outsl, outsw, outsb

  这些是多次访问同一地址的辅助函数，通常用于在内核内存字节流和 FIFO 缓冲区之间
  复制数据。与普通的 MMIO 访问函数不同，这些在大端内核上不执行字节交换，因此 FIFO
  寄存器中的第一个字节对应于内存缓冲区中的第一个字节，而与架构无关。

## 设备内存映射模式


某些架构支持多种映射设备内存的模式。ioremap_*() 变体围绕这些架构特定模式提供了
一个公共抽象，具有一组共享的语义。

ioremap() 是最常见的映射类型，适用于典型的设备内存（例如 I/O 寄存器）。如果架构
支持，其他模式可以提供更弱或更强的保证。按从最常见到最不常见的顺序排列，它们如下：

### ioremap()


默认模式，适用于大多数内存映射设备，例如控制寄存器。使用 ioremap() 映射的内存
具有以下特征：

- 未缓存 - 绕过 CPU 侧缓存，所有读写都由设备直接处理
- 无推测操作 - 除非执行该操作的指令已在已提交的程序流中到达，否则 CPU 不得向该
  内存发出读或写
- 无重排序 - CPU 不得相对于彼此重排对该内存映射的访问。在某些架构上，这依赖于
  readl_relaxed()/writel_relaxed() 中的屏障
- 无重复 - CPU 不得为单条程序指令发出多次读或写
- 无写合并 - 每次 I/O 操作都导致向设备发出一次离散的读或写，多次写不会被合并成
  更大的写。在使用 __raw I/O 访问函数或指针解引用时，这可能被强制执行也可能不被
  强制执行
- 不可执行 - 不允许 CPU 从该内存推测指令执行（这或许不言而喻，但你也不允许跳入
  设备内存）

在许多平台和总线（例如 PCI）上，通过 ioremap() 映射发出的写操作是提交的（posted），
这意味着 CPU 不会等待写操作实际到达目标设备就退役该写指令。

在许多平台上，I/O 访问必须相对于访问大小对齐；否则将导致异常或不可预测的结果。

### ioremap_wc()


将 I/O 内存映射为带写合并的普通内存。与 ioremap() 不同：

- CPU 可能会推测性地发出程序实际未执行的、对设备的读操作，并且基本上可以选择读取
  它想要的任何内容
- 只要结果从程序的角度看是一致的，CPU 就可以重排操作
- 即使程序只发出了一次写，CPU 也可能多次写入同一位置
- CPU 可以将多次写合并为单次更大的写

该模式通常用于视频帧缓冲，可以提高写操作的性能。它也可用于设备中的其他内存块
（例如缓冲区或共享内存），但必须小心，因为如果没有显式屏障，访问相对于普通
ioremap() MMIO 寄存器访问不保证有序。

在 PCI 总线上，对标记为 `IORESOURCE_PREFETCH` 的 MMIO 区域使用 ioremap_wc() 通常
是安全的，但不能在没有该标志的区域上使用。对于片上设备，没有相应的标志，但驱动
可以在已知安全的设备上使用 ioremap_wc()。

### ioremap_wt()


将 I/O 内存映射为带写透（write-through）缓存的普通内存。像 ioremap_wc() 一样，
但此外：

- CPU 可以缓存对设备发出的写以及从设备的读，并从该缓存提供读

该模式有时用于视频帧缓冲，此时驱动仍期望写在及时到达设备（而不是卡在 CPU 缓存中），
但读可以从缓存提供以提高效率。不过如今它很少有用，因为帧缓冲驱动通常只执行写操作，
对此 ioremap_wc() 更高效（因为它不会无谓地破坏缓存）。大多数驱动不应使用它。

### ioremap_np()


像 ioremap() 一样，但显式请求非提交写语义。在某些架构和总线上，ioremap() 映射具有
提交写语义，这意味着写操作从 CPU 的角度看似乎“完成”了，而所写数据实际到达目标设备
之前就已经这样了。写操作相对于来自同一设备的其他写和读仍然是有序的，但由于提交写
语义，对于其他设备则不是如此。ioremap_np() 显式请求非提交语义，这意味着写指令在
设备已接收到（并在某种程度上特定于平台地确认了）所写数据之前，不会显得已完成。

此映射模式主要存在以满足需要这种特定映射模式才能正确工作的、带总线 fabric 的平台。
这些平台为需要 ioremap_np() 语义的资源设置 `IORESOURCE_MEM_NONPOSTED` 标志，可移植
驱动应使用一个在适当时自动选择它的抽象（参见下文 `Higher-level ioremap abstractions`_
一节）。

裸 ioremap_np() 仅在某些架构上可用；在其他架构上它总是返回 NULL。驱动通常不应使用它，
除非它们是特定于平台的，或者它们能从支持的非提交写中获益，否则可以回退到 ioremap()。
确保提交写完成的常规做法是在写之后做一次虚拟读，如 `Accessing the device`_ 中所述，
它在所有平台上都适用于 ioremap()。

ioremap_np() 绝不应用于 PCI 驱动。PCI 内存空间写操作始终是提交的，即使在实现了
ioremap_np() 的架构上也是如此。对 PCI BAR 使用 ioremap_np() 最好的结果是提交写语义，
最坏的结果是完全失效。

注意，非提交写语义与 CPU 侧排序保证是正交的。CPU 仍然可以选择在非提交写指令退役之前
发出其他读或写。有关 CPU 侧细节，请参阅前一节关于 MMIO 访问函数的内容。

### ioremap_uc()


ioremap_uc() 仅在对带 PAT 扩展的旧 x86-32 系统以及 ia64（其 ioremap() 行为略有不同）
上有意义，除此之外 ioremap_uc() 默认返回 NULL。

可移植的驱动应避免使用 ioremap_uc()，而改用 ioremap()。

### ioremap_cache()


ioremap_cache() 实际上将 I/O 内存映射为普通 RAM。可以使用 CPU 写回缓存，并且 CPU
可以自由地将该设备当作一块 RAM 来对待。这绝不应用于有任何副作用、或在读时不返回
先前写入数据的设备内存。

它也不应用于实际的 RAM，因为返回的指针是一个 `__iomem` 标记。memremap() 可用于将
位于线性内核内存区域之外的普通 RAM 映射到常规指针。

可移植的驱动应避免使用 ioremap_cache()。

### 架构示例


以下是上述模式如何映射到 ARM64 架构上内存属性设置的说明：

+------------------------+--------------------------------------------+
| API                    | Memory region type and cacheability        |
+------------------------+--------------------------------------------+
| ioremap_np()           | Device-nGnRnE                              |
+------------------------+--------------------------------------------+
| ioremap()              | Device-nGnRE                               |
+------------------------+--------------------------------------------+
| ioremap_uc()           | (not implemented)                          |
+------------------------+--------------------------------------------+
| ioremap_wc()           | Normal-Non Cacheable                       |
+------------------------+--------------------------------------------+
| ioremap_wt()           | (not implemented; fallback to ioremap)     |
+------------------------+--------------------------------------------+
| ioremap_cache()        | Normal-Write-Back Cacheable                |
+------------------------+--------------------------------------------+

## 高层 ioremap 抽象


与其使用上述原始的 ioremap() 模式，更鼓励驱动使用高层 API。这些 API 可以实现特定于
平台的逻辑，以在任何给定总线上自动选择合适的 ioremap 模式，使与平台无关的驱动无需
任何特殊情况即可在这些平台上工作。在撰写本文时，以下 ioremap() 包装函数具有此类逻辑：

devm_ioremap_resource()

  如果 struct resource 上设置了 `IORESOURCE_MEM_NONPOSTED` 标志，可以根据平台要求
  在 ioremap() 之上自动选择 ioremap_np()。使用 devres 在驱动 probe() 函数失败或设备
  从其驱动解绑时自动取消映射资源。

  文档见 Documentation/driver-api/driver-model/devres.rst。

of_address_to_resource()

  为需要非提交写某些总线的平台自动设置 `IORESOURCE_MEM_NONPOSTED` 标志（参见
  nonposted-mmio 和 posted-mmio 设备树属性）。

of_iomap()

  映射设备树中 `reg` 属性描述的资源，完成所有必需的转换。根据平台要求如上自动选择
  ioremap_np()。

pci_ioremap_bar()、pci_ioremap_wc_bar()

  映射 PCI 基址中描述的资源，无需先提取物理地址。

pci_iomap()、pci_iomap_wc()

  像 pci_ioremap_bar()/pci_ioremap_bar() 一样，但在与 ioread32()/iowrite32() 及类似
  访问函数结合使用时也适用于 I/O 空间

pcim_iomap()

  像 pci_iomap() 一样，但使用 devres 在驱动 probe() 函数失败或设备从其驱动解绑时
  自动取消映射资源

  文档见 Documentation/driver-api/driver-model/devres.rst。

不使用这些包装函数可能会使驱动在某些对映射 I/O 内存有更严格规则的平台 unusable。

## 泛化对系统与 I/O 内存的访问


   :doc: overview

   :internal:

## 提供的公共函数


   :internal:
