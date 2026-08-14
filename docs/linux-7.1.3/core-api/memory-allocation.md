
## 内存分配指南


Linux 提供了多种用于内存分配的 API。您可以使用 `kmalloc` 或 `kmem_cache_alloc` 系列分配小块，使用 `vmalloc` 及其变体分配大的虚拟连续区域，或者直接使用 `alloc_pages` 从页分配器请求页。也可以使用更专门的分配器，例如 `cma_alloc` 或 `zs_malloc`。

大多数内存分配 API 使用 GFP 标志来表达应如何分配该内存。GFP 这个缩写代表 “get free pages”，即底层的分配函数。

分配 API 的多样性，加上众多的 GFP 标志，使得 “我应该如何分配内存？” 这个问题并不容易回答，尽管很可能您应该使用

```

  kzalloc(<size>, GFP_KERNEL);

```
当然，有些情况下必须使用其他分配 API 和不同的 GFP 标志。

## Get Free Page 标志


GFP 标志控制分配器的行为。它们告诉可以使用哪些内存区（zone），分配器应该多努力地寻找空闲内存，内存是否可以被用户空间访问等等。Documentation/core-api/mm-api.rst <mm-api-gfp-flags> 提供了 GFP 标志及其组合的参考文档，这里我们简要概述它们的推荐使用方式：

  - 大多数时候 `GFP_KERNEL` 就是您需要的。用于内核数据结构、可 DMA 内存、inode 缓存的内存，所有这些以及许多其他分配类型都可以使用 `GFP_KERNEL`。请注意，使用 `GFP_KERNEL` 隐含了 `GFP_RECLAIM`，这意味着在内存在压力下时可能触发直接回收（direct reclaim）；调用上下文必须允许睡眠。
  - 如果分配是在原子上下文中执行的，例如中断处理程序，使用 `GFP_NOWAIT`。该标志阻止直接回收以及 IO 或文件系统操作。因此，在内存压力下 `GFP_NOWAIT` 分配很可能失败。此标志的用户需要在适当的地方提供合适的回退以应付此类失败。
  - 如果您认为访问内存保留（memory reserves）是合理的，并且除非分配成功否则内核将承受压力，您可以使用 `GFP_ATOMIC`。
  - 由用户空间触发的不受信任分配应受 kmem 记账约束，并且必须设置 `__GFP_ACCOUNT` 位。对于应该记账的 `GFP_KERNEL` 分配，有一个方便的 `GFP_KERNEL_ACCOUNT` 快捷方式。
  - 用户空间分配应使用 `GFP_USER`、`GFP_HIGHUSER` 或 `GFP_HIGHUSER_MOVABLE` 标志之一。标志名越长，限制越少。

    `GFP_HIGHUSER_MOVABLE` 不要求分配的内存能被内核直接访问，并隐含数据是可移动的。

    `GFP_HIGHUSER` 意味着分配的内存不可移动，但不要求能被内核直接访问。一个例子可能是将映射数据直接映射到用户空间但没有寻址限制的硬件分配。

    `GFP_USER` 意味着分配的内存不可移动，且必须能被内核直接访问。

您可能注意到现有代码中相当多的分配指定了 `GFP_NOIO` 或 `GFP_NOFS`。历史上，它们用于防止由直接内存回收回调到 FS 或 IO 路径并阻塞在已持有的资源上所引起的递归死锁。自 4.12 起，解决此问题的首选方式是使用 Documentation/core-api/gfp_mask-from-fs-io.rst <gfp_mask_from_fs_io> 中描述的新作用域 API。

其他遗留的 GFP 标志是 `GFP_DMA` 和 `GFP_DMA32`。它们用于确保分配的内存能被寻址能力有限的硬件访问。因此，除非您正在为具有此类限制的设备编写驱动，否则避免使用这些标志。即使对于具有限制的硬件，也最好使用 `dma_alloc*` API。

### GFP 标志与回收行为


