## The genalloc/genpool subsystem


内核中有许多内存分配子系统，每个都针对特定的需求。然而，有时内核开发者需要为特定范围的专用内存实现一个新的分配器；这些内存通常位于某处的设备上。该设备的驱动作者当然可以写一个小分配器来完成工作，但那正是用几十个测试不佳的分配器塞满内核的途径。早2005 年，Jes Sorensen sym53c8xx_2 驱动中提取了其中一个分配器，并将其作为一个用于创建临时（ad hoc）内存分配器的通用模块发布_。这段代码在 2.6.13 版本中被合并；从那以后已经做了大量修改

使用此分配器的代码应包含 <linux/genalloc.h>。一切始于使用以下之一创建一个池
   :functions: gen_pool_create		

   :functions: devm_gen_pool_create

调用 gen_pool_create() 将创建一个池。分配的粒度min_alloc_order 设置；它是一个以 2 为底的对数，类似于页分配器使用的那些，但它指的是字节而不是页。因此，如果 min_alloc_order 传入 3，那么所有分配都将是 8 字节的倍数。增min_alloc_order 会减少跟踪池中内存所需的内存。nid 参数指定应使用哪NUMA 节点来分配内部簿记结构；如果调用者不关心，可以传-1
"托管"接口 devm_gen_pool_create() 将池绑定到特定设备。除此之外，它会在给定设备被销毁时自动清理该池
使用以下方式关闭一个池
   :functions: gen_pool_destroy

值得注意的是，如果给定池中仍有未释放的分配，此函数会采取相当极端的步骤——调BUG()，使整个系统崩溃。你已经被警告了
新创建的池没有可分配的内存。在这种状态下它相当无用，因此首先要做的通常就是向池中添加内存。这可以通过以下之一完成
   :functions: gen_pool_add

   :functions: gen_pool_add_owner

调用 gen_pool_add() 会将addr（在内核虚拟地址空间中）开始的 size 字节内存放入给定池，再次使用 nid 作为辅助内存分配的节ID。gen_pool_add_virt() 变体将显式的物理地址与该内存关联；仅当该池将用于 DMA 分配时才需要这样做
用于从池中分配内存（以及将其归还）的函数是：

   :functions: gen_pool_alloc

   :functions: gen_pool_dma_alloc

   :functions: gen_pool_free_owner

正如人们所期望的，gen_pool_alloc() 将从给定池中分配 size< 字节。gen_pool_dma_alloc() 变体分配用于 DMA 操作的内存，并在 dma 所指向的空间中返回关联的物理地址。这只有在内存是通过 gen_pool_add_virt() 添加时才有效。请注意，此函数偏离了通常使用 unsigned long 值表示内核地址genpool 模式；它反而返回一void *
这些都看起来相对简单；事实上，一些开发者显然觉得它太简单了。毕竟，上面的接口无法控制分配函数如何选择要返回的哪一块特定内存。如果需要这类控制，以下函数会引起你的兴趣：

   :functions: gen_pool_alloc_algo_owner

   :functions: gen_pool_set_algo

使用 gen_pool_alloc_algo() 的分配会指定一个用于选择待分配内存的算法；默认算法可以通过 gen_pool_set_algo() 设置。data 值会传递给算法；大多数会忽略它，但偶尔会需要。自然，可以编写一个专用算法，但已经有一套相当丰富的可用算法
- gen_pool_first_fit 是一个简单的首次适配（first-fit）分配器；如果未指定其他算法，这是默认算法
- gen_pool_first_fit_align 强制分配具有特定的对齐（通过 genpool_data_align 结构中的 data 传入）
- gen_pool_first_fit_order_align 将分配对齐到大小的次数（order）。例如，一60 字节的分配将因此64 字节对齐
- gen_pool_best_fit，正如人们所期望的，是一个简单的最佳适配（best-fit）分配器
- gen_pool_fixed_alloc 在池内特定的偏移（通过 data 参数genpool_data_fixed 结构中传入）处分配。如果指示的内存不可用，则分配失败
还有少数其他函数，主要用于查询池中可用空间或遍历内存块等目的。然而，大多数用户应该不需要超出上述描述的内容。幸运的话，让更多人了解这个模块将有助于防止将来编写专用内存分配器
   :functions: gen_pool_virt_to_phys

   :functions: gen_pool_for_each_chunk

   :functions: gen_pool_has_addr

   :functions: gen_pool_avail

   :functions: gen_pool_size

   :functions: gen_pool_get

   :functions: of_gen_pool_get
