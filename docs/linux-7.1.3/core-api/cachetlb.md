## Linux 下的缓存与 TLB 刷新

:Author: David S. Miller <davem@redhat.com>

本文档描述了由 Linux VM 子系统调用的缓存/TLB 刷新接口。它逐一列举了每个接口，
描述了其预期用途，以及在调用该接口之后期望产生的副作用。

下面描述的副作用是针对单处理器（uniprocessor）实现、以及在该单个处理器上应当
发生什么而陈述的。SMP 的情况是一个简单的扩展，即你只需扩展定义，使得某个特定
接口的副作用发生在系统中的所有处理器上。不要让这一点把你吓到，以为 SMP 的
缓存/TLB 刷新必然效率低下，这事实上是一个存在许多优化可能性的领域。例如，如果
能够证实某个用户地址空间从未在某个 cpu 上执行过（参见 mm_cpumask()），就无需
在该 cpu 上对这个地址空间执行刷新。

首先是 TLB 刷新接口，因为它们最简单。在 Linux 中，"TLB" 被抽象为 cpu 用来缓存
从软件页表获得的虚拟->物理地址转换的某种东西。这意味着，如果软件页表发生了改变，
在这个 "TLB" 缓存中就可能存在过时的转换。因此，当发生软件页表更改时，内核会在
页表更改*之后*调用以下刷新方法之一：

1) `void flush_tlb_all(void)`

	最彻底的刷新。在这个接口运行之后，任何先前的页表修改都将对 cpu 可见。

	这通常是在内核页表被更改时调用的，因为这类转换本质上是"全局"的。

2) `void flush_tlb_mm(struct mm_struct *mm)`

	这个接口从 TLB 中刷新整个用户地址空间。运行之后，这个接口必须确保地址
	空间 'mm' 的任何先前的页表修改都将对 cpu 可见。也就是说，在运行之后，
	TLB 中将不会有 'mm' 的任何条目。

	这个接口用于处理整个地址空间的页表操作，例如 fork 和 exec 期间发生的
	操作。

3) ``void flush_tlb_range(struct vm_area_struct *vma,
   unsigned long start, unsigned long end)``

	这里我们要从 TLB 中刷新一个特定范围（用户）的虚拟地址转换。运行之后，
	这个接口必须确保地址空间 'vma->vm_mm' 在 'start' 到 'end-1' 范围内的任何
	先前的页表修改都将对 cpu 可见。也就是说，在运行之后，对于 'start' 到
	'end-1' 范围内的虚拟地址，TLB 中将不会有 'mm' 的条目。

	"vma" 是用于该区域的后备存储。主要地，这用于 munmap() 类型的操作。

	提供这个接口是希望移植（port）能够找到一种合适的高效方法，从 TLB 中移除
	多个页大小的转换，而不是让内核对每个可能被修改的条目调用 flush_tlb_page
	（见下文）。

4) `void flush_tlb_page(struct vm_area_struct *vma, unsigned long addr)`

	这一次我们需要从 TLB 中移除 PAGE_SIZE 大小的转换。'vma' 是 Linux 用来
	跟踪一个进程的 mmap'd 区域的备份结构，地址空间可以通过 vma->vm_mm 获得。
	此外，还可以用 (vma->vm_flags & VM_EXEC) 来测试该区域是否可执行（从而在
	split-tlb 类型的设置中，可能存在于 'instruction TLB' 中）。

	运行之后，这个接口必须确保地址空间 'vma->vm_mm' 对于用户虚拟地址 'addr'
	的任何先前的页表修改都将对 cpu 可见。也就是说，在运行之后，对于虚拟地址
	'addr'，'vma->vm_mm' 的 TLB 中将不会有条目。

	这主要用于缺页（fault）处理期间。

5) ``void update_mmu_cache_range(struct vm_fault *vmf,
   struct vm_area_struct **vma, unsigned long address, pte_t **ptep,
   unsigned int nr)``

	在每一次缺页结束时，调用这个例程来告诉体系结构相关的代码：地址空间
	"vma->vm_mm" 在虚拟地址 "address" 处、针对 "nr" 个连续页，软件页表中现在
	已经存在转换。

	这个例程也在其它各种传递 NULL "vmf" 的地方被调用。

	一个移植可以以它选择的任何方式使用这些信息。例如，它可以利用这个事件来
	为软件管理的 TLB 配置预加载 TLB 转换。sparc64 移植目前就是这样做的。

