## 异构内存管理（HMM
提供基础设施和辅助函数，将非传统内存（如板载 GPU 内存这样的设备内存）集成常规内核路径中，其基石是为这类内存提供专门的 struct page（参见本文档5 7 节）
HMM 还提供可选的 SVM（Share Virtual Memory，共享虚拟内存）辅助函数，即允许
设备以与 CPU 一致的方式透明地访问程序地址，这意味着 CPU 上任何有效的指针对于
设备同样是有效的指针。这对于简化使GPU、DSP FPGA 来代表进程执行各种计算的
高级异构计算变得越来越必要
本文档划分如下：第一节阐述与使用设备特定内存分配器相关的问题。第二节阐述许多
平台固有的硬件限制。第三节概述 HMM 的设计。第四节解释 CPU 页表镜像如何工作以及
HMM 在此上下文中的目的。第五节处理设备内存在内核中如何表示。最后一节介绍一种新迁移辅助函数，允许利用设DMA 引擎
## 使用设备特定内存分配器的问题

具有大量板载内存（数 GB）的设备（如 GPU）历来通过其专用的驱动特定 API 管理内存。这在由设备驱动分配和管理的内存与常规应用程序内存（私有匿名内存、共享内存或
常规文件后端内存）之间造成了割裂。此后我将此方面称为分割地址空间（split address
space）。我用共享地址空间（shared address space）来指代相反的情况：即任何应用程内存区域都可以被设备透明地使用
分割地址空间之所以发生，是因为设备只能访问通过设备特定 API 分配的内存。这意味着
从设备的角度来看，程序中的所有内存对象并不平等，这会使依赖大量库的大型程序变复杂
具体地，这意味着想要利用GPU 这样设备的代码，需要在通用分配的内存（malloc、mmap
私有、mmap 共享）与通过设备驱动 API 分配的内存（最终仍以设备文件的 mmap 结束）之复制对象
对于扁平数据集（数组、网格、图像……）这不太难实现，但对于复杂数据集（列表、树……）
则很难做对。复制一个复杂数据集需要重新映射其每个元素之间的所有指针关系。这容易出错并且由于重复的数据集和地址，程序变得更难调试
分割地址空间还意味着库无法透明地使用它们从核心程序或另一个库获得的数据，因此每个
库可能不得不使用设备特定内存分配器来复制其输入数据集。大型项目受此困扰，并且由于
各种内存复制而浪费资源
复制每个API 以接受由每个设备特定分配器分配的内存作为输入或输出，并不是一个可行的
选择。这将导致库入口点的组合爆炸
最后，随着高级语言构造（C++ 中，也在其他语言中）的发展，编译器现在可以在程序不知情的情况下利GPU 和其他设备。编译器识别的一些模式只有在共享地址空间下才实现。对于所有的其他模式，使用共享地址空间也更为合理
## I/O 总线、设备内存特
I/O 总线由于一些限制而削弱了共享地址空间。大多数 I/O 总线只允许设备对主内存进基本的内存访问；甚至缓存一致性也常常是可选的。CPU 对设备内存的访问限制更多。往往
它不具备缓存一致性
如果我们只考虑 PCIE 总线，那么设备可以访问主内存（通常通过一IOMMU）并CPU 保持
缓存一致。但是，它只允许设备对主内存进行有限的原子操作集合。反方向更糟：CPU 只能
访问设备内存的有限范围，并且不能在其上执行原子操作。因此，从内核的角度来看，设备内不能被视作与普通内存相同
另一个削弱因素是有限的带宽（PCIE 4.0 16 通道下约 32GBytes/s）。这比最快的 GPU
内存 TBytes/s）小 33 倍。最后一个限制是延迟。设备访问主内存的延迟比设备访问其自内存高一整个数量级
一些平台正在开发新I/O 总线或对 PCIE 的增修改，以解决其中一些限制（OpenCAPICCIX）。它们主要允CPU 与设备之间的双向缓存一致性，并允许架构支持的所有原子操作遗憾的是，并非所有平台都遵循这一趋势，一些主要架构在没有硬件解决方案的情况下被遗下来
因此，要使共享地址空间有意义，我们不仅必须允许设备访问任何内存，还必须在设备使内存时允许将任何内存迁移到设备内存（在迁移发生时阻塞 CPU 访问）
## 共享地址空间与迁
HMM 旨在提供两个主要特性。第一个是通过在设备页表中复制 CPU 页表来共享地址空间，使对于进程地址空间中任何有效的主存地址，相同的地址指向相同的物理内存
为了实现这一点，HMM 提供了一组辅助函数来填充设备页表，同时跟CPU 页表的更新。设页表的更新不CPU 页表更新那样简单。要更新设备页表，你必须分配一个缓冲区（或使用预分缓冲区的池）并在其中写入 GPU 特定命令来执行更新（取消映射、缓存失效和刷新……）。这不能
通过所有设备的通用代码完成。正因为如此，HMM 提供辅助函数来把一切可以抽取的共性提取出来，
同时将硬件特定细节留给设备驱动
HMM 提供的第二个机制是一种新ZONE_DEVICE 内存，它允许为设备内存的每一页分配一struct page。这些页是特殊的，因CPU 无法映射它们。然而，它们允许使用现有的迁移机制将
主内存迁移到设备内存，并且从 CPU 的角度来看，一切看起来像是一个被换出到磁盘的页。使struct page 可以与现有的 mm 机制实现最简单、最干净的集成。这里同样，HMM 只提供辅助函数，
首先用于为设备内存热插拔新的 ZONE_DEVICE 内存，其次用于执行迁移。迁移什么以及何时迁移的
策略决策留给设备驱动
注意，CPU 对任何设备页的访问都会触发缺页异常并迁移回主内存。例如，当支撑给CPU 地址
A 的页从主内存页迁移到设备页时，任何对地址 A CPU 访问都会触发缺页异常，并启动一迁移回主内存的过程
有了这两个特性，HMM 不仅允许设备镜像进程地址空间并保CPU 和设备页表同步，还通过迁移
正在被设备主动使用的数据集部分来利用设备内存
## 地址空间镜像实现API