内存分配可能触发直接回收或后台回收，理解页分配器为满足某个请求会多努力是有用的。

  - `GFP_KERNEL & ~__GFP_RECLAIM` - 乐观分配，完全_不_尝试释放内存。最轻量的模式，甚至不会唤醒后台回收。应谨慎使用，因为它可能耗尽内存，使下一个用户遇到更激进的回收。

  - `GFP_KERNEL & ~__GFP_DIRECT_RECLAIM`（或 `GFP_NOWAIT`）- 乐观分配，不尝试从当前上下文释放内存，但如果 zone 低于低水位线（low watermark）可以唤醒 kswapd 来回收内存。可以用于原子上下文，或者当请求是一个性能优化且慢路径有另一个回退时。

  - `(GFP_KERNEL|__GFP_HIGH) & ~__GFP_DIRECT_RECLAIM`（即 `GFP_ATOMIC`）- 非睡眠分配，带有昂贵的回退，因此它可以访问一部分内存保留。通常用于中断/底半部（bottom-half）上下文，带有昂贵的慢路径回退。

  - `GFP_KERNEL` - 允许后台和直接回收，并使用**默认**的页分配器行为。这意味着非昂贵的分配请求基本上不会失败，但对此行为没有保证，因此调用者必须正确检查失败（例如，目前 OOM killer 的牺牲品是允许失败的）。

  - `GFP_KERNEL | __GFP_NORETRY` - 覆盖默认分配器行为，所有分配请求尽早失败，而不是引起破坏性的回收（在此实现中是一轮回收）。不会调用 OOM killer。

  - `GFP_KERNEL | __GFP_RETRY_MAYFAIL` - 覆盖默认分配器行为，所有分配请求都真的非常努力。如果回收无法取得任何进展，请求将失败。不会触发 OOM killer。

  - `GFP_KERNEL | __GFP_NOFAIL` - 覆盖默认分配器行为，所有分配请求将无限循环直到成功。这可能非常危险，特别是对于较大的阶（order）。

## 选择内存分配器


分配内存最直接的方式是使用 kmalloc() 系列中的一个函数。并且，为了安全起见，最好使用将内存置零的例程，例如 kzalloc()。如果您需要为数组分配内存，有 kmalloc_array() 和 kcalloc() 辅助函数。辅助函数 struct_size()、array_size() 和 array3_size() 可用于安全地计算对象大小而不发生溢出。

用 `kmalloc` 可分配块的最大尺寸是受限的。实际限制取决于硬件和内核配置，但最佳实践是对小于页大小的对象使用 `kmalloc`。

用 `kmalloc` 分配的块的地址至少对齐到 ARCH_KMALLOC_MINALIGN 字节。对于大小为 2 的幂的情况，对齐也保证至少为相应的大小。对于其他大小，对齐保证至少为大小的最大 2 的幂因子。

用 kmalloc() 分配的块可以用 krealloc() 调整大小。类似于 kmalloc_array()：提供了一个用于调整数组大小的辅助函数 krealloc_array()。

对于大的分配，您可以使用 vmalloc() 和 vzalloc()，或者直接从页分配器请求页。由 `vmalloc` 及相关函数分配的内存不是物理连续的。

如果您不确定分配大小是否对 `kmalloc` 来说过大，可以使用 kvmalloc() 及其变体。它会尝试用 `kmalloc` 分配内存，如果分配失败，将用 `vmalloc` 重试。对于哪些 GFP 标志可以与 `kvmalloc` 一起使用是有限制的；请参阅 kvmalloc_node() 参考文档。请注意，`kvmalloc` 可能返回物理上不连续的内存。

如果您需要分配许多相同的对象，可以使用 slab 缓存分配器。该缓存在使用前必须通过 kmem_cache_create() 或 kmem_cache_create_usercopy() 设置。如果缓存的一部分可能被复制到用户空间，则应使用第二个函数。缓存创建后，kmem_cache_alloc() 及其便捷包装函数可以从该缓存分配内存。

当分配的内存不再需要时，必须将其释放。

由 `kmalloc` 分配的对象可以用 `kfree` 或 `kvfree` 释放。由 `kmem_cache_alloc` 分配的对象可以用 `kmem_cache_free`、`kfree` 或 `kvfree` 释放，其中后两个可能更方便，因为不需要 kmem_cache 指针。

同样的规则适用于 _bulk 和 _rcu 变体的释放函数。

由 `vmalloc` 分配的内存可以用 `vfree` 或 `kvfree` 释放。由 `kvmalloc` 分配的内存可以用 `kvfree` 释放。由 `kmem_cache_create` 创建的缓存应仅在先释放所有已分配对象之后，用 `kmem_cache_destroy` 释放。