接下来，我们有缓存刷新接口。一般地，当 Linux 正在把一个已有的虚拟->物理映射
更改成一个新的值时，
```
	1) flush_cache_mm(mm);
	   change_all_page_tables_of(mm);
	   flush_tlb_mm(mm);

	2) flush_cache_range(vma, start, end);
	   change_range_of_page_tables(mm, start, end);
	   flush_tlb_range(vma, start, end);

	3) flush_cache_page(vma, addr, pfn);
	   set_pte(pte_pointer, new_pte_val);
	   flush_tlb_page(vma, addr);

```
缓存级别的刷新总是最先进行，因为这允许我们正确地处理那些缓存是严格（strict）
的、并且要求在把一个虚拟地址从缓存中刷新时，该虚拟地址必须存在一个虚拟->物理
转换的系统。HyperSparc cpu 就是这样一种具有此属性的 cpu。

下面的缓存刷新例程只需要处理对于特定 cpu 而言必要的缓存刷新。大多数情况下，这些
例程必须为具有虚拟索引（virtually indexed）缓存的 cpu 实现，这类缓存在虚拟->物理
转换被更改或移除时必须被刷新。所以，例如，IA32 处理器的物理索引物理标记（physically
indexed physically tagged）缓存就无需实现这些接口，因为缓存是完全同步的，并且
不依赖于转换信息。

以下是这些例程，逐一说明：

1) `void flush_cache_mm(struct mm_struct *mm)`

	这个接口从缓存中刷新整个用户地址空间。也就是说，在运行之后，将不会有
	与 'mm' 关联的缓存行。

	这个接口用于处理整个地址空间的页表操作，例如退出（exit）和 exec 期间
	发生的操作。

2) `void flush_cache_dup_mm(struct mm_struct *mm)`

	这个接口从缓存中刷新整个用户地址空间。也就是说，在运行之后，将不会有
	与 'mm' 关联的缓存行。

	这个接口用于处理整个地址空间的页表操作，例如 fork 期间发生的操作。

	这个选项与 flush_cache_mm 是分开的，以便为 VIPT 缓存允许一些优化。

3) ``void flush_cache_range(struct vm_area_struct *vma,
   unsigned long start, unsigned long end)``

	这里我们要从缓存中刷新一个特定范围（用户）的虚拟地址。在运行之后，对于
	'vma->vm_mm' 在 'start' 到 'end-1' 范围内的虚拟地址，缓存中将不会有条目。

	"vma" 是用于该区域的后备存储。主要地，这用于 munmap() 类型的操作。

	提供这个接口是希望移植能够找到一种合适的高效方法，从缓存中移除多个页大小
	的区域，而不是让内核对每个可能被修改的条目调用 flush_cache_page（见
	下文）。

4) `void flush_cache_page(struct vm_area_struct *vma, unsigned long addr, unsigned long pfn)`

	这一次我们需要从缓存中移除一个 PAGE_SIZE 大小的范围。'vma' 是 Linux 用来
	跟踪一个进程的 mmap'd 区域的备份结构，地址空间可以通过 vma->vm_mm 获得。
	此外，还可以用 (vma->vm_flags & VM_EXEC) 来测试该区域是否可执行（从而在
	"Harvard" 类型的缓存布局中，可能存在于 'instruction cache' 中）。

	'pfn' 指示 'addr' 所转换到的物理页帧（把这个值左移 PAGE_SHIFT 就得到物理
	地址）。应当从缓存中移除的正是这个映射。

	在运行之后，对于虚拟地址 'addr'（它转换到 'pfn'）的 'vma->vm_mm'，缓存中
	将不会有条目。

	这主要用于缺页处理期间。

