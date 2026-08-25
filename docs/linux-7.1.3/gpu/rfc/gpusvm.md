
## GPU SVM 章节


## 一致认可的设计原则


- migrate_to_ram 路径
 - 仅依赖核MM 的概念（迁移 PTE、页引用以及页锁定） - 在该路径中，除用于硬件交互的锁之外，不使用任何驱动特定的锁。这些锁并非必需	 通常也不是好主意去发明驱动自定义的锁来弥补核MM 的竞争问题 - 驱动特定锁引发问题的一个例子发生在修复 do_swap_page 以锁定缺页页之前。在
	 migrate_to_ram 中一个驱动独占锁在足够多线程读取缺页页时会产生稳定的
	 活锁 - 支持部分迁移（即，尝试迁移的页的一个子集实际可以迁移，只有缺页页保证会	 迁移） - 驱动通过重试循环而非加锁来处理混合迁移- 驱逐（Eviction - 驱逐定义为在没有任何虚拟地址的情况下将数据从 GPU 迁回 CPU，以释放 GPU 内存 - 仅查看物理内存数据结构和锁，而非查看虚拟内存数据结构和锁 - 不查mm/vma 结构体，也不依赖它们被锁定 - 上述两点的理由是：CPU 虚拟地址可能随时改变，而物理页保持稳定 - 需GPU 虚拟地址GPU 页表失效，通过能够访问 GPU 虚拟地址notifier 来处理- GPU 缺页 - mmap_read 仅用于需要该锁的核心 MM 函数周围，并且应力求只在 GPU SVM 层获	 mmap_read 锁 - 大的重试循环用于处理gpu 页表mmu notifier 范围无论最终叫什么的所	  mmu notifier 的竞争 - 竞争（尤其是针对并发驱逐或 migrate_to_ram）不应在缺页侧通过尝试持有锁来处理	 而应使用重试循环处理。一个可能的例外是在初次迁移VRAM 期间持有 BO 	 dma-resv 锁，因为这是一个定义良好的锁，可以mmap_read 锁之下获取 - 上述方法的一个可能问题是，如果驱动有严格的迁移策略，要求 GPU 访问发生GPU
	 内存中。并发的 CPU 访问可能因无限重试而导致活锁。虽然当前没GPU SVM 	 用户（Xe）采用这样的策略，但未来很可能会加入。理想情况下，这应在核心 MM
	 一侧解决，而非通过驱动侧锁- 物理内存到虚拟的回指指针
 - 这行不通，因为不应存在从物理内存到虚拟内存的指针。mremap() 就是核心 MM 在不
	 通知驱动地址变更的情况下更新虚拟地址的例子，驱动只收到失notifier - 物理内存回指指针（page->zone_device_data）从分配到页释放应保持稳定。除非页	 空闲的，否则安全地针对并发用户更新它将非常困难- GPU 页表 - Notifier 锁仅保护范围树、某个范围（而非 seqno，因notifier 范围更宽）的	  有效状态、页表项以及 mmu notifier seqno 跟踪，它并不是用于防止竞争的全局锁 - 所有竞争都通过上述大的重试来处理
## 基线设计概述


   :doc: Overview

   :doc: Locking

   :doc: Partial Unmapping of Ranges

   :doc: Examples

## drm_pagemap 设计概述


   :doc: Overview

   :doc: Migration

## 可能的未来设计特

- 并发 GPU 缺页
 - CPU 缺页是并发的，因此拥有并GPU 缺页也合乎情理 - 通过驱动 GPU 缺页处理程序中细粒度的锁应当可以实现 - 预计不需要对 GPU SVM 做改动- 混合系统与设备页的范 - 如果需要，可以相当容易地添加到 drm_gpusvm_get_pages 中- GPU 支持
 - 工作进行中，预计在最初合GPU SVM 之后会有补丁 - 理想情况下可以几乎不改动、甚至完全不改动 GPU SVM 来完成- 用基数树取代范围
 - 为更快的 notifier 可能是可取的- 复合设备 - Nvidia、AMD Intel 都一致认为，迁移设备层中昂贵的核MM 函数是性能瓶颈	 拥有复合设备页应当能通过减少这些昂贵调用的数量来帮助提升性能- 用于迁移的更高阶 dma 映射
 - 4k dma 映射Intel 硬件上的迁移性能有负面影响，更高阶（2M）的 dma 映射应当
	 有所帮助- GPU SVM 之上构建通用userptr 实现
- 驱动侧的 madvise 实现与迁移策- 在相关改动落地时，引Leon / Nvidia 待合并的 dma-mapping API 变更
