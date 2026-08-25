## DMA 属

本文档描述了linux/dma-mapping.h 中定义的 DMA 属性的语义
### DMA_ATTR_WEAK_ORDERING


DMA_ATTR_WEAK_ORDERING 指明对该映射的读写可以弱序（weakly ordered）执行，
即读和写之间可以互相越过（pass each other）
由于各平台对 DMA_ATTR_WEAK_ORDERING 的实现是可选的，未实现它的平台简单地忽略该属性，并表现出默认行为
### DMA_ATTR_WRITE_COMBINE


DMA_ATTR_WRITE_COMBINE 指明对映射的写操作可以被缓冲以提升性能
由于各平台对 DMA_ATTR_WRITE_COMBINE 的实现是可选的，未实现它的平台简单地忽略该属性，并表现出默认行为
### DMA_ATTR_NO_KERNEL_MAPPING


DMA_ATTR_NO_KERNEL_MAPPING 让平台避免为所分配的缓冲创建内核虚拟映射在某些架构上，创建这样的映射并非易事，并且会消耗非常有限的资源（例内核虚拟地址空间dma consistent 地址空间）。使用该属性分配的缓冲只能
通过调用 dma_mmap_attrs() 传递给用户空间。使用该 API 时，你保证不解引dma_alloc_attr() 返回的指针。你可以将其视为一cookie，必传递给 dma_mmap_attrs() dma_free_attrs()。请确保这两次调用也都设置了
该属性
由于各平台对 DMA_ATTR_NO_KERNEL_MAPPING 的实现是可选的，未实现它的平台
会简单地忽略该属性，并表现出默认行为
### DMA_ATTR_SKIP_CPU_SYNC


默认情况下，dma_map_{single,page,sg} 函数族会将给定缓冲从 CPU 域转移到
设备域。一些高级用例可能需要在多个设备之间共享一个缓冲。这要求为每设备分别创建映射，通常通过对给定缓冲多次调dma_map_{single,page,sg}
函数、每次传入参与缓冲共享的各设备的设备指针来实现。第一次调用将缓冲
从“CPU”域转移到“device”域，这会同步该区域CPU 缓存（通常意味着根据
dma 方向刷新或使缓存失效）。然而，针对其他设备后续dma_map_{single,page,sg}()
的调用，会对 CPU 缓存执行完全相同的同步操作。CPU 缓存同步可能是一项耗时操作，尤其是当缓冲很大时，因此如有可能应尽量避免。DMA_ATTR_SKIP_CPU_SYNC
允许平台代码跳过给定缓冲CPU 缓存同步，前提是它已经被转移到“device”域该属性也可用dma_unmap_{single,page,sg} 函数族，以在释放映射后强制缓停留在设备域。请小心使用该属性！

### DMA_ATTR_FORCE_CONTIGUOUS


默认情况下，如果分配出的缓冲能够作为连续块映射到设备 dma 地址空间DMA-mapping 子系统允许由 dma_alloc_attrs() 函数从离散页面拼装出该缓冲通过指定该属性，所分配的缓冲被强制在物理内存中也保持连续
### DMA_ATTR_ALLOC_SINGLE_PAGES


这是DMA-mapping 子系统的一种提示：试图以能提供更好 TLB 效率（即值得用大页来构建映射）的方式分配内存可能并不划算。在以下情况下你可能
希望指定该属性：

- 你知道对该内存的访问不会TLB 抖动（thrash）  你可能知道访问很可能是顺序的，或者虽然不是顺序的，但你不太可能在
  很可能位于不同物理页的多个地址之间来回切换（ping-pong）- 你知道访问该内存TLB 缺失的代价足够小，以至于可以忽略不计。如果你
  正在执行解密或解压缩之类的繁重操作，就可能是这种情况- 你知道该 DMA 映射相当短暂。如果你预期映射的生命周期很短，那么优化
  分配（避免凑出大页）可能比获取大页带来的轻微性能提升更值得
设置该提示并不能保证你不会得到大页，但它意味着我们不会那么努力地去
获取大页
	  although ARM64 patches will likely be posted soon.

### DMA_ATTR_NO_WARN


这告DMA-mapping 子系统抑制分配失败报告（类似__GFP_NOWARN）
在某些架构上，分配失败会以错误消息的形式报告到系统日志。虽然这有助识别和调试问题，但那些能自行处理失败（例如稍后重试）的驱动并不会这些消息困扰，并且根据重试机制的实现方式，它们实际上可能用根本不问题的错误消息淹没系统日志
因此，这提供了一种方式，让驱动在分配失败不成问题的调用处避免这些错误
消息，而不应打扰日志

### DMA_ATTR_PRIVILEGED


一些高级外设，例如远程处理器和 GPU，会以特权“supervisor”和非特权“user两种模式访问 DMA 缓冲。该属性用于向 DMA-mapping 子系统指明，该缓冲在
提升后的特权级别是完全可访问的（理想情况下在较低特权级别是不可访问或
至少只读的）
### DMA_ATTR_MMIO


该属性指明物理地址并非普通系统内存。它不得kmap*()/phys_to_virt()/
phys_to_page() 等函数一起使用，它可能不可缓存，并且可能不允许使CPU
load/store 指令访问
通常这用于描MMIO 地址，或其他不可缓存的寄存器地址。在对这类地址进行
DMA 映射时，我们将该操作称为点对点（Peer to Peer），因为一个设备正对另一个设备执DMA。对PCI 设备，必须使p2pdma API 来判DMA_ATTR_MMIO 是否合适
对于需要刷新缓存以保证 DMA 一致性的架构，DMA_ATTR_MMIO 不会执行任何
缓存刷新。所提供的地址绝不能映射为 CPU 可缓存的
### DMA_ATTR_DEBUGGING_IGNORE_CACHELINES


该属性指明，对于DMA_FROM_DEVICE DMA_BIDIRECTIONAL 映射的缓冲，CPU
缓存行可能会重叠
当调用者为位于同一缓存行内的多个小缓冲建立映射时，就可能发生这种重叠在这种情况下，调用者必须保证在映射建立之后，CPU 不会弄脏（dirty）这缓存行。满足该条件时，多个缓冲可以安全地共享同一个缓存行，而不会有
数据损坏的风险
所有共享同一缓存行的映射都必须设置该属性，以抑制关于重叠映射的 DMA
调试告警
### DMA_ATTR_REQUIRE_COHERENT


带有 DMA_ATTR_REQUIRE_COHERENT DMA 映射请求，在任何需SWIOTLB 缓存管理的系统上都会失败。它只应用于支持需要和用户空间进程保持持续
硬件 DMA 一致性的 uAPI 设计，例RDMA DRM。所映射的内存至少必须是
来自 pin_user_pages() 或类似函数的用户空间内存
在构建其 uAPI 时，驱动应尽可能考虑使用 dma_mmap_pages() 来代替该接口
它绝不能被仅与内核内存协作的内核态驱动所使用