5) `void flush_cache_kmaps(void)`

	只有当平台使用了 highmem 时才需要实现这个例程。它将在所有的 kmap 被
	失效之前被调用。

	在运行之后，缓存中将不会有从内核虚拟地址范围 PKMAP_ADDR(0) 到
	PKMAP_ADDR(LAST_PKMAP) 的条目。

	这个例程应当在 asm/highmem.h 中实现。

6) `void flush_cache_vmap(unsigned long start, unsigned long end)`
   `void flush_cache_vunmap(unsigned long start, unsigned long end)`

	在这两个接口中，我们要从缓存中刷新一个特定范围（内核）的虚拟地址。在
	运行之后，对于 'start' 到 'end-1' 范围内的虚拟地址，内核地址空间的缓存中
	将不会有条目。

	这两个例程中的第一个在 vmap_range() 安装好页表项之后被调用。第二个在
	vunmap_range() 删除页表项之前被调用。

还有另一整类 cpu 缓存问题，目前需要一整套完全不同的接口来正确处理。最大的问题是
处理器数据缓存中的虚拟别名（virtual aliasing）问题。

你的移植容易受 D-cache 中的虚拟别名影响吗？嗯，如果你的 D-cache 是虚拟索引的、
大小大于 PAGE_SIZE、并且不阻止同一个物理地址的多个缓存行同时存在，那你就遇到了
这个问题。

如果你的 D-cache 有这个问题，首先正确地定义 asm/shmparam.h 中的 SHMLBA，它应该
本质上是你的虚拟寻址 D-cache 的大小（如果大小是可变的，则是最大的可能大小）。这
个设置将强制 SYSv IPC 层只允许用户进程以这个值的倍数来 mmap 共享内存。

  这并不能修复共享 mmap，去看看 sparc64 移植，那里有一种解决此问题的方法
  （特别是 SPARC_FLAG_MMAPSHARED）。

接下来，你必须解决所有其它情况下的 D-cache 别名问题。请记住这样一个事实：对于一个
被映射到某个用户地址空间的给定页，总是至少还有一个映射，即内核在其从 PAGE_OFFSET
开始的线性映射中的映射。所以，一旦第一个用户把一个给定的物理页映射到它的地址空间，
由于内核已经在其虚拟地址处映射了这个页，D-cache 别名问题的潜在存在就立即产生了。

  `void copy_user_page(void **to, void **from, unsigned long addr, struct page *page)`
  `void clear_user_page(void **to, unsigned long addr, struct page **page)`

	这两个例程把数据存入用户匿名页或 COW 页。它允许一个移植高效地避免用户空间
	与内核之间的 D-cache 别名问题。

	例如，一个移植可以在复制期间把 'from' 和 'to' 临时映射到内核虚拟地址。这两个
	页的虚拟地址是这样选择的：内核的加载/存储指令恰好发生在与该页的用户映射
	具有相同"颜色"的虚拟地址上。例如，Sparc64 就使用了这种技术。

	'addr' 参数告诉用户最终将把这个页映射到的虚拟地址，而 'page' 参数给出了指向
	目标 struct page 的指针。

	如果 D-cache 别名不是问题，这两个例程可以简单地直接调用 memcpy/memset，不做
	其它任何事情。

  `void flush_dcache_folio(struct folio *folio)`

        必须在以下情况调用这个例程：

	  a) 内核写入了一个位于页缓存（page cache）中、和/或位于高端内存中的页
	  b) 内核即将从一个页缓存页读取，并且该页的用户空间共享/可写映射可能存在。
	     注意，{get,pin}_user_pages{_fast} 已经对用户在地址空间中找到的任何页调用
	     了 flush_dcache_folio，因此驱动代码很少需要考虑这一点。