地址空间镜像的主要目标是允许将一CPU 页表复制到设备页表；HMM 帮助保持两者同步。想镜像进程地址空间的设备驱动必须从一个：

```
 int mmu_interval_notifier_insert(struct mmu_interval_notifier *interval_sub,
				  struct mm_struct *mm, unsigned long start,
				  unsigned long length,
				  const struct mmu_interval_notifier_ops *ops);
```

开始。在 ops->invalidate() 回调期间，设备驱动必须对该范围执行更新动作（将范围标记为只读或完全取消映射等）。设备必须在驱动回调返回之前完成更新
当设备驱动想要填充一段虚拟地址时，它可以：

```
  int hmm_range_fault(struct hmm_range *range);
```

如果请求了写访问（见下文），它将在缺失或只读条目上触发缺页异常。缺页异常使用通用mm
缺页代码路径，即
```
 int driver_populate_range(...)
 {
      struct hmm_range range;
      ...

      range.notifier = &interval_sub;
      range.start = ...;
      range.end = ...;
      range.hmm_pfns = ...;

      if (!mmget_not_zero(interval_sub->notifier.mm))
          return -EFAULT;

 again:
      range.notifier_seq = mmu_interval_read_begin(&interval_sub);
      mmap_read_lock(mm);
      ret = hmm_range_fault(&range);
      if (ret) {
          mmap_read_unlock(mm);
          if (ret == -EBUSY)
                 goto again;
          return ret;
      }
      mmap_read_unlock(mm);

      take_lock(driver->update);
      if (mmu_interval_read_retry(&ni, range.notifier_seq) {
          release_lock(driver->update);
          goto again;
      }

      /* 使用 pfns 数组内容来更新设备页表，
       * update 锁的保护*/

      release_lock(driver->update);
      return 0;
 }
```

driver->update 锁与驱动在其 invalidate() 回调内部获取的锁是同一把锁。在调用
mmu_interval_read_retry() 之前必须持有该锁，以避免与并发的 CPU 页表更新发生竞争
## 利用 default_flags pfn_flags_mask

hmm_range 结构体有两个字段，default_flags pfn_flags_mask，它们为整个范围指定缺页快照策略，而不必为 pfns 数组中的每个条目设置
例如，如果设备驱动想要为一段范围中的页至少获取读权限：

```
    range->default_flags = HMM_PFN_REQ_FAULT;
    range->pfn_flags_mask = 0;
```

并如上所述调hmm_range_fault()。这将为范围中所有页至少以读权限填充缺页
现在假设驱动想做同样的事，但范围中有一页例外，需要写权限
```
    range->default_flags = HMM_PFN_REQ_FAULT;
    range->pfn_flags_mask = HMM_PFN_REQ_WRITE;
    range->pfns[index_of_write] = HMM_PFN_REQ_WRITE;
```

