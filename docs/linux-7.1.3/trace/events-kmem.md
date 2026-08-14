## Subsystem Trace Points: kmem


kmem 跟踪系统捕获内核中与对象和页面分配相关的事件。大致可以分为五个主要类别。

  - 未知类型的小对象的 Slab 分配（kmalloc）
  - 已知类型的小对象的 Slab 分配
  - 页面分配
  - Per-CPU 分配器活动
  - 外部碎片

本文档描述了每个跟踪点是什么，以及它们为何可能有用。

## 1. Slab allocation of small objects of unknown type

```

  kmalloc		call_site=%lx ptr=%p bytes_req=%zu bytes_alloc=%zu gfp_flags=%s
  kmalloc_node	call_site=%lx ptr=%p bytes_req=%zu bytes_alloc=%zu gfp_flags=%s node=%d
  kfree		call_site=%lx ptr=%p

```
这些事件的高度活跃可能表明有必要使用一个特定的缓存（cache），特别是当 kmalloc slab 页由于分配模式而出现严重内部碎片时。通过将 kmalloc 与 kfree 关联起来，有可能识别出内存泄漏以及分配发生的位置。


## 2. Slab allocation of small objects of known type

```

  kmem_cache_alloc	call_site=%lx ptr=%p bytes_req=%zu bytes_alloc=%zu gfp_flags=%s
  kmem_cache_alloc_node	call_site=%lx ptr=%p bytes_req=%zu bytes_alloc=%zu gfp_flags=%s node=%d
  kmem_cache_free		call_site=%lx ptr=%p

```
这些事件在使用方式上与 kmalloc 相关事件类似，只是更容易将事件定位到特定的缓存。在撰写本文时，尚无法获取正在从哪个 slab 分配的信息，但 call_site 通常可以用来推断该信息。

## 3. Page allocation

```

  mm_page_alloc		  page=%p pfn=%lu order=%d migratetype=%d gfp_flags=%s
  mm_page_alloc_zone_locked page=%p pfn=%lu order=%u migratetype=%d cpu=%d percpu_refill=%d
  mm_page_free		  page=%p pfn=%lu order=%d
  mm_page_free_batched	  page=%p pfn=%lu order=%d cold=%d

```
这四个事件处理页面的分配与释放。mm_page_alloc 是页面分配器活动的一个简单指示器。页面可能从 per-CPU 分配器（高性能）或伙伴（buddy）分配器分配。

如果页面直接从伙伴分配器分配，则会触发 mm_page_alloc_zone_locked 事件。该事件很重要，因为大量的活动意味着 zone->lock 上的活动很高。获取该锁会通过禁用中断、在 CPU 之间弄脏缓存行以及对多个 CPU 串行化而损害性能。

当调用者直接释放一个页面时，只会触发 mm_page_free 事件。这里大量的活动可能表明调用者应当批处理它们的活动。

当页面被批量释放时，还会触发 mm_page_free_batched。大致而言，页面会成批地从 LRU 锁上取下，并通过一个页列表批量释放。这里大量的活动可能表明系统正处于内存压力下，也可能表明 lruvec->lru_lock 上存在争用。

## 4. Per-CPU Allocator Activity

```

  mm_page_alloc_zone_locked	page=%p pfn=%lu order=%u migratetype=%d cpu=%d percpu_refill=%d
  mm_page_pcpu_drain		page=%p pfn=%lu order=%d cpu=%d migratetype=%d

```
在页面分配器前面是一个 per-cpu 页面分配器。它仅用于 order-0 页面，可以减少 zone->lock 上的争用，并减少在 struct page 上的写入量。

当 per-CPU 列表为空或分配了错误类型的页面时，会获取一次 zone->lock 并重新填充 per-CPU 列表。对每个分配的页面都会触发 mm_page_alloc_zone_locked 事件，该事件会指示它是否用于 percpu_refill。

当 per-CPU 列表过满时，会释放一定数量的页面，每个页面都会触发一个 mm_page_pcpu_drain 事件。

这些事件的个体性质是为了能够在分配和释放之间跟踪页面。连续发生的一批 drain 或 refill 意味着获取了一次 zone->lock。大量的 per-CPU refill 和 drain 可能意味着 CPU 之间的负载不均衡，即过多的工作集中在一个地方。它也可能表明 per-CPU 列表应当更大的尺寸。最后，大量在一个 CPU 上 refill 而在另一个 CPU 上 drain，可能是导致大量因 CPU 之间写入而产生的缓存行弹跳（cache line bounce）的一个因素，值得调查是否可以通过某种算法变更让页面在同一个 CPU 上分配和释放。

## 5. External Fragmentation

```

  mm_page_alloc_extfrag		page=%p pfn=%lu alloc_order=%d fallback_order=%d pageblock_order=%d alloc_migratetype=%d fallback_migratetype=%d fragmenting=%d change_ownership=%d

```
外部碎片会影响高阶分配是否会成功。对于某些类型的硬件，这很重要，不过在可能的情况下会尽量避免。如果系统正在使用大页（huge page），并且需要在系统生命周期内能够调整池的大小，那么这个值很重要。

该事件的大量出现意味着内存正在碎片化，高阶分配将在未来的某个时刻开始失败。减少该事件发生的其中一种方法是，按 3**pageblock_size**nr_online_nodes 的增量增大 min_free_kbytes，其中 pageblock_size 通常是默认大页大小。