```
	     这个例程只需为那些有可能被映射到用户进程地址空间的页缓存页调用。
	     所以例如，在页缓存中处理 vfs 符号链接的 VFS 层代码根本不需要调用这个
	     接口。

```
	"内核写入一个页缓存页" 这个短语具体是指：内核执行存储指令，在该页的内核
	虚拟映射处弄脏（dirty）了该页中的数据。为了处理 D-cache 别名，这里刷新很
	重要，以确保这些内核存储对该页的用户空间映射可见。

	相反的情况同样重要：如果存在共享+可写映射这个文件的用户，我们必须确保
	内核对这些页的读取将看到用户所做的最近的存储。

	如果 D-cache 别名不是问题，这个例程可以在该体系结构上简单地定义为一个空
	操作（nop）。

        在 folio->flags（PG_arch_1）中保留了一个位作为"体系结构私有"。内核保证，
	对于 pagecache 页，当这样一个页首次进入 pagecache 时会清除这个位。

	这使得这些接口能够更高效地实现。它允许人们"推迟"（可能无限期地）实际的
	刷新——如果当前没有用户进程映射这个页的话。关于如何进行这一点，请参阅
	sparc64 的 flush_dcache_folio 和 update_mmu_cache_range 实现作为例子。

	思路是：首先在 flush_dcache_folio() 时刻，如果 folio_flush_mapping() 返回
	一个映射，并且该映射上的 mapping_mapped() 返回 %false，就只是标记体系结构
	私有的页标志位。之后，在 update_mmu_cache_range() 中，会检查这个标志位，
	如果已设置就执行刷新并清除该标志位。

	.. important::

			如果你推迟了刷新，通常重要的是：实际的刷新发生在与该页
			进行弄脏存储的 cpu 相同的 CPU 上。同样，请参阅 sparc64
			以了解如何处理这一点的例子。

  ``void copy_to_user_page(struct vm_area_struct *vma, struct page *page,
  unsigned long user_vaddr, void *dst, void *src, int len)``
  ``void copy_from_user_page(struct vm_area_struct *vma, struct page *page,
  unsigned long user_vaddr, void *dst, void *src, int len)``

	当内核需要在任意的用户页之间复制任意数据（例如为了 ptrace()）时，它将
	使用这两个例程。

	任何需要发生的必要缓存刷新或其它一致性操作都应该在这里进行。如果处理器的
	指令缓存不监听（snoop）cpu 存储，你很可能需要为 copy_to_user_page() 刷新
	指令缓存。

  ``void flush_anon_page(struct vm_area_struct *vma, struct page *page,
  unsigned long vmaddr)``

  	当内核需要访问一个匿名页的内容时，它调用这个函数（目前只有 get_user_pages()）。
	注意：flush_dcache_folio() 故意不对匿名页起作用。默认实现是一个空操作（并且
	对于所有一致（coherent）的体系结构都应保持如此）。对于不一致（incoherent）的
	体系结构，它应该刷新 vmaddr 处该页的缓存。

  ``void flush_icache_range(unsigned long start, unsigned long end)``

  	当内核存储到它将要从中执行的地址时（例如当加载模块时），调用这个函数。

	如果 icache 不监听存储，那么这个例程就需要刷新它。

  ``void flush_icache_page(struct vm_area_struct *vma, struct page *page)``

	flush_icache_page 的全部功能都可以在 flush_dcache_folio 和
	update_mmu_cache_range 中实现。未来，希望完全移除这个接口。

最后一类 API 用于内核内部那些被故意设置成别名（aliased）的地址范围上的
I/O。这类别名是通过使用 vmap/vmalloc API 建立的。由于内核 I/O 经由物理页进行，
I/O 子系统假定用户映射和内核偏移（offset）映射是仅有的别名。这对于 vmap 别名
并不成立，所以内核中任何试图对 vmap 区域做 I/O 的东西都必须手动管理一致性。
它必须通过在做 I/O 之前刷新 vmap 范围、并在 I/O 返回之后使该范围失效来做到
这一点。

  `void flush_kernel_vmap_range(void *vaddr, int size)`

       刷新 vmap 区域中给定虚拟地址范围的内核缓存。这是为了确保内核在 vmap
       范围内修改的任何数据都对物理页可见。这个设计是为了让这个区域能够安全地
       进行 I/O。注意，这个 API **不会**同时刷新该区域的偏移映射别名。

  `void invalidate_kernel_vmap_range(void *vaddr, int size) invalidates`

       令 vmap 区域中给定虚拟地址范围的缓存失效，这可以防止处理器在 I/O 正发生于
       物理页期间、通过推测性地读取数据而把缓存弄脏（stale）。这只对于读取数据
       进入 vmap 区域是必要的。