这样，HMM 将以至少读（即有效）权限填充所有页，而对于地址 == range->start +
(index_of_write << PAGE_SHIFT)，它将以写权限填充缺页，即如CPU pte 没有设置写权限，
那么 HMM 将调handle_mm_fault()
hmm_range_fault 完成后，标志位被设置为页表的当前状态，即如果页是可写的，将设置
HMM_PFN_VALID | HMM_PFN_WRITE銆。
## 从核心内核角度表示和管理设备内存

曾尝试过几种不同的设计来支持设备内存。第一种使用一个设备特定的数据结构来保存关于已迁移
内存的信息，并且 HMM mm 代码的各个位置挂钩，以处理对任何由设备内存支撑的地址的访问结果这最终复制了 struct page 的大部分字段，并且还需要更新许多内核代码路径以理解这种类型的内存
大多数内核代码路径从不尝试访问页背后的内存，而只关心 struct page 的内容。因此，HMM 转直接使用 struct page 来表示设备内存，这使得大多数内核代码路径意识不到其中的差异。我们只
需要确保永远没有人尝试CPU 一侧映射这些页
## 向设备内存迁移及从设备内存迁
由于 CPU 不能直接访问设备内存，设备驱动必须使用硬DMA 或设备特定的加载/存储指令迁移数据。migrate_vma_setup()、migrate_vma_pages() migrate_vma_finalize() 函数
旨在使驱动更易编写，并集中各驱动之间通用的代码
在将页迁移到设备私有内存之前，需要创建特殊的设备私有 `struct page`。它们将被用作特殊的
"交换"页表项，以便 CPU 进程如果试图访问已迁移到设备私有内存的页时会触发缺页异常
```
    struct resource *res;
    struct dev_pagemap pagemap;

    res = request_free_mem_region(&iomem_resource, /* 瀛楄妭鏁?*/,
                                  "name of driver resource");
    pagemap.type = MEMORY_DEVICE_PRIVATE;
    pagemap.range.start = res->start;
    pagemap.range.end = res->end;
    pagemap.nr_range = 1;
    pagemap.ops = &device_devmem_ops;
    memremap_pages(&pagemap, numa_node_id());

    memunmap_pages(&pagemap);
    release_mem_region(pagemap.range.start, range_len(&pagemap.range));
```

当资源可以与一`struct device` 绑定时，还有 devm_request_free_mem_region()devm_memremap_pages()、devm_memunmap_pages() devm_release_mem_region()
整体迁移步骤与在系统内存内迁NUMA 页类似（参见 Documentation/mm/page_migration.rst），
但步骤被拆分到设备驱动特定代码和共享通用代码之间
1. `mmap_read_lock()`

   设备驱动必须migrate_vma_setup() 传递一`struct vm_area_struct`，因此在迁移期间
   需要持mmap_read_lock() mmap_write_lock()
2. `migrate_vma_setup(struct migrate_vma *args)`

   设备驱动初始`struct migrate_vma` 字段并将指针传递给 migrate_vma_setup()   `args->flags` 字段用于过滤应该迁移哪些源页。例如，设置 `MIGRATE_VMA_SELECT_SYSTEM`
   将只迁移系统内存，`MIGRATE_VMA_SELECT_DEVICE_PRIVATE` 将只迁移驻留在设备私有内存中   页。如果设置了后一个标志，`args->pgmap_owner` 字段用于标识由该驱动拥有的设备私有页。这
   避免了尝试迁移驻留在其他设备中的设备私有页。目前只有匿名的私有 VMA 范围可以在系统内存和
   设备私有内存之间相互迁移
   在遍历页表时，migrate_vma_setup() 做的第一步之一是用
   `mmu_notifier_invalidate_range_start()` 鍜?`mmu_notifier_invalidate_range_end()`
   调用包裹页表遍历，以将待迁移PFN 填入 `args->src` 数组   `invalidate_range_start()` 回调会收到一`struct mmu_notifier_range`，其 `event` 字段
   被设置为 `MMU_NOTIFY_MIGRATE`，其 `owner` 字段被设置为传递给 migrate_vma_setup()    `args->pgmap_owner` 字段。这允许设备驱动跳过失效回调，只失效实际正在迁移的设备私MMU
   映射。这将在下一节中进一步解释
   在遍历页表时，`pte_none()` `is_zero_pfn()` 条目会得到一个有效的" PFN 存储   `args->src` 数组中。这让驱动分配设备私有内存并清零它，而不是复制一个全零的页。指向系   内存或设备私struct page 的有PTE 条目会被 `lock_page()` 锁定，从 LRU 中隔离（如果   系统内存，因为设备私有页不在 LRU 上），从进程中取消映射，并在原始 PTE 的位置插入一个特殊的
   迁移 PTE。migrate_vma_setup() 还会清空 `args->dst` 数组
