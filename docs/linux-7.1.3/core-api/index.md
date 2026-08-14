## 核心 API 文档

本页是核心内核 API 文档手册的索引，汇总了核心工具、数据结构与底层库，以及内存管理、锁与并发等通用内核机制各章节的入口，便于开发者按主题查阅内核核心 API。


这是核心内核 API 手册的开篇。非常欢迎为这本手册进行文档转换（和撰写！）工作！

## 核心工具


本节包含通用以及“核心中的核心”文档。前者是 docbook 时代遗留的大量 kerneldoc 信息大杂烩；如果哪天有人有精力，确实应该把它拆分整理。

- [kernel-api](kernel-api)
- [workqueue](workqueue)
- [watch_queue](watch_queue)
- [printk-basics](printk-basics)
- [printk-formats](printk-formats)
- [printk-index](printk-index)
- [symbol-namespaces](symbol-namespaces)
- [asm-annotations](asm-annotations)
- [real-time/index](real-time/index)
- [housekeeping.rst](housekeeping.rst)

## 数据结构与底层工具


内核各处都会用到的库功能。

- [kobject](kobject)
- [kref](kref)
- [cleanup](cleanup)
- [assoc_array](assoc_array)
- [folio_queue](folio_queue)
- [xarray](xarray)
- [maple_tree](maple_tree)
- [idr](idr)
- [circular-buffers](circular-buffers)
- [rbtree](rbtree)
- [generic-radix-tree](generic-radix-tree)
- [packing](packing)
- [this_cpu_ops](this_cpu_ops)
- [timekeeping](timekeeping)
- [errseq](errseq)
- [wrappers/atomic_t](wrappers/atomic_t)
- [wrappers/atomic_bitops](wrappers/atomic_bitops)
- [floating-point](floating-point)
- [union_find](union_find)
- [min_heap](min_heap)
- [parser](parser)
- [list](list)

## 底层进入与退出


- [entry](entry)

## 并发原语


Linux 如何避免所有事情同时发生。更多相关文档请参阅 Documentation/locking/index.rst。

- [refcount-vs-atomic](refcount-vs-atomic)
- [irq/index](irq/index)
- [local_ops](local_ops)
- [padata](padata)
- [../RCU/index](../RCU/index)
- [wrappers/memory-barriers.rst](wrappers/memory-barriers.rst)

## 底层硬件管理


缓存管理、CPU 热插拔管理等。

- [cachetlb](cachetlb)
- [cpu_hotplug](cpu_hotplug)
- [memory-hotplug](memory-hotplug)
- [genericirq](genericirq)
- [protection-keys](protection-keys)

## 内存管理


如何在内核中分配和使用内存。注意，在 Documentation/mm/index.rst 中有大量更多关于内存管理的文档。

- [memory-allocation](memory-allocation)
- [unaligned-memory-access](unaligned-memory-access)
- [dma-api](dma-api)
- [dma-api-howto](dma-api-howto)
- [dma-attributes](dma-attributes)
- [dma-isa-lpc](dma-isa-lpc)
- [swiotlb](swiotlb)
- [mm-api](mm-api)
- [cgroup](cgroup)
- [genalloc](genalloc)
- [pin_user_pages](pin_user_pages)
- [boot-time-mm](boot-time-mm)
- [gfp_mask-from-fs-io](gfp_mask-from-fs-io)
- [kho/index](kho/index)

## 内核调试接口


- [debug-objects](debug-objects)
- [tracepoint](tracepoint)
- [debugging-via-ohci1394](debugging-via-ohci1394)

## 其他


不适合放在其他地方、或尚未分类的文档。

- [librs](librs)
- [liveupdate](liveupdate)
- [netlink](netlink)
