
:Original: Documentation/core-api/index.rst

:翻译:

 司延腾 Yanteng Si <siyanteng@loongson.cn>


## 核心API文档


这是核心内核API手册的首页。 非常感谢为本手册转换(和编写!)的文档!

## 核心实用程序


本节包含通用的和“核心中的核心”文档。 第一部分是 docbook 时期遗留下
来的大量 kerneldoc 信息；有朝一日，若有人有动力的话，应当把它们拆分
出来。

- [kernel-api](kernel-api)
- [printk-basics](printk-basics)
- [printk-formats](printk-formats)
- [workqueue](workqueue)
- [watch_queue](watch_queue)
- [symbol-namespaces](symbol-namespaces)

## 数据结构和低级实用程序


在整个内核中使用的函数库。

- [kobject](kobject)
- [kref](kref)
- [assoc_array](assoc_array)
- [xarray](xarray)
- [rbtree](rbtree)
- [idr](idr)
- [circular-buffers](circular-buffers)
- [generic-radix-tree](generic-radix-tree)
- [packing](packing)
- [this_cpu_ops](this_cpu_ops)
- [union_find](union_find)

=======

Todolist:

   timekeeping
   errseq

## 并发原语


Linux如何让一切同时发生。 详情请参阅
[/locking/index](/locking/index)

- [irq/index](irq/index)
- [refcount-vs-atomic](refcount-vs-atomic)
- [local_ops](local_ops)
- [padata](padata)

Todolist:

   ../RCU/index

## 低级硬件管理


缓存管理，CPU热插拔管理等。

- [cachetlb](cachetlb)
- [cpu_hotplug](cpu_hotplug)
- [genericirq](genericirq)
- [memory-hotplug](memory-hotplug)
- [protection-keys](protection-keys)

Todolist:


   memory-hotplug
   cpu_hotplug
   genericirq


## 内存管理


如何在内核中分配和使用内存。请注意，在
[/mm/index](/mm/index) 中有更多的内存管理文档。

- [memory-allocation](memory-allocation)
- [unaligned-memory-access](unaligned-memory-access)
- [mm-api](mm-api)
- [genalloc](genalloc)
- [boot-time-mm](boot-time-mm)
- [gfp_mask-from-fs-io](gfp_mask-from-fs-io)

Todolist:

   dma-api
   dma-api-howto
   dma-attributes
   dma-isa-lpc
   pin_user_pages

## 内核调试的接口


Todolist:

   debug-objects
   tracepoint
   debugging-via-ohci1394

## 其它文档


不适合放在其它地方或尚未归类的文件；

Todolist:

   librs


#    Indices


   - genindex