3. 设备驱动分配目标页并将源页复制到目标页
   驱动检查每`src` 条目，看 `MIGRATE_PFN_MIGRATE` 位是否被设置，并跳过不迁移的条目。设   驱动也可以选择不填写该页的 `dst` 数组来跳过迁移某一页
   然后驱动要么分配一个设备私struct page，要么分配一个系统内存页，用 `lock_page()` 锁定
   该页，并填写
```
     dst[i] = migrate_pfn(page_to_pfn(dpage));

   既然驱动现在知道该页正在被迁移，它可以失效设备私MMU 映射，并将设备私有内存复制到系统
   内存或另一个设备私有页。核Linux 内核处理 CPU 页表失效，因此设备驱动只需要失效它自己   MMU 映射
   驱动可以使用 ``migrate_pfn_to_page(src[i])`` 来获取源页的 ``struct page``，并将源页复   到目标，或者如果指针为 ``NULL``（意味着源页尚未在系统内存中填充），则清空目标设备私有内存```

4. `migrate_vma_pages()`

   这一步是迁移实际上被"提交"的地方
   如果源页是一`pte_none()` `is_zero_pfn()` 页，这是新分配的页被插入CPU 页表   地方。如CPU 线程在同一页上发生缺页，这可能失败。但是，页表是被锁定的，且只会插入一个新
   页中的一个。如果驱动在竞争中失败，它会看到 `MIGRATE_PFN_MIGRATE` 位被清除
   如果源页被锁定、隔离等，源 `struct page` 信息现在被复制到目标 `struct page`，在 CPU 一   完成迁移
5. 设备驱动为仍在迁移的页更新设MMU 页表，回滚不迁移的页
   如果 `src` 条目仍有 `MIGRATE_PFN_MIGRATE` 位设置，设备驱动可以更新设备 MMU，并   `MIGRATE_PFN_WRITE` 位被设置时设置写使能位
6. `migrate_vma_finalize()`

   这一步用新页的页表项替换特殊的迁移页表项，并释放对源和目`struct page` 的引用
7. `mmap_read_unlock()`

   现在可以释放锁了
## 独占访问内存

一些设备具有诸如原PTE 位这样的特性，可用于实现对系统内存的原子访问。为了支持对共享虚拟
内存页的原子操作，这样的设备需要对该页的独占访问，排除来自 CPU 的任何用户空间访问`make_device_exclusive()` 函数可用于使一段内存范围对用户空间不可访问
这将给定范围中页的所有映射替换为特殊的交换条目。任何访问该交换条目的尝试都会导致一次缺异常，该异常通过用原始映射替换该条目来解决。驱动通过 MMU notifier 被告知映射已更改，此它将不再对该页拥有独占访问。独占访问保证持续到驱动释放页锁和页引用为止，在此时任何 CPU 该页的缺页都可以按上述方式继续
## 内存 cgroup（memcg）与 rss 统计

目前，设备内存像 rss 计数器中的任何常规页一样被统计（如果设备页用于匿名内存则为 anonymous用于文件后端页则file，用于共享内存则shmem）。这是一个深思熟虑的选择，以使可能在不知的情况下开始使用设备内存的现有应用程序不受影响地继续运行
一个缺点是，OOM killer 可能会杀掉一个使用大量设备内存而使用少量常规系统内存的应用程序，从不会释放太多系统内存。我们想在决定以不同方式统计设备内存之前，收集更多关于在存在设备内存的情下应用程序和系统如何在内存压力下反应的真实世界经验
对于内存 cgroup 也做了同样的决定。设备内存页被计入常规页所计入的同一内存 cgroup 中。这确实
简化了向设备内存和从设备内存的迁移。这也意味着从设备内存迁回常规内存不会失败，因为它不会超内存 cgroup 限制。一旦我们在设备内存的使用及其对内存资源控制的影响方面获得更多经验，我们可能
会在以后重新考虑这一选择
注意，设备内存永远不能被设备驱动固定（pin），也不能通过 GUP 固定，因此这样的内存总是在进退出时被释放。或者，在共享内存或文件后端内存的情况下，当最后一个引用被丢弃时释